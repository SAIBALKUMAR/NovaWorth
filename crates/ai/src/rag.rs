//! Hybrid retrieval: cosine similarity plus Reciprocal Rank Fusion (RRF).
//!
//! RRF merges multiple ranked lists (e.g. a keyword/BM25 ranking and a vector
//! ranking) into one fused ordering without needing to normalize raw scores.

use std::collections::HashMap;

/// Cosine similarity between two equal-length vectors.
pub fn cosine(a: &[f64], b: &[f64]) -> f64 {
    crate::embeddings::cosine(a, b)
}

/// RRF score for a document appearing at rank `r` (0-based).
pub fn rrf_score(rank: usize, k: f64) -> f64 {
    1.0 / (k + rank as f64)
}

/// Fuse multiple ranked lists of document ids into a single score map.
pub fn reciprocal_rank_fusion(rankings: &[Vec<u64>], k: f64) -> HashMap<u64, f64> {
    let mut scores: HashMap<u64, f64> = HashMap::new();
    for ranking in rankings {
        for (rank, doc) in ranking.iter().enumerate() {
            *scores.entry(*doc).or_insert(0.0) += rrf_score(rank, k);
        }
    }
    scores
}

/// Fused rankings sorted by descending score. `(doc_id, score)`.
pub fn fused_ranking(rankings: &[Vec<u64>], k: f64) -> Vec<(u64, f64)> {
    let scores = reciprocal_rank_fusion(rankings, k);
    let mut vec: Vec<(u64, f64)> = scores.into_iter().collect();
    vec.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn documents_in_multiple_lists_rank_higher() {
        let keyword = vec![1, 2, 3];
        let vector = vec![2, 4, 5];
        let fused = fused_ranking(&[keyword, vector], 60.0);
        assert_eq!(fused[0].0, 2, "doc 2 appears in both lists and should win");
    }

    #[test]
    fn rrf_score_decreases_with_rank() {
        assert!(rrf_score(0, 60.0) > rrf_score(1, 60.0));
    }
}