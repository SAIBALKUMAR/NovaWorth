//! Unified application error type.
//! All service crates return `AppError` and convert it to HTTP responses.

use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    // ── Database ──────────────────────────────────────────────────────────────
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    // ── Auth ──────────────────────────────────────────────────────────────────
    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Account locked — try again after {minutes} minutes")]
    AccountLocked { minutes: u64 },

    #[error("Session expired — please log in again")]
    SessionExpired,

    #[error("Insufficient permissions: required role {required}")]
    Forbidden { required: String },

    // ── Validation ────────────────────────────────────────────────────────────
    #[error("Validation failed: {0}")]
    Validation(String),

    // ── Not Found ─────────────────────────────────────────────────────────────
    #[error("{resource} not found")]
    NotFound { resource: String },

    // ── Economy ───────────────────────────────────────────────────────────────
    #[error("Insufficient funds: need {need} {currency}, have {have}")]
    InsufficientFunds { need: i64, have: i64, currency: String },

    #[error("Daily earning velocity cap exceeded")]
    VelocityCapExceeded,

    #[error("Ledger integrity check failed: hash mismatch")]
    LedgerIntegrityFailure,

    // ── Learning ──────────────────────────────────────────────────────────────
    #[error("Gate locked: {reason}")]
    GateLocked { reason: String },

    #[error("Gate on cooldown — retry after {retry_after}")]
    GateCooldown { retry_after: chrono::DateTime<chrono::Utc> },

    // ── Rate Limiting ─────────────────────────────────────────────────────────
    #[error("Rate limit exceeded — slow down")]
    RateLimited,

    // ── AI Gateway ────────────────────────────────────────────────────────────
    #[error("AI gateway error: {0}")]
    AiGateway(String),

    #[error("AI monthly budget exhausted")]
    AiBudgetExhausted,

    #[error("Daily AI budget exhausted — try again tomorrow")]
    AiDailyBudgetExhausted,

    // ── External Services ─────────────────────────────────────────────────────
    #[error("Email delivery failed: {0}")]
    EmailFailed(String),

    #[error("HTTP client error: {0}")]
    HttpClient(#[from] reqwest::Error),

    // ── Internal ──────────────────────────────────────────────────────────────
    #[error("Internal server error: {0}")]
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code, message) = match &self {
            AppError::InvalidCredentials        => (StatusCode::UNAUTHORIZED,   "INVALID_CREDENTIALS",   self.to_string()),
            AppError::AccountLocked { .. }      => (StatusCode::FORBIDDEN,      "ACCOUNT_LOCKED",        self.to_string()),
            AppError::SessionExpired            => (StatusCode::UNAUTHORIZED,   "SESSION_EXPIRED",       self.to_string()),
            AppError::Forbidden { .. }          => (StatusCode::FORBIDDEN,      "FORBIDDEN",             self.to_string()),
            AppError::Validation(_)             => (StatusCode::BAD_REQUEST,    "VALIDATION_ERROR",      self.to_string()),
            AppError::NotFound { .. }           => (StatusCode::NOT_FOUND,      "NOT_FOUND",             self.to_string()),
            AppError::InsufficientFunds { .. }  => (StatusCode::PAYMENT_REQUIRED, "INSUFFICIENT_FUNDS", self.to_string()),
            AppError::VelocityCapExceeded       => (StatusCode::TOO_MANY_REQUESTS, "VELOCITY_CAP",      self.to_string()),
            AppError::RateLimited               => (StatusCode::TOO_MANY_REQUESTS, "RATE_LIMITED",      self.to_string()),
            AppError::GateLocked { .. }         => (StatusCode::FORBIDDEN,      "GATE_LOCKED",           self.to_string()),
            AppError::GateCooldown { .. }       => (StatusCode::TOO_MANY_REQUESTS, "GATE_COOLDOWN",     self.to_string()),
            AppError::AiBudgetExhausted         => (StatusCode::SERVICE_UNAVAILABLE, "AI_BUDGET_EXHAUSTED", self.to_string()),
            AppError::AiDailyBudgetExhausted    => (StatusCode::TOO_MANY_REQUESTS, "AI_DAILY_BUDGET_EXHAUSTED", self.to_string()),
            AppError::AiGateway(_)              => (StatusCode::BAD_GATEWAY,    "AI_GATEWAY_ERROR",      self.to_string()),
            AppError::EmailFailed(_)            => (StatusCode::BAD_GATEWAY,    "EMAIL_FAILED",          self.to_string()),
            AppError::LedgerIntegrityFailure    => (StatusCode::INTERNAL_SERVER_ERROR, "LEDGER_INTEGRITY", self.to_string()),
            _                                   => (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "An internal error occurred".to_string()),
        };

        // Log internal errors
        if status.is_server_error() {
            tracing::error!(error = %self, "Server error");
        }

        (status, Json(json!({
            "error": {
                "code":    code,
                "message": message,
                "status":  status.as_u16(),
            }
        }))).into_response()
    }
}
