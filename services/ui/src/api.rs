//! Typed REST client for the Education Economy API.
//!
//! Requests use a same-origin base (""), so in production Caddy routes
//! `/auth/*`, `/economy/*`, `/social/*` → the API and `/generate/*` → GenAI.
//! Cookies for the session are therefore sent automatically.

use serde::{Deserialize, Serialize};

use crate::state::AuthUser;

const API_BASE: &str = "";

#[derive(Serialize)]
struct LoginBody<'a> {
    email: &'a str,
    password: &'a str,
}

#[derive(Serialize)]
struct RegisterBody<'a> {
    email: &'a str,
    username: &'a str,
    password: &'a str,
}

#[derive(Serialize)]
struct ExplainBody<'a> {
    user_id: &'a str,
    topic: &'a str,
}

#[derive(Deserialize)]
pub struct Wallet {
    pub xp: i64,
    pub ec: i64,
}

#[derive(Deserialize)]
pub struct LeaderboardRow {
    pub username: String,
    pub xp_balance: i64,
}

#[derive(Deserialize)]
struct ExplainResponse {
    text: String,
}

/// Extract the human-readable message from the AppError envelope.
async fn error_message(resp: reqwest::Response) -> String {
    match resp.json::<serde_json::Value>().await {
        Ok(v) => v["error"]["message"]
            .as_str()
            .unwrap_or("Something went wrong")
            .to_string(),
        Err(_) => "Something went wrong".to_string(),
    }
}

pub async fn login(email: &str, password: &str) -> Result<AuthUser, String> {
    let resp = reqwest::Client::new()
        .post(format!("{API_BASE}/auth/login"))
        .json(&LoginBody { email, password })
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.status().is_success() {
        resp.json::<AuthUser>().await.map_err(|e| e.to_string())
    } else {
        Err(error_message(resp).await)
    }
}

pub async fn register(email: &str, username: &str, password: &str) -> Result<AuthUser, String> {
    let resp = reqwest::Client::new()
        .post(format!("{API_BASE}/auth/register"))
        .json(&RegisterBody {
            email,
            username,
            password,
        })
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.status().is_success() {
        resp.json::<AuthUser>().await.map_err(|e| e.to_string())
    } else {
        Err(error_message(resp).await)
    }
}

pub async fn wallet() -> Result<Wallet, String> {
    let resp = reqwest::Client::new()
        .get(format!("{API_BASE}/economy/wallet"))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.status().is_success() {
        resp.json::<Wallet>().await.map_err(|e| e.to_string())
    } else {
        Err(error_message(resp).await)
    }
}

pub async fn leaderboard(limit: i64) -> Result<Vec<LeaderboardRow>, String> {
    let resp = reqwest::Client::new()
        .get(format!("{API_BASE}/social/leaderboard?limit={limit}"))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.status().is_success() {
        resp.json::<Vec<LeaderboardRow>>()
            .await
            .map_err(|e| e.to_string())
    } else {
        Err(error_message(resp).await)
    }
}

pub async fn explain(user_id: &str, topic: &str) -> Result<String, String> {
    let resp = reqwest::Client::new()
        .post(format!("{API_BASE}/generate/explain"))
        .json(&ExplainBody { user_id, topic })
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.status().is_success() {
        let r: ExplainResponse = resp.json().await.map_err(|e| e.to_string())?;
        Ok(r.text)
    } else {
        Err(error_message(resp).await)
    }
}