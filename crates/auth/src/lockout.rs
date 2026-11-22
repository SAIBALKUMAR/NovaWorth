//! Brute-force login lockout.
//!
//! Tracks failed login attempts per email and locks the account for a short window
//! after `MAX_FAILED_LOGIN_ATTEMPTS`. A successful login clears the counter.

use chrono::{DateTime, Utc};
use sqlx::PgPool;

use common::constants::{LOCKOUT_DURATION_MINS, MAX_FAILED_LOGIN_ATTEMPTS};
use common::errors::AppError;

/// Return the timestamp until which the account is locked, if currently locked.
pub async fn is_locked(pool: &PgPool, email: &str) -> Result<Option<DateTime<Utc>>, AppError> {
    let row: Option<(DateTime<Utc>,)> =
        sqlx::query_as("SELECT locked_until FROM login_attempts WHERE email = $1")
            .bind(email)
            .fetch_optional(pool)
            .await
            .map_err(AppError::Database)?;

    Ok(row.and_then(|(until,)| if until > Utc::now() { Some(until) } else { None }))
}

/// Record a failed login attempt, locking the account past the threshold.
pub async fn record_failure(pool: &PgPool, email: &str) -> Result<(), AppError> {
    sqlx::query(
        "INSERT INTO login_attempts (email, failed_count, locked_until)
         VALUES ($1, 1, CASE WHEN 1 >= $2 THEN now() + make_interval(mins => $3) ELSE NULL END)
         ON CONFLICT (email) DO UPDATE SET
           failed_count = login_attempts.failed_count + 1,
           locked_until = CASE
             WHEN login_attempts.failed_count + 1 >= $2
             THEN now() + make_interval(mins => $3)
             ELSE login_attempts.locked_until
           END,
           updated_at = now()",
    )
    .bind(email)
    .bind(MAX_FAILED_LOGIN_ATTEMPTS as i32)
    .bind(LOCKOUT_DURATION_MINS as i32)
    .execute(pool)
    .await
    .map_err(AppError::Database)?;
    Ok(())
}

/// Clear the failure counter after a successful login.
pub async fn clear_failures(pool: &PgPool, email: &str) -> Result<(), AppError> {
    sqlx::query("DELETE FROM login_attempts WHERE email = $1")
        .bind(email)
        .execute(pool)
        .await
        .map_err(AppError::Database)?;
    Ok(())
}