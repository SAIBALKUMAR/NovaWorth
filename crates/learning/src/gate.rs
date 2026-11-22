//! Level-gate state machine (pure).
//!
//! Encodes the gate promotion rules: pass threshold, exponential retry cooldown,
//! and readiness lock. The stateful, transactional orchestration lives in the
//! `assessment` crate's [`gate_saga`]; this module holds the shared rules.

use chrono::{DateTime, Duration, Utc};
use common::constants::{
    GATE_COOLDOWN_ATTEMPT_1_H, GATE_COOLDOWN_ATTEMPT_2_H, GATE_COOLDOWN_ATTEMPT_3_PLUS_H,
    GATE_MASTERY_THRESHOLD, GATE_PASS_SCORE,
};

/// Lifecycle states of a level-gate attempt.
#[derive(Debug, Clone, PartialEq)]
pub enum GateState {
    NotStarted,
    InProgress { attempt: u32, started_at: DateTime<Utc> },
    Passed { score: f64 },
    Failed { attempt: u32, next_retry_at: DateTime<Utc> },
    Locked { reason: String },
}

/// Retry cooldown in hours for a given attempt number (1-based).
pub fn cooldown_hours(attempt: u32) -> i64 {
    match attempt {
        1 => GATE_COOLDOWN_ATTEMPT_1_H,
        2 => GATE_COOLDOWN_ATTEMPT_2_H,
        _ => GATE_COOLDOWN_ATTEMPT_3_PLUS_H,
    }
}

/// Whether `mastery` satisfies the readiness requirement to sit the gate.
pub fn mastery_prerequisite_met(mastery: f64) -> bool {
    mastery >= GATE_MASTERY_THRESHOLD
}

/// Build the `Locked` reason for insufficient mastery (human-readable).
pub fn locked_reason(mastery: f64) -> String {
    format!(
        "Mastery {:.0}% < {:.0}% required",
        mastery * 100.0,
        GATE_MASTERY_THRESHOLD * 100.0
    )
}

/// Decide the next gate state given a final normalized score (0.0–1.0).
pub fn evaluate(score: f64, attempt: u32, now: DateTime<Utc>) -> GateState {
    if score >= GATE_PASS_SCORE {
        GateState::Passed { score }
    } else {
        GateState::Failed {
            attempt,
            next_retry_at: now + Duration::hours(cooldown_hours(attempt)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pass_threshold() {
        let now = Utc::now();
        assert!(matches!(evaluate(0.75, 1, now), GateState::Passed { .. }));
        assert!(matches!(evaluate(0.74, 1, now), GateState::Failed { .. }));
    }

    #[test]
    fn cooldown_schedule() {
        assert_eq!(cooldown_hours(1), 1);
        assert_eq!(cooldown_hours(2), 4);
        assert_eq!(cooldown_hours(3), 24);
        assert_eq!(cooldown_hours(99), 24);
    }

    #[test]
    fn mastery_prerequisite() {
        assert!(mastery_prerequisite_met(0.80));
        assert!(!mastery_prerequisite_met(0.79));
    }
}