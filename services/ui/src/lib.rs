//! Leptos 0.7 client-side app (wasm32 target only).
//!
//! The whole library is gated to `wasm32` so the native-target build of the
//! workspace produces an empty `ui` crate (no wasm-only types leak into host).

#![cfg(target_arch = "wasm32")]

pub mod api;
pub mod app;
pub mod components;
pub mod pages;
pub mod state;

pub use app::App;