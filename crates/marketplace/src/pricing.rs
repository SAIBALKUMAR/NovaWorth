//! Deterministic live pricing: a cost floor times a bounded supply/demand multiplier.
//! This is the only place a price is computed — never inside an AI/ML model.

use common::constants::{
    DYNAMIC_PRICE_CAP, PRICE_DEMAND_SENSITIVITY, PRICE_SUPPLY_SENSITIVITY, SUPPLY_TARGET_UNITS,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LivePrice {
    pub base_ec: i64,
    pub ec_price: i64,
    pub supply_factor: f64,
    pub demand_factor: f64,
}

#[inline]
fn clamp(v: f64, lo: f64, hi: f64) -> f64 {
    v.max(lo).min(hi)
}

/// Compute the live EC price.
///
/// * `base_ec` — cost floor in EC (the existing cost-plus `products.price_ec`).
/// * `avail_now` — effective supply in units.
/// * `d_now` — recent demand intensity.
/// * `d_baseline` — the demand level considered "normal".
///
/// The price never drops below `base_ec` and never exceeds
/// `base_ec * (1 + DYNAMIC_PRICE_CAP)`.
pub fn live_price(base_ec: i64, avail_now: i64, d_now: f64, d_baseline: f64) -> LivePrice {
    let base = base_ec as f64;

    let supply_factor = {
        let target = SUPPLY_TARGET_UNITS as f64;
        let shortage = if target > 0.0 {
            (target - avail_now as f64) / target
        } else {
            0.0
        };
        1.0 + PRICE_SUPPLY_SENSITIVITY * clamp(shortage, -1.0, 1.0)
    };

    let demand_factor = {
        let rel = if d_baseline > 0.0 {
            (d_now - d_baseline) / d_baseline
        } else {
            0.0
        };
        // Demand only applies upward surge pressure; the cost floor handles weak demand.
        (1.0 + PRICE_DEMAND_SENSITIVITY * clamp(rel, -1.0, 1.0)).max(1.0)
    };

    let multiplier = clamp(supply_factor * demand_factor, 1.0, 1.0 + DYNAMIC_PRICE_CAP);
    let ec_price = (base * multiplier).round() as i64;

    LivePrice {
        base_ec,
        ec_price,
        supply_factor,
        demand_factor,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::constants::DEMAND_BASELINE;

    #[test]
    fn price_never_below_cost_when_abundant() {
        let p = live_price(1000, 10_000, DEMAND_BASELINE, DEMAND_BASELINE);
        assert_eq!(p.ec_price, 1000); // abundant supply → down-pressure floored to cost
        assert!(p.supply_factor < 1.0);
    }

    #[test]
    fn price_never_exceeds_cap() {
        let p = live_price(1000, 0, 1_000_000.0, DEMAND_BASELINE);
        let cap = (1000.0 * (1.0 + DYNAMIC_PRICE_CAP)).round() as i64;
        assert!(p.ec_price <= cap);
    }

    #[test]
    fn shortage_raises_price_vs_abundance() {
        let scarce = live_price(1000, 0, DEMAND_BASELINE, DEMAND_BASELINE);
        let abundant = live_price(1000, 10_000, DEMAND_BASELINE, DEMAND_BASELINE);
        assert!(scarce.ec_price > abundant.ec_price);
    }

    #[test]
    fn no_demand_is_neutral() {
        // With d_now == baseline, the demand factor must be exactly neutral.
        let p = live_price(1000, SUPPLY_TARGET_UNITS, DEMAND_BASELINE, DEMAND_BASELINE);
        assert!((p.demand_factor - 1.0).abs() < f64::EPSILON);
    }
}