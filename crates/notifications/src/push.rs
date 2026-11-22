//! Web Push (VAPID).
//!
//! Data model + delivery skeleton. Full VAPID delivery requires ECDSA P-256 signing
//! of a JWT (a `p256`/`jose` dependency); that is intentionally deferred to keep the
//! crate dependency-light. See `common::constants` for related limits.

use common::errors::AppError;

/// A browser push subscription (POSTed by the client).
#[derive(Debug, Clone, serde::Deserialize)]
pub struct PushSubscription {
    pub endpoint: String,
    pub p256dh: String,
    pub auth: String,
}

/// A push notification payload.
#[derive(Debug, Clone, serde::Serialize)]
pub struct WebPushMessage {
    pub title: String,
    pub body: String,
    pub url: Option<String>,
}

pub struct PushService;

impl PushService {
    /// Deliver a push message. Signing keys are required in a real deployment.
    pub async fn send(
        &self,
        _subscription: &PushSubscription,
        _message: &WebPushMessage,
    ) -> Result<(), AppError> {
        // NOTE: wire VAPID (RFC 8292) here with a p256-based JWT once keys are
        // provisioned. The transport is a standard HTTP POST to `subscription.endpoint`.
        Err(AppError::Internal(
            "Web Push delivery requires VAPID signing keys".into(),
        ))
    }
}