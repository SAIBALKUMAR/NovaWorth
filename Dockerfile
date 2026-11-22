# ── Stage 1: build the workspace binaries ──────────────────────────────────────
FROM rust:1.82-slim AS builder

RUN apt-get update \
    && apt-get install -y --no-install-recommends pkg-config libssl-dev ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY . .

# Build the two backend service binaries. The UI is a client-side Leptos app
# built separately with `trunk build` (see Makefile) and served statically by Caddy.
RUN cargo build --release --locked --bin api --bin genai

# ── Stage 2: minimal runtime ───────────────────────────────────────────────────
FROM debian:bookworm-slim AS runtime

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/api   /app/bin/api
COPY --from=builder /app/target/release/genai /app/bin/genai
COPY migrations /app/migrations

EXPOSE 4000 5000

# The service is selected per-container via `command: ["/app/bin/<service>"]`.
CMD ["/app/bin/api"]