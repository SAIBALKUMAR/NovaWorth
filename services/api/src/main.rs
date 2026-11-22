//! API Service — Axum REST API
//! Runs on API_PORT (default 4000)

use std::sync::Arc;
use std::time::Duration;
use tower_http::{
    cors::{CorsLayer, Any},
    trace::TraceLayer,
    compression::CompressionLayer,
    timeout::TimeoutLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

mod routes;
mod state;

pub use state::AppState;

#[tokio::main]
async fn main() {
    // ── Tracing ──────────────────────────────────────────────────────────────
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "api=debug,tower_http=debug".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // ── Config ────────────────────────────────────────────────────────────────
    let config = common::config::AppConfig::from_env();
    let port   = config.api_port;

    tracing::info!(port, "Starting API service");

    // ── Database ──────────────────────────────────────────────────────────────
    let pool = db::pool::create_pool(&config.database_url).await;
    db::pool::run_migrations(&pool).await;
    tracing::info!("Database connected and migrations applied");

    // ── State ─────────────────────────────────────────────────────────────────
    let state = Arc::new(AppState::new(config.clone(), pool).await);

    // ── CORS ──────────────────────────────────────────────────────────────────
    let origin = config.app_url
        .parse::<axum::http::HeaderValue>()
        .expect("APP_URL is not a valid header value");

    let cors = CorsLayer::new()
        .allow_origin(origin)
        .allow_methods(Any)
        .allow_headers(Any);

    // ── Router ────────────────────────────────────────────────────────────────
    let app = axum::Router::new()
        .merge(routes::health::router())
        .merge(routes::auth::router(Arc::clone(&state)))
        .merge(routes::economy::router(Arc::clone(&state)))
        .merge(routes::social::router(Arc::clone(&state)))
        .layer(cors)
        .layer(CompressionLayer::new())
        .layer(TimeoutLayer::new(Duration::from_secs(
            common::constants::OUTGOING_HTTP_TIMEOUT_SECS,
        )))
        .layer(TraceLayer::new_for_http());

    // ── Serve ─────────────────────────────────────────────────────────────────
    let addr = format!("0.0.0.0:{port}");
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap_or_else(|e| panic!("Cannot bind to {addr}: {e}"));

    tracing::info!("API listening on http://{addr}");
    axum::serve(listener, app).await.unwrap();
}
