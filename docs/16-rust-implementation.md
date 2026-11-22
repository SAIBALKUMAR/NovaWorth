# Rust Implementation — Reference

> The repository has been **re-implemented in Rust**. This document is the authoritative map of the code. The design/product docs in `docs/00-…`–`docs/15-…` remain the product baseline, but their *technology-stack* sections (`Next.js`, `Drizzle`, `BullMQ`, `Lucia`, `Turborepo`) are **superseded** by this document and by [`SETUP.md`](../SETUP.md).

## 1. Executive summary

| Concern | Choice |
|---|---|
| Language / workspace | Rust (edition 2021), Cargo workspace monorepo |
| HTTP API | **Axum 0.8** (with `axum-login`, `tower-sessions`) |
| UI | **Leptos 0.7** client-side WASM, built with **Trunk** |
| Database | **SQLx 0.9** (async, compile-time migrations) + **Neon Postgres** |
| Cache / queues | **Redis** via the `fred` client |
| Auth | Argon2id password hashing, OAuth2 (GitHub/Google), cookie sessions, login lockout |
| AI | OpenAI `gpt-4o-mini` / `text-embedding-3-small` via `reqwest` (budget-gated gateway) |
| Scoring / learning | Pure-Rust power-law formulas, BKT, SM-2, IRT, recommendation maths (no ML crates) |
| Infrastructure | Hetzner CAX21 VPS + Docker Compose + **Caddy** (auto-HTTPS) + Neon + Cloudflare R2 + Resend |

The backend exposes a REST API; the UI is a same-origin SPA proxied by Caddy in production.

## 2. Workspace layout

```
education-economy/
├── Cargo.toml                      # [workspace] + shared [workspace.dependencies]
├── .env.example
├── docker-compose.yml              # local dev: postgres (pgvector) + redis
├── docker-compose.prod.yml         # prod: api, genai, redis, caddy
├── Dockerfile                      # multi-stage build (api + genai binaries)
├── Caddyfile                       # static UI + reverse proxy (HTTPS)
├── Makefile                        # dev/build/test/migrate/ui
├── SETUP.md                        # install + deploy runbook
├── migrations/
│   ├── 0001_initial_schema.sql     # users, sessions
│   ├── 0002_economy.sql            # sponsor_pools, wallets, ledger_entries
│   ├── 0003_learning.sql           # domains, skills, levels, learner_progress, attempts
│   ├── 0004_marketplace.sql        # products, orders, order_items, escrow_holds, disputes
│   ├── 0005_social.sql             # guilds, guild_members, reputation_events, league_seasons
│   ├── 0006_notifications.sql      # notifications, email_log
│   └── 0007_security.sql           # login_attempts (brute-force lockout)
├── crates/                         # 11 domain libraries (see §4)
└── services/
    ├── api/                        # Axum REST API
    ├── genai/                      # Axum AI-gateway service
    └── ui/                         # Leptos 0.7 CSR app (Trunk)
```

## 3. Dependency stack (workspace deps)

| Crate | Version | Role |
|---|---|---|
| `axum` | 0.8 | HTTP server |
| `axum-login` / `tower-sessions` | 0.18 / 0.14 | session auth |
| `leptos` / `leptos_router` / `leptos_meta` | 0.7 | Wasm UI |
| `sqlx` | 0.9 | async Postgres (runtime queries; `migrate!` embeds migrations) |
| `pgvector` | 0.4 | vector type (RAG) |
| `argon2` / `password-hash` | 0.5 | Argon2id password hashing |
| `oauth2` | 5.0 | GitHub / Google OAuth |
| `reqwest` | 0.12 | OpenAI + Resend HTTP calls |
| `fred` | 9 | Redis client |
| `sha2` | 0.10 | ledger hash-chain |
| `validator` | 0.19 | request validation |
| `thiserror` / `anyhow` | 2 / 1 | error types |
| `tracing` / `tracing-subscriber` | 0.1 / 0.3 | logging |

## 4. Crates (domain libraries)

All crates are pure libraries returning `common::errors::AppError`; persistence uses **runtime** `sqlx` queries (no compile-time `query!` macros — the tree builds without a live database).

| Crate | Modules | What it implements |
|---|---|---|
| `crates/common` | `config`, `constants`, `errors`, `types` | Env config, canonical constants, `AppError` (+HTTP mapping), domain types (`Currency`, `EntryType`, `UserRole`, `Pagination`, `ListResponse`) |
| `crates/db` | `pool`, `users`, `sessions` | `PgPool` init, `sqlx::migrate!`, user queries, session cleanup |
| `crates/auth` | `backend`, `password`, `oauth`, `lockout` | axum-login backend, Argon2id hash/verify, GitHub/Google OAuth clients, brute-force lockout |
| `crates/economy` | `xp`, `ec`, `ledger`, `hash_chain`, `reconcile` | Power-law XP/EC scoring, append-only hash-chained ledger + wallet cache, nightly reconciliation |
| `crates/learning` | `bkt`, `sm2`, `difficulty`, `gate`, `progress` | Mastery estimation, spaced repetition, adaptive difficulty, gate state machine, streaks |
| `crates/assessment` | `irt`, `rubric`, `gate_saga` | Item Response Theory, rubric grading, transactional gate→award→unlock saga |
| `crates/recommendations` | `collaborative`, `cold_start`, `fit_score`, `fairness` | Item-item CF, content cold-start, 7-factor fit score, MMR diversity re-ranking |
| `crates/ai` | `gateway`, `embeddings`, `rag`, `guardrails`, `prompts`, `cache` | Budget-gated OpenAI gateway, embeddings, RRF hybrid retrieval, PII/injection guardrails, semantic cache |
| `crates/notifications` | `email`, `in_app`, `push` | Resend email, Postgres in-app feed, Web-Push types |
| `crates/marketplace` | `order`, `escrow`, `dispute`, `settlement` | Atomic order+stock reservation, escrow lifecycle, disputes, 92/8 settlement |
| `crates/social` | `guild`, `league`, `reputation`, `moderation` | Guilds (cap + founder level), seasons/leaderboard, decaying reputation, moderation heuristics |

## 5. Services

### `services/api` — Axum REST API (`:4000`)
- `state.rs` → `AppState` (config, pool, auth backend, OAuth clients, HTTP client).
- `routes/`:
  - `auth` — `POST /auth/register`, `/auth/login`, `/auth/logout`, `GET /auth/me`, `GET /auth/github[/callback]`, `/auth/google[/callback]` (callbacks now **create a session**).
  - `economy` — `GET /economy/wallet` (XP + EC for the session user).
  - `social` — `GET /social/leaderboard?limit=`.
  - `health` — `GET /health`.

### `services/genai` — AI gateway service (`:5000`)
- `routes/generate` — `POST /generate/explain` (`{user_id, topic}`) routing through `ai::gateway::AiGateway` (budget-gated).

### `services/ui` — Leptos 0.7 client-side app (Trunk)
- `src/app.rs` (router + auth provider) · `src/pages/{home,login,register,dashboard}` · `src/components/{navbar,ui}` · `src/api.rs` (typed REST client) · `src/state.rs` (`AuthUser` signal) · `index.html` + `style.css` (dark design system).

## 6. Database schema (migrations)

All schema is plain SQL in `migrations/*.sql`, applied automatically by the API at startup via `sqlx::migrate!`.

| Migration | Tables created |
|---|---|
| `0001_initial_schema.sql` | `users`, `sessions` |
| `0002_economy.sql` | `sponsor_pools`, `wallets`, `ledger_entries` (append-only, hash-chained), `user_balances` view |
| `0003_learning.sql` | `domains`, `skills`, `levels`, `learner_skills`, `learner_progress`, `attempts` |
| `0004_marketplace.sql` | `products`, `orders`, `order_items`, `escrow_holds`, `disputes` |
| `0005_social.sql` | `guilds`, `guild_members`, `reputation_events`, `league_seasons` |
| `0006_notifications.sql` | `notifications`, `email_log` |
| `0007_security.sql` | `login_attempts` (brute-force lockout) |

Key invariants enforced in code:
- **Ledger** — amounts must be positive; XP is non-spendable (debits rejected); balance is guarded non-negative; each entry chains `SHA-256(prev | payload)`; wallets are an in-transaction cache reconciled against the ledger.
- **Achievements** — EC awards are gated by repeat decay (`1.0 → 0.2 → 0.05 → 0`), a daily velocity cap, and a funded sponsor pool (else XP-only).

## 7. Infrastructure

| File | Purpose |
|---|---|
| `docker-compose.yml` | Local dev: Postgres (`pgvector/pgvector:pg17`) + Redis (`redis:7-alpine`) |
| `docker-compose.prod.yml` | Prod: `api`, `genai`, `redis`, `caddy` (Caddy mounts the built UI at `/srv/www`) |
| `Dockerfile` | Multi-stage: build `api` + `genai` in `rust:1.82-slim`, copy into `debian:bookworm-slim` |
| `Caddyfile` | Auto-HTTPS; static UI + SPEX fallback; proxies `/auth|economy|social|api/*` → api, `/generate/*` → genai |
| `Makefile` | `dev`, `build`, `test`, `migrate`, `lint`, `fmt`, `check`, `ui` (`trunk build`), `ui-serve` |
| `.github/workflows/deploy.yml` | CI/CD: `cargo test` → build + push GHCR image → SSH deploy |
| `SETUP.md` | Step-by-step install + deploy runbook |

## 8. Product edge cases implemented (UX-first)

- **Auth** — OAuth callbacks now create a session; login brute-force lockout (`MAX_FAILED_LOGIN_ATTEMPTS` → `AccountLocked`).
- **Gate** — idempotent `begin`/`complete` (race-safe via `rows_affected`), score validation, repeat decay, daily EC velocity cap, `learner_skills.current_level` advancement.
- **Escrow/order** — atomic stock reservation + balance pre-check, escrow compensation on partial failure, release/refund state guards, settlement split.
- **Social** — guild member cap + founder minimum level enforced.
- **AI** — instance monthly + per-user daily token budgets, PII redaction, prompt-injection rejection.

## 9. Design → code traceability (LLD to Rust)

The numbered design docs remain the product spec; this table maps each low-level design doc to the Rust code that implements it.

| Design doc | Rust implementation |
|---|---|
| `15-lld/01-identity-auth.md` | `crates/auth` (`backend`, `password`, `oauth`, `lockout`) + `services/api/src/routes/auth.rs` |
| `15-lld/02-ledger.md` | `crates/economy` (`ledger`, `xp`, `ec`, `hash_chain`, `reconcile`) + `migrations/0002_economy.sql` |
| `15-lld/03-learning-engine.md` | `crates/learning` (`bkt`, `sm2`, `difficulty`, `progress`) |
| `15-lld/04-gate-flow.md` | `crates/assessment/src/gate_saga.rs` + `crates/learning/src/gate.rs` |
| `15-lld/05-ai-gateway.md` | `crates/ai` (`gateway`, `embeddings`, `rag`, `guardrails`, `prompts`, `cache`) + `services/genai` |
| `15-lld/06-onboarding-state.md` | `crates/recommendations` (`fit_score`, `cold_start`, `collaborative`, `fairness`) — the FSM/UI flow is not yet wired |
| `15-lld/07-recommendation.md` | `crates/recommendations` |
| `15-lld/08-marketplace-escrow.md` | `crates/marketplace` (`order`, `escrow`, `dispute`, `settlement`) |
| `15-lld/09-notifications.md` | `crates/notifications` (`email`, `in_app`, `push`) |
| `15-lld/10-content-cms.md` | _Not yet implemented (backlog)_ |
| `15-lld/11-social-guilds.md` | `crates/social` (`guild`, `league`, `reputation`, `moderation`) |
| `15-lld/12-analytics-pipeline.md` | _Not yet implemented (PostHog/warehouse deferred)_ |
| `15-lld/13-edge-cases-master.md` | Encoded across the crates — summarized in §8 |

## 10. Run & deploy

- Local install/deploy steps: see **`SETUP.md`** at the repo root (accounts, `.env`, `cargo run -p api`, `cargo run -p genai`, `trunk serve`, Docker + Caddy).

## 11. Status & supersession note

- **Product/design baseline** — `docs/00-…`–`docs/15-…` remain the source of truth for *what* the product is (economy math, skill taxonomy, scoring, personas, phases, edge cases).
- **This document** supersedes the *technology-stack* sections of those docs, which described a Next.js / Drizzle / BullMQ / Lucia plan that was **replaced by the Rust implementation** during the build.
- Banners pointing to this document have been added to `06-platforms/01-architecture.md`, `06-platforms/07-local-dev-guide.md`, `06-platforms/08-deployment-guide.md`, and `00-foundations/05-budget-stack.md`.