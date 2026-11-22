//! Brute-force login lockout, scoped to (email, ip) so an attacker cannot lock a
//! victim's account from their own IP while the victim logs in from elsewhere.

use chrono::{DateTime, Utc};
use sqlx::PgPool;

use common::constants::{LOCKOUT_DURATION_MINS, MAX_FAILED_LOGIN_ATTEMPTS};
use common::errors::AppError;

/// Stable, non-cryptographic hash of a client IP for grouping (djb2).
pub fn hash_ip(ip: &str) -> String {
    let mut h = 5381u64;
    for b in ip.bytes() {
        h = h.wrapping_mul(33).wrapping_add(b as u64);
    }
    format!("{h:x}")
}

/// Return the timestamp until which the account is locked, if currently locked.
pub async fn is_locked(
    pool: &PgPool,
    email: &str,
    ip_hash: &str,
) -> Result<Option<DateTime<Utc>>, AppError> {
    let row: Option<(DateTime<Utc>,)> = sqlx::query_as(
        "SELECT locked_until FROM login_attempts WHERE email = $1 AND ip_hash = $2",
    )
    .bind(email)
    .bind(ip_hash)
    .fetch_optional(pool)
    .await
    .map_err(AppError::Database)?;

    Ok(row.and_then(|(until,)| if until > Utc::now() { Some(until) } else { None }))
}

/// Record a failed login attempt, locking the account past the threshold.
pub async fn record_failure(pool: &PgPool, email: &str, ip_hash: &str) -> Result<(), AppError> {
    sqlx::query(
        "INSERT INTO login_attempts (email, ip_hash, failed_count, locked_until)
         VALUES ($1, $2, 1, CASE WHEN 1 >= $3 THEN now() + make_interval(mins => $4) ELSE NULL END)
         ON CONFLICT (email, ip_hash) DO UPDATE SET
           failed_count = login_attempts.failed_count + 1,
           locked_until = CASE
             WHEN login_attempts.failed_count + 1 >= $3
             THEN now() + make_interval(mins => $4)
             ELSE login_attempts.locked_until
           END,
           updated_at = now()",
    )
    .bind(email)
    .bind(ip_hash)
    .bind(MAX_FAILED_LOGIN_ATTEMPTS as i32)
    .bind(LOCKOUT_DURATION_MINS as i32)
    .execute(pool)
    .await
    .map_err(AppError::Database)?;
    Ok(())
}

/// Clear the failure counter after a successful login.
pub async fn clear_failures(pool: &PgPool, email: &str, ip_hash: &str) -> Result<(), AppError> {
    sqlx::query("DELETE FROM login_attempts WHERE email = $1 AND ip_hash = $2")
        .bind(email)
        .bind(ip_hash)
        .execute(pool)
        .await
        .map_err(AppError::Database)?;
    Ok(())
}