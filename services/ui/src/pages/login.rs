//! Login page.

use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::components::A;
use leptos_router::hooks::use_navigate;

use crate::api;
use crate::components::ui::Card;
use crate::state::use_auth;

#[component]
pub fn LoginPage() -> impl IntoView {
    let (email, set_email) = signal(String::new());
    let (password, set_password) = signal(String::new());
    let (error, set_error) = signal(None::<String>);
    let (loading, set_loading) = signal(false);

    let auth = use_auth();
    let navigate = use_navigate();

    view! {
        <div class="auth-wrap">
            <Card title="Sign in">
                {move || error.get().map(|e| view! { <div class="alert alert-danger">{e}</div> })}

                <label class="field">
                    <span class="field-label">"Email"</span>
                    <input type="email" placeholder="you@example.com"
                        prop:value=move || email.get()
                        on:input=move |ev| set_email.set(event_target_value(&ev)) />
                </label>

                <label class="field">
                    <span class="field-label">"Password"</span>
                    <input type="password" placeholder="••••••••"
                        prop:value=move || password.get()
                        on:input=move |ev| set_password.set(event_target_value(&ev)) />
                </label>

                <button class="btn btn-primary" style="width:100%" disabled=move || loading.get()
                    on:click=move |_| {
                        let nav = navigate.clone();
                        let e = email.get();
                        let p = password.get();
                        set_loading.set(true);
                        set_error.set(None);
                        spawn_local(async move {
                            match api::login(&e, &p).await {
                                Ok(user) => {
                                    auth.set(Some(user));
                                    nav("/dashboard", Default::default());
                                }
                                Err(msg) => {
                                    set_error.set(Some(msg));
                                    set_loading.set(false);
                                }
                            }
                        });
                    }
                >
                    {move || if loading.get() { "Signing in…" } else { "Sign in" }}
                </button>

                <p class="muted">"No account? " <A href="/register">"Register"</A></p>
            </Card>
        </div>
    }
}