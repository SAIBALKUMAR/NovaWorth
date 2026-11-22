//! Seasonal league leaderboards.
//!
//! Seasons are 12-week windows; XP accumulated since season start is the ranking
//! score. For the scaffold, ranking reads the live XP wallet (a close proxy that
//! avoids a dedicated scoring table); production can snapshot per-season totals.

use chrono::{DateTime, Utc};
use sqlx::PgPool;

use common::constants::LEAGUE_SEASON_WEEKS;
use common::errors::AppError;

/// Compute the current 12-week season number from a timestamp.
pub fn season_number(now: DateTime<Utc>) -> i64 {
    let weeks = now.timestamp() / (60 * 60 * 24 * 7);
    weeks / LEAGUE_SEASON_WEEKS as i64
}

/// A leaderboard row.
#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct LeaderboardRow {
    pub username: String,
    pub xp_balance: i64,
}

/// Top-`limit` learners by XP.
pub async fn leaderboard(pool: &PgPool, limit: i64) -> Result<Vec<LeaderboardRow>, AppError> {
    sqlx::query_as::<_, LeaderboardRow>(
        "SELECT u.username, w.xp_balance
         FROM wallets w
         JOIN users u ON u.id = w.user_id
         ORDER BY w.xp_balance DESC
         LIMIT $1",
    )
    .bind(limit)
    .fetch_all(pool)
    .await
    .map_err(AppError::Database)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn season_advances_every_twelve_weeks() {
        let base = Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap();
        let later = base + chrono::Duration::weeks(LEAGUE_SEASON_WEEKS as i64);
        assert_eq!(season_number(later), season_number(base) + 1);
    }
}