-- Migration: 0005_social
-- Guilds, memberships, reputation events, and league seasons.

-- ── Guilds ────────────────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS guilds (
    id          UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    name        TEXT        NOT NULL UNIQUE,
    description TEXT,
    founder_id  UUID        NOT NULL REFERENCES users(id),
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- ── Guild membership ──────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS guild_members (
    guild_id   UUID        NOT NULL REFERENCES guilds(id) ON DELETE CASCADE,
    user_id    UUID        NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role       TEXT        NOT NULL DEFAULT 'member',        -- founder | officer | member
    joined_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (guild_id, user_id)
);

CREATE INDEX IF NOT EXISTS idx_guild_members_user ON guild_members (user_id);

-- ── Reputation events (trust score inputs) ────────────────────────────────────────
CREATE TABLE IF NOT EXISTS reputation_events (
    id         UUID           PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id    UUID           NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    event_type TEXT           NOT NULL,                      -- peer_review | mentoring | integrity | bounty
    weight     NUMERIC(8,4)   NOT NULL,
    source_id  UUID,
    created_at TIMESTAMPTZ    NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_reputation_user ON reputation_events (user_id, created_at DESC);

-- ── League seasons (12-week cycles) ───────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS league_seasons (
    id         UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    season     INT         NOT NULL UNIQUE,
    started_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    ended_at   TIMESTAMPTZ
);