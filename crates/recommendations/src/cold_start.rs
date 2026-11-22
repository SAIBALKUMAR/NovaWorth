//! Content-based cold-start matching.
//!
//! When there are no ratings/interactions yet, match a learner's profile vector to
//! item feature vectors using cosine similarity over the union of feature keys.

use std::collections::HashMap;

/// Cosine similarity over `HashMap` feature spaces (missing keys are 0.0).
pub fn content_similarity(
    user_profile: &HashMap<String, f64>,
    item_features: &HashMap<String, f64>,
) -> f64 {
    let keys: std::collections::HashSet<&String> =
        user_profile.keys().chain(item_features.keys()).collect();

    let (mut dot, mut na, mut nb) = (0.0, 0.0, 0.0);
    for k in keys {
        let a = user_profile.get(k).copied().unwrap_or(0.0);
        let b = item_features.get(k).copied().unwrap_or(0.0);
        dot += a * b;
        na += a * a;
        nb += b * b;
    }
    if na == 0.0 || nb == 0.0 {
        0.0
    } else {
        dot / (na.sqrt() * nb.sqrt())
    }
}

/// Rank a catalog of items against a learner profile, returning sorted `(idx, sim)`.
pub fn rank_items(
    user_profile: &HashMap<String, f64>,
    items: &[HashMap<String, f64>],
) -> Vec<(usize, f64)> {
    let mut scored: Vec<(usize, f64)> = items
        .iter()
        .enumerate()
        .map(|(i, feats)| (i, content_similarity(user_profile, feats)))
        .collect();
    scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    scored
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identical_profiles_similarity_one() {
        let a = HashMap::from([("math".into(), 1.0), ("writing".into(), 0.5)]);
        assert!((content_similarity(&a, &a) - 1.0).abs() < 1e-9);
    }

    #[test]
    fn disjoint_profiles_similarity_zero() {
        let a = HashMap::from([("math".into(), 1.0)]);
        let b = HashMap::from([("cooking".into(), 1.0)]);
        assert_eq!(content_similarity(&a, &b), 0.0);
    }

    #[test]
    fn ranking_orders_by_similarity() {
        let user = HashMap::from([("math".into(), 1.0)]);
        let items = vec![
            HashMap::from([("cooking".into(), 1.0)]),
            HashMap::from([("math".into(), 1.0)]),
        ];
        let ranked = rank_items(&user, &items);
        assert_eq!(ranked[0].0, 1); // the math item ranks first
    }
}