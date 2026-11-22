//! Mastery-point (XP) scoring.
//!
//! XP is **non-spendable, non-transferable, and never decays**. It measures mastery
//! and drives rank/progression only. The canonical formula is:
//!
//! ```text
//! XP(L) = 100 · L^1.5 · m_d · Q · M
//! ```
//!
//! where `L` is the level (1–100), `m_d` the domain multiplier (0.9–1.3),
//! `Q` the quality multiplier (0.8–1.25), and `M` the mastery/transfer multiplier
//! (0.85–1.5). Re-peating an already-passed level yields only 10% XP (capped at one
//! repeat) — mastery is permanent.

/// Quality multiplier bounds (see scoring model doc).
pub const Q_MIN: f64 = 0.8;
pub const Q_MAX: f64 = 1.25;

/// Mastery/transfer multiplier bounds.
pub const M_MIN: f64 = 0.85;
pub const M_MAX: f64 = 1.5;

/// Clamp `v` into `[lo, hi]`.
#[inline]
fn clamp(v: f64, lo: f64, hi: f64) -> f64 {
    v.max(lo).min(hi)
}

/// Base XP for a level *before* quality/mastery multipliers.
pub fn xp_base(level: u32, domain_multiplier: f64) -> f64 {
    100.0 * (level as f64).powf(1.5) * domain_multiplier
}

/// The multiplier applied to XP from quality and mastery evidence.
pub fn xp_multiplier(quality: f64, mastery: f64) -> f64 {
    clamp(quality, Q_MIN, Q_MAX) * clamp(mastery, M_MIN, M_MAX)
}

/// XP awarded for a **first-time** pass of `level`.
///
/// Returns the rounded integer award. A distinction/outstanding pass or an
/// above-level transfer result will push this above the base by the multiplier.
pub fn xp_award(level: u32, domain_multiplier: f64, quality: f64, mastery: f64) -> i64 {
    (xp_base(level, domain_multiplier) * xp_multiplier(quality, mastery)).round() as i64
}

/// XP awarded on a **repeat** pass of a level already completed.
///
/// Repeating yields 10% of the first-pass award, and only once. This preserves
/// "mastery is permanent" while making grinding non-profitable.
pub fn xp_repeat_award(level: u32, domain_multiplier: f64, quality: f64, mastery: f64) -> i64 {
    (xp_award(level, domain_multiplier, quality, mastery) as f64 * 0.10).round() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_is_power_law() {
        // xp_base(10) vs xp_base(1): 10^1.5 ≈ 31.62
        assert!((xp_base(10, 1.1) / xp_base(1, 1.1) - 10f64.powf(1.5)).abs() < 1e-9);
    }

    #[test]
    fn award_respects_multipliers() {
        let modest = xp_award(10, 1.1, 0.8, 0.85);
        let strong = xp_award(10, 1.1, 1.25, 1.5);
        assert!(strong > modest);
        assert!(modest > 0);
    }

    #[test]
    fn repeat_is_one_tenth() {
        let first = xp_award(20, 1.0, 1.0, 1.0);
        let repeat = xp_repeat_award(20, 1.0, 1.0, 1.0);
        assert_eq!(repeat, (first as f64 * 0.10).round() as i64);
    }

    #[test]
    fn higher_level_is_worth_more() {
        assert!(xp_award(50, 1.1, 1.0, 1.0) > xp_award(10, 1.1, 1.0, 1.0));
    }
}