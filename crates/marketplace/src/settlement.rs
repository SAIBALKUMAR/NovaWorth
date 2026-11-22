//! Merchant settlement split (92/8 platform fee).

use common::constants::MERCHANT_SPLIT;

/// The settlement breakdown for a gross EC amount.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Settlement {
    pub merchant_ec: i64,
    pub platform_fee_ec: i64,
}

/// Split a gross EC amount into merchant payout + platform fee.
pub fn compute(gross_ec: i64) -> Settlement {
    let merchant = (gross_ec as f64 * MERCHANT_SPLIT).round() as i64;
    Settlement {
        merchant_ec: merchant,
        platform_fee_ec: gross_ec - merchant,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_is_92_8() {
        let s = compute(100);
        assert_eq!(s.merchant_ec, 92);
        assert_eq!(s.platform_fee_ec, 8);
    }

    #[test]
    fn split_sums_to_gross() {
        let s = compute(103);
        assert_eq!(s.merchant_ec + s.platform_fee_ec, 103);
    }
}