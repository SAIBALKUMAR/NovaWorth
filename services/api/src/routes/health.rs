//! Health check endpoint.

use axum::{routing::get, Router, Json};
use serde_json::json;

pub fn router() -> Router {
    Router::new().route("/health", get(health_handler))
}

async fn health_handler() -> Json<serde_json::Value> {
    Json(json!({ "status": "ok", "service": "api" }))
}
