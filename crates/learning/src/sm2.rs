//! SM-2 spaced-repetition scheduler.
//!
//! Schedules the next review of a mastery item using the classic SuperMemo-2
//! algorithm: quality grades 0–5 update ease and interval.

use chrono::{DateTime, Duration, Utc};
use common::constants::{SM2_INITIAL_EF, SM2_MIN_EF};

/// Learner's spacing state for one item.
#[derive(Debug, Clone, Copy)]
pub struct Sm2State {
    pub easiness: f64,
    pub interval_days: u32,
    pub repetitions: u32,
}

impl Default for Sm2State {
    fn default() -> Self {
        Self {
            easiness: SM2_INITIAL_EF,
            interval_days: 0,
            repetitions: 0,
        }
    }
}

/// Review an item with a quality grade (0–5, ≥3 = recalled).
pub fn review(state: &Sm2State, quality: u32) -> Sm2State {
    let q = quality.min(5);

    if q < 3 {
        // Failed recall: reset repetitions, keep a short interval.
        return Sm2State {
            easiness: state.easiness.max(SM2_MIN_EF),
            interval_days: 1,
            repetitions: 0,
        };
    }

    let repetitions = state.repetitions + 1;
    let interval_days = match repetitions {
        1 => 1,
        2 => 6,
        _ => (state.interval_days as f64 * state.easiness).round() as u32,
    };

    // Easiness factor update: EF' = EF + (0.1 - (5-q)*(0.08 + (5-q)*0.02))
    let ef = state.easiness
        + 0.1
        - (5.0 - q as f64) * (0.08 + (5.0 - q as f64) * 0.02);

    Sm2State {
        easiness: ef.max(SM2_MIN_EF),
        interval_days,
        repetitions,
    }
}

/// Absolute timestamp for the next review.
pub fn next_review_at(state: &Sm2State, now: DateTime<Utc>) -> DateTime<Utc> {
    now + Duration::days(state.interval_days as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn successful_review_grows_interval() {
        let s = Sm2State::default();
        let s1 = review(&s, 5);
        assert_eq!(s1.repetitions, 1);
        assert_eq!(s1.interval_days, 1);

        let s2 = review(&s1, 5);
        assert_eq!(s2.interval_days, 6);

        let s3 = review(&s2, 5);
        assert!(s3.interval_days > 6);
    }

    #[test]
    fn failed_review_resets_repetitions() {
        let s = Sm2State { interval_days: 30, repetitions: 4, easiness: 2.5 };
        let r = review(&s, 1);
        assert_eq!(r.repetitions, 0);
        assert_eq!(r.interval_days, 1);
    }

    #[test]
    fn easiness_never_below_min() {
        let s = Sm2State { interval_days: 10, repetitions: 2, easiness: SM2_MIN_EF };
        let r = review(&s, 0);
        assert!(r.easiness >= SM2_MIN_EF);
    }
}