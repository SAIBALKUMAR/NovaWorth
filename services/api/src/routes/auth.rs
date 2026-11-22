//! Authentication routes:
//! POST /auth/register
//! POST /auth/login
//! POST /auth/logout
//! GET  /auth/me
//! GET  /auth/github          → redirect to GitHub
//! GET  /auth/github/callback → exchange code, upsert user, set session
//! GET  /auth/google          → redirect to Google
//! GET  /auth/google/callback → exchange code, upsert user, set session

use std::sync::Arc;
use axum::{
    extract::{Query, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Redirect},
    routing::{get, post},
    Json, Router,
};
use oauth2::{AuthorizationCode, CsrfToken, Scope, TokenResponse};
use serde::{Deserialize, Serialize};
use validator::Validate;
use common::errors::AppError;
use auth::backend::{AuthenticatedUser, AuthSession, Credentials};
use db::users::{self, CreateUserParams};

use crate::state::AppState;

pub fn router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/auth/register",        post(register))
        .route("/auth/login",           post(login))
        .route("/auth/logout",          post(logout))
        .route("/auth/me",              get(me))
        .route("/auth/github",          get(github_login))
        .route("/auth/github/callback", get(github_callback))
        .route("/auth/google",          get(google_login))
        .route("/auth/google/callback", get(google_callback))
        .with_state(state)
}

// ── Request / Response types ──────────────────────────────────────────────────

#[derive(Debug, Deserialize, Validate)]
struct RegisterRequest {
    #[validate(email)]
    email:    String,
    #[validate(length(min = 3, max = 32))]
    username: String,
    #[validate(length(min = 8, max = 128))]
    password: String,
    captcha:  String,
}

#[derive(Debug, Deserialize)]
struct LoginRequest {
    email:    String,
    password: String,
    captcha:  String,
    totp:     Option<String>,
}

#[derive(Debug, Serialize)]
struct UserResponse {
    id:       uuid::Uuid,
    email:    String,
    username: String,
    role:     String,
}

impl From<&AuthenticatedUser> for UserResponse {
    fn from(u: &AuthenticatedUser) -> Self {
        Self {
            id:       u.id,
            email:    u.email.clone(),
            username: u.username.clone(),
            role:     u.role.clone(),
        }
    }
}

#[derive(Debug, Deserialize)]
struct OAuthCallback {
    code:  String,
    #[allow(dead_code)]
    state: String,
}

// ── Handlers ──────────────────────────────────────────────────────────────────

/// Resolve the client IP from proxy headers (Caddy injects X-Forwarded-For).
fn client_ip(headers: &HeaderMap) -> String {
    headers
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.split(',').next())
        .map(|s| s.trim().to_string())
        .or_else(|| {
            headers
                .get("x-real-ip")
                .and_then(|v| v.to_str().ok())
                .map(|s| s.trim().to_string())
        })
        .unwrap_or_else(|| "unknown".to_string())
}

async fn register(
    State(state): State<Arc<AppState>>,
    mut auth:     AuthSession,
    headers:      HeaderMap,
    Json(body):   Json<RegisterRequest>,
) -> Result<impl IntoResponse, AppError> {
    body.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    let ip_hash = auth::lockout::hash_ip(&client_ip(&headers));
    auth::captcha::verify(&state.http_client, &state.config.turnstile_secret, &body.captcha).await?;
    auth::rate_limit::check(&state.pool, "register", &ip_hash, 10, 3600).await?;

    if users::email_exists(&state.pool, &body.email).await? {
        return Err(AppError::Validation("Email already registered".into()));
    }
    if users::username_exists(&state.pool, &body.username).await? {
        return Err(AppError::Validation("Username already taken".into()));
    }

    let hash = auth::password::hash_password(&body.password)?;
    let user = users::create_user(
        &state.pool,
        CreateUserParams {
            email:         body.email.clone(),
            username:      body.username.clone(),
            password_hash: Some(hash),
        },
    )
    .await?;

    let auth_user = AuthenticatedUser {
        id:            user.id,
        email:         user.email.clone(),
        username:      user.username.clone(),
        password_hash: user.password_hash.clone(),
        role:          user.role.clone(),
    };
    let _token = auth::verification::issue_token(&state.pool, user.id).await?;
    tracing::info!(user_id = %user.id, "Issued email verification token (deliver via email in production)");

    auth.login(&auth_user)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    tracing::info!(user_id = %user.id, "New user registered");
    Ok((StatusCode::CREATED, Json(UserResponse::from(&auth_user))))
}

async fn login(
    State(state): State<Arc<AppState>>,
    mut auth:    AuthSession,
    headers:     HeaderMap,
    Json(body):  Json<LoginRequest>,
) -> Result<Json<UserResponse>, AppError> {
    let ip_hash = auth::lockout::hash_ip(&client_ip(&headers));

    // 0. Bot defense: captcha, then a per-IP rate limit.
    auth::captcha::verify(&state.http_client, &state.config.turnstile_secret, &body.captcha).await?;
    auth::rate_limit::check(&state.pool, "login", &ip_hash, 20, 3600).await?;

    // 1. Lockout check, scoped to (email, ip) so nobody can lock a victim out.
    if let Some(locked_until) = auth::lockout::is_locked(&state.pool, &body.email, &ip_hash).await? {
        let minutes = (locked_until - chrono::Utc::now()).num_minutes().max(1);
        return Err(AppError::AccountLocked {
            minutes: minutes as u64,
        });
    }

    let creds = Credentials {
        email:    body.email.clone(),
        password: body.password.clone(),
    };

    // 2. Authenticate; record a failure on bad creds.
    let user = match auth.authenticate(creds).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            auth::lockout::record_failure(&state.pool, &body.email, &ip_hash).await?;
            return Err(AppError::InvalidCredentials);
        }
        Err(e) => return Err(AppError::Internal(e.to_string())),
    };

    // 3. MFA step-up when TOTP is enabled on the account.
    if let Some(secret) = auth::mfa::enabled_secret(&state.pool, user.id).await? {
        let code = body.totp.as_deref().unwrap_or("");
        let now = chrono::Utc::now().timestamp() as u64;
        if !auth::mfa::verify(&secret, code, now) {
            return Err(AppError::Validation("Invalid 2FA code".into()));
        }
    }

    // 4. Success: clear failures and open a session.
    auth::lockout::clear_failures(&state.pool, &body.email, &ip_hash).await?;
    auth.login(&user)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    tracing::info!(user_id = %user.id, "User logged in");
    Ok(Json(UserResponse::from(&user)))
}

async fn logout(mut auth: AuthSession) -> Result<Json<serde_json::Value>, AppError> {
    auth.logout()
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    Ok(Json(serde_json::json!({ "message": "Logged out successfully" })))
}

async fn me(auth: AuthSession) -> Result<Json<UserResponse>, AppError> {
    let user = auth.user.ok_or(AppError::SessionExpired)?;
    Ok(Json(UserResponse::from(&user)))
}

// ── OAuth: GitHub ─────────────────────────────────────────────────────────────

async fn github_login(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let (url, _csrf) = state
        .github_oauth
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("user:email".to_string()))
        .url();
    Redirect::to(url.as_str())
}

async fn github_callback(
    State(state):    State<Arc<AppState>>,
    mut auth:       AuthSession,
    Query(params):   Query<OAuthCallback>,
) -> Result<impl IntoResponse, AppError> {
    let code = AuthorizationCode::new(params.code.clone());

    // Exchange code for token using reqwest-based async HTTP
    let http = oauth2::reqwest::ClientBuilder::new(reqwest::Client::new())
        .build()
        .map_err(|e| AppError::Internal(e.to_string()))?;

    let token = state
        .github_oauth
        .exchange_code(code)
        .request_async(&http)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    #[derive(Deserialize)]
    struct GithubUser {
        login: String,
        email: Option<String>,
    }

    #[derive(Deserialize)]
    struct GithubEmail {
        email:    String,
        primary:  bool,
        verified: bool,
    }

    let gh_user: GithubUser = state
        .http_client
        .get("https://api.github.com/user")
        .bearer_auth(token.access_token().secret())
        .header("User-Agent", "novaworth/1.0")
        .send()
        .await?
        .json()
        .await?;

    let email = if let Some(e) = gh_user.email {
        e
    } else {
        let emails: Vec<GithubEmail> = state
            .http_client
            .get("https://api.github.com/user/emails")
            .bearer_auth(token.access_token().secret())
            .header("User-Agent", "novaworth/1.0")
            .send()
            .await?
            .json()
            .await?;

        emails
            .into_iter()
            .find(|e| e.primary && e.verified)
            .map(|e| e.email)
            .ok_or_else(|| {
                AppError::Validation(
                    "No verified primary email on GitHub account".into(),
                )
            })?
    };

    let auth_user = upsert_oauth_user(&state.pool, &email, &gh_user.login).await?;
    auth.login(&auth_user)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    Ok(Redirect::to(&state.config.app_url))
}

// ── OAuth: Google ─────────────────────────────────────────────────────────────

async fn google_login(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let (url, _csrf) = state
        .google_oauth
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/userinfo.email".to_string(),
        ))
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/userinfo.profile".to_string(),
        ))
        .url();
    Redirect::to(url.as_str())
}

async fn google_callback(
    State(state):  State<Arc<AppState>>,
    mut auth:     AuthSession,
    Query(params): Query<OAuthCallback>,
) -> Result<impl IntoResponse, AppError> {
    let code = AuthorizationCode::new(params.code.clone());

    let http = oauth2::reqwest::ClientBuilder::new(reqwest::Client::new())
        .build()
        .map_err(|e| AppError::Internal(e.to_string()))?;

    let token = state
        .google_oauth
        .exchange_code(code)
        .request_async(&http)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    #[derive(Deserialize)]
    struct GoogleUser {
        email: String,
        name:  Option<String>,
    }

    let guser: GoogleUser = state
        .http_client
        .get("https://www.googleapis.com/oauth2/v2/userinfo")
        .bearer_auth(token.access_token().secret())
        .send()
        .await?
        .json()
        .await?;

    let username = guser
        .name
        .unwrap_or_else(|| guser.email.split('@').next().unwrap_or("user").to_string());

    let auth_user = upsert_oauth_user(&state.pool, &guser.email, &username).await?;
    auth.login(&auth_user)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    Ok(Redirect::to(&state.config.app_url))
}

// ── Shared OAuth helper ───────────────────────────────────────────────────────

async fn upsert_oauth_user(
    pool:     &sqlx::PgPool,
    email:    &str,
    username: &str,
) -> Result<AuthenticatedUser, AppError> {
    if !users::email_exists(pool, email).await? {
        // Generate a unique username if collision
        let uname = if users::username_exists(pool, username).await? {
            format!("{username}_{}", &uuid::Uuid::new_v4().to_string()[..6])
        } else {
            username.to_string()
        };

        users::create_user(
            pool,
            CreateUserParams {
                email:         email.to_string(),
                username:      uname,
                password_hash: None, // OAuth user — no password
            },
        )
        .await?;
    }

    let row = users::find_by_email(pool, email)
        .await?
        .ok_or_else(|| AppError::Internal("OAuth user lookup failed".into()))?;

    Ok(AuthenticatedUser {
        id:            row.id,
        email:         row.email,
        username:      row.username,
        password_hash: row.password_hash,
        role:          row.role,
    })
}
