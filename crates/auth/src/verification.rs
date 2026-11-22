//! Email verification tokens.
//!
//! A short-lived bearer token is issued on registration (or on resend) and
//! consumed exactly once to mark `users.email_verified_at`. The token is stored
//! in plaintext in `email_verification_tokens` — acceptable only because it is a
//! short-lived, single-purpose secret; protect the table as sensitive if that policy changes.

use sqlx::PgPool;
use uuid::Uuid;

use common::errors::AppError;

/// Token lifetime in hours.
const TOKEN_TTL_HOURS: i32 = 24;

/// Generate a random token, store it, and return the plaintext value.
/// The caller is responsible for delivering it out-of-band (email).
pub async fn issue_token(pool: &PgPool, user_id: Uuid) -> Result<String, AppError> {
    let token = format!("{}{}", Uuid::new_v4().simple(), Uuid::new_v4().simple());

    sqlx::query(
        "INSERT INTO email_verification_tokens (token, user_id, expires_at)
         VALUES ($1, $2, now() + make_interval(hours => $3))",
    )
    .bind(&token)
    .bind(user_id)
    .bind(TOKEN_TTL_HOURS)
    .execute(pool)
    .await
    .map_err(AppError::Database)?;

    Ok(token)
}

/// Consume a token: delete it if valid & unexpired, mark the user verified, and
/// return the user id. Consuming is atomic (the DELETE ... RETURNING is the gate).
pub async fn verify_token(pool: &PgPool, token: &str) -> Result<Uuid, AppError> {
    let row: Option<(Uuid,)> = sqlx::query_as(
        "DELETE FROM email_verification_tokens
         WHERE token = $1 AND expires_at > now()
         RETURNING user_id",
    )
    .bind(token)
    .fetch_optional(pool)
    .await
    .map_err(AppError::Database)?;

    let user_id = row
        .map(|(id,)| id)
        .ok_or_else(|| AppError::Validation("Invalid or expired verification token".into()))?;

    sqlx::query("UPDATE users SET email_verified_at = now(), updated_at = now() WHERE id = $1")
        .bind(user_id)
        .execute(pool)
        .await
        .map_err(AppError::Database)?;

    Ok(user_id)
}