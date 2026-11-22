//! Shared client-side auth state (provided once in [`App`](crate::app::App)).

use leptos::prelude::*;

/// The user payload returned by `/auth/login` and `/auth/register`.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AuthUser {
    pub id: String,
    pub email: String,
    pub username: String,
    pub role: String,
}

/// Provide the auth state to the component tree.
pub fn provide_auth_state() {
    provide_context(RwSignal::new(None::<AuthUser>));
}

/// Read the current auth state signal.
pub fn use_auth() -> RwSignal<Option<AuthUser>> {
    expect_context::<RwSignal<Option<AuthUser>>>()
}