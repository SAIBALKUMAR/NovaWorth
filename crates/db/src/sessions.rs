//! Session store queries for tower-sessions.

use sqlx::PgPool;
use common::errors::AppError;

pub async fn cleanup_expired(pool: &PgPool) -> Result<u64, AppError> {
    let result = sqlx::query("DELETE FROM sessions WHERE expiry_date < now()")
        .execute(pool)
        .await
        .map_err(AppError::Database)?;
    Ok(result.rows_affected())
}
