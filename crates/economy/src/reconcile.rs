//! Daily wallet-vs-ledger reconciliation.
//!
//! `wallets` is a fast cache; `ledger_entries` is the immutable source of truth. Any
//! divergence between the two signals either a bug or tampering and must be surfaced.

use sqlx::PgPool;
use uuid::Uuid;

use common::errors::AppError;

/// Reconstructed balance for one currency, and whether it matches the ledger.
#[derive(Debug, Clone)]
pub struct CurrencyReconciliation {
    pub currency: String,
    pub wallet_balance: i64,
    pub ledger_balance: i64,
}

impl CurrencyReconciliation {
    pub fn diff(&self) -> i64 {
        self.wallet_balance - self.ledger_balance
    }

    pub fn is_balanced(&self) -> bool {
        self.diff() == 0
    }
}

/// Reconcile a single user's XP and EC balances against the ledger.
pub async fn reconcile_user(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Vec<CurrencyReconciliation>, AppError> {
    sqlx::query("INSERT INTO wallets (user_id) VALUES ($1) ON CONFLICT (user_id) DO NOTHING")
        .bind(user_id)
        .execute(pool)
        .await
        .map_err(AppError::Database)?;

    let (xp_wallet, ec_wallet): (i64, i64) = sqlx::query_as(
        "SELECT xp_balance, ec_balance FROM wallets WHERE user_id = $1",
    )
    .bind(user_id)
    .fetch_one(pool)
    .await
    .map_err(AppError::Database)?;

    let rows: Vec<(String, i64)> = sqlx::query_as(
        "SELECT currency,
                COALESCE(SUM(CASE WHEN entry_type = 'CREDIT' THEN amount ELSE -amount END), 0)
         FROM ledger_entries
         WHERE user_id = $1
         GROUP BY currency",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
    .map_err(AppError::Database)?;

    let mut xp_ledger = 0i64;
    let mut ec_ledger = 0i64;
    for (currency, balance) in &rows {
        match currency.as_str() {
            "XP" => xp_ledger = *balance,
            "EC" => ec_ledger = *balance,
            _ => {}
        }
    }

    Ok(vec![
        CurrencyReconciliation {
            currency: "XP".into(),
            wallet_balance: xp_wallet,
            ledger_balance: xp_ledger,
        },
        CurrencyReconciliation {
            currency: "EC".into(),
            wallet_balance: ec_wallet,
            ledger_balance: ec_ledger,
        },
    ])
}

/// Reconcile every user. Returns `(user_id, is_balanced)` for monitoring.
pub async fn reconcile_all(pool: &PgPool) -> Result<Vec<(Uuid, bool)>, AppError> {
    let user_ids: Vec<Uuid> = sqlx::query_scalar("SELECT id FROM users")
        .fetch_all(pool)
        .await
        .map_err(AppError::Database)?;

    let mut report = Vec::with_capacity(user_ids.len());
    for uid in user_ids {
        let rows = reconcile_user(pool, uid).await?;
        report.push((uid, rows.iter().all(CurrencyReconciliation::is_balanced)));
    }
    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reconciliation_diff_math() {
        let r = CurrencyReconciliation {
            currency: "EC".into(),
            wallet_balance: 100,
            ledger_balance: 90,
        };
        assert_eq!(r.diff(), 10);
        assert!(!r.is_balanced());
    }
}