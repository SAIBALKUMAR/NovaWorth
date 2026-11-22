//! Escrow service: lock a learner's EC on order placement, then release on
//! fulfillment or refund on cancellation/dispute.

use sqlx::PgPool;
use uuid::Uuid;

use common::errors::AppError;
use common::types::{Currency, EntryType};
use economy::ledger::{self, LedgerEntryInput};

/// Manages the escrow lifecycle backed by `escrow_holds` + the EC ledger.
pub struct EscrowService {
    pool: PgPool,
}

impl EscrowService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Debit the learner's EC and hold it against a pending order.
    ///
    /// Idempotent: a repeated hold on an already-escrowed order is a no-op. If the
    /// escrow record insert fails after the debit, the debit is compensated so the
    /// learner never loses EC.
    pub async fn hold(
        &self,
        user_id: Uuid,
        order_id: Uuid,
        amount: i64,
    ) -> Result<(), AppError> {
        if amount <= 0 {
            return Err(AppError::Validation(
                "Escrow amount must be positive".into(),
            ));
        }

        let order_status = self.order_status(order_id).await?;
        if order_status.as_deref() == Some("escrowed") {
            return Ok(()); // Already held; treat as success.
        }
        if order_status.as_deref() != Some("pending") {
            return Err(AppError::Validation(format!(
                "Cannot escrow an order in status '{}'",
                order_status.as_deref().unwrap_or("unknown")
            )));
        }

        // 1. Spend the EC (single-sided debit; ledger enforces non-negative balance).
        ledger::post_transaction(
            &self.pool,
            &[LedgerEntryInput {
                user_id,
                entry_type: EntryType::Debit,
                currency: Currency::EC,
                amount,
                reason: "ESCROW_HOLD".into(),
                reference_id: Some(order_id),
                funding_pool_id: None,
            }],
        )
        .await?;

        // 2. Record the held funds, compensating if the write fails.
        let hold_result = sqlx::query(
            "INSERT INTO escrow_holds (order_id, user_id, amount, status)
             VALUES ($1, $2, $3, 'held')",
        )
        .bind(order_id)
        .bind(user_id)
        .bind(amount)
        .execute(&self.pool)
        .await;

        if let Err(db_err) = hold_result {
            let _ = self.compensate(user_id, order_id, amount).await;
            return Err(AppError::Database(db_err));
        }

        // 3. Advance the order to escrowed.
        sqlx::query("UPDATE orders SET status = 'escrowed', updated_at = now() WHERE id = $1")
            .bind(order_id)
            .execute(&self.pool)
            .await
            .map_err(AppError::Database)?;

        Ok(())
    }

    /// Release escrow on confirmed fulfillment (only from the 'held' state).
    pub async fn release(&self, order_id: Uuid) -> Result<(), AppError> {
        let status = self.hold_status(order_id).await?;
        if status.as_deref() != Some("held") {
            return Err(AppError::Validation(format!(
                "Escrow is not held (status '{}'); cannot release",
                status.as_deref().unwrap_or("missing")
            )));
        }

        sqlx::query(
            "UPDATE escrow_holds SET status = 'released', released_at = now()
             WHERE order_id = $1 AND status = 'held'",
        )
        .bind(order_id)
        .execute(&self.pool)
        .await
        .map_err(AppError::Database)?;

        sqlx::query("UPDATE orders SET status = 'fulfilled', updated_at = now() WHERE id = $1")
            .bind(order_id)
            .execute(&self.pool)
            .await
            .map_err(AppError::Database)?;

        Ok(())
    }

    /// Refund held EC back to the learner (only from the 'held' state).
    pub async fn refund(
        &self,
        order_id: Uuid,
        user_id: Uuid,
        amount: i64,
    ) -> Result<(), AppError> {
        let status = self.hold_status(order_id).await?;
        if status.as_deref() != Some("held") {
            return Err(AppError::Validation(format!(
                "Escrow is not held (status '{}'); cannot refund",
                status.as_deref().unwrap_or("missing")
            )));
        }

        // 1. Credit the EC back.
        ledger::post_transaction(
            &self.pool,
            &[LedgerEntryInput {
                user_id,
                entry_type: EntryType::Credit,
                currency: Currency::EC,
                amount,
                reason: "ESCROW_REFUND".into(),
                reference_id: Some(order_id),
                funding_pool_id: None,
            }],
        )
        .await?;

        // 2. Mark the hold refunded and the order refunded.
        sqlx::query("UPDATE escrow_holds SET status = 'refunded' WHERE order_id = $1")
            .bind(order_id)
            .execute(&self.pool)
            .await
            .map_err(AppError::Database)?;

        sqlx::query("UPDATE orders SET status = 'refunded', updated_at = now() WHERE id = $1")
            .bind(order_id)
            .execute(&self.pool)
            .await
            .map_err(AppError::Database)?;

        Ok(())
    }

    // ── State helpers ──────────────────────────────────────────────────────────

    async fn order_status(&self, order_id: Uuid) -> Result<Option<String>, AppError> {
        let row: Option<(String,)> = sqlx::query_as("SELECT status FROM orders WHERE id = $1")
            .bind(order_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(AppError::Database)?;
        Ok(row.map(|r| r.0))
    }

    async fn hold_status(&self, order_id: Uuid) -> Result<Option<String>, AppError> {
        let row: Option<(String,)> =
            sqlx::query_as("SELECT status FROM escrow_holds WHERE order_id = $1")
                .bind(order_id)
                .fetch_optional(&self.pool)
                .await
                .map_err(AppError::Database)?;
        Ok(row.map(|r| r.0))
    }

    /// Compensating credit used when the escrow record fails after a debit.
    async fn compensate(&self, user_id: Uuid, order_id: Uuid, amount: i64) -> Result<(), AppError> {
        ledger::post_transaction(
            &self.pool,
            &[LedgerEntryInput {
                user_id,
                entry_type: EntryType::Credit,
                currency: Currency::EC,
                amount,
                reason: "ESCROW_HOLD_COMPENSATION".into(),
                reference_id: Some(order_id),
                funding_pool_id: None,
            }],
        )
        .await
    }
}