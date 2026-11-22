//! OAuth2 provider configurations for GitHub and Google.

use oauth2::{
    basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl,
};
use common::config::AppConfig;

/// Build the GitHub OAuth2 client from configuration.
pub fn github_client(cfg: &AppConfig) -> BasicClient {
    BasicClient::new(ClientId::new(cfg.github_client_id.clone()))
        .set_client_secret(ClientSecret::new(cfg.github_client_secret.clone()))
        .set_auth_uri(
            AuthUrl::new("https://github.com/login/oauth/authorize".to_string())
                .expect("Invalid GitHub auth URL"),
        )
        .set_token_uri(
            TokenUrl::new("https://github.com/login/oauth/access_token".to_string())
                .expect("Invalid GitHub token URL"),
        )
        .set_redirect_uri(
            RedirectUrl::new(cfg.github_redirect_url.clone())
                .expect("Invalid GitHub redirect URL"),
        )
}

/// Build the Google OAuth2 client from configuration.
pub fn google_client(cfg: &AppConfig) -> BasicClient {
    BasicClient::new(ClientId::new(cfg.google_client_id.clone()))
        .set_client_secret(ClientSecret::new(cfg.google_client_secret.clone()))
        .set_auth_uri(
            AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
                .expect("Invalid Google auth URL"),
        )
        .set_token_uri(
            TokenUrl::new("https://oauth2.googleapis.com/token".to_string())
                .expect("Invalid Google token URL"),
        )
        .set_redirect_uri(
            RedirectUrl::new(cfg.google_redirect_url.clone())
                .expect("Invalid Google redirect URL"),
        )
}
