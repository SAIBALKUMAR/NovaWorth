//! Password reset tokens.

use argon2::password_hash::rand_core::{OsRng, RngCore};
use sqlx::PgPool;
use uuid::Uuid;

use common::errors::AppError;

const RESET_TTL_HOURS: i32 = 1;

fn random_token() -> String {
    let mut bytes = [0u8; 32];
    OsRng.fill_bytes(&mut bytes);
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// Issue a one-hour reset token and return its plaintext value.
pub async fn issue_token(pool: &PgPool, user_id: Uuid) -> Result<String, AppError> {
    let token = random_token();
    sqlx::query(
        "INSERT INTO password_reset_tokens (token, user_id, expires_at)
         VALUES ($1, $2, now() + make_interval(hours => $3))",
    )
    .bind(&token)
    .bind(user_id)
    .bind(RESET_TTL_HOURS)
    .execute(pool)
    .await
    .map_err(AppError::Database)?;
    Ok(token)
}

/// Consume a reset token (single-use, expiring) and return the user id.
pub async fn consume_token(pool: &PgPool, token: &str) -> Result<Uuid, AppError> {
    let row: Option<(Uuid,)> = sqlx::query_as(
        "DELETE FROM password_reset_tokens WHERE token = $1 AND expires_at > now()
         RETURNING user_id",
    )
    .bind(token)
    .fetch_optional(pool)
    .await
    .map_err(AppError::Database)?;

    row.map(|(id,)| id)
        .ok_or_else(|| AppError::Validation("Invalid or expired reset token".into()))
}

/// Set a new password for a user.
pub async fn reset_password(
    pool: &PgPool,
    user_id: Uuid,
    new_password: &str,
) -> Result<(), AppError> {
    if new_password.len() < common::constants::MIN_PASSWORD_LEN {
        return Err(AppError::Validation("Password too short".into()));
    }
    let hash = crate::password::hash_password(new_password)?;
    sqlx::query("UPDATE users SET password_hash = $2, updated_at = now() WHERE id = $1")
        .bind(user_id)
        .bind(hash)
        .execute(pool)
        .await
        .map_err(AppError::Database)?;
    Ok(())
}