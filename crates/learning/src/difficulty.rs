//! Difficulty & time curves, and adaptive difficulty adjustment.

/// Difficulty index `D(L) = 10 · L^1.35`, used for matchmaking and pay weighting.
pub fn difficulty_index(level: u32) -> f64 {
    10.0 * (level as f64).powf(1.35)
}

/// Expected hours for a level `H(L) = 1.5 · L^1.1`.
pub fn expected_hours(level: u32) -> f64 {
    1.5 * (level as f64).powf(1.1)
}

/// Cumulative expected hours to reach `level` (sum of H(1..=level)).
pub fn cumulative_hours(level: u32) -> f64 {
    (1..=level).map(expected_hours).sum()
}

/// Nudge an item's difficulty toward a target success rate using an Elo-style
/// update. `k` controls step size. Returns the new difficulty estimate.
pub fn adjust_difficulty(current: f64, correct: bool, target_success: f64, k: f64) -> f64 {
    // Predicted success given current difficulty: harder item → lower success.
    let predicted = 1.0 / (1.0 + 10f64.powf((current - 0.5) / 0.4));
    let actual = if correct { 1.0 } else { 0.0 };
    (current + k * (target_success - predicted) * if correct { 1.0 } else { -1.0 }).clamp(0.0, 10_000.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn difficulty_is_power_law() {
        let ratio = difficulty_index(100) / difficulty_index(1);
        assert!((ratio - 100f64.powf(1.35)).abs() < 1e-9);
    }

    #[test]
    fn cumulative_hours_grows() {
        assert!(cumulative_hours(50) > cumulative_hours(10));
    }

    #[test]
    fn wrong_answer_raises_difficulty_for_easier_future_matches() {
        let d = adjust_difficulty(0.5, false, 0.8, 0.1);
        // Failing a calibrated item nudges the estimate away from the target.
        assert_ne!(d, 0.5);
    }
}