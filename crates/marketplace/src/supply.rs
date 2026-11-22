//! Live supply feed for the marketplace.
//!
//! Supply is the hard half of dynamic pricing. A merchant (or a sensor) pushes
//! authoritative on-hand quantities; an AI "supply oracle" pushes the
//! replenishment forecast (`incoming_rate`, `lead_time_hours`) with a
//! confidence in `[0, 1)`. Events are append-only; the materialized
//! `product_supply_state` is what the price engine reads.

use sqlx::PgPool;
use uuid::Uuid;

use common::constants::SUPPLY_MIN_CONFIDENCE;
use common::errors::AppError;

/// Well-known supply source labels (must match the CHECK in `0008_supply_feed.sql`).
pub mod source {
    pub const MERCHANT: &str = "merchant_push";
    pub const AI: &str = "ai_inference";
    pub const SENSOR: &str = "sensor";
}

/// Where a supply observation came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SupplySource {
    Merchant,
    Ai,
    Sensor,
}

impl SupplySource {
    pub fn as_str(self) -> &'static str {
        match self {
            SupplySource::Merchant => source::MERCHANT,
            SupplySource::Ai => source::AI,
            SupplySource::Sensor => source::SENSOR,
        }
    }
}

/// A normalized supply observation.
#[derive(Debug, Clone)]
pub struct SupplyEvent {
    pub product_id: Uuid,
    pub region: String,
    /// On-hand units; `None` for a pure AI forecast.
    pub qty_on_hand: Option<i64>,
    /// Replenishment rate in units per hour.
    pub incoming_rate: f64,
    /// Expected replenishment horizon in hours.
    pub lead_time_hours: f64,
    pub source: SupplySource,
    /// Trust in this observation, in `[0, 1]`. Ground truth uses 1.0;
    /// AI inference must stay below 1.0.
    pub confidence: f64,
}

/// Effective available supply in units: on-hand plus expected inflow over the lead window.
pub fn effective_supply(qty_on_hand: i64, incoming_rate: f64, lead_time_hours: f64) -> i64 {
    let inflow = (incoming_rate * lead_time_hours).round() as i64;
    qty_on_hand.saturating_add(inflow)
}

pub struct SupplyService {
    pool: PgPool,
}

impl SupplyService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Append a supply event and refresh the materialized `product_supply_state`.
    ///
    /// A higher-confidence event wins for the replenishment forecast; `trust_level`
    /// is the running maximum. On-hand quantity is only updated by ground-truth
    /// sources (merchant/sensor) — an AI-only event never zeroes a merchant's stock.
    pub async fn ingest(&self, ev: &SupplyEvent) -> Result<(), AppError> {
        if ev.incoming_rate < 0.0 || ev.lead_time_hours < 0.0 {
            return Err(AppError::Validation(
                "Supply rate and lead time must be non-negative".into(),
            ));
        }
        if let Some(q) = ev.qty_on_hand {
            if q < 0 {
                return Err(AppError::Validation("On-hand quantity must be >= 0".into()));
            }
        }
        if !(0.0..=1.0).contains(&ev.confidence) {
            return Err(AppError::Validation(
                "Supply confidence must be within [0, 1]".into(),
            ));
        }
        if ev.confidence < SUPPLY_MIN_CONFIDENCE {
            return Err(AppError::Validation(format!(
                "Supply confidence {} is below the minimum {}",
                ev.confidence, SUPPLY_MIN_CONFIDENCE
            )));
        }

        let is_ground_truth = matches!(ev.source, SupplySource::Merchant | SupplySource::Sensor);

        let mut tx = self.pool.begin().await.map_err(AppError::Database)?;

        sqlx::query(
            "INSERT INTO supply_events
                (product_id, region, qty_on_hand, incoming_rate, lead_time_hours, source, confidence)
             VALUES ($1, $2, $3, $4, $5, $6, $7)",
        )
        .bind(ev.product_id)
        .bind(ev.region.as_str())
        .bind(ev.qty_on_hand)
        .bind(ev.incoming_rate)
        .bind(ev.lead_time_hours)
        .bind(ev.source.as_str())
        .bind(ev.confidence)
        .execute(&mut *tx)
        .await
        .map_err(AppError::Database)?;

        sqlx::query(
            "INSERT INTO product_supply_state
                (product_id, region, qty_on_hand, incoming_rate, lead_time_hours, trust_level, source, updated_at)
             VALUES ($1, $2, $3, $4, $5, $7, $6, now())
             ON CONFLICT (product_id)
             DO UPDATE SET
               region         = EXCLUDED.region,
               qty_on_hand    = CASE WHEN $8 THEN EXCLUDED.qty_on_hand ELSE product_supply_state.qty_on_hand END,
               incoming_rate  = CASE WHEN EXCLUDED.confidence >= product_supply_state.trust_level THEN EXCLUDED.incoming_rate ELSE product_supply_state.incoming_rate END,
               lead_time_hours = CASE WHEN EXCLUDED.confidence >= product_supply_state.trust_level THEN EXCLUDED.lead_time_hours ELSE product_supply_state.lead_time_hours END,
               trust_level    = GREATEST(product_supply_state.trust_level, EXCLUDED.confidence),
               source         = EXCLUDED.source,
               updated_at     = now()",
        )
        .bind(ev.product_id)
        .bind(ev.region.as_str())
        .bind(ev.qty_on_hand.unwrap_or(0))
        .bind(ev.incoming_rate)
        .bind(ev.lead_time_hours)
        .bind(ev.source.as_str())
        .bind(ev.confidence)
        .bind(is_ground_truth)
        .execute(&mut *tx)
        .await
        .map_err(AppError::Database)?;

        tx.commit().await.map_err(AppError::Database)?;
        Ok(())
    }

    /// Effective available supply in units for a product, if known.
    pub async fn available(&self, product_id: Uuid) -> Result<i64, AppError> {
        let row: Option<(i64, f64, f64)> = sqlx::query_as(
            "SELECT qty_on_hand, incoming_rate, lead_time_hours
             FROM product_supply_state WHERE product_id = $1",
        )
        .bind(product_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::Database)?;

        Ok(row
            .map(|(on_hand, rate, lead)| effective_supply(on_hand, rate, lead))
            .unwrap_or(0))
    }

    /// Recent demand intensity as a rolling sum of demand events over 24h.
    pub async fn demand_now(&self, product_id: Uuid) -> Result<f64, AppError> {
        let (d_now,): (f64,) = sqlx::query_as(
            "SELECT COALESCE(SUM(event_count), 0)::float8
             FROM demand_events
             WHERE product_id = $1 AND as_of > now() - interval '24 hours'",
        )
        .bind(product_id)
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::Database)?;

        Ok(d_now)
    }
}