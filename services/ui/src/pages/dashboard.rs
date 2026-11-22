//! Learner dashboard: wallet, leaderboard, and AI tutor.

use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::api;
use crate::components::ui::{Badge, Card};
use crate::state::use_auth;

#[component]
pub fn DashboardPage() -> impl IntoView {
    let auth = use_auth();

    let (wallet, set_wallet) = signal(None::<api::Wallet>);
    let (board, set_board) = signal(Vec::<api::LeaderboardRow>::new());
    let (topic, set_topic) = signal(String::new());
    let (explain_result, set_explain_result) = signal(None::<String>);
    let (explain_error, set_explain_error) = signal(None::<String>);
    let (explain_loading, set_explain_loading) = signal(false);

    // Initial load (component setup runs once on mount).
    spawn_local(async move {
        let w = api::wallet().await.ok();
        set_wallet.set(w);
        let b = api::leaderboard(10).await.unwrap_or_default();
        set_board.set(b);
    });

    view! {
        <div class="dashboard">
            <h1>"Dashboard"</h1>
            {move || auth.get().map(|u| view! {
                <p class="muted">"Signed in as " <strong>{u.username}</strong></p>
            })}

            <div class="grid">
                <Card title="Your wallet">
                    {move || match wallet.get() {
                        Some(w) => view! {
                            <div class="wallet">
                                <Badge label=format!("{} XP", w.xp)/>
                                <Badge label=format!("{} EC", w.ec)/>
                            </div>
                        }.into_any(),
                        None => view! {
                            <p class="muted">"Sign in to see your balances."</p>
                        }.into_any(),
                    }}
                </Card>

                <Card title="Leaderboard">
                    <ul class="board">
                        {move || board.get().into_iter().map(|r| view! {
                            <li><span>{r.username}</span><span>{r.xp_balance}" XP"</span></li>
                        }).collect::<Vec<_>>()}
                    </ul>
                </Card>
            </div>

            <Card title="Ask the AI tutor">
                <input type="text" placeholder="e.g. explain quadratic equations"
                    prop:value=move || topic.get()
                    on:input=move |ev| set_topic.set(event_target_value(&ev)) />

                <button class="btn btn-primary" disabled=move || explain_loading.get()
                    on:click=move |_| {
                        let t = topic.get().trim().to_string();
                        if t.is_empty() {
                            set_explain_error.set(Some("Enter a topic first.".into()));
                            return;
                        }
                        let user_id = auth.get().map(|u| u.id).unwrap_or_else(|| "guest".into());
                        set_explain_loading.set(true);
                        set_explain_error.set(None);
                        set_explain_result.set(None);
                        spawn_local(async move {
                            match api::explain(&user_id, &t).await {
                                Ok(text) => {
                                    set_explain_result.set(Some(text));
                                    set_explain_loading.set(false);
                                }
                                Err(msg) => {
                                    set_explain_error.set(Some(msg));
                                    set_explain_loading.set(false);
                                }
                            }
                        });
                    }
                >
                    {move || if explain_loading.get() { "Thinking…" } else { "Explain" }}
                </button>

                {move || explain_error.get().map(|e| view! { <div class="alert alert-danger">{e}</div> })}
                {move || explain_result.get().map(|t| view! { <div class="explain-result">{t}</div> })}
            </Card>
        </div>
    }
}