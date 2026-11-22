//! Fixed-window rate limiting (DB-backed). Redis/fred is the production path.

use sqlx::PgPool;

use common::errors::AppError;

/// Enforce a fixed-window limit for `scope:key`.
pub async fn check(
    pool: &PgPool,
    scope: &str,
    key: &str,
    max: i32,
    window_secs: i64,
) -> Result<(), AppError> {
    let full = format!("{scope}:{key}");
    let now = chrono::Utc::now();

    let row: Option<(i32, chrono::DateTime<chrono::Utc>)> = sqlx::query_as(
        "SELECT count, window_start FROM rate_limits WHERE key = $1",
    )
    .bind(&full)
    .fetch_optional(pool)
    .await
    .map_err(AppError::Database)?;

    let expired = row
        .as_ref()
        .map(|(_, start)| (now - *start).num_seconds() >= window_secs)
        .unwrap_or(true);

    if expired {
        sqlx::query(
            "INSERT INTO rate_limits (key, window_start, count) VALUES ($1, now(), 1)
             ON CONFLICT (key) DO UPDATE SET count = 1, window_start = now()",
        )
        .bind(&full)
        .execute(pool)
        .await
        .map_err(AppError::Database)?;
        return Ok(());
    }

    let count = row.map(|(c, _)| c).unwrap_or(0);
    if count >= max {
        return Err(AppError::RateLimited);
    }

    sqlx::query("UPDATE rate_limits SET count = count + 1 WHERE key = $1")
        .bind(&full)
        .execute(pool)
        .await
        .map_err(AppError::Database)?;
    Ok(())
}