.RECIPEPREFIX := >
.PHONY: dev build test migrate migrate-new lint fmt check clean ui ui-serve

dev:
> docker compose up -d
> cargo watch -x 'run -p api'

build:
> cargo build --release --bin api --bin genai

# Client-side Leptos UI (wasm) — requires `cargo install trunk`.
ui:
> cd services/ui && trunk build

ui-serve:
> cd services/ui && trunk serve

test:
> cargo test --workspace

migrate:
> sqlx migrate run --database-url $$DATABASE_URL

migrate-new:
> sqlx migrate add $(name)

lint:
> cargo clippy --workspace --all-targets -- -D warnings

fmt:
> cargo fmt --all

check:
> cargo check --workspace

clean:
> cargo clean
> docker compose down -v