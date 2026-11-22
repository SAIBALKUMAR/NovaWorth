//! Social routes:
//! GET /social/leaderboard → top learners by XP.

use std::sync::Arc;

use axum::{
    extract::{Query, State},
    routing::get,
    Json, Router,
};

use common::errors::AppError;
use social::league;

use crate::state::AppState;

pub fn router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/social/leaderboard", get(leaderboard))
        .with_state(state)
}

#[derive(serde::Deserialize)]
struct LeaderboardQuery {
    limit: Option<i64>,
}

async fn leaderboard(
    State(state): State<Arc<AppState>>,
    Query(params): Query<LeaderboardQuery>,
) -> Result<Json<Vec<league::LeaderboardRow>>, AppError> {
    let limit = params.limit.unwrap_or(10).clamp(1, 100);
    let rows = league::leaderboard(&state.pool, limit).await?;
    Ok(Json(rows))
}