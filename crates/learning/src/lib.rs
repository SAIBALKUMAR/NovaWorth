//! Learning crate — mastery estimation, spaced repetition, adaptive difficulty,
//! the level-gate state machine, and progress/streak tracking.
//!
//! This crate is pure learning-science; it delegates persistence to `db` and
//! awards to `economy`.

pub mod bkt;
pub mod difficulty;
pub mod gate;
pub mod progress;
pub mod sm2;
