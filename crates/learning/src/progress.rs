//! Progress & streak tracking.

use chrono::NaiveDate;

/// Compute the current contiguous streak ending at `today` (inclusive).
pub fn current_streak(days: &[NaiveDate], today: NaiveDate) -> u32 {
    let mut streak = 0u32;
    let mut cursor = today;
    loop {
        if days.contains(&cursor) {
            streak += 1;
            cursor = cursor - chrono::Duration::days(1);
        } else {
            break;
        }
    }
    streak
}

/// Longest contiguous streak anywhere in the (assumed sorted) date list.
pub fn longest_streak(days: &[NaiveDate]) -> u32 {
    if days.is_empty() {
        return 0;
    }
    let mut best = 1u32;
    let mut run = 1u32;
    for pair in days.windows(2) {
        if pair[1].signed_duration_since(pair[0]).num_days() == 1 {
            run += 1;
            best = best.max(run);
        } else {
            run = 1;
        }
    }
    best
}

/// Simple completion ratio: `done / total`, always in `[0, 1]`.
pub fn completion_ratio(done: u32, total: u32) -> f64 {
    if total == 0 {
        0.0
    } else {
        (done as f64 / total as f64).clamp(0.0, 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn d(y: i32, m: u32, day: u32) -> NaiveDate {
        chrono::Utc.with_ymd_and_hms(y, m, day, 0, 0, 0).unwrap().date_naive()
    }

    #[test]
    fn current_streak_counts_back() {
        let today = d(2026, 1, 5);
        let days = vec![d(2026, 1, 3), d(2026, 1, 4), d(2026, 1, 5)];
        assert_eq!(current_streak(&days, today), 3);
    }

    #[test]
    fn broken_today_resets_streak() {
        let today = d(2026, 1, 5);
        let days = vec![d(2026, 1, 3), d(2026, 1, 4)];
        assert_eq!(current_streak(&days, today), 0);
    }

    #[test]
    fn longest_streak_finds_chain() {
        let days = vec![d(2026, 1, 1), d(2026, 1, 2), d(2026, 1, 3), d(2026, 1, 5)];
        assert_eq!(longest_streak(&days), 3);
    }

    #[test]
    fn completion_is_bounded() {
        assert_eq!(completion_ratio(5, 10), 0.5);
        assert_eq!(completion_ratio(0, 0), 0.0);
        assert_eq!(completion_ratio(20, 10), 1.0);
    }
}