-- Migration: 0002_economy
-- Sponsor pools, wallets, append-only double-entry ledger, and audit view.

-- ── Sponsor funding pools ────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS sponsor_pools (
    id          UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    name        TEXT        NOT NULL,
    purpose     TEXT,
    region      TEXT,
    budget_fiat BIGINT      NOT NULL DEFAULT 0,       -- funding in minor units (cents)
    anchor_rate NUMERIC(12,6) NOT NULL DEFAULT 0.05,  -- 1 EC subsidy value in fiat
    reserved_ec BIGINT      NOT NULL DEFAULT 0,       -- EC earmarked but not emitted
    emitted_ec  BIGINT      NOT NULL DEFAULT 0,       -- EC actually minted
    redeemed_ec BIGINT      NOT NULL DEFAULT 0,       -- EC spent back out of circulation
    status      TEXT        NOT NULL DEFAULT 'active',
    expires_at  TIMESTAMPTZ,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_sponsor_pools_status ON sponsor_pools (status);

-- ── Wallets (fast authoritative balance; ledger is source of truth) ──────────────
CREATE TABLE IF NOT EXISTS wallets (
    user_id    UUID        PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    xp_balance BIGINT      NOT NULL DEFAULT 0,
    ec_balance BIGINT      NOT NULL DEFAULT 0,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- ── Append-only double-entry ledger ───────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS ledger_entries (
    id              UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id         UUID        NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    entry_type      TEXT        NOT NULL CHECK (entry_type IN ('CREDIT', 'DEBIT')),
    currency        TEXT        NOT NULL CHECK (currency IN ('XP', 'EC')),
    amount          BIGINT      NOT NULL CHECK (amount > 0),
    balance_after   BIGINT      NOT NULL,
    reason          TEXT        NOT NULL,
    reference_id    UUID,
    funding_pool_id UUID        REFERENCES sponsor_pools(id),
    prev_hash       TEXT,
    entry_hash      TEXT        NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_ledger_user_currency ON ledger_entries (user_id, currency);
CREATE INDEX IF NOT EXISTS idx_ledger_created         ON ledger_entries (created_at DESC);
CREATE INDEX IF NOT EXISTS idx_ledger_reference       ON ledger_entries (reference_id);

-- ── Audit view: computed balance from the ledger only ─────────────────────────────
-- Reconciliation compares this view against wallets. A non-zero diff signals drift.
CREATE OR REPLACE VIEW user_balances AS
SELECT
    user_id,
    currency,
    COALESCE(SUM(CASE WHEN entry_type = 'CREDIT' THEN amount ELSE -amount END), 0) AS balance
FROM ledger_entries
GROUP BY user_id, currency;