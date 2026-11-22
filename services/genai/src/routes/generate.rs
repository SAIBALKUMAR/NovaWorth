//! AI generation endpoints (wired to the budget-aware gateway).

use std::sync::Arc;

use axum::{extract::State, routing::post, Json, Router};

use ai::gateway::AiGateway;
use ai::prompts::{explanation_prompt, AiTask, EXPLANATION_SYSTEM};
use common::errors::AppError;

pub fn router(gateway: Arc<AiGateway>) -> Router {
    Router::new()
        .route("/generate/explain", post(explain))
        .with_state(gateway)
}

#[derive(serde::Deserialize)]
struct ExplainRequest {
    user_id: String,
    topic: String,
}

#[derive(serde::Serialize)]
struct ExplainResponse {
    text: String,
}

async fn explain(
    State(gateway): State<Arc<AiGateway>>,
    Json(body): Json<ExplainRequest>,
) -> Result<Json<ExplainResponse>, AppError> {
    let user_prompt = explanation_prompt(&body.topic);
    let text = gateway
        .complete(&body.user_id, AiTask::Explanation, EXPLANATION_SYSTEM, &user_prompt)
        .await?;
    Ok(Json(ExplainResponse { text }))
}