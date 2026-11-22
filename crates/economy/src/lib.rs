//! Economy crate — XP/EC scoring and the append-only, hash-chained double-entry
//! ledger that turns verified learning into auditable value.
//!
//! Subsystems:
//! - [`xp`]    — non-spendable mastery-point scoring (power law)
//! - [`ec`]    — spendable Education-Credit scoring (multipliers + decay)
//! - [`ledger`]— double-entry transaction engine with wallet caching
//! - [`hash_chain`] — SHA-256 tamper-evidence chain
//! - [`reconcile`]  — wallet-vs-ledger reconciliation

pub mod ec;
pub mod hash_chain;
pub mod ledger;
pub mod reconcile;
pub mod xp;
