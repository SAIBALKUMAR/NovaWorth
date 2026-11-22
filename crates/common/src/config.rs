//! Loads configuration from environment variables.
//! All env reads happen here — zero `std::env::var` calls elsewhere.

use dotenvy::dotenv;
use std::env;

/// Full application configuration loaded from environment.
#[derive(Debug, Clone)]
pub struct AppConfig {
    // Database
    pub database_url: String,

    // Redis
    pub redis_url: String,

    // Service ports
    pub ui_port: u16,
    pub api_port: u16,
    pub genai_port: u16,

    // Internal service URLs
    pub internal_api_url: String,
    pub internal_genai_url: String,

    // Auth
    pub session_secret: String,

    // OAuth
    pub github_client_id: String,
    pub github_client_secret: String,
    pub github_redirect_url: String,

    pub google_client_id: String,
    pub google_client_secret: String,
    pub google_redirect_url: String,

    // AI
    pub openai_api_key: String,
    pub openai_monthly_budget_usd: u32,

    // Email
    pub resend_api_key: String,
    pub email_from: String,

    // Storage
    pub r2_account_id: String,
    pub r2_access_key_id: String,
    pub r2_secret_access_key: String,
    pub r2_bucket: String,
    pub r2_public_url: String,

    // App
    pub app_env: AppEnv,
    pub app_url: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AppEnv {
    Development,
    Production,
}

impl AppEnv {
    pub fn is_production(&self) -> bool {
        *self == Self::Production
    }
}

impl AppConfig {
    /// Load from environment. Call once at startup.
    /// Panics with a clear message if any required variable is missing.
    pub fn from_env() -> Self {
        dotenv().ok(); // Load .env if present (no-op in production)

        Self {
            database_url:               require("DATABASE_URL"),
            redis_url:                  require("REDIS_URL"),
            ui_port:                    parse("UI_PORT", 3000),
            api_port:                   parse("API_PORT", 4000),
            genai_port:                 parse("GENAI_PORT", 5000),
            internal_api_url:           env_or("INTERNAL_API_URL", "http://localhost:4000"),
            internal_genai_url:         env_or("INTERNAL_GENAI_URL", "http://localhost:5000"),
            session_secret:             require("SESSION_SECRET"),
            github_client_id:           env_or("GITHUB_CLIENT_ID", ""),
            github_client_secret:       env_or("GITHUB_CLIENT_SECRET", ""),
            github_redirect_url:        env_or("GITHUB_REDIRECT_URL", "http://localhost:4000/auth/github/callback"),
            google_client_id:           env_or("GOOGLE_CLIENT_ID", ""),
            google_client_secret:       env_or("GOOGLE_CLIENT_SECRET", ""),
            google_redirect_url:        env_or("GOOGLE_REDIRECT_URL", "http://localhost:4000/auth/google/callback"),
            openai_api_key:             env_or("OPENAI_API_KEY", ""),
            openai_monthly_budget_usd:  parse("OPENAI_MONTHLY_BUDGET_USD", 20),
            resend_api_key:             env_or("RESEND_API_KEY", ""),
            email_from:                 env_or("EMAIL_FROM", "Education Economy <no-reply@example.com>"),
            r2_account_id:              env_or("R2_ACCOUNT_ID", ""),
            r2_access_key_id:           env_or("R2_ACCESS_KEY_ID", ""),
            r2_secret_access_key:       env_or("R2_SECRET_ACCESS_KEY", ""),
            r2_bucket:                  env_or("R2_BUCKET", "education-economy-assets"),
            r2_public_url:              env_or("R2_PUBLIC_URL", ""),
            app_env:                    parse_env(),
            app_url:                    env_or("APP_URL", "http://localhost:3000"),
        }
    }
}

fn require(key: &str) -> String {
    env::var(key).unwrap_or_else(|_| panic!("Required env var {key} is not set"))
}

fn env_or(key: &str, default: &str) -> String {
    env::var(key).unwrap_or_else(|_| default.to_string())
}

fn parse<T: std::str::FromStr>(key: &str, default: T) -> T
where
    T::Err: std::fmt::Debug,
{
    env::var(key)
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}

fn parse_env() -> AppEnv {
    match env::var("APP_ENV").as_deref() {
        Ok("production") => AppEnv::Production,
        _                => AppEnv::Development,
    }
}
