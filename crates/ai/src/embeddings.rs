//! Embedding generation.
//!
//! Two paths are provided: a deterministic local hashing fallback (zero network,
//! works offline for dev/tests) and the OpenAI `text-embedding-3-small` endpoint
//! for production-quality vectors stored in pgvector.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use common::errors::AppError;

pub const OPENAI_EMBEDDING_MODEL: &str = "text-embedding-3-small";

/// Deterministic L2-normalized bag-of-trigrams embedding (fallback).
pub fn hash_embedding(text: &str, dims: usize) -> Vec<f64> {
    let mut vec = vec![0.0_f64; dims];
    let norm = text.to_lowercase();
    let tokens: Vec<&str> = norm.split(|c: char| !c.is_alphanumeric()).filter(|s| !s.is_empty()).collect();

    // Word tokens.
    for w in &tokens {
        bump(&mut vec, w, dims);
    }
    // Character trigrams for substring robustness.
    let chars: Vec<char> = norm.chars().collect();
    for window in chars.windows(3) {
        let trigram: String = window.iter().collect();
        if trigram.len() == 3 {
            bump(&mut vec, &trigram, dims);
        }
    }

    l2_normalize(&mut vec);
    vec
}

fn bump(vec: &mut [f64], token: &str, dims: usize) {
    let mut h = DefaultHasher::new();
    token.hash(&mut h);
    let idx = (h.finish() as usize) % dims;
    vec[idx] += 1.0;
}

fn l2_normalize(vec: &mut [f64]) {
    let norm: f64 = vec.iter().map(|v| v * v).sum::<f64>().sqrt();
    if norm > 0.0 {
        for v in vec.iter_mut() {
            *v /= norm;
        }
    }
}

/// Cosine similarity between two vectors.
pub fn cosine(a: &[f64], b: &[f64]) -> f64 {
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

/// Generate an embedding via the OpenAI embeddings endpoint.
pub async fn embed_openai(
    client: &reqwest::Client,
    api_key: &str,
    text: &str,
) -> Result<Vec<f64>, AppError> {
    let body = serde_json::json!({
        "model": OPENAI_EMBEDDING_MODEL,
        "input": text
    });

    let resp = client
        .post("https://api.openai.com/v1/embeddings")
        .bearer_auth(api_key)
        .json(&body)
        .send()
        .await
        .map_err(|e| AppError::AiGateway(e.to_string()))?;

    let status = resp.status();
    let json: serde_json::Value = resp
        .json()
        .await
        .map_err(|e| AppError::AiGateway(e.to_string()))?;
    if !status.is_success() {
        return Err(AppError::AiGateway(json.to_string()));
    }

    json["data"][0]["embedding"]
        .as_array()
        .map(|arr| arr.iter().filter_map(|v| v.as_f64()).collect())
        .ok_or_else(|| AppError::AiGateway("Missing embedding in response".into()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_embedding_is_deterministic() {
        let a = hash_embedding("quadratic equations", 64);
        let b = hash_embedding("quadratic equations", 64);
        assert_eq!(a, b);
    }

    #[test]
    fn hash_embedding_is_unit_norm() {
        let v = hash_embedding("learning economy", 128);
        let norm: f64 = v.iter().map(|x| x * x).sum::<f64>().sqrt();
        assert!((norm - 1.0).abs() < 1e-9);
    }

    #[test]
    fn identical_text_is_most_similar() {
        assert!((cosine(&hash_embedding("rust", 64), &hash_embedding("rust", 64)) - 1.0).abs() < 1e-9);
    }
}