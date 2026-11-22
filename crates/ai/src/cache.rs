//! In-process semantic cache with TTL.
//!
//! A single-instance cache keyed by the hash of `(task, system, user)`. Swap the
//! backing store for Redis in a multi-replica deployment (the Redis key prefix is
//! already defined in `common::constants`).

use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::Mutex;
use std::time::{Duration, Instant};

struct Entry {
    value: String,
    expires_at: Instant,
}

/// Thread-safe in-memory cache.
#[derive(Default)]
pub struct SemanticCache {
    inner: Mutex<HashMap<String, Entry>>,
}

impl SemanticCache {
    pub fn new() -> Self {
        Self::default()
    }

    /// Deterministic cache key from task/system/user.
    pub fn key_for(task: &str, system: &str, user: &str) -> String {
        let mut h = DefaultHasher::new();
        task.hash(&mut h);
        system.hash(&mut h);
        user.hash(&mut h);
        format!("ai:cache:{:x}", h.finish())
    }

    /// Return a cached value if present and unexpired.
    pub fn get(&self, key: &str) -> Option<String> {
        let mut map = self.inner.lock().ok()?;
        match map.get(key) {
            Some(entry) if entry.expires_at > Instant::now() => Some(entry.value.clone()),
            _ => {
                map.remove(key);
                None
            }
        }
    }

    /// Store a value for `ttl`.
    pub fn set(&self, key: &str, value: String, ttl: Duration) {
        if let Ok(mut map) = self.inner.lock() {
            map.insert(
                key.to_string(),
                Entry {
                    value,
                    expires_at: Instant::now() + ttl,
                },
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_is_deterministic() {
        assert_eq!(
            SemanticCache::key_for("a", "b", "c"),
            SemanticCache::key_for("a", "b", "c")
        );
        assert_ne!(
            SemanticCache::key_for("a", "b", "c"),
            SemanticCache::key_for("x", "b", "c")
        );
    }

    #[test]
    fn get_returns_set_value() {
        let c = SemanticCache::new();
        c.set("k", "hello".into(), Duration::from_secs(60));
        assert_eq!(c.get("k").as_deref(), Some("hello"));
    }

    #[test]
    fn expired_entries_evict() {
        let c = SemanticCache::new();
        c.set("k", "hello".into(), Duration::from_millis(1));
        std::thread::sleep(Duration::from_millis(5));
        assert!(c.get("k").is_none());
    }
}