//! Recommendations crate — the four recommendation components:
//! - [`collaborative`] — item-item collaborative filtering (cosine similarity)
//! - [`cold_start`]    — content-based matching for new learners/items
//! - [`fit_score`]     — the multi-factor 0–100 fit algorithm
//! - [`fairness`]      — MMR diversity re-ranking to avoid filter bubbles

pub mod cold_start;
pub mod collaborative;
pub mod fairness;
pub mod fit_score;