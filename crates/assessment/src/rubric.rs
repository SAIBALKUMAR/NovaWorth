//! Rubric scoring for practical/artifact assessment.
//!
//! Encodes the canonical pass rules: a passing submission must average ≥ 2.5/4 with
//! **no** criterion below 2.0; a distinction averages ≥ 3.5/4.

/// Rubric scale ceiling.
pub const RUBRIC_MAX: f64 = 4.0;
/// Passing average on the 0–4 scale.
pub const PASS_AVG: f64 = 2.5;
/// Minimum per-criterion score to pass.
pub const PASS_MIN_CRITERION: f64 = 2.0;
/// Distinction average.
pub const DISTINCTION_AVG: f64 = 3.5;

/// A scored, weighted criterion.
#[derive(Debug, Clone)]
pub struct CriterionScore {
    pub criterion: String,
    /// Raw score on the 0–4 scale.
    pub score: f64,
    /// Relative weight (defaults to equal weighting if all are 0).
    pub weight: f64,
}

/// Weighted average score on the 0–4 scale (falls back to equal weights).
pub fn weighted_average(scores: &[CriterionScore]) -> f64 {
    if scores.is_empty() {
        return 0.0;
    }
    let total_weight: f64 = scores.iter().map(|c| c.weight).sum();
    if total_weight <= 0.0 {
        return scores.iter().map(|c| c.score).sum::<f64>() / scores.len() as f64;
    }
    scores.iter().map(|c| c.score * c.weight).sum::<f64>() / total_weight
}

/// Every criterion must be at or above `min_per`.
pub fn meets_minimum(scores: &[CriterionScore], min_per: f64) -> bool {
    scores.iter().all(|c| c.score >= min_per)
}

/// Whether the submission passes (average + per-criterion floor).
pub fn is_pass(scores: &[CriterionScore]) -> bool {
    weighted_average(scores) >= PASS_AVG && meets_minimum(scores, PASS_MIN_CRITERION)
}

/// Whether the submission reaches distinction.
pub fn is_distinction(scores: &[CriterionScore]) -> bool {
    weighted_average(scores) >= DISTINCTION_AVG
}

/// Normalize a 0–4 average to the 0–1 gate scale.
pub fn normalize(avg: f64) -> f64 {
    (avg / RUBRIC_MAX).clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn criterion(name: &str, score: f64) -> CriterionScore {
        CriterionScore { criterion: name.into(), score, weight: 1.0 }
    }

    #[test]
    fn equal_weight_average() {
        let s = vec![criterion("a", 2.0), criterion("b", 4.0)];
        assert!((weighted_average(&s) - 3.0).abs() < 1e-9);
    }

    #[test]
    fn pass_requires_floor() {
        let good = vec![criterion("a", 2.5), criterion("b", 3.0), criterion("c", 2.5)];
        let floored = vec![criterion("a", 4.0), criterion("b", 4.0), criterion("c", 1.9)];
        assert!(is_pass(&good));
        assert!(!is_pass(&floored));
    }

    #[test]
    fn distinction_threshold() {
        let s = vec![criterion("a", 3.6), criterion("b", 3.6)];
        assert!(is_distinction(&s));
        assert!(!is_distinction(&vec![criterion("a", 3.0)]));
    }

    #[test]
    fn normalize_bounds() {
        assert!((normalize(2.5) - 0.625).abs() < 1e-9);
        assert_eq!(normalize(10.0), 1.0);
    }
}