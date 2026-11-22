//! GenAI Service — Axum service wrapping OpenAI.
//! Runs on GENAI_PORT (default 5000)
//! Internal-only: called by the API service, never directly by the UI.

use std::sync::Arc;
use std::time::Duration;
use tower_http::{trace::TraceLayer, timeout::TimeoutLayer, compression::CompressionLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

mod routes;

#[tokio::main]
async fn main() {
    // ── Tracing ──────────────────────────────────────────────────────────────
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "genai=debug,tower_http=debug".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // ── Config ────────────────────────────────────────────────────────────────
    let config = common::config::AppConfig::from_env();
    let port   = config.genai_port;

    tracing::info!(port, "Starting GenAI service");

    let gateway = Arc::new(ai::gateway::AiGateway::new(config.openai_api_key.clone()));

    // ── Router ────────────────────────────────────────────────────────────────
    let app = axum::Router::new()
        .merge(routes::health::router())
        .merge(routes::generate::router(gateway))
        .layer(CompressionLayer::new())
        .layer(TimeoutLayer::new(Duration::from_secs(60))) // AI calls need longer timeout
        .layer(TraceLayer::new_for_http());

    // ── Serve ─────────────────────────────────────────────────────────────────
    let addr = format!("0.0.0.0:{port}");
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap_or_else(|e| panic!("Cannot bind to {addr}: {e}"));

    tracing::info!("GenAI listening on http://{addr}");
    axum::serve(listener, app).await.unwrap();
}
