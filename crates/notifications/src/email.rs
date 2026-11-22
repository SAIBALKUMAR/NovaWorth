//! Transactional email delivery via the Resend HTTP API.

use common::constants::RESEND_MONTHLY_CAP;
use common::errors::AppError;
use reqwest::Client;
use serde_json::json;

/// Sends HTML transactional emails through Resend.
pub struct EmailService {
    client: Client,
    api_key: String,
    from: String,
}

impl EmailService {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            from: "Education Economy <no-reply@yourdomain.com>".to_string(),
        }
    }

    pub fn with_from(mut self, from: impl Into<String>) -> Self {
        self.from = from.into();
        self
    }

    /// Low-level send.
    pub async fn send(&self, to: &str, subject: &str, html: &str) -> Result<(), AppError> {
        let body = json!({
            "from": self.from,
            "to": [to],
            "subject": subject,
            "html": html
        });

        let resp = self
            .client
            .post("https://api.resend.com/emails")
            .bearer_auth(&self.api_key)
            .json(&body)
            .send()
            .await
            .map_err(|e| AppError::EmailFailed(e.to_string()))?;

        let status = resp.status();
        if !status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            return Err(AppError::EmailFailed(format!(
                "Resend status {status}: {text}"
            )));
        }
        Ok(())
    }

    /// Welcome email for a newly registered learner.
    pub async fn send_welcome(&self, to: &str, username: &str) -> Result<(), AppError> {
        let html = format!(
            "<h1>🎓 Welcome, {username}!</h1><p>Your learning journey starts now.</p>"
        );
        self.send(to, "Welcome to Education Economy", &html).await
    }

    /// Level-complete email with the XP/EC awarded.
    pub async fn send_level_complete(
        &self,
        to: &str,
        username: &str,
        level_name: &str,
        xp: i64,
        ec: i64,
    ) -> Result<(), AppError> {
        let html = format!(
            r#"<h1>🎓 Level complete!</h1>
               <p>Hey <strong>{username}</strong>, you passed <strong>{level_name}</strong>!</p>
               <p><span style="color:#f59e0b">+{xp} XP</span> ·
                  <span style="color:#10b981">+{ec} EC</span></p>"#
        );
        self.send(to, "🎓 You passed a level!", &html).await
    }
}

/// Return the Resend free-tier monthly cap (documentation/telemetry helper).
pub fn monthly_email_cap() -> u32 {
    RESEND_MONTHLY_CAP
}