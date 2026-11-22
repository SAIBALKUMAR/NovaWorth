//! Item-item collaborative filtering, implemented without external ML crates.

use std::collections::HashMap;

/// Sparse rating matrix: `user_id -> (item_id -> rating)`.
pub type RatingMatrix = HashMap<u32, HashMap<u32, f64>>;

/// Cosine similarity between two equal-length vectors.
pub fn cosine(a: &[f64], b: &[f64]) -> Option<f64> {
    if a.len() != b.len() || a.is_empty() {
        return None;
    }
    let (mut dot, mut na, mut nb) = (0.0, 0.0, 0.0);
    for (x, y) in a.iter().zip(b) {
        dot += x * y;
        na += x * x;
        nb += y * y;
    }
    if na == 0.0 || nb == 0.0 {
        None
    } else {
        Some(dot / (na.sqrt() * nb.sqrt()))
    }
}

/// Mean rating of one user across all rated items.
pub fn user_mean(matrix: &RatingMatrix, user: u32) -> Option<f64> {
    let ratings = matrix.get(&user)?;
    if ratings.is_empty() {
        return None;
    }
    Some(ratings.values().sum::<f64>() / ratings.len() as f64)
}

/// Similarity between two items across users who rated both.
fn item_similarity(matrix: &RatingMatrix, item_a: u32, item_b: u32) -> Option<f64> {
    let mut a = Vec::new();
    let mut b = Vec::new();
    for (_, ratings) in matrix {
        if let (Some(&ra), Some(&rb)) = (ratings.get(&item_a), ratings.get(&item_b)) {
            a.push(ra);
            b.push(rb);
        }
    }
    cosine(&a, &b)
}

/// Predict a rating for `(user, item)` using the `k` most similar items the user
/// has already rated. Falls back to the user's mean, then the global mean.
pub fn predict_rating(matrix: &RatingMatrix, user: u32, item: u32, k: usize) -> Option<f64> {
    let ratings = matrix.get(&user)?;

    // Weighted sum of the user's ratings, weighted by item-item similarity.
    let mut weighted = 0.0;
    let mut norm = 0.0;
    let mut neighbors: Vec<(f64, f64)> = ratings
        .iter()
        .filter(|(j, _)| **j != item)
        .filter_map(|(j, &r)| item_similarity(matrix, item, *j).map(|s| (s, r)))
        .collect();
    neighbors.sort_by(|a, b| b.0.abs().partial_cmp(&a.0.abs()).unwrap_or(std::cmp::Ordering::Equal));

    for (sim, r) in neighbors.into_iter().take(k.max(1)) {
        weighted += sim * r;
        norm += sim.abs();
    }

    if norm > 0.0 {
        Some(weighted / norm)
    } else {
        user_mean(matrix, user).or_else(|| {
            let all: Vec<f64> = matrix.values().flat_map(|m| m.values().copied()).collect();
            if all.is_empty() { None } else { Some(all.iter().sum::<f64>() / all.len() as f64) }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn matrix() -> RatingMatrix {
        // Alice & Bob both over-rate item 2, giving item 1 and 2 high similarity.
        let mut m = HashMap::new();
        m.insert(1u32, HashMap::from([(1u32, 5.0), (2u32, 3.0)]));
        m.insert(2u32, HashMap::from([(1u32, 4.0), (2u32, 2.0), (3u32, 4.0)]));
        m
    }

    #[test]
    fn cosine_of_identical_is_one() {
        assert!((cosine(&[1.0, 2.0], &[1.0, 2.0]).unwrap() - 1.0).abs() < 1e-9);
    }

    #[test]
    fn cosine_of_zero_vector_is_none() {
        assert!(cosine(&[0.0, 0.0], &[1.0, 0.0]).is_none());
    }

    #[test]
    fn prediction_falls_in_rating_range() {
        let p = predict_rating(&matrix(), 2, 1, 3).unwrap();
        assert!(p > 0.0 && p <= 5.0);
    }

    #[test]
    fn unknown_user_returns_none() {
        assert!(predict_rating(&matrix(), 999, 1, 3).is_none());
    }
}