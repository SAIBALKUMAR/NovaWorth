-- Migration: 0006_notifications
-- In-app notifications and email delivery log.

-- ── In-app notifications ──────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS notifications (
    id         UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id    UUID        NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    kind       TEXT        NOT NULL DEFAULT 'system',        -- system | level_up | economy | social | marketplace
    title      TEXT        NOT NULL,
    body       TEXT,
    read       BOOLEAN     NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_notifications_user ON notifications (user_id, read, created_at DESC);

-- ── Transactional email log (for Resend deliverability tracking) ──────────────────
CREATE TABLE IF NOT EXISTS email_log (
    id         UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id    UUID        REFERENCES users(id) ON DELETE SET NULL,
    to_email   TEXT        NOT NULL,
    subject    TEXT        NOT NULL,
    status     TEXT        NOT NULL DEFAULT 'sent',          -- sent | failed | bounced
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_email_log_created ON email_log (created_at DESC);