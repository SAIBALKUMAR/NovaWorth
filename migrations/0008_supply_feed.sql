-- Migration: 0008_supply_feed
-- Live supply observations, materialized supply state, demand signals, and the
-- deterministic price-tick audit trail for the dynamic-pricing engine.

-- ── Append-only supply observations ─────────────────────────────────────────
CREATE TABLE IF NOT EXISTS supply_events (
    id              UUID            PRIMARY KEY DEFAULT gen_random_uuid(),
    product_id      UUID            NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    region          TEXT            NOT NULL DEFAULT 'US_EU',
    qty_on_hand     BIGINT,                     -- NULL for a pure AI forecast
    incoming_rate   DOUBLE PRECISION NOT NULL DEFAULT 0,  -- units per hour
    lead_time_hours DOUBLE PRECISION NOT NULL DEFAULT 0,
    source          TEXT            NOT NULL CHECK (source IN ('merchant_push', 'ai_inference', 'sensor')),
    confidence      DOUBLE PRECISION NOT NULL DEFAULT 1.0 CHECK (confidence >= 0 AND confidence <= 1),
    as_of           TIMESTAMPTZ      NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_supply_events_product ON supply_events (product_id, as_of DESC);

-- ── Materialized current supply state (read by the price engine) ───────────
CREATE TABLE IF NOT EXISTS product_supply_state (
    product_id      UUID            PRIMARY KEY REFERENCES products(id) ON DELETE CASCADE,
    region          TEXT            NOT NULL DEFAULT 'US_EU',
    qty_on_hand     BIGINT          NOT NULL DEFAULT 0,
    incoming_rate   DOUBLE PRECISION NOT NULL DEFAULT 0,
    lead_time_hours DOUBLE PRECISION NOT NULL DEFAULT 0,
    trust_level     DOUBLE PRECISION NOT NULL DEFAULT 1.0,
    source          TEXT            NOT NULL DEFAULT 'merchant_push',
    updated_at      TIMESTAMPTZ      NOT NULL DEFAULT now()
);

-- ── Demand signals (orders, carts, wishlists, searches) ─────────────────────
CREATE TABLE IF NOT EXISTS demand_events (
    id             UUID           PRIMARY KEY DEFAULT gen_random_uuid(),
    product_id     UUID           NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    region         TEXT           NOT NULL DEFAULT 'US_EU',
    kind           TEXT           NOT NULL CHECK (kind IN ('order', 'cart', 'wishlist', 'search')),
    event_count    BIGINT         NOT NULL DEFAULT 0,
    window_minutes INTEGER        NOT NULL DEFAULT 60,
    as_of          TIMESTAMPTZ     NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_demand_events_product ON demand_events (product_id, as_of DESC);

-- ── Price audit trail (append-only; one row per evaluation) ────────────────
CREATE TABLE IF NOT EXISTS price_ticks (
    id            UUID            PRIMARY KEY DEFAULT gen_random_uuid(),
    product_id    UUID            NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    region        TEXT            NOT NULL DEFAULT 'US_EU',
    base_ec       BIGINT          NOT NULL,
    supply_factor DOUBLE PRECISION NOT NULL,
    demand_factor DOUBLE PRECISION NOT NULL,
    ec_price      BIGINT          NOT NULL,
    reason        TEXT            NOT NULL,
    as_of         TIMESTAMPTZ      NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_price_ticks_product ON price_ticks (product_id, as_of DESC);