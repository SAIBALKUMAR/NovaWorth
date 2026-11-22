# Live Supply & Pricing

## 1. Purpose

The marketplace starts from a **cost-plus floor** (`EC_price = (fiat_cost + fulfillment + margin) / anchor_rate`), stored as `products.price_ec`. This document adds a **live price counter** on top of that floor:

- A live, deterministic **EC price** driven by **supply** and **demand**.
- An **AI supply oracle** that uploads how much supply currently exists (stock) and how fast it replenishes (flow).

Price is always a **pure function of observed supply and observed demand** — never set by an AI/ML black box. The AI supplies *quantities and a confidence score*; a deterministic, auditable policy turns those into a price.

---

## 2. Supply is a flow, not a stock

| Category | What "supply" means |
|---|---|
| Physical goods | `on_hand` + `incoming_rate` × `lead_time_hours` + perishability |
| Services | provider *open time slots* (calendar capacity) |
| Digital / licences | near-unbounded, gated by licence/rate limits |
| AI services | compute/GPU + model rate limits + token budget |

Canonical supply record = **on-hand (authoritative) + replenishment forecast (AI)**.

---

## 3. How supply is decided & uploaded (the AI oracle)

Supply is never *invented*; it is **observed and forecast**. Two channels write `supply_events`:

1. **Ground truth (merchant push / sensor / ERP):** `qty_on_hand` is authoritative, `source ∈ {merchant_push, sensor}`, `confidence = 1.0`.
2. **AI inference (pull / forecast):** the oracle ingests exogenous signals — logistics/carrier telemetry, weather (perishables), energy spot price, model-provider capacity, merchant order history — and publishes `incoming_rate`, `lead_time_hours`, and a `confidence ∈ [0, 1)`.

**Merge rule (anti-gaming):**
- An event is rejected if `confidence < SUPPLY_MIN_CONFIDENCE` (stale/untrusted).
- A **higher-confidence** event wins for the replenishment forecast.
- `qty_on_hand` is updated **only** by ground-truth sources; an AI-only event never zeroes a merchant's stock.
- `trust_level` is the running maximum confidence.

An event is append-only; the **materialized** `product_supply_state` is what the price engine reads.

---

## 4. Price function

```
base_ec       = products.price_ec                      # cost floor (exists)

shortage      = clamp( (SUPPLY_TARGET - avail_now) / SUPPLY_TARGET, -1, 1 )
supply_factor = 1 + PRICE_SUPPLY_SENSITIVITY × shortage

surge         = clamp( (d_now - d_baseline) / d_baseline, -1, 1 )
demand_factor = max(1, 1 + PRICE_DEMAND_SENSITIVITY × surge)

ec_price      = round( base_ec × clamp(supply_factor × demand_factor, 1, 1 + DYNAMIC_PRICE_CAP) )
```

- `base_ec` is the hard **floor** — price never drops below cost (protects the 8% platform fee on settlement).
- `DYNAMIC_PRICE_CAP` caps the dynamic premium (default `+20%`, matching the existing §5 policy).
- Demand only applies **upward surge** pressure; the cost floor already handles weak demand.

Demand signal (`demand_now`) is a rolling sum of `demand_events` (`order`, `cart`, `wishlist`, `search`) over 24h. In Phase 0–2 this is a stub that returns 0 until the analytics pipeline (`docs/10-data-ai/05-analytics.md`) lands — so `demand_factor = 1.0`.

---

## 5. Guardrails (non-negotiable)

- **Essentials never use live upward pricing** — cap + waitlist + priority (existing `02-catalog-pricing.md` §4).
- **Never personalized**; price is a per-product, per-region public value.
- **Bounded** (`±20%`) and **versioned** — every evaluation writes a `price_ticks` row.
- **Smoothing**: don't reprice more often than `PRICE_UPDATE_MIN_INTERVAL_SECS`.
- **Below-cost never allowed** (`ec_price ≥ base_ec`).
- **Anti-gaming:** supply is confidence-weighted; merchants under-reporting supply are down-weighted by their integrity/`trust_level`.

---

## 6. Schema

`migrations/0008_supply_feed.sql`:

| Table | Role |
|---|---|
| `supply_events` | append-only supply observations (merchant/AI/sensor) |
| `product_supply_state` | materialized current supply read by the price engine |
| `demand_events` | demand signals (orders, carts, wishlists, searches) |
| `price_ticks` | append-only price audit trail |

---

## 7. API

| Method | Path | Purpose |
|---|---|---|
| `GET` | `/marketplace/quote/:product_id` | compute and return the live price + supply status |
| `POST` | `/marketplace/supply` | ingest a supply observation (merchant push / AI inference) |

`POST /marketplace/supply` body:

```json
{
  "product_id": "…",
  "region": "US_EU",
  "qty_on_hand": 120,
  "incoming_rate": 5.0,
  "lead_time_hours": 48,
  "source": "ai_inference",
  "confidence": 0.8
}
```

---

## 8. Requirements

- `FR-MKT-501`: price is a deterministic function of supply + demand, never an ML output.
- `FR-MKT-502`: `ec_price` is never below the cost floor.
- `FR-MKT-503`: dynamic price is bounded by `DYNAMIC_PRICE_CAP`.
- `FR-MKT-504`: every supply event is append-only and confidence-gated.
- `FR-MKT-505`: every price evaluation writes a `price_ticks` row.
- `FR-MKT-506`: essentials are exempt from live upward pricing.
- `NFR-MKT-501`: quote latency < 100 ms.

---

## 9. Risks & open items

- **AI-inferred supply is gamed** → confidence + merchant integrity score; oracle is paid for accuracy, not profit.
- **Volatility harms the poorest learners** → essentials capped/waitlisted; posted-price smoothing.
- **The anchor**: when AI collapses `fiat_cost`, `base_ec` falls and the *same EC buys more* (deflation passes through) — but sponsors can then cut the nominal pool (see economics review). The price counter gives the *signal*; funding the pool is a separate fiscal decision.