//! Root component: router, auth provider, and app chrome.

use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;

use crate::components::navbar::Navbar;
use crate::pages::{
    dashboard::DashboardPage, home::HomePage, login::LoginPage, register::RegisterPage,
};
use crate::state::provide_auth_state;

#[component]
pub fn App() -> impl IntoView {
    provide_auth_state();

    view! {
        <Router>
            <div class="app-shell">
                <Navbar/>
                <main class="app-main">
                    <Routes fallback=|| view! { <HomePage/> }>
                        <Route path=path!("/") view=HomePage/>
                        <Route path=path!("/login") view=LoginPage/>
                        <Route path=path!("/register") view=RegisterPage/>
                        <Route path=path!("/dashboard") view=DashboardPage/>
                    </Routes>
                </main>
                <footer class="app-footer">"NovaWorth — Learn. Earn. Grow."</footer>
            </div>
        </Router>
    }
}