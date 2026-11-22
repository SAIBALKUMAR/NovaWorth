-- Migration: 0004_marketplace
-- Products, orders, escrow holdings and disputes.

-- ── Curated catalog ───────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS products (
    id          UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    name        TEXT        NOT NULL,
    description TEXT,
    category    TEXT        NOT NULL DEFAULT 'essentials',  -- essentials | tools | services | experiences | learning | status
    price_ec    BIGINT      NOT NULL CHECK (price_ec > 0),
    stock       INT         NOT NULL DEFAULT 0,
    active      BOOLEAN     NOT NULL DEFAULT TRUE,
    merchant_id UUID,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_products_active ON products (active, category);

-- ── Orders ────────────────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS orders (
    id           UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id      UUID        NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    status       TEXT        NOT NULL DEFAULT 'pending',     -- pending | escrowed | fulfilled | cancelled | refunded
    ec_total     BIGINT      NOT NULL CHECK (ec_total > 0),
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at   TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_orders_user ON orders (user_id, status);

-- ── Order line items ──────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS order_items (
    id        UUID   PRIMARY KEY DEFAULT gen_random_uuid(),
    order_id  UUID   NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
    product_id UUID  NOT NULL REFERENCES products(id),
    quantity  INT    NOT NULL CHECK (quantity > 0),
    price_ec  BIGINT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_order_items_order ON order_items (order_id);

-- ── Escrow holds (funds locked until fulfillment) ─────────────────────────────────
CREATE TABLE IF NOT EXISTS escrow_holds (
    id          UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    order_id    UUID        NOT NULL REFERENCES orders(id),
    user_id     UUID        NOT NULL REFERENCES users(id),
    amount      BIGINT      NOT NULL CHECK (amount > 0),
    status      TEXT        NOT NULL DEFAULT 'held',          -- held | released | refunded
    released_at TIMESTAMPTZ,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_escrow_order ON escrow_holds (order_id, status);

-- ── Disputes ──────────────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS disputes (
    id          UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    order_id    UUID        NOT NULL REFERENCES orders(id),
    user_id     UUID        NOT NULL REFERENCES users(id),
    reason      TEXT        NOT NULL,
    status      TEXT        NOT NULL DEFAULT 'open',          -- open | resolved_restitution | resolved_refund | rejected
    resolution  TEXT,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    resolved_at TIMESTAMPTZ
);

CREATE INDEX IF NOT EXISTS idx_disputes_status ON disputes (status);