//! Gate saga: the transactional flow that takes a learner from "begin gate" to
//! "award XP/EC and unlock the next level".
//!
//! A saga here is a stateful, compensating sequence of steps. It reuses the pure
//! rules in [`learning::gate`] for thresholds/cooldowns and [`economy`] for scoring
//! and ledger mutation.

use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use common::errors::AppError;
use common::types::{Currency, EntryType};
use economy::ledger::{self, LedgerEntryInput};
use economy::{ec, xp};
use learning::gate::{self, GateState};

/// Orchestrates a single level-gate lifecycle against a Postgres pool.
pub struct GateSaga {
    pool: PgPool,
}

impl GateSaga {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Begin a gate attempt. Idempotent-ish: returns `Locked` if mastery is too low,
    /// `Failed { next_retry_at }` if still cooling down, else `InProgress`.
    pub async fn begin_attempt(
        &self,
        user_id: Uuid,
        level_id: Uuid,
    ) -> Result<GateState, AppError> {
        let mastery = self.current_mastery(user_id, level_id).await?;
        if !gate::mastery_prerequisite_met(mastery) {
            return Ok(GateState::Locked {
                reason: gate::locked_reason(mastery),
            });
        }

        if let Some((attempt, next_retry_at)) = self.cooldown(user_id, level_id).await? {
            if next_retry_at > Utc::now() {
                return Ok(GateState::Failed {
                    attempt,
                    next_retry_at,
                });
            }
        }

        // Idempotency: return the existing in-progress attempt rather than opening a
        // second one (double-click / duplicate request).
        if let Some(started_at) = self.pending_started_at(user_id, level_id).await? {
            return Ok(GateState::InProgress {
                attempt: self.attempt_count(user_id, level_id).await?,
                started_at,
            });
        }

        let attempt = self.increment_attempt(user_id, level_id).await?;
        Ok(GateState::InProgress {
            attempt,
            started_at: Utc::now(),
        })
    }

    /// Complete a gate attempt with a normalized score (0.0–1.0).
    ///
    /// On a first pass this records the result, awards XP/EC (with repeat decay),
    /// and unlocks the level. Re-submission of an already-completed attempt is
    /// idempotent: it returns the current state without re-awarding.
    pub async fn complete_attempt(
        &self,
        user_id: Uuid,
        level_id: Uuid,
        score: f64,
    ) -> Result<GateState, AppError> {
        // Reject non-finite or out-of-range scores (protects award math).
        if !score.is_finite() || !(0.0..=1.0).contains(&score) {
            return Err(AppError::Validation(
                "Gate score must be between 0 and 1".into(),
            ));
        }

        // Idempotency: only a pending attempt may be completed. A duplicate call
        // reconstructs the current state without awarding again.
        if !self.pending_attempt_exists(user_id, level_id).await? {
            return self.reconstruct_state(user_id, level_id, score).await;
        }

        let passed = score >= common::constants::GATE_PASS_SCORE;
        // Count prior passes *before* recording this one, for repeat decay.
        let prior_passes = if passed {
            self.prior_pass_count(user_id, level_id).await?
        } else {
            0
        };

        let Some(attempt) = self.record_result(user_id, level_id, score).await? else {
            // A concurrent completion consumed the pending row; treat as a retry.
            return self.reconstruct_state(user_id, level_id, score).await;
        };
        let state = gate::evaluate(score, attempt, Utc::now());

        if passed {
            self.award_and_unlock(user_id, level_id, score, prior_passes).await?;
        }
        Ok(state)
    }

    /// Award XP/EC and unlock. EC is gated by repeat decay and the daily velocity
    /// cap, and requires a funded sponsor pool (else XP-only mode).
    async fn award_and_unlock(
        &self,
        user_id: Uuid,
        level_id: Uuid,
        score: f64,
        prior_passes: usize,
    ) -> Result<(), AppError> {
        let (xp_amt, mut ec_amt) = self.compute_awards(level_id, score, prior_passes).await?;

        // XP is always awarded (non-spendable, permanent).
        ledger::post_transaction(
            &self.pool,
            &[LedgerEntryInput {
                user_id,
                entry_type: EntryType::Credit,
                currency: Currency::XP,
                amount: xp_amt,
                reason: "GATE_PASS".into(),
                reference_id: Some(level_id),
                funding_pool_id: None,
            }],
        )
        .await?;

        // EC emission: apply the daily velocity cap so a single day of grinding
        // can't exceed the fairness limit (XP and level progress are unaffected).
        if ec_amt > 0 {
            let earned_today = self.ec_earned_last_24h(user_id).await?;
            let remaining = (common::constants::EC_DAILY_VELOCITY_CAP - earned_today).max(0);
            ec_amt = ec_amt.min(remaining);

            if ec_amt > 0 {
                if let Some(pool_id) = self.eligible_pool(ec_amt).await? {
                    self.mint_ec(user_id, level_id, ec_amt, pool_id).await?;
                } else {
                    tracing::warn!(
                        user_id = %user_id,
                        level_id = %level_id,
                        "No funded sponsor pool; skipping EC emission (XP-only mode)"
                    );
                }
            } else {
                tracing::info!(
                    user_id = %user_id,
                    "EC daily velocity cap reached; awarding XP only"
                );
            }
        }

        self.unlock(user_id, level_id).await?;
        Ok(())
    }

    /// Map a normalized gate score to the canonical quality multiplier band.
    fn quality_for(score: f64) -> f64 {
        if score >= 0.98 {
            1.25
        } else if score >= 0.95 {
            1.15
        } else if score >= 0.80 {
            1.0
        } else {
            0.8
        }
    }

    /// Compute `(xp, ec)` awards from the level's number, domain multiplier, and
    /// the repeat tier (prior passes).
    async fn compute_awards(
        &self,
        level_id: Uuid,
        score: f64,
        prior_passes: usize,
    ) -> Result<(i64, i64), AppError> {
        let row: Option<(i32, f64)> = sqlx::query_as(
            "SELECT lv.level, d.multiplier::float8
             FROM levels lv
             JOIN skills s  ON s.id = lv.skill_id
             JOIN domains d ON d.code = s.domain_code
             WHERE lv.id = $1",
        )
        .bind(level_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::Database)?;

        let (level, mult) = row.unwrap_or((1, 1.0));
        let quality = Self::quality_for(score);
        let level = level as u32;

        // First pass earns the full XP; repeats earn 10% (mastery is permanent but
        // not farmable).
        let xp_amt = if prior_passes == 0 {
            xp::xp_award(level, mult, quality, 1.0)
        } else {
            xp::xp_repeat_award(level, mult, quality, 1.0)
        };

        let ec_amt = ec::ec_award(
            level,
            mult,
            quality,
            1.0,
            1.0,
            ec::RepeatDecay::from_index(prior_passes),
            false,
        );

        Ok((xp_amt, ec_amt))
    }

    /// Find an active pool with enough reserved EC to back an emission.
    async fn eligible_pool(&self, ec_amt: i64) -> Result<Option<Uuid>, AppError> {
        let row: Option<(Uuid,)> = sqlx::query_as(
            "SELECT id FROM sponsor_pools
             WHERE status = 'active' AND reserved_ec >= $1
             ORDER BY created_at LIMIT 1",
        )
        .bind(ec_amt)
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::Database)?;
        Ok(row.map(|r| r.0))
    }

    /// Mint EC to the learner and record the emission against the pool.
    ///
    /// NOTE: in a hardened deployment the pool claim and ledger post share one
    /// transaction; for the scaffold eligibility gates emission and the ledger post
    /// is independently validated.
    async fn mint_ec(
        &self,
        user_id: Uuid,
        level_id: Uuid,
        ec_amt: i64,
        pool_id: Uuid,
    ) -> Result<(), AppError> {
        let claimed = sqlx::query(
            "UPDATE sponsor_pools
             SET emitted_ec = emitted_ec + $2, reserved_ec = reserved_ec - $2
             WHERE id = $1 AND reserved_ec >= $2",
        )
        .bind(pool_id)
        .bind(ec_amt)
        .execute(&self.pool)
        .await
        .map_err(AppError::Database)?;

        // Refuse to mint when the pool is under-reserved. The CHECK in 0009 is the
        // hard DB invariant against races; this is the friendly application gate.
        if claimed.rows_affected() == 0 {
            return Err(AppError::Validation(
                "Sponsor pool has insufficient reserved EC".into(),
            ));
        }

        ledger::post_transaction(
            &self.pool,
            &[LedgerEntryInput {
                user_id,
                entry_type: EntryType::Credit,
                currency: Currency::EC,
                amount: ec_amt,
                reason: "GATE_PASS".into(),
                reference_id: Some(level_id),
                funding_pool_id: Some(pool_id),
            }],
        )
        .await
    }

    /// Mark the level as passed and advance the learner's highest level for that
    /// skill (kept monotonically increasing).
    async fn unlock(&self, user_id: Uuid, level_id: Uuid) -> Result<(), AppError> {
        sqlx::query(
            "INSERT INTO learner_progress (user_id, level_id, status, mastery, gate_passed_at)
             VALUES ($1, $2, 'passed', 1.0, now())
             ON CONFLICT (user_id, level_id)
             DO UPDATE SET status = 'passed', gate_passed_at = now(), updated_at = now()",
        )
        .bind(user_id)
        .bind(level_id)
        .execute(&self.pool)
        .await
        .map_err(AppError::Database)?;

        sqlx::query(
            "INSERT INTO learner_skills (user_id, skill_id, current_level, status)
             SELECT $1, lv.skill_id, lv.level, 'active'
             FROM levels lv WHERE lv.id = $2
             ON CONFLICT (user_id, skill_id)
             DO UPDATE SET
                current_level = GREATEST(learner_skills.current_level, EXCLUDED.current_level),
                updated_at = now()",
        )
        .bind(user_id)
        .bind(level_id)
        .execute(&self.pool)
        .await
        .map_err(AppError::Database)?;
        Ok(())
    }

    // ── Progress / attempt helpers ─────────────────────────────────────────────

    async fn current_mastery(&self, user_id: Uuid, level_id: Uuid) -> Result<f64, AppError> {
        let row: Option<(f64,)> = sqlx::query_as(
            "SELECT mastery::float8 FROM learner_progress WHERE user_id = $1 AND level_id = $2",
        )
        .bind(user_id)
        .bind(level_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::Database)?;
        Ok(row.map(|r| r.0).unwrap_or(0.0))
    }

    async fn cooldown(
        &self,
        user_id: Uuid,
        level_id: Uuid,
    ) -> Result<Option<(u32, chrono::DateTime<Utc>)>, AppError> {
        let attempts = self.attempt_count(user_id, level_id).await?;
        if attempts == 0 {
            return Ok(None);
        }
        let row: Option<(String, chrono::DateTime<Utc>)> = sqlx::query_as(
            "SELECT status, submitted_at FROM attempts
             WHERE user_id = $1 AND level_id = $2
             ORDER BY submitted_at DESC NULLS LAST LIMIT 1",
        )
        .bind(user_id)
        .bind(level_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::Database)?;

        if let Some((status, at)) = row {
            if status == "failed" {
                let retry = at + chrono::Duration::hours(gate::cooldown_hours(attempts));
                return Ok(Some((attempts, retry)));
            }
        }
        Ok(None)
    }

    async fn attempt_count(&self, user_id: Uuid, level_id: Uuid) -> Result<u32, AppError> {
        let row: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM attempts WHERE user_id = $1 AND level_id = $2",
        )
        .bind(user_id)
        .bind(level_id)
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::Database)?;
        Ok(row.0 as u32)
    }

    async fn increment_attempt(&self, user_id: Uuid, level_id: Uuid) -> Result<u32, AppError> {
        sqlx::query(
            "INSERT INTO attempts (user_id, level_id, kind, status)
             VALUES ($1, $2, 'gate', 'pending')",
        )
        .bind(user_id)
        .bind(level_id)
        .execute(&self.pool)
        .await
        .map_err(AppError::Database)?;
        self.attempt_count(user_id, level_id).await
    }

    async fn record_result(&self, user_id: Uuid, level_id: Uuid, score: f64) -> Result<Option<u32>, AppError> {
        let passed = score >= common::constants::GATE_PASS_SCORE;
        let status = if passed { "passed" } else { "failed" };
        let result = sqlx::query(
            "UPDATE attempts
             SET score = $3, status = $4, integrity_score = CASE WHEN EXTRACT(EPOCH FROM (now() - started_at)) < $5 THEN 0.4 ELSE 1.0 END, submitted_at = now()
             WHERE id = (
                SELECT id FROM attempts
                WHERE user_id = $1 AND level_id = $2 AND status = 'pending'
                ORDER BY started_at DESC LIMIT 1
             )",
        )
        .bind(user_id)
        .bind(level_id)
        .bind(score)
        .bind(status).bind(common::constants::MIN_GATE_DURATION_SECS)
        .execute(&self.pool)
        .await
        .map_err(AppError::Database)?;

        // No pending row was updated → a concurrent call already completed it.
        if result.rows_affected() == 0 {
            return Ok(None);
        }
        Ok(Some(self.attempt_count(user_id, level_id).await?))
    }

    // ── Edge-case helpers ──────────────────────────────────────────────────────

    /// Whether an in-flight (`pending`) attempt exists for this user/level.
    async fn pending_attempt_exists(&self, user_id: Uuid, level_id: Uuid) -> Result<bool, AppError> {
        let row: (bool,) = sqlx::query_as(
            "SELECT EXISTS(
                SELECT 1 FROM attempts
                WHERE user_id = $1 AND level_id = $2 AND status = 'pending'
             )",
        )
        .bind(user_id)
        .bind(level_id)
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::Database)?;
        Ok(row.0)
    }

    /// Number of times this level has already been passed (drives repeat decay).
    async fn prior_pass_count(&self, user_id: Uuid, level_id: Uuid) -> Result<usize, AppError> {
        let row: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM attempts
             WHERE user_id = $1 AND level_id = $2 AND status = 'passed'",
        )
        .bind(user_id)
        .bind(level_id)
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::Database)?;
        Ok(row.0 as usize)
    }

    /// Sum of EC credited in the last 24h (daily velocity limit).
    async fn ec_earned_last_24h(&self, user_id: Uuid) -> Result<i64, AppError> {
        let row: (i64,) = sqlx::query_as(
            "SELECT COALESCE(SUM(amount), 0) FROM ledger_entries
             WHERE user_id = $1 AND currency = 'EC' AND entry_type = 'CREDIT'
               AND created_at > now() - interval '24 hours'",
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::Database)?;
        Ok(row.0)
    }

    /// Reconstruct the current gate state when there is nothing pending (idempotent
    /// re-submission path).
    async fn reconstruct_state(
        &self,
        user_id: Uuid,
        level_id: Uuid,
        score: f64,
    ) -> Result<GateState, AppError> {
        let row: Option<(String,)> = sqlx::query_as(
            "SELECT status FROM learner_progress WHERE user_id = $1 AND level_id = $2",
        )
        .bind(user_id)
        .bind(level_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::Database)?;

        if matches!(row.as_ref().map(|r| r.0.as_str()), Some("passed")) {
            return Ok(GateState::Passed { score });
        }
        if let Some((attempt, next_retry_at)) = self.cooldown(user_id, level_id).await? {
            return Ok(GateState::Failed { attempt, next_retry_at });
        }
        Ok(GateState::NotStarted)
    }

    /// Timestamp of the current pending attempt, if one exists.
    async fn pending_started_at(
        &self,
        user_id: Uuid,
        level_id: Uuid,
    ) -> Result<Option<chrono::DateTime<Utc>>, AppError> {
        let row: Option<(chrono::DateTime<Utc>,)> = sqlx::query_as(
            "SELECT started_at FROM attempts
             WHERE user_id = $1 AND level_id = $2 AND status = 'pending'
             ORDER BY started_at DESC LIMIT 1",
        )
        .bind(user_id)
        .bind(level_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::Database)?;
        Ok(row.map(|r| r.0))
    }
}