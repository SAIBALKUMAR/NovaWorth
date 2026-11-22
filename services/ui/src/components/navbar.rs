//! Top navigation bar, aware of the current auth state.

use leptos::prelude::*;
use leptos_router::components::A;

use crate::state::use_auth;

#[component]
pub fn Navbar() -> impl IntoView {
    let auth = use_auth();

    view! {
        <nav class="navbar">
            <A href="/" class="brand">"🎓 NovaWorth"</A>
            <div class="nav-links">
                <A href="/dashboard">"Dashboard"</A>
                {move || {
                    if let Some(user) = auth.get() {
                        view! { <span class="nav-user">{user.username}</span> }.into_any()
                    } else {
                        view! {
                            <A href="/login">"Sign in"</A>
                            <A href="/register" class="btn btn-primary">"Get started"</A>
                        }.into_any()
                    }
                }}
            </div>
        </nav>
    }
}