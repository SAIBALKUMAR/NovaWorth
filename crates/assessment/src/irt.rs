//! Item Response Theory — 2PL / 3PL models and ability estimation.
//!
//! `θ` is the latent ability, `b` item difficulty, `a` discrimination, and `c` the
//! lower-asymptote guessing parameter.

/// Logistic function.
pub fn logistic(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

/// 2PL probability of a correct response (no guessing).
pub fn probability_correct_2pl(ability: f64, difficulty: f64, discrimination: f64) -> f64 {
    logistic(discrimination * (ability - difficulty))
}

/// 3PL probability of a correct response, with guessing parameter `c ∈ [0, 0.5]`.
pub fn probability_correct_3pl(
    ability: f64,
    difficulty: f64,
    discrimination: f64,
    guessing: f64,
) -> f64 {
    let c = guessing.clamp(0.0, 0.5);
    c + (1.0 - c) * probability_correct_2pl(ability, difficulty, discrimination)
}

/// 3PL item Fisher information at a given ability level.
pub fn item_information(
    ability: f64,
    difficulty: f64,
    discrimination: f64,
    guessing: f64,
) -> f64 {
    let c = guessing.clamp(0.0, 0.5);
    let p = probability_correct_3pl(ability, difficulty, discrimination, c);
    let q = 1.0 - p;
    let a2 = discrimination * discrimination;
    // Standard 3PL information formula.
    a2 * (q / p.max(1e-9)) * ((p - c).powi(2) / (1.0 - c).powi(2))
}

/// Maximum-likelihood ability estimate via a few Newton–Raphson iterations over a
/// 2PL model. Inputs are `(difficulty, discrimination, correct)` observations.
pub fn ability_estimate(items: &[(f64, f64, bool)], iters: usize) -> f64 {
    let mut theta = 0.0;
    for _ in 0..iters.max(1) {
        let mut grad = 0.0;
        let mut hess = 0.0;
        for &(b, a, correct) in items {
            let p = probability_correct_2pl(theta, b, a);
            grad += a * (if correct { 1.0 } else { 0.0 } - p);
            hess += a * a * p * (1.0 - p);
        }
        if hess.abs() < 1e-9 {
            break;
        }
        theta -= grad / hess;
    }
    theta
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn higher_ability_means_higher_probability() {
        assert!(probability_correct_3pl(2.0, 0.0, 1.0, 0.2) > probability_correct_3pl(-2.0, 0.0, 1.0, 0.2));
    }

    #[test]
    fn guessing_sets_floor() {
        let p = probability_correct_3pl(-10.0, 0.0, 1.0, 0.25);
        assert!(p >= 0.25 - 1e-9);
    }

    #[test]
    fn ability_estimate_moves_with_performance() {
        let easy = ability_estimate(&[(0.0, 1.0, true), (0.5, 1.0, true), (1.0, 1.0, true)], 20);
        let hard = ability_estimate(&[(3.0, 1.0, true), (4.0, 1.0, false), (5.0, 1.0, false)], 20);
        assert!(easy > hard);
    }

    #[test]
    fn information_is_positive() {
        assert!(item_information(0.5, 0.5, 1.0, 0.2) > 0.0);
    }
}