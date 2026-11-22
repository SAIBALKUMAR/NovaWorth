//! Cloudflare Turnstile verification.

use common::errors::AppError;
use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize)]
struct SiteVerify {
    success: bool,
}

/// Verify a Turnstile token. An empty `secret` disables the check (dev mode).
pub async fn verify(client: &Client, secret: &str, token: &str) -> Result<(), AppError> {
    if secret.is_empty() {
        return Ok(());
    }
    if token.is_empty() {
        return Err(AppError::Validation("Captcha required".into()));
    }

    let resp: SiteVerify = client
        .post("https://challenges.cloudflare.com/turnstile/v0/siteverify")
        .form(&[("secret", secret), ("response", token)])
        .send()
        .await
        .map_err(AppError::HttpClient)?
        .json()
        .await
        .map_err(AppError::HttpClient)?;

    if !resp.success {
        return Err(AppError::Validation("Captcha verification failed".into()));
    }
    Ok(())
}