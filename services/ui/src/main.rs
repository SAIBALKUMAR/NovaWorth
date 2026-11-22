//! UI entry point (client-side WASM).
//!
//! On `wasm32` this mounts the Leptos app to `<body>`. On native targets it is a
//! deliberate no-op so `cargo check --workspace` stays green without a wasm toolchain.

fn main() {
    #[cfg(target_arch = "wasm32")]
    {
        console_error_panic_hook::set_once();
        leptos::mount::mount_to_body(ui::App);
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        eprintln!("UI: client-side WASM app — build with `trunk build` (see Makefile).");
    }
}
