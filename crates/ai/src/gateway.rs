//! AI gateway — budget-aware LLM inference with guardrails and semantic caching.

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use common::constants::{AI_CACHE_TTL_SECS, AI_DAILY_TOKEN_CAP_PER_USER, AI_MONTHLY_TOKEN_CAP};
use common::errors::AppError;
use reqwest::Client;
use serde_json::json;

use crate::cache::SemanticCache;
use crate::guardrails;
use crate::prompts::AiTask;

/// Maps a task to a stable cache/task name.
fn task_name(task: AiTask) -> &'static str {
    match task {
        AiTask::Explanation => "explanation",
        AiTask::SocraticQuestion => "socratic",
        AiTask::FeedbackOnAnswer => "feedback",
        AiTask::ContentDraft => "content_draft",
        AiTask::Embedding => "embedding",
    }
}

/// The gateway: one entry point for all LLM work.
pub struct AiGateway {
    client: Client,
    api_key: String,
    cache: SemanticCache,
    /// Cumulative tokens consumed this billing period (instance-wide cap).
    spent_tokens: Arc<AtomicU64>,
    /// Hard token cap enforced on the whole instance per month.
    token_cap: u64,
    /// Per-user tokens consumed today, keyed by user id: (day, tokens).
    daily_usage: Mutex<HashMap<String, (u64, u64)>>,
    /// Per-user daily token cap.
    daily_cap: u64,
}

impl AiGateway {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            cache: SemanticCache::new(),
            spent_tokens: Arc::new(AtomicU64::new(0)),
            token_cap: AI_MONTHLY_TOKEN_CAP,
            daily_usage: Mutex::new(HashMap::new()),
            daily_cap: AI_DAILY_TOKEN_CAP_PER_USER,
        }
    }

    /// Override the instance-wide monthly token cap.
    pub fn with_token_cap(mut self, cap: u64) -> Self {
        self.token_cap = cap;
        self
    }

    /// Override the per-user daily token cap.
    pub fn with_daily_cap(mut self, cap: u64) -> Self {
        self.daily_cap = cap;
        self
    }

    /// Tokens consumed so far (monitoring signal).
    pub fn spent_tokens(&self) -> u64 {
        self.spent_tokens.load(Ordering::Relaxed)
    }

    /// Run a guarded, cached, budget-checked completion.
    pub async fn complete(
        &self,
        user_id: &str,
        task: AiTask,
        system: &str,
        user: &str,
    ) -> Result<String, AppError> {
        // 1. Guardrails on the user payload.
        let scan = guardrails::scan(user);
        if scan.injection_suspected {
            return Err(AppError::Validation(
                "Prompt rejected by safety guardrail".into(),
            ));
        }
        let user_clean = scan.redacted;

        // 2. Semantic cache.
        let key = SemanticCache::key_for(task_name(task), system, &user_clean);
        if let Some(hit) = self.cache.get(&key) {
            return Ok(hit);
        }

        // 3. Budget gates (instance-wide monthly + per-user daily).
        self.check_budget()?;
        self.check_daily_budget(user_id)?;

        // 4. Inference call.
        let body = json!({
            "model": task.model(),
            "messages": [
                { "role": "system", "content": system },
                { "role": "user",   "content": user_clean }
            ],
            "max_tokens": task.max_tokens(),
            "temperature": task.temperature()
        });

        let resp = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .bearer_auth(&self.api_key)
            .json(&body)
            .send()
            .await
            .map_err(|e| AppError::AiGateway(e.to_string()))?;

        let status = resp.status();
        let payload: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| AppError::AiGateway(e.to_string()))?;

        if !status.is_success() {
            return Err(AppError::AiGateway(payload.to_string()));
        }

        let content = payload["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or_default()
            .to_string();

        if let Some(tokens) = payload["usage"]["total_tokens"].as_u64() {
            self.record_usage(user_id, tokens);
        }

        // 5. Cache the response.
        self.cache
            .set(&key, content.clone(), Duration::from_secs(AI_CACHE_TTL_SECS));

        Ok(content)
    }

    fn check_budget(&self) -> Result<(), AppError> {
        if self.spent_tokens() >= self.token_cap {
            Err(AppError::AiBudgetExhausted)
        } else {
            Ok(())
        }
    }

    /// Current UTC day number (for daily rollover of per-user counters).
    fn current_day() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs() / 86_400)
            .unwrap_or(0)
    }

    /// Reject the call if this user has exhausted their daily allowance.
    fn check_daily_budget(&self, user_id: &str) -> Result<(), AppError> {
        let day = Self::current_day();
        let mut map = self.daily_usage.lock().unwrap_or_else(|e| e.into_inner());
        let entry = map.entry(user_id.to_string()).or_insert((day, 0));
        if entry.0 != day {
            *entry = (day, 0);
        }
        if entry.1 >= self.daily_cap {
            return Err(AppError::AiDailyBudgetExhausted);
        }
        Ok(())
    }

    fn record_usage(&self, user_id: &str, tokens: u64) {
        self.spent_tokens.fetch_add(tokens, Ordering::Relaxed);

        let day = Self::current_day();
        if let Ok(mut map) = self.daily_usage.lock() {
            let entry = map.entry(user_id.to_string()).or_insert((day, 0));
            if entry.0 != day {
                *entry = (day, 0);
            }
            entry.1 += tokens;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_names_match_variants() {
        assert_eq!(task_name(AiTask::Explanation), "explanation");
        assert_eq!(task_name(AiTask::ContentDraft), "content_draft");
    }
}