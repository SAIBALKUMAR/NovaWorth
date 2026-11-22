//! Economy routes:
//! GET /economy/wallet → current XP + EC balances for the authenticated learner.

use std::sync::Arc;

use axum::{extract::State, routing::get, Json, Router};

use auth::backend::AuthSession;
use common::errors::AppError;
use common::types::Currency;
use economy::ledger;

use crate::state::AppState;

pub fn router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/economy/wallet", get(wallet))
        .with_state(state)
}

#[derive(serde::Serialize)]
struct WalletResponse {
    xp: i64,
    ec: i64,
}

async fn wallet(
    State(state): State<Arc<AppState>>,
    auth: AuthSession,
) -> Result<Json<WalletResponse>, AppError> {
    let user_id = auth.user.ok_or(AppError::SessionExpired)?.id;

    let xp = ledger::get_balance(&state.pool, user_id, Currency::XP).await?;
    let ec = ledger::get_balance(&state.pool, user_id, Currency::EC).await?;

    Ok(Json(WalletResponse { xp, ec }))
}