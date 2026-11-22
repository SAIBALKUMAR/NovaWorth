//! Global constants — no magic numbers/strings anywhere else in the codebase.

// ── Economy ─────────────────────────────────────────────────────────────────
/// Minimum mastery probability required to pass a level gate (BKT posterior)
pub const GATE_MASTERY_THRESHOLD: f64 = 0.80;

/// Maximum XP that can be awarded for a single level gate pass
pub const MAX_XP_PER_GATE: i64 = 10_000;

/// Maximum EC that can be awarded per gate (anti-abuse)
pub const MAX_EC_PER_GATE: i64 = 500;

/// EC velocity cap: max EC earnable per 24-hour rolling window
pub const EC_DAILY_VELOCITY_CAP: i64 = 2_000;

/// Repeat-attempt decay factor (each re-attempt awards this fraction of base)
pub const EC_REPEAT_DECAY: f64 = 0.5;

/// Marketplace commission split: merchant receives this fraction
pub const MERCHANT_SPLIT: f64 = 0.92;
/// Platform fee fraction
pub const PLATFORM_FEE: f64 = 0.08;

// ── Learning ─────────────────────────────────────────────────────────────────
/// Default BKT prior P(known) for a new skill
pub const BKT_PRIOR: f64 = 0.30;
/// Default BKT P(learn | unknown) transition probability
pub const BKT_TRANSIT: f64 = 0.10;
/// Default BKT P(slip | known)
pub const BKT_SLIP: f64 = 0.08;
/// Default BKT P(guess | unknown)
pub const BKT_GUESS: f64 = 0.20;

/// SM-2 initial easiness factor
pub const SM2_INITIAL_EF: f64 = 2.5;
/// SM-2 minimum easiness factor
pub const SM2_MIN_EF: f64 = 1.3;

// ── Gate retry cooldowns (hours) ─────────────────────────────────────────────
pub const GATE_COOLDOWN_ATTEMPT_1_H: i64 = 1;
pub const GATE_COOLDOWN_ATTEMPT_2_H: i64 = 4;
pub const GATE_COOLDOWN_ATTEMPT_3_PLUS_H: i64 = 24;

/// Score required to pass a gate assessment (0.0–1.0)
pub const GATE_PASS_SCORE: f64 = 0.75;

// ── Auth ─────────────────────────────────────────────────────────────────────
/// Session cookie name
pub const SESSION_COOKIE_NAME: &str = "ee_session";

/// Session TTL in seconds (24 hours)
pub const SESSION_TTL_SECS: i64 = 86_400;

/// Maximum failed login attempts before account lockout
pub const MAX_FAILED_LOGIN_ATTEMPTS: u32 = 5;

/// Account lockout duration in minutes
pub const LOCKOUT_DURATION_MINS: u64 = 15;

/// Minimum password length
pub const MIN_PASSWORD_LEN: usize = 8;

/// Maximum password length (Argon2 input cap)
pub const MAX_PASSWORD_LEN: usize = 128;

// ── API ───────────────────────────────────────────────────────────────────────
/// Default page size for paginated endpoints
pub const DEFAULT_PAGE_SIZE: u32 = 20;
/// Maximum page size allowed
pub const MAX_PAGE_SIZE: u32 = 100;

/// Request timeout in seconds for outgoing HTTP calls
pub const OUTGOING_HTTP_TIMEOUT_SECS: u64 = 15;

// ── AI / GenAI ───────────────────────────────────────────────────────────────
/// OpenAI model for most tasks
pub const OPENAI_MODEL_DEFAULT: &str = "gpt-4o-mini";

/// Max tokens for explanation tasks
pub const AI_MAX_TOKENS_EXPLANATION: u32 = 512;
/// Max tokens for feedback tasks
pub const AI_MAX_TOKENS_FEEDBACK: u32 = 384;
/// Max tokens for content draft tasks
pub const AI_MAX_TOKENS_DRAFT: u32 = 1_024;

/// Semantic cache TTL in seconds (7 days)
pub const AI_CACHE_TTL_SECS: u64 = 604_800;

/// Monthly token budget (gpt-4o-mini @ $0.15/1M tokens → $20 ≈ 133M tokens)
pub const AI_MONTHLY_TOKEN_CAP: u64 = 130_000_000;

/// Per-learner daily token cap (≈ $0.01–0.02/day at 4o-mini pricing).
pub const AI_DAILY_TOKEN_CAP_PER_USER: u64 = 50_000;

/// Redis key for monthly token counter
pub const REDIS_KEY_AI_TOKENS: &str = "ai:monthly_tokens_spent";

// ── Redis Keys ────────────────────────────────────────────────────────────────
pub const REDIS_KEY_SESSION_PREFIX: &str = "session:";
pub const REDIS_KEY_RATE_LIMIT_PREFIX: &str = "rl:";
pub const REDIS_KEY_LEAGUE_PREFIX: &str = "league:season:";
pub const REDIS_KEY_AI_CACHE_PREFIX: &str = "ai:cache:";

// ── Notifications ─────────────────────────────────────────────────────────────
/// Resend free tier monthly email cap
pub const RESEND_MONTHLY_CAP: u32 = 3_000;

// ── Leagues ───────────────────────────────────────────────────────────────────
/// Season duration in weeks
pub const LEAGUE_SEASON_WEEKS: u64 = 12;
/// Top % promoted at season end
pub const LEAGUE_PROMOTION_PCT: f64 = 0.20;
/// Bottom % relegated at season end
pub const LEAGUE_RELEGATION_PCT: f64 = 0.20;

// ── Social ────────────────────────────────────────────────────────────────────
/// Maximum members per guild
pub const GUILD_MAX_MEMBERS: u32 = 50;
/// Minimum level to found a guild
pub const GUILD_FOUNDER_MIN_LEVEL: u32 = 10;
/// Minimum level to become a mentor
pub const MENTOR_MIN_LEVEL: u32 = 60;

// ── Marketplace dynamic pricing & live supply ───────────────────────────────
/// Supply-shortage price sensitivity coefficient. Zero disables supply-driven pricing.
pub const PRICE_SUPPLY_SENSITIVITY: f64 = 0.2;
/// Demand-surge price sensitivity coefficient. Zero disables demand-driven pricing.
pub const PRICE_DEMAND_SENSITIVITY: f64 = 0.2;
/// Hard cap on dynamic price as a fraction above the cost floor (±20% policy).
pub const DYNAMIC_PRICE_CAP: f64 = 0.20;
/// Minimum seconds between successive price-tick writes for a product.
pub const PRICE_UPDATE_MIN_INTERVAL_SECS: u64 = 300;
/// Supply-event confidence below which an event is rejected as untrusted/stale.
pub const SUPPLY_MIN_CONFIDENCE: f64 = 0.20;
/// Default expected on-hand supply (units) used to detect a shortage.
pub const SUPPLY_TARGET_UNITS: i64 = 20;
/// Default demand baseline (units/window) for the demand-surge factor.
pub const DEMAND_BASELINE: f64 = 10.0;
/// Minimum gate attempt duration (seconds) considered plausible; faster completion
/// is flagged as low integrity (server-side timing heuristic).
pub const MIN_GATE_DURATION_SECS: i64 = 10;
