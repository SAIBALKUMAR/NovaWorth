//! In-app notification feed, persisted in Postgres.

use sqlx::PgPool;
use uuid::Uuid;

use common::errors::AppError;

/// A persisted notification.
#[derive(Debug, Clone, serde::Serialize, sqlx::FromRow)]
pub struct NotificationRow {
    pub id: Uuid,
    pub kind: String,
    pub title: String,
    pub body: Option<String>,
    pub read: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Insert a notification and return its id.
pub async fn create_notification(
    pool: &PgPool,
    user_id: Uuid,
    kind: &str,
    title: &str,
    body: Option<&str>,
) -> Result<Uuid, AppError> {
    let row: (Uuid,) = sqlx::query_as(
        "INSERT INTO notifications (user_id, kind, title, body)
         VALUES ($1, $2, $3, $4)
         RETURNING id",
    )
    .bind(user_id)
    .bind(kind)
    .bind(title)
    .bind(body)
    .fetch_one(pool)
    .await
    .map_err(AppError::Database)?;
    Ok(row.0)
}

/// List a user's notifications, newest first. `unread_only` filters to unread.
pub async fn list_notifications(
    pool: &PgPool,
    user_id: Uuid,
    unread_only: bool,
    limit: i64,
) -> Result<Vec<NotificationRow>, AppError> {
    let rows = if unread_only {
        sqlx::query_as::<_, NotificationRow>(
            "SELECT id, kind, title, body, read, created_at
             FROM notifications
             WHERE user_id = $1 AND read = FALSE
             ORDER BY created_at DESC LIMIT $2",
        )
    } else {
        sqlx::query_as::<_, NotificationRow>(
            "SELECT id, kind, title, body, read, created_at
             FROM notifications
             WHERE user_id = $1
             ORDER BY created_at DESC LIMIT $2",
        )
    }
    .bind(user_id)
    .bind(limit)
    .fetch_all(pool)
    .await
    .map_err(AppError::Database)?;

    Ok(rows)
}

/// Mark a specific notification read.
pub async fn mark_read(pool: &PgPool, notification_id: Uuid) -> Result<(), AppError> {
    sqlx::query("UPDATE notifications SET read = TRUE WHERE id = $1")
        .bind(notification_id)
        .execute(pool)
        .await
        .map_err(AppError::Database)?;
    Ok(())
}

/// Count unread notifications.
pub async fn unread_count(pool: &PgPool, user_id: Uuid) -> Result<i64, AppError> {
    let row: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM notifications WHERE user_id = $1 AND read = FALSE",
    )
    .bind(user_id)
    .fetch_one(pool)
    .await
    .map_err(AppError::Database)?;
    Ok(row.0)
}