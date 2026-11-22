-- Migration: 0007_security
-- Brute-force login lockout tracking.

CREATE TABLE IF NOT EXISTS login_attempts (
    email        TEXT PRIMARY KEY,
    failed_count INT         NOT NULL DEFAULT 0,
    locked_until TIMESTAMPTZ,
    updated_at   TIMESTAMPTZ NOT NULL DEFAULT now()
);