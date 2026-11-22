//! Education-Credit (EC) scoring.
//!
//! EC is **spendable** and funded strictly by sponsor pools. The canonical formula is:
//!
//! ```text
//! EC(L) = 10 · L^1.2 · m_d · Q · E · M · R
//! ```
//!
//! `Q` = quality, `E` = effort, `M` = mastery/transfer, `R` = repeat decay.
//! The combined multiplier is clamped to `[0.6, 2.0]`. A "growth dividend" adds
//! +0.10 multiplier for learners in the bottom aptitude quartile at levels ≤ 30,
//! to blunt a rich-get-richer spiral.

/// Multiplier floor/cap for `Q · E · M · growth`.
pub const MULTIPLIER_FLOOR: f64 = 0.6;
pub const MULTIPLIER_CAP: f64 = 2.0;

const E_MIN: f64 = 0.9;
const E_MAX: f64 = 1.2;

/// Growth dividend applied to bottom-quartile learners below level 31.
pub const GROWTH_DIVIDEND: f64 = 0.10;

/// Repeat-attempt decay for EC (`1.0 → 0.2 → 0.05 → 0`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepeatDecay {
    First,
    Second,
    Third,
    FourthOrMore,
}

impl RepeatDecay {
    /// Build from a zero-based repeat index (0 = first pass, 1 = 2nd, …).
    pub fn from_index(index: usize) -> Self {
        match index {
            0 => Self::First,
            1 => Self::Second,
            2 => Self::Third,
            _ => Self::FourthOrMore,
        }
    }

    /// EC multiplier for this repeat tier.
    pub fn multiplier(self) -> f64 {
        match self {
            Self::First => 1.0,
            Self::Second => 0.2,
            Self::Third => 0.05,
            Self::FourthOrMore => 0.0,
        }
    }
}

#[inline]
fn clamp(v: f64, lo: f64, hi: f64) -> f64 {
    v.max(lo).min(hi)
}

/// Base EC for a level, before multipliers.
pub fn ec_base(level: u32, domain_multiplier: f64) -> f64 {
    10.0 * (level as f64).powf(1.2) * domain_multiplier
}

/// Combined EC multiplier from quality, effort, mastery, growth dividend.
pub fn ec_multiplier(quality: f64, effort: f64, mastery: f64, growth_dividend: bool) -> f64 {
    let growth = if growth_dividend { 1.0 + GROWTH_DIVIDEND } else { 1.0 };
    clamp(
        clamp(quality, crate::xp::Q_MIN, crate::xp::Q_MAX)
            * clamp(effort, E_MIN, E_MAX)
            * clamp(mastery, crate::xp::M_MIN, crate::xp::M_MAX)
            * growth,
        MULTIPLIER_FLOOR,
        MULTIPLIER_CAP,
    )
}

/// EC awarded for a pass of `level`, given evidence and repeat tier.
pub fn ec_award(
    level: u32,
    domain_multiplier: f64,
    quality: f64,
    effort: f64,
    mastery: f64,
    repeat: RepeatDecay,
    growth_dividend: bool,
) -> i64 {
    (ec_base(level, domain_multiplier)
        * ec_multiplier(quality, effort, mastery, growth_dividend)
        * repeat.multiplier())
    .round() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repeat_decay_sequence() {
        assert_eq!(RepeatDecay::from_index(0).multiplier(), 1.0);
        assert_eq!(RepeatDecay::from_index(1).multiplier(), 0.2);
        assert_eq!(RepeatDecay::from_index(2).multiplier(), 0.05);
        assert_eq!(RepeatDecay::from_index(3).multiplier(), 0.0);
        assert_eq!(RepeatDecay::from_index(99).multiplier(), 0.0);
    }

    #[test]
    fn multiplier_is_capped_and_floored() {
        // Extreme inputs must clamp to the cap.
        assert!((ec_multiplier(1.25, 1.2, 1.5, true) - MULTIPLIER_CAP).abs() < 1e-9);
        // Terrible inputs must clamp to the floor.
        assert!((ec_multiplier(0.0, 0.0, 0.0, false) - MULTIPLIER_FLOOR).abs() < 1e-9);
    }

    #[test]
    fn growth_dividend_increases_award() {
        let base = ec_award(10, 1.1, 1.0, 1.0, 1.0, RepeatDecay::First, false);
        let grown = ec_award(10, 1.1, 1.0, 1.0, 1.0, RepeatDecay::First, true);
        assert!(grown > base);
    }

    #[test]
    fn fourth_repeat_earns_zero() {
        assert_eq!(ec_award(10, 1.1, 1.0, 1.0, 1.0, RepeatDecay::FourthOrMore, false), 0);
    }
}