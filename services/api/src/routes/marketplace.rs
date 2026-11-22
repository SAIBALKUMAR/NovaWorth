//! Marketplace routes: a live priced quote and supply ingestion.

use std::sync::Arc;

use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use uuid::Uuid;

use auth::backend::AuthSession;
use common::constants::{DEMAND_BASELINE, SUPPLY_TARGET_UNITS};
use common::errors::AppError;
use marketplace::{
    pricing,
    supply::{SupplyEvent, SupplyService, SupplySource},
};

use crate::state::AppState;

pub fn router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/marketplace/quote/:product_id", get(quote))
        .route("/marketplace/supply", post(ingest_supply))
        .with_state(state)
}

#[derive(serde::Serialize)]
struct QuoteResponse {
    product_id: Uuid,
    base_ec: i64,
    ec_price: i64,
    supply_factor: f64,
    demand_factor: f64,
    supply_status: &'static str,
}

/// Compute the live EC price for a product and record an audit tick.
async fn quote(
    State(state): State<Arc<AppState>>,
    _auth: AuthSession,
    Path(product_id): Path<Uuid>,
) -> Result<Json<QuoteResponse>, AppError> {
    let svc = SupplyService::new(state.pool.clone());

    let base_ec = base_cost_ec(&state.pool, product_id).await?;
    let avail_now = svc.available(product_id).await?;
    let d_now = svc.demand_now(product_id).await?;

    let price = pricing::live_price(base_ec, avail_now, d_now, DEMAND_BASELINE);

    let supply_status = if avail_now <= 0 {
        "OUT_OF_STOCK"
    } else if avail_now < SUPPLY_TARGET_UNITS {
        "LOW_STOCK"
    } else {
        "AVAILABLE"
    };

    record_tick(&state.pool, product_id, &price, supply_status).await?;

    Ok(Json(QuoteResponse {
        product_id,
        base_ec: price.base_ec,
        ec_price: price.ec_price,
        supply_factor: price.supply_factor,
        demand_factor: price.demand_factor,
        supply_status,
    }))
}

#[derive(Deserialize)]
struct SupplyPayload {
    product_id: Uuid,
    region: Option<String>,
    qty_on_hand: Option<i64>,
    incoming_rate: f64,
    lead_time_hours: f64,
    source: String,
    confidence: f64,
}

/// Accept a supply observation (merchant push, sensor, or AI-inferred forecast).
async fn ingest_supply(
    State(state): State<Arc<AppState>>,
    auth: AuthSession,
    Json(payload): Json<SupplyPayload>,
) -> Result<Json<serde_json::Value>, AppError> {
    // Only non-learners may inject supply: merchants, sensors (service), or the
    // AI supply oracle (which authenticates with a sponsor/admin identity).
    let user = auth.user.ok_or(AppError::SessionExpired)?;
    if user.role == "learner" {
        return Err(AppError::Forbidden {
            required: "merchant/admin/sponsor".into(),
        });
    }

    let source = match payload.source.as_str() {
        "merchant_push" | "merchant" => SupplySource::Merchant,
        "ai_inference" | "ai" => SupplySource::Ai,
        "sensor" => SupplySource::Sensor,
        other => {
            return Err(AppError::Validation(format!(
                "Unknown supply source: {other}"
            )))
        }
    };

    let event = SupplyEvent {
        product_id: payload.product_id,
        region: payload.region.unwrap_or_else(|| "US_EU".to_string()),
        qty_on_hand: payload.qty_on_hand,
        incoming_rate: payload.incoming_rate,
        lead_time_hours: payload.lead_time_hours,
        source,
        confidence: payload.confidence,
    };

    SupplyService::new(state.pool.clone()).ingest(&event).await?;

    Ok(Json(serde_json::json!({ "ok": true })))
}

async fn base_cost_ec(pool: &sqlx::PgPool, product_id: Uuid) -> Result<i64, AppError> {
    let (ec,): (i64,) = sqlx::query_as("SELECT price_ec FROM products WHERE id = $1")
        .bind(product_id)
        .fetch_one(pool)
        .await
        .map_err(AppError::Database)?;
    Ok(ec)
}

async fn record_tick(
    pool: &sqlx::PgPool,
    product_id: Uuid,
    price: &pricing::LivePrice,
    supply_status: &str,
) -> Result<(), AppError> {
    sqlx::query(
        "INSERT INTO price_ticks
            (product_id, region, base_ec, supply_factor, demand_factor, ec_price, reason, as_of)
         VALUES ($1, 'US_EU', $2, $3, $4, $5, $6, now())",
    )
    .bind(product_id)
    .bind(price.base_ec)
    .bind(price.supply_factor)
    .bind(price.demand_factor)
    .bind(price.ec_price)
    .bind(format!("quote:{supply_status}"))
    .execute(pool)
    .await
    .map_err(AppError::Database)?;
    Ok(())
}