//! Server-side exam grading.
//!
//! Replaces the client-supplied `score` with a score computed from submitted
//! responses against the authoritative `quiz_items.correct_index`.

use sqlx::PgPool;
use uuid::Uuid;

use common::errors::AppError;

/// A learner's answer to one quiz item.
#[derive(Debug, Clone, Copy)]
pub struct SubmittedAnswer {
    pub item_id: Uuid,
    pub chosen_index: i32,
}

/// Weighted score in `[0, 1]` from item points, or `None` if there are no items.
pub fn score_from(items: &[(Uuid, i32, i32)], answers: &[SubmittedAnswer]) -> Option<f64> {
    if items.is_empty() {
        return None;
    }
    let mut total: i64 = 0;
    let mut earned: i64 = 0;
    for (item_id, correct_index, points) in items {
        total += *points as i64;
        if let Some(a) = answers.iter().find(|a| a.item_id == *item_id) {
            if a.chosen_index == *correct_index {
                earned += *points as i64;
            }
        }
    }
    if total == 0 {
        return None;
    }
    Some(earned as f64 / total as f64)
}

/// Grade a submission end-to-end: load the level's items, compute the score, and
/// persist each response (with its correctness) to `quiz_responses`.
pub async fn grade_submission(
    pool: &PgPool,
    attempt_id: Uuid,
    level_id: Uuid,
    answers: &[SubmittedAnswer],
) -> Result<f64, AppError> {
    let items: Vec<(Uuid, i32, i32)> = sqlx::query_as(
        "SELECT id, correct_index, points FROM quiz_items WHERE level_id = $1 ORDER BY sort_order",
    )
    .bind(level_id)
    .fetch_all(pool)
    .await
    .map_err(AppError::Database)?;

    let score = score_from(&items, answers)
        .ok_or_else(|| AppError::Validation("Level has no quiz items to grade".into()))?;

    for (item_id, correct_index, _points) in &items {
        let chosen = answers.iter().find(|a| a.item_id == *item_id).map(|a| a.chosen_index);
        if let Some(chosen_index) = chosen {
            sqlx::query(
                "INSERT INTO quiz_responses (attempt_id, item_id, chosen_index, correct)
                 VALUES ($1, $2, $3, $4)",
            )
            .bind(attempt_id)
            .bind(item_id)
            .bind(chosen_index)
            .bind(chosen_index == *correct_index)
            .execute(pool)
            .await
            .map_err(AppError::Database)?;
        }
    }

    Ok(score)
}

/// Server-compute a score for a level without persisting (used by the grading API).
pub async fn grade_level(
    pool: &PgPool,
    level_id: Uuid,
    answers: &[SubmittedAnswer],
) -> Result<f64, AppError> {
    let items: Vec<(Uuid, i32, i32)> = sqlx::query_as(
        "SELECT id, correct_index, points FROM quiz_items WHERE level_id = $1 ORDER BY sort_order",
    )
    .bind(level_id)
    .fetch_all(pool)
    .await
    .map_err(AppError::Database)?;

    score_from(&items, answers)
        .ok_or_else(|| AppError::Validation("Level has no quiz items to grade".into()))
}