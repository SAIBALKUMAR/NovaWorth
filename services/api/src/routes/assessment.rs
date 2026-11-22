//! Assessment routes: server-side exam grading.

use std::sync::Arc;
use axum::{extract::{Path, State}, routing::post, Json, Router};
use serde::Deserialize;
use uuid::Uuid;
use auth::backend::AuthSession;
use common::errors::AppError;
use assessment::grading::{self, SubmittedAnswer};
use crate::state::AppState;

pub fn router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/assessments/:level_id/grade", post(grade))
        .with_state(state)
}

#[derive(Deserialize)]
struct AnswerIn { item_id: Uuid, chosen_index: i32 }

#[derive(Deserialize)]
struct GradeRequest { answers: Vec<AnswerIn> }

async fn grade(
    State(state): State<Arc<AppState>>,
    _auth: AuthSession,
    Path(level_id): Path<Uuid>,
    Json(body): Json<GradeRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    let answers: Vec<SubmittedAnswer> = body
        .answers
        .into_iter()
        .map(|a| SubmittedAnswer { item_id: a.item_id, chosen_index: a.chosen_index })
        .collect();

    let score = grading::grade_level(&state.pool, level_id, &answers).await?;
    Ok(Json(serde_json::json!({ "score": score })))
}