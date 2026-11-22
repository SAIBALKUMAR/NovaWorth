-- Migration: 0009_security_hardening
-- Resolves four P0 holes: EC invariant (unbacked mint), email verification,
-- assessment integrity telemetry, and the security event audit log.

-- ── 1. EC invariant: sponsor pools can never go negative ────────────────────
ALTER TABLE sponsor_pools
    ADD CONSTRAINT sponsor_pools_reserved_nonneg CHECK (reserved_ec >= 0),
    ADD CONSTRAINT sponsor_pools_emitted_nonneg  CHECK (emitted_ec  >= 0),
    ADD CONSTRAINT sponsor_pools_redeemed_nonneg CHECK (redeemed_ec >= 0);

-- ── 2. Email verification ───────────────────────────────────────────────────
ALTER TABLE users ADD COLUMN IF NOT EXISTS email_verified_at TIMESTAMPTZ;

CREATE TABLE IF NOT EXISTS email_verification_tokens (
    token      TEXT        PRIMARY KEY,               -- short-lived bearer secret
    user_id    UUID        NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_email_verify_user ON email_verification_tokens (user_id);

-- ── 3. Assessment integrity telemetry (further anti-proxy/proctoring signals) ─
ALTER TABLE attempts
    ADD COLUMN IF NOT EXISTS duration_secs  INTEGER NOT NULL DEFAULT 0,
    ADD COLUMN IF NOT EXISTS paste_events   INTEGER NOT NULL DEFAULT 0,
    ADD COLUMN IF NOT EXISTS focus_losses   INTEGER NOT NULL DEFAULT 0,
    ADD COLUMN IF NOT EXISTS tab_switches   INTEGER NOT NULL DEFAULT 0;

-- ── 4. Security event audit log (append-only) ───────────────────────────────
CREATE TABLE IF NOT EXISTS security_events (
    id         UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    actor_id   UUID        REFERENCES users(id) ON DELETE SET NULL,
    kind       TEXT        NOT NULL,                -- login | register | verify | gate_flag | rate_limit | supply_ingest | admin_action
    severity   TEXT        NOT NULL DEFAULT 'info', -- info | warn | critical
    detail     JSONB,
    ip_hash    TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_security_events_actor ON security_events (actor_id, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_security_events_kind  ON security_events (kind, created_at DESC);