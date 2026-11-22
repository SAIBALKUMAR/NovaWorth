//! Security routes: email verification, password reset, 2FA, session revocation.

use std::sync::Arc;
use axum::{extract::State, routing::post, Json, Router};
use serde::Deserialize;
use auth::backend::AuthSession;
use common::errors::AppError;
use crate::state::AppState;

pub fn router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/auth/verify-email", post(verify_email))
        .route("/auth/forgot-password", post(forgot_password))
        .route("/auth/reset-password", post(reset_password))
        .route("/auth/2fa/setup", post(two_fa_setup))
        .route("/auth/2fa/verify", post(two_fa_verify))
        .route("/auth/sessions/revoke", post(revoke_sessions))
        .with_state(state)
}

#[derive(Deserialize)]
struct VerifyEmail { token: String }

async fn verify_email(
    State(state): State<Arc<AppState>>,
    Json(b): Json<VerifyEmail>,
) -> Result<Json<serde_json::Value>, AppError> {
    let user_id = auth::verification::verify_token(&state.pool, &b.token).await?;
    Ok(Json(serde_json::json!({ "verified": user_id.to_string() })))
}

#[derive(Deserialize)]
struct ForgotPassword { email: String }

async fn forgot_password(
    State(state): State<Arc<AppState>>,
    Json(b): Json<ForgotPassword>,
) -> Result<Json<serde_json::Value>, AppError> {
    // Anti-enumeration: always return success, even for unknown emails.
    if let Some(user) = db::users::find_by_email(&state.pool, &b.email).await? {
        let _token = auth::reset::issue_token(&state.pool, user.id).await?;
        tracing::info!(user_id = %user.id, "Password reset token issued (deliver via email in production)");
    }
    Ok(Json(serde_json::json!({ "ok": true })))
}

#[derive(Deserialize)]
struct ResetPassword { token: String, new_password: String }

async fn reset_password(
    State(state): State<Arc<AppState>>,
    Json(b): Json<ResetPassword>,
) -> Result<Json<serde_json::Value>, AppError> {
    let user_id = auth::reset::consume_token(&state.pool, &b.token).await?;
    auth::reset::reset_password(&state.pool, user_id, &b.new_password).await?;
    auth::sessions::revoke_all(&state.pool, user_id).await?; // invalidate other sessions
    Ok(Json(serde_json::json!({ "ok": true })))
}

async fn two_fa_setup(
    State(state): State<Arc<AppState>>,
    auth: AuthSession,
) -> Result<Json<serde_json::Value>, AppError> {
    let user = auth.user.ok_or(AppError::SessionExpired)?;
    let secret = auth::mfa::setup(&state.pool, user.id).await?;
    Ok(Json(serde_json::json!({ "secret": secret })))
}

#[derive(Deserialize)]
struct TwoFaVerify { code: String }

async fn two_fa_verify(
    State(state): State<Arc<AppState>>,
    auth: AuthSession,
    Json(b): Json<TwoFaVerify>,
) -> Result<Json<serde_json::Value>, AppError> {
    let user = auth.user.ok_or(AppError::SessionExpired)?;
    let now = chrono::Utc::now().timestamp() as u64;
    auth::mfa::confirm_code(&state.pool, user.id, &b.code, now).await?;
    Ok(Json(serde_json::json!({ "ok": true })))
}

async fn revoke_sessions(
    State(state): State<Arc<AppState>>,
    auth: AuthSession,
) -> Result<Json<serde_json::Value>, AppError> {
    let user = auth.user.ok_or(AppError::SessionExpired)?;
    let n = auth::sessions::revoke_all(&state.pool, user.id).await?;
    Ok(Json(serde_json::json!({ "revoked": n })))
}