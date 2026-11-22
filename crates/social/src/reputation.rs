//! Decaying reputation score.
//!
//! Reputation is the weighted, time-decayed sum of trust events (peer review,
//! mentoring, integrity, bounties). Older events contribute less.

use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use common::errors::AppError;

/// Default half-life for reputation decay, in days.
pub const REPUTATION_HALF_LIFE_DAYS: f64 = 30.0;

/// Decay a weight by age using exponential half-life decay.
pub fn decay_weight(weight: f64, age_days: f64, half_life_days: f64) -> f64 {
    if half_life_days <= 0.0 {
        return weight;
    }
    weight * 0.5_f64.powf(age_days.max(0.0) / half_life_days)
}

/// A user's current reputation score.
pub async fn reputation_score(pool: &PgPool, user_id: Uuid) -> Result<f64, AppError> {
    let rows: Vec<(f64, chrono::DateTime<Utc>)> = sqlx::query_as(
        "SELECT weight::float8, created_at FROM reputation_events WHERE user_id = $1",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
    .map_err(AppError::Database)?;

    let now = Utc::now();
    let score = rows
        .iter()
        .map(|(weight, at)| {
            let age_days = (now - *at).num_seconds() as f64 / 86_400.0;
            decay_weight(*weight, age_days, REPUTATION_HALF_LIFE_DAYS)
        })
        .sum();

    Ok(score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_half_life_halves_weight() {
        let w = decay_weight(10.0, REPUTATION_HALF_LIFE_DAYS, REPUTATION_HALF_LIFE_DAYS);
        assert!((w - 5.0).abs() < 1e-9);
    }

    #[test]
    fn zero_half_life_preserves_weight() {
        assert_eq!(decay_weight(10.0, 100.0, 0.0), 10.0);
    }

    #[test]
    fn negative_age_is_clamped() {
        assert_eq!(decay_weight(10.0, -5.0, 30.0), 10.0);
    }
}