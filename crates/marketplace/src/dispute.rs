//! Dispute service: open, list, and resolve order disputes.

use sqlx::PgPool;
use uuid::Uuid;

use common::errors::AppError;

/// Dispute lifecycle helper over the `disputes` table.
pub struct DisputeService {
    pool: PgPool,
}

impl DisputeService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Open a dispute for an order and return its id.
    pub async fn open(
        &self,
        order_id: Uuid,
        user_id: Uuid,
        reason: &str,
    ) -> Result<Uuid, AppError> {
        let row: (Uuid,) = sqlx::query_as(
            "INSERT INTO disputes (order_id, user_id, reason, status)
             VALUES ($1, $2, $3, 'open')
             RETURNING id",
        )
        .bind(order_id)
        .bind(user_id)
        .bind(reason)
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::Database)?;
        Ok(row.0)
    }

    /// Resolve an open dispute with an outcome and an optional resolution note.
    pub async fn resolve(
        &self,
        dispute_id: Uuid,
        status: &str,
        resolution: Option<&str>,
    ) -> Result<(), AppError> {
        sqlx::query(
            "UPDATE disputes
             SET status = $2, resolution = $3, resolved_at = now()
             WHERE id = $1",
        )
        .bind(dispute_id)
        .bind(status)
        .bind(resolution)
        .execute(&self.pool)
        .await
        .map_err(AppError::Database)?;
        Ok(())
    }
}