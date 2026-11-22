//! Bayesian Knowledge Tracing (BKT).
//!
//! Models the learner's latent probability of "knowing" a skill and updates it
//! after each observed response. Used to decide when a learner is ready to sit a
//! level gate (`mastery >= 0.80`).

use common::constants::{BKT_GUESS, BKT_PRIOR, BKT_SLIP, BKT_TRANSIT};

/// The four BKT parameters per skill.
#[derive(Debug, Clone, Copy)]
pub struct BktParams {
    /// Prior P(known) before any observation.
    pub p_init: f64,
    /// P(learn | unknown) on each opportunity.
    pub p_transit: f64,
    /// P(wrong | known) — the "slip".
    pub p_slip: f64,
    /// P(right | unknown) — the "guess".
    pub p_guess: f64,
}

impl Default for BktParams {
    fn default() -> Self {
        Self {
            p_init: BKT_PRIOR,
            p_transit: BKT_TRANSIT,
            p_slip: BKT_SLIP,
            p_guess: BKT_GUESS,
        }
    }
}

/// Probability of a correct response under the current belief.
fn p_correct(p_known: f64, params: &BktParams) -> f64 {
    let p = p_known * (1.0 - params.p_slip) + (1.0 - p_known) * params.p_guess;
    p.max(1e-6) // avoid division by zero
}

/// One Bayesian update + forgetting-free transition step.
fn step(p_known: f64, correct: bool, params: &BktParams) -> f64 {
    let p_right = p_correct(p_known, params);
    let posterior = if correct {
        p_known * (1.0 - params.p_slip) / p_right
    } else {
        p_known * params.p_slip / (1.0 - p_right).max(1e-6)
    };
    (posterior + (1.0 - posterior) * params.p_transit).clamp(0.0, 1.0)
}

/// Fold a full sequence of responses into a final mastery posterior.
pub fn update_mastery(prior: f64, responses: &[bool], params: &BktParams) -> f64 {
    responses.iter().fold(prior, |p, &ok| step(p, ok, params))
}

/// Return the full mastery trajectory, including the initial prior.
pub fn mastery_trajectory(prior: f64, responses: &[bool], params: &BktParams) -> Vec<f64> {
    let mut out = Vec::with_capacity(responses.len() + 1);
    out.push(prior);
    let mut p = prior;
    for &ok in responses {
        p = step(p, ok, params);
        out.push(p);
    }
    out
}

/// Whether the learner is ready to advance (mastery at/above threshold).
pub fn is_ready(mastery: f64) -> bool {
    mastery >= common::constants::GATE_MASTERY_THRESHOLD
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mastery_rises_with_correct_answers() {
        let params = BktParams::default();
        let m = update_mastery(BKT_PRIOR, &[true, true, true, true, true], &params);
        assert!(m > BKT_PRIOR, "mastery should rise, got {m}");
        assert!(is_ready(m));
    }

    #[test]
    fn mastery_falls_with_wrong_answers() {
        let params = BktParams::default();
        let m = update_mastery(0.9, &[false, false, false, false, false], &params);
        assert!(m < 0.9, "mastery should fall, got {m}");
    }

    #[test]
    fn mastery_stays_in_unit_interval() {
        let params = BktParams::default();
        for &p in &[0.0, 0.5, 1.0] {
            let m = update_mastery(p, &[true, false, true, false, true, true], &params);
            assert!((0.0..=1.0).contains(&m));
        }
    }

    #[test]
    fn trajectory_length_matches() {
        let params = BktParams::default();
        let traj = mastery_trajectory(0.3, &[true, false, true], &params);
        assert_eq!(traj.len(), 4);
    }
}