-- Migration: 0003_learning
-- Skill/level catalog, learner progress, and assessment attempts.

-- ── Domains (20, one per taxonomy folder) ─────────────────────────────────────────
CREATE TABLE IF NOT EXISTS domains (
    code       TEXT           PRIMARY KEY,
    name       TEXT           NOT NULL,
    multiplier NUMERIC(5,2)   NOT NULL DEFAULT 1.0
);

-- ── Skills (500 total) ────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS skills (
    id                TEXT        PRIMARY KEY,               -- e.g. 'CMP-06'
    domain_code       TEXT        NOT NULL REFERENCES domains(code),
    name              TEXT        NOT NULL,
    archetype         TEXT        NOT NULL DEFAULT 'foundation',
    level_100_outcome TEXT,
    safety_class      TEXT        NOT NULL DEFAULT 'standard',
    version           TEXT        NOT NULL DEFAULT '1.0.0',
    created_at        TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_skills_domain ON skills (domain_code);

-- ── Levels (100 per skill) ────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS levels (
    id             UUID           PRIMARY KEY DEFAULT gen_random_uuid(),
    skill_id       TEXT           NOT NULL REFERENCES skills(id),
    level          INT            NOT NULL CHECK (level BETWEEN 1 AND 100),
    title          TEXT,
    outcome        TEXT,
    expected_hours NUMERIC(10,2),
    difficulty     NUMERIC(12,2),
    version        TEXT           NOT NULL DEFAULT '1.0.0',
    UNIQUE (skill_id, level)
);

CREATE INDEX IF NOT EXISTS idx_levels_skill ON levels (skill_id);

-- ── Learner ↔ skill enrolment + highest level ─────────────────────────────────────
CREATE TABLE IF NOT EXISTS learner_skills (
    user_id       UUID        NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    skill_id      TEXT        NOT NULL REFERENCES skills(id),
    current_level INT         NOT NULL DEFAULT 0,
    mastery       NUMERIC(5,4) NOT NULL DEFAULT 0,           -- BKT posterior 0..1
    status        TEXT        NOT NULL DEFAULT 'active',
    updated_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (user_id, skill_id)
);

-- ── Per-level progress ────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS learner_progress (
    user_id        UUID        NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    level_id       UUID        NOT NULL REFERENCES levels(id),
    status         TEXT        NOT NULL DEFAULT 'not_started', -- not_started | in_progress | passed | failed
    mastery        NUMERIC(5,4) NOT NULL DEFAULT 0,
    attempts       INT         NOT NULL DEFAULT 0,
    gate_passed_at TIMESTAMPTZ,
    updated_at     TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (user_id, level_id)
);

-- ── Assessment attempts ───────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS attempts (
    id              UUID           PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id         UUID           NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    level_id        UUID           NOT NULL REFERENCES levels(id),
    kind            TEXT           NOT NULL DEFAULT 'practice', -- practice | gate | diagnostic
    score           NUMERIC(5,4),
    integrity_score NUMERIC(5,4)    NOT NULL DEFAULT 1.0,
    status          TEXT           NOT NULL DEFAULT 'pending',  -- pending | passed | failed
    rule_version    TEXT           NOT NULL DEFAULT 'score-2026.1',
    started_at      TIMESTAMPTZ    NOT NULL DEFAULT now(),
    submitted_at    TIMESTAMPTZ
);

CREATE INDEX IF NOT EXISTS idx_attempts_user_level ON attempts (user_id, level_id);
CREATE INDEX IF NOT EXISTS idx_attempts_submitted  ON attempts (submitted_at DESC);