//! Multi-factor fit score (0–100).
//!
//! Combines the seven onboarding signals into a single, explainable recommendation
//! score. All inputs are normalized to `[0, 1]`.

/// The seven fit factors (each 0.0–1.0).
#[derive(Debug, Clone, Copy)]
pub struct FitInput {
    /// Aptitude alignment (diagnostic).
    pub aptitude: f64,
    /// Declared interest.
    pub interest: f64,
    /// Transferable experience.
    pub experience: f64,
    /// Local opportunity/demand.
    pub opportunity: f64,
    /// Feasibility given constraints/time.
    pub feasibility: f64,
    /// Sponsor/employer demand signal.
    pub demand: f64,
    /// Pace compatibility.
    pub pace: f64,
}

/// Default weights — sum to 1.0 and mirror the product's stated priorities.
#[derive(Debug, Clone, Copy)]
pub struct FitWeights {
    pub aptitude: f64,
    pub interest: f64,
    pub experience: f64,
    pub opportunity: f64,
    pub feasibility: f64,
    pub demand: f64,
    pub pace: f64,
}

impl Default for FitWeights {
    fn default() -> Self {
        Self {
            aptitude: 0.25,
            interest: 0.20,
            experience: 0.15,
            opportunity: 0.10,
            feasibility: 0.10,
            demand: 0.10,
            pace: 0.10,
        }
    }
}

/// Weighted fit score scaled to 0–100.
pub fn fit_score(input: &FitInput, weights: &FitWeights) -> f64 {
    let w = weights;
    let weighted = input.aptitude * w.aptitude
        + input.interest * w.interest
        + input.experience * w.experience
        + input.opportunity * w.opportunity
        + input.feasibility * w.feasibility
        + input.demand * w.demand
        + input.pace * w.pace;
    (weighted * 100.0).clamp(0.0, 100.0)
}

/// Return the contributing terms for explainability ("why this score").
pub fn explain(input: &FitInput, weights: &FitWeights) -> Vec<(&'static str, f64)> {
    vec![
        ("aptitude", input.aptitude * weights.aptitude),
        ("interest", input.interest * weights.interest),
        ("experience", input.experience * weights.experience),
        ("opportunity", input.opportunity * weights.opportunity),
        ("feasibility", input.feasibility * weights.feasibility),
        ("demand", input.demand * weights.demand),
        ("pace", input.pace * weights.pace),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_input_scores_100() {
        let input = FitInput {
            aptitude: 1.0, interest: 1.0, experience: 1.0, opportunity: 1.0,
            feasibility: 1.0, demand: 1.0, pace: 1.0,
        };
        assert!((fit_score(&input, &FitWeights::default()) - 100.0).abs() < 1e-9);
    }

    #[test]
    fn zero_input_scores_zero() {
        let input = FitInput {
            aptitude: 0.0, interest: 0.0, experience: 0.0, opportunity: 0.0,
            feasibility: 0.0, demand: 0.0, pace: 0.0,
        };
        assert!((fit_score(&input, &FitWeights::default()) - 0.0).abs() < 1e-9);
    }

    #[test]
    fn weights_sum_to_one() {
        let w = FitWeights::default();
        let sum = w.aptitude + w.interest + w.experience + w.opportunity
            + w.feasibility + w.demand + w.pace;
        assert!((sum - 1.0).abs() < 1e-9);
    }

    #[test]
    fn explanation_has_seven_terms() {
        let input = FitInput {
            aptitude: 0.5, interest: 0.5, experience: 0.5, opportunity: 0.5,
            feasibility: 0.5, demand: 0.5, pace: 0.5,
        };
        assert_eq!(explain(&input, &FitWeights::default()).len(), 7);
    }
}