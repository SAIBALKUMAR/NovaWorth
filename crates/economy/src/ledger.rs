//! Append-only ledger engine.
//!
//! Every state change is expressed as a set of entries. Wallets are updated within
//! the same transaction as the entries they describe, and each entry embeds the hash
//! of its predecessor so tampering is detectable. Counterparty balancing (the second
//! side of double-entry) lives in sponsor pools for mints and escrow holds for
//! spending, and is checked by the nightly reconciliation worker.

use std::collections::HashMap;

use sqlx::PgPool;
use uuid::Uuid;

use common::errors::AppError;
use common::types::{Currency, EntryType};

use crate::hash_chain::entry_hash;

/// A single leg of a ledger transaction.
#[derive(Debug, Clone)]
pub struct LedgerEntryInput {
    pub user_id: Uuid,
    pub entry_type: EntryType,
    pub currency: Currency,
    /// Positive amount, in whole EC/XP units.
    pub amount: i64,
    pub reason: String,
    pub reference_id: Option<Uuid>,
    pub funding_pool_id: Option<Uuid>,
}

fn entry_type_str(t: EntryType) -> &'static str {
    match t {
        EntryType::Credit => "CREDIT",
        EntryType::Debit => "DEBIT",
    }
}

/// Ensure a wallet row exists for the user (idempotent).
async fn ensure_wallet(pool: &PgPool, user_id: Uuid) -> Result<(), AppError> {
    sqlx::query("INSERT INTO wallets (user_id) VALUES ($1) ON CONFLICT (user_id) DO NOTHING")
        .bind(user_id)
        .execute(pool)
        .await
        .map_err(AppError::Database)?;
    Ok(())
}

/// Read the cached balance for a user + currency (XP or EC).
pub async fn get_balance(
    pool: &PgPool,
    user_id: Uuid,
    currency: Currency,
) -> Result<i64, AppError> {
    ensure_wallet(pool, user_id).await?;
    let row: (i64, i64) = sqlx::query_as(
        "SELECT xp_balance, ec_balance FROM wallets WHERE user_id = $1",
    )
    .bind(user_id)
    .fetch_one(pool)
    .await
    .map_err(AppError::Database)?;

    Ok(match currency {
        Currency::XP => row.0,
        Currency::EC => row.1,
    })
}

/// Latest hash for a user's ledger chain, if any entries exist.
async fn last_hash(
    pool: &PgPool,
    user_id: Uuid,
    currency: Currency,
) -> Result<Option<String>, AppError> {
    let row: Option<(String,)> = sqlx::query_as(
        "SELECT entry_hash FROM ledger_entries
         WHERE user_id = $1 AND currency = $2
         ORDER BY created_at DESC, id DESC LIMIT 1",
    )
    .bind(user_id)
    .bind(currency.to_string())
    .fetch_optional(pool)
    .await
    .map_err(AppError::Database)?;

    Ok(row.map(|r| r.0))
}

/// Apply a set of ledger legs atomically.
///
/// * Credits mint value; debits spend it. Each leg updates the wallet cache.
/// * A negative resulting balance is rejected (`InsufficientFunds`).
/// * XP is mastery (non-fungible) and may move single-sided.
/// * Counterparty balancing (the second side of double-entry) is enforced at the
///   application layer — sponsor pools for mints, escrow holds for spending.
pub async fn post_transaction(
    pool: &PgPool,
    inputs: &[LedgerEntryInput],
) -> Result<(), AppError> {
    if inputs.is_empty() {
        return Ok(());
    }

    // ── Guardrails: reject invalid legs before touching the database ──────────
    for input in inputs {
        if input.amount <= 0 {
            return Err(AppError::Validation(
                "Ledger amounts must be positive".into(),
            ));
        }
        if input.currency == Currency::XP && input.entry_type == EntryType::Debit {
            return Err(AppError::Validation(
                "XP is non-spendable and cannot be debited".into(),
            ));
        }
    }

    let mut tx = pool.begin().await.map_err(AppError::Database)?;

    // Per-account working state threaded through a single transaction.
    let mut balances: HashMap<(Uuid, String), i64> = HashMap::new();
    let mut prev_hashes: HashMap<(Uuid, String), Option<String>> = HashMap::new();

    for input in inputs {
        let key = (input.user_id, input.currency.to_string());

        // 1. Load (and row-lock) the account balance.
        let balance: i64 = if let Some(&b) = balances.get(&key) {
            b
        } else {
            sqlx::query(
                "INSERT INTO wallets (user_id) VALUES ($1) ON CONFLICT (user_id) DO NOTHING",
            )
            .bind(input.user_id)
            .execute(&mut *tx)
            .await
            .map_err(AppError::Database)?;

            let row: (i64, i64) = sqlx::query_as(
                "SELECT xp_balance, ec_balance FROM wallets WHERE user_id = $1 FOR UPDATE",
            )
            .bind(input.user_id)
            .fetch_one(&mut *tx)
            .await
            .map_err(AppError::Database)?;

            let b = match input.currency {
                Currency::XP => row.0,
                Currency::EC => row.1,
            };
            balances.insert(key.clone(), b);
            b
        };

        // 2. Load (and thread) the previous chain hash.
        let prev_hash: Option<String> = if let Some(h) = prev_hashes.get(&key) {
            h.clone()
        } else {
            let h = last_hash(&mut *tx, input.user_id, input.currency).await?;
            prev_hashes.insert(key.clone(), h.clone());
            h
        };

        // 3. Apply the direction and guard against negative balance.
        let new_balance = match input.entry_type {
            EntryType::Credit => balance.checked_add(input.amount),
            EntryType::Debit => balance.checked_sub(input.amount),
        }
        .ok_or_else(|| AppError::Validation("Ledger balance overflow".into()))?;

        if new_balance < 0 {
            return Err(AppError::InsufficientFunds {
                need: input.amount,
                have: balance,
                currency: input.currency.to_string(),
            });
        }

        // 4. Compute the tamper-evident entry hash.
        let hash = entry_hash(
            prev_hash.as_deref(),
            &input.user_id.to_string(),
            &input.currency.to_string(),
            input.amount,
            &input.reason,
        );

        // 5. Persist the entry.
        sqlx::query(
            "INSERT INTO ledger_entries
                (user_id, entry_type, currency, amount, balance_after, reason,
                 reference_id, funding_pool_id, prev_hash, entry_hash)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)",
        )
        .bind(input.user_id)
        .bind(entry_type_str(input.entry_type))
        .bind(input.currency.to_string())
        .bind(input.amount)
        .bind(new_balance)
        .bind(&input.reason)
        .bind(input.reference_id)
        .bind(input.funding_pool_id)
        .bind(&prev_hash)
        .bind(&hash)
        .execute(&mut *tx)
        .await
        .map_err(AppError::Database)?;

        balances.insert(key.clone(), new_balance);
        prev_hashes.insert(key, Some(hash));
    }

    // 6. Flush the authoritative wallet cache within the same transaction.
    for ((user_id, currency), balance) in &balances {
        match currency.as_str() {
            "XP" => sqlx::query(
                "UPDATE wallets SET xp_balance = $2, updated_at = now() WHERE user_id = $1",
            )
            .bind(user_id)
            .bind(balance)
            .execute(&mut *tx)
            .await
            .map_err(AppError::Database)?,
            "EC" => sqlx::query(
                "UPDATE wallets SET ec_balance = $2, updated_at = now() WHERE user_id = $1",
            )
            .bind(user_id)
            .bind(balance)
            .execute(&mut *tx)
            .await
            .map_err(AppError::Database)?,
            _ => {}
        }
    }

    tx.commit().await.map_err(AppError::Database)?;
    Ok(())
}