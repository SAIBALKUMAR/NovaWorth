-- Migration: 0010_security_auth
-- Server-side exam grading, rate limiting, IP-scoped lockout, password reset,
-- TOTP MFA, and per-user session tracking.

-- ── 1. Server-side exam grading ──────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS quiz_items (
    id            UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    level_id      UUID        NOT NULL REFERENCES levels(id) ON DELETE CASCADE,
    prompt        TEXT        NOT NULL,
    options       JSONB       NOT NULL,             -- ["A", "B", ...]
    correct_index INT         NOT NULL CHECK (correct_index >= 0),
    points        INT         NOT NULL DEFAULT 1 CHECK (points > 0),
    sort_order    INT         NOT NULL DEFAULT 0,
    version       TEXT        NOT NULL DEFAULT '1.0.0',
    created_at    TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_quiz_items_level ON quiz_items (level_id);

CREATE TABLE IF NOT EXISTS quiz_responses (
    id           UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    attempt_id   UUID        NOT NULL,             -- references attempts(id)
    item_id      UUID        NOT NULL REFERENCES quiz_items(id) ON DELETE CASCADE,
    chosen_index INT         NOT NULL,
    correct      BOOLEAN     NOT NULL,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_quiz_responses_attempt ON quiz_responses (attempt_id);

-- ── 2. Fixed-window rate limiting (DB-backed; Redis/fred later) ──────────────
CREATE TABLE IF NOT EXISTS rate_limits (
    key          TEXT        PRIMARY KEY,
    window_start TIMESTAMPTZ NOT NULL DEFAULT now(),
    count        INT         NOT NULL DEFAULT 0
);

-- ── 3. IP-scoped login lockout ───────────────────────────────────────────────
ALTER TABLE login_attempts ADD COLUMN IF NOT EXISTS ip_hash TEXT NOT NULL DEFAULT '';
ALTER TABLE login_attempts DROP CONSTRAINT IF EXISTS login_attempts_pkey;
ALTER TABLE login_attempts ADD PRIMARY KEY (email, ip_hash);

-- ── 4. Password reset ────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS password_reset_tokens (
    token      TEXT        PRIMARY KEY,
    user_id    UUID        NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_reset_user ON password_reset_tokens (user_id);

-- ── 5. TOTP MFA ──────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS user_totp (
    user_id    UUID        PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    secret     TEXT        NOT NULL,               -- hex-encoded 32-byte secret
    confirmed  BOOLEAN     NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- ── 6. Per-user session tracking (sign-out-everywhere) ──────────────────────
CREATE TABLE IF NOT EXISTS user_sessions (
    session_id TEXT        PRIMARY KEY,
    user_id    UUID        NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    ip_hash    TEXT        NOT NULL DEFAULT '',
    user_agent TEXT        NOT NULL DEFAULT '',
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_user_sessions_user ON user_sessions (user_id);