//! Shared application state passed to all Axum handlers.

use sqlx::PgPool;
use oauth2::basic::BasicClient;
use common::config::AppConfig;
use auth::{backend::AuthBackend, oauth};

#[derive(Clone)]
pub struct AppState {
    pub config:       AppConfig,
    pub pool:         PgPool,
    pub auth_backend: AuthBackend,
    pub github_oauth: BasicClient,
    pub google_oauth: BasicClient,
    pub http_client:  reqwest::Client,
}

impl AppState {
    pub async fn new(config: AppConfig, pool: PgPool) -> Self {
        let auth_backend = AuthBackend::new(pool.clone());
        let github_oauth = oauth::github_client(&config);
        let google_oauth = oauth::google_client(&config);
        let http_client  = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(
                common::constants::OUTGOING_HTTP_TIMEOUT_SECS,
            ))
            .user_agent("education-economy/1.0")
            .build()
            .expect("Failed to build HTTP client");

        Self {
            config,
            pool,
            auth_backend,
            github_oauth,
            google_oauth,
            http_client,
        }
    }
}
