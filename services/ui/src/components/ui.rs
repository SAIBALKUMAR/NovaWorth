//! Base UI primitives (dark design system).

use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum Variant {
    Primary,
    Secondary,
    Ghost,
    Danger,
}

impl Variant {
    fn class(self) -> &'static str {
        match self {
            Variant::Primary => "btn-primary",
            Variant::Secondary => "btn-secondary",
            Variant::Ghost => "btn-ghost",
            Variant::Danger => "btn-danger",
        }
    }
}

/// A styled, optionally clickable button.
#[component]
pub fn Button(
    #[prop(into)] label: String,
    #[prop(default = Variant::Primary)] variant: Variant,
    #[prop(default = false)] disabled: bool,
    #[prop(default = false)] loading: bool,
    #[prop(optional)] on_click: Option<Callback<()>>,
) -> impl IntoView {
    let classes = format!("btn {}", variant.class());

    view! {
        <button
            class=classes
            disabled=disabled || loading
            on:click=move |_| {
                if let Some(cb) = on_click.as_ref() {
                    cb.run(());
                }
            }
        >
            {move || if loading {
                view! { <Spinner/> }.into_any()
            } else {
                view! { {label.clone()} }.into_any()
            }}
        </button>
    }
}

#[component]
pub fn Spinner() -> impl IntoView {
    view! { <span class="spinner" aria-label="Loading"></span> }
}

#[component]
pub fn Card(
    children: Children,
    #[prop(optional, into)] title: String,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    view! {
        <div class=format!("card {}", class)>
            {(!title.is_empty()).then(|| view! { <h3 class="card-title">{title}</h3> })}
            <div class="card-body">{children()}</div>
        </div>
    }
}

#[component]
pub fn Badge(#[prop(into)] label: String) -> impl IntoView {
    view! { <span class="badge">{label}</span> }
}

#[component]
pub fn MasteryBar(#[prop(into)] label: String, value: f64) -> impl IntoView {
    let pct = (value.clamp(0.0, 1.0) * 100.0).round() as u32;

    view! {
        <div class="mastery">
            <div class="mastery-label"><span>{label}</span><span>{pct}"%"</span></div>
            <div class="mastery-track"><div class="mastery-fill" style=format!("width: {}%", pct)></div></div>
        </div>
    }
}