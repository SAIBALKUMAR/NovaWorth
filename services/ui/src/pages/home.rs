//! Marketing / landing page.

use leptos::prelude::*;
use leptos_router::components::A;

use crate::components::ui::Card;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <section class="hero">
            <h1>"Learn. Prove. Earn. Redeem."</h1>
            <p class="hero-sub">
                "Verified human capability is the scarce asset. Climb a real 100-level ladder,
                prove mastery, and earn rewards that matter."
            </p>
            <div class="hero-actions">
                <A href="/register" class="btn btn-primary">"Get started"</A>
                <A href="/dashboard" class="btn btn-secondary">"View dashboard"</A>
            </div>
        </section>

        <section class="feature-grid">
            <Card title="500 skills"><p>"20 domains × 25 skills, each with a genuine 100-level ladder."</p></Card>
            <Card title="A fair economy"><p>"Earn XP for mastery and EC for verified work — never for engagement."</p></Card>
            <Card title="Proof that matters"><p>"Adaptive gates, artifacts, role-play, and expert panels at the top."</p></Card>
        </section>
    }
}