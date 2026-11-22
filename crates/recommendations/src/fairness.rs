//! Fairness-aware diversity re-ranking (Maximal Marginal Relevance, MMR).
//!
//! Balances "relevance to the learner" with "diversity of the result set" to avoid
//! filter bubbles. Returns the candidate indices in chosen order.

/// A candidate with a pre-computed relevance score (0–1) and a feature vector.
#[derive(Debug, Clone)]
pub struct Candidate {
    pub relevance: f64,
    pub features: Vec<f64>,
}

fn cosine(a: &[f64], b: &[f64]) -> f64 {
    if a.len() != b.len() {
        return 0.0;
    }
    let (mut dot, mut na, mut nb) = (0.0, 0.0, 0.0);
    for (x, y) in a.iter().zip(b) {
        dot += x * y;
        na += x * x;
        nb += y * y;
    }
    if na == 0.0 || nb == 0.0 {
        0.0
    } else {
        dot / (na.sqrt() * nb.sqrt())
    }
}

/// Greedy MMR re-ranking.
///
/// `lambda` weights relevance vs. diversity (1.0 = pure relevance, 0.0 = pure
/// diversity). Returns up to `top_k` indices in ranked order.
pub fn mmr_rerank(candidates: &[Candidate], lambda: f64, top_k: usize) -> Vec<usize> {
    let lambda = lambda.clamp(0.0, 1.0);
    if candidates.is_empty() || top_k == 0 {
        return Vec::new();
    }

    let mut selected: Vec<usize> = Vec::new();
    let mut remaining: Vec<usize> = (0..candidates.len()).collect();

    while !remaining.is_empty() && selected.len() < top_k {
        let mut best_idx = 0;
        let mut best_pos = 0;
        let mut best_score = f64::NEG_INFINITY;

        for (pos, &idx) in remaining.iter().enumerate() {
            let relevance = candidates[idx].relevance;
            let max_sim = selected
                .iter()
                .map(|&s| cosine(&candidates[idx].features, &candidates[s].features))
                .fold(0.0_f64, f64::max);
            let score = lambda * relevance - (1.0 - lambda) * max_sim;
            if score > best_score {
                best_score = score;
                best_idx = idx;
                best_pos = pos;
            }
        }

        selected.push(best_idx);
        remaining.remove(best_pos);
    }

    selected
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cand(rel: f64, feats: &[f64]) -> Candidate {
        Candidate { relevance: rel, features: feats.to_vec() }
    }

    #[test]
    fn pure_relevance_returns_top_first() {
        let cs = vec![cand(0.9, &[1.0, 0.0]), cand(0.5, &[0.0, 1.0])];
        let order = mmr_rerank(&cs, 1.0, 2);
        assert_eq!(order[0], 0);
    }

    #[test]
    fn pure_diversity_spreads_selection() {
        // With lambda 0, MMR minimizes between-item similarity.
        let cs = vec![cand(1.0, &[1.0, 0.0]), cand(1.0, &[1.0, 0.0]), cand(1.0, &[0.0, 1.0])];
        let order = mmr_rerank(&cs, 0.0, 2);
        // Second pick should differ from the first.
        assert_ne!(order[0], order[1]);
    }

    #[test]
    fn respects_top_k() {
        let cs = vec![cand(0.8, &[1.0]), cand(0.7, &[1.0]), cand(0.6, &[1.0])];
        assert_eq!(mmr_rerank(&cs, 0.5, 2).len(), 2);
    }

    #[test]
    fn empty_input_gives_empty_output() {
        assert!(mmr_rerank(&[], 0.5, 5).is_empty());
    }
}