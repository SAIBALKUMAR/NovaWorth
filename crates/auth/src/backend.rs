//! axum-login AuthBackend implementation.

use axum_login::{AuthUser, AuthnBackend, UserId};
use sqlx::PgPool;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use async_trait::async_trait;
use common::errors::AppError;
use crate::password::verify_password;
use db::users;

/// The authenticated user stored in the session.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AuthenticatedUser {
    pub id:            Uuid,
    pub email:         String,
    pub username:      String,
    pub password_hash: Option<String>,
    pub role:          String,
}

impl AuthUser for AuthenticatedUser {
    type Id = Uuid;

    fn id(&self) -> Uuid {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.password_hash
            .as_deref()
            .unwrap_or("")
            .as_bytes()
    }
}

/// Credentials submitted during login.
#[derive(Debug, Clone, Deserialize)]
pub struct Credentials {
    pub email:    String,
    pub password: String,
}

/// The axum-login backend wired to our Postgres user store.
#[derive(Clone)]
pub struct AuthBackend {
    pub pool: PgPool,
}

impl AuthBackend {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AuthnBackend for AuthBackend {
    type User        = AuthenticatedUser;
    type Credentials = Credentials;
    type Error       = AppError;

    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        let Some(row) = users::find_by_email(&self.pool, &creds.email).await? else {
            return Ok(None);
        };

        let verified = row
            .password_hash
            .as_ref()
            .map(|h| verify_password(&creds.password, h))
            .unwrap_or(false);

        if verified {
            Ok(Some(AuthenticatedUser {
                id:            row.id,
                email:         row.email,
                username:      row.username,
                password_hash: row.password_hash,
                role:          row.role,
            }))
        } else {
            Ok(None)
        }
    }

    async fn get_user(
        &self,
        id: &UserId<Self>,
    ) -> Result<Option<Self::User>, Self::Error> {
        let row = users::find_by_id(&self.pool, *id).await?;
        Ok(row.map(|r| AuthenticatedUser {
            id:            r.id,
            email:         r.email,
            username:      r.username,
            password_hash: r.password_hash,
            role:          r.role,
        }))
    }
}

/// Convenience type alias used in Axum extractors.
pub type AuthSession = axum_login::AuthSession<AuthBackend>;
