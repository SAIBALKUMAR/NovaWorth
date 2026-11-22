//! Per-user session tracking so a user can see and revoke active sessions
//! ("sign out everywhere").

use sqlx::PgPool;
use uuid::Uuid;

use common::errors::AppError;

pub struct SessionRecord<'a> {
    pub session_id: &'a str,
    pub user_id: Uuid,
    pub ip_hash: &'a str,
    pub user_agent: &'a str,
}

/// Record a session so it can be listed/revoked later.
pub async fn record(pool: &PgPool, s: &SessionRecord<'_>) -> Result<(), AppError> {
    sqlx::query(
        "INSERT INTO user_sessions (session_id, user_id, ip_hash, user_agent)
         VALUES ($1, $2, $3, $4)
         ON CONFLICT (session_id) DO NOTHING",
    )
    .bind(s.session_id)
    .bind(s.user_id)
    .bind(s.ip_hash)
    .bind(s.user_agent)
    .execute(pool)
    .await
    .map_err(AppError::Database)?;
    Ok(())
}

/// Revoke every session for a user: delete both the tracking rows and the
/// underlying opaque `sessions` store rows (tower-sessions).
pub async fn revoke_all(pool: &PgPool, user_id: Uuid) -> Result<u64, AppError> {
    let ids: Vec<(String,)> = sqlx::query_as("SELECT session_id FROM user_sessions WHERE user_id = $1")
        .bind(user_id)
        .fetch_all(pool)
        .await
        .map_err(AppError::Database)?;

    let mut n = 0u64;
    for (sid,) in &ids {
        let a = sqlx::query("DELETE FROM user_sessions WHERE session_id = $1")
            .bind(sid.as_str())
            .execute(pool)
            .await
            .map_err(AppError::Database)?;
        let b = sqlx::query("DELETE FROM sessions WHERE id = $1")
            .bind(sid.as_str())
            .execute(pool)
            .await
            .map_err(AppError::Database)?;
        n += a.rows_affected() + b.rows_affected();
    }
    Ok(n)
}