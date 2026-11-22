# NovaWorth — Setup & Deployment Runbook

> Step-by-step guide: install the toolchain, create every required account, run the app locally, and deploy it to a VPS.
> Pricing/limits verified against provider pages **November 2026** — re-check before you rely on them.

## 0. What you are building

This repo is a Rust workspace with:

| Component | Dir / service | Default URL |
|---|---|---|
| API (auth, economy, marketplace, social) | `services/api` | `http://localhost:4000` |
| GenAI gateway (OpenAI + budget) | `services/genai` | `http://localhost:5000` |
| UI (Leptos 0.7 client-side wasm) | `services/ui` | `http://localhost:8080` (trunk dev) |
| Postgres | **Neon** (serverless, external) | managed |
| Redis (cache/queues) | `docker-compose.yml` | `localhost:6379` |
| Reverse proxy + TLS | **Caddy** (prod) | `:80` / `:443` |
| Object storage (future assets) | **Cloudflare R2** | external |
| Emails | **Resend** | external |

The final production shape: **Caddy** serves the static UI and proxies `/auth/*`, `/economy/*`, `/social/*`, `/api/*` → the API, and `/generate/*` → GenAI, with Postgres hosted on Neon.

---

## 1. Accounts & tools you will need (summary)

| Thing | Site | Cost |
|---|---|---|
| GitHub account | https://github.com | Free |
| Neon Postgres project | https://neon.com | Free (1 GB storage, 100 compute-hrs/mo, no card) |
| Cloudflare R2 bucket + API token | https://dash.cloudflare.com | Free (10 GB-month, egress free) |
| Resend API key | https://resend.com | Free (3,000 emails/mo) |
| OpenAI API key | https://platform.openai.com | Pay per token (prepaid ~$5) |
| Hetzner Cloud server | https://console.hetzner.cloud | ~€5–8/mo |
| A domain name (for HTTPS) | any registrar | ~$10/yr (optional for a public URL) |

**Tools:** Rust, the `wasm32` target, `trunk`, (optional) `sqlx-cli`, Docker, Git.

## 2. Install the toolchain

### 2.1 Install Rust (once)
- **Windows:** go to https://rustup.rs → download `rustup-init.exe` → run it → accept the default (MSVC) options.
- **macOS / Linux:** run `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Verify with `rustc --version` (any 1.82+ toolchain is fine).

### 2.2 Add the WebAssembly target (needed for the UI)
```bash
rustup target add wasm32-unknown-unknown
```

### 2.3 Install `trunk` (builds/serves the Leptos UI)
```bash
cargo install --locked trunk
trunk --version
```

### 2.4 (Optional) install `sqlx-cli` only if you want to run migrations manually
> Migrations are embedded and run automatically when the API starts, so this is optional.
```bash
cargo install sqlx-cli --no-default-features --features postgres,rustls
```

### 2.5 Install Docker + Git
- **Docker Desktop** (Windows/macOS): https://www.docker.com/products/docker-desktop/
- **Docker Engine** (Linux server): `curl -fsSL https://get.docker.com | sh`
- **Git:** https://git-scm.com/downloads
- Verify: `git --version`, `docker --version`, `docker compose version`

## 3. Create accounts & collect credentials

Store every value from this section in a scratch notes file — you'll paste them into `.env` in step 4.

### 3.1 GitHub (code + CI/CD)
1. Go to https://github.com/signup → create a free account.
2. Create the repository: https://github.com/new → Repository name `novaworth` → **Private** → **Create repository**. (We push the code here in step 6.)

### 3.2 Neon — serverless Postgres
1. Go to https://neon.com → **Sign up** (GitHub / Google / email). No credit card required.
2. Dashboard → **Create a project** → name `novaworth` → choose the region **closest to your VPS** (e.g., `AWS eu-central` for Germany, or `AWS us-east` for the US) → **Create project**.
3. Open the project → **Connect** (or the "Connection string" panel) → copy the **Pooled** connection string. It looks like:
   `postgresql://<user>:<pass>@ep-xxxx.region.aws.neon.tech/neondb?sslmode=require`
   → Save this as **`DATABASE_URL`**.
   > Free tier (as of Nov 2026): 1 GB storage per project, 100 compute-hours/month, no card.

### 3.3 Cloudflare R2 — object storage
1. Go to https://dash.cloudflare.com/sign-up → create a free account.
2. Left sidebar → **R2** → **Create bucket** → name `novaworth-assets` → **Create bucket**.
3. Still on R2 → **Manage R2 API Tokens** → **Create API token** → grant *Object Read & Write* for that bucket → copy **Access Key ID** and **Secret Access Key**.
4. Copy your **Account ID** (shown on the R2 overview / dashboard sidebar).
   → Save as **`R2_ACCOUNT_ID`**, **`R2_ACCESS_KEY_ID`**, **`R2_SECRET_ACCESS_KEY`**, bucket **`novaworth-assets`**.
   > Free tier (as of Nov 2026): 10 GB-month storage, egress **free**.

### 3.4 Resend — transactional email
1. Go to https://resend.com → **Sign up** → verify your email.
2. Dashboard → **API Keys** → **Create API Key** → copy it (shown only once).
   → Save as **`RESEND_API_KEY`**.
3. To send from your own domain later: **Domains → Add Domain** → follow the DNS (SPF/DKIM) instructions.
   > Free tier (as of Nov 2026): 3,000 emails/month, 100/day, 3 domains. For local dev you can email your own address without adding a domain.

### 3.5 OpenAI — LLM + embeddings
1. Go to https://platform.openai.com/signup → create an account.
2. Go to **Billing** and add credit (minimum top-up is typically **$5**).
3. Go to **API keys → Create new secret key** → copy it.
   → Save as **`OPENAI_API_KEY`**.
4. The app defaults to `gpt-4o-mini` and `text-embedding-3-small`. OpenAI has been deprecating `gpt-4o-mini`; check https://platform.openai.com/docs/pricing and, if you switch to `gpt-4.1-mini`, update `OPENAI_MODEL_DEFAULT` in `crates/common/src/constants.rs`.

### 3.6 Hetzner Cloud — the VPS where the app runs
1. Go to https://console.hetzner.cloud → **Register** (name, address, payment method). New accounts are sometimes manually verified first — allow a few hours.
2. **Create a project** → pick a location: **Nuremberg**/**Falkenstein** (Germany), **Helsinki** (Finland), or **Ashburn** (US).
3. **Add Server / Create server**:
   - **Image:** Ubuntu 24.04
   - **Type:** **CAX21** (4 vCPU ARM · 8 GB RAM · 80 GB NVMe) — or the smaller **CAX11** (2 vCPU · 4 GB · 40 GB)
   - **SSH key:** add one (generate first: `ssh-keygen -t ed25519` and paste `~/.ssh/id_ed25519.pub`)
   - **Networking:** public IPv4 + IPv6
   - **Name:** `novaworth`
4. After creation, note the server's **public IPv4 address** (used for DNS and SSH).

### Credentials checklist
`DATABASE_URL` · `SESSION_SECRET` (make one up, min 32 random chars) · `OPENAI_API_KEY` · `RESEND_API_KEY` · `R2_ACCOUNT_ID` · `R2_ACCESS_KEY_ID` · `R2_SECRET_ACCESS_KEY` · VPS IPv4 · SSH key path.

## 4. Configure the project (`.env`)

1. Clone the repo and enter it:
   ```bash
   git clone https://github.com/<you>/novaworth.git
   cd novaworth
   ```
2. Copy the template and open it:
   ```bash
   # Windows (PowerShell):  Copy-Item .env.example .env
   cp .env.example .env
   ```
3. Fill in the real values:

   ```env
   # From Neon (3.2)
   DATABASE_URL=postgresql://USER:PASS@ep-xxxx.region.aws.neon.tech/neondb?sslmode=require

   # Local dev Redis (Docker container, unchanged)
   REDIS_URL=redis://localhost:6379

   # Any random string, 32+ characters (generate e.g. with: openssl rand -hex 32)
   SESSION_SECRET=change_me_to_a_long_random_secret

   # From OpenAI (3.5)
   OPENAI_API_KEY=sk-...
   OPENAI_MONTHLY_BUDGET_USD=20

   # From Resend (3.4)
   RESEND_API_KEY=re_...
   EMAIL_FROM=NovaWorth <no-reply@novaworth.ai>

   # From Cloudflare R2 (3.3) — used for asset storage (optional in the current build)
   R2_ACCOUNT_ID=...
   R2_ACCESS_KEY_ID=...
   R2_SECRET_ACCESS_KEY=...
   R2_BUCKET=novaworth-assets

   APP_ENV=development
   # Origin allowed by the API's CORS. For local trunk dev use the UI port.
   APP_URL=http://localhost:8080
   ```

## 5. Run locally

### 5.1 Start Postgres + Redis (Docker)
```bash
docker compose up -d
docker compose ps          # postgres + redis should be "running"
```

### 5.2 Build / test the backend
```bash
cargo check --workspace
cargo test --workspace
```

### 5.3 Run the services
```bash
# Terminal 1 — API (applies migrations automatically on startup)
cargo run -p api            # http://localhost:4000

# Terminal 2 — GenAI gateway
cargo run -p genai          # http://localhost:5000
```

### 5.4 Smoke-test the API (curl)
```bash
# Health
curl http://localhost:4000/health

# Register a user (returns a user JSON; session cookie is set)
curl -X POST http://localhost:4000/auth/register \
  -H "Content-Type: application/json" \
  -d '{"email":"you@example.com","username":"you","password":"password123"}'

# Login (store cookies so the wallet call works)
curl -c cookies.txt -X POST http://localhost:4000/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"you@example.com","password":"password123"}'

# Wallet (authenticated)
curl -b cookies.txt http://localhost:4000/economy/wallet

# Public leaderboard
curl "http://localhost:4000/social/leaderboard?limit=10"

# AI explain (GenAI, on :5000)
curl -X POST http://localhost:5000/generate/explain \
  -H "Content-Type: application/json" \
  -d '{"user_id":"guest","topic":"quadratic equations"}'
```

### 5.5 Run the UI (Leptos)
```bash
cd services/ui
trunk serve                # http://localhost:8080
```
> The UI calls the API at a **same-origin** base (works out-of-the-box in production via Caddy). For local full-stack testing, either (a) test the API with the curl commands above, or (b) run a local Caddy that routes `localhost` to `trunk`/`:4000`/`:5000` exactly like production — see the `Caddyfile` at the repo root.

## 6. Deploy to production (Hetzner VPS + Neon + Caddy)

> **Where it runs:** your app containers live on a single Hetzner CAX21 VPS; Postgres is Neon (managed); emails via Resend; assets via R2; the domain is TLS-terminated by Caddy (auto Let's Encrypt).

### 6.1 (Recommended) Point a domain at the VPS
1. In your DNS provider (or Cloudflare, if you moved your domain there), create an **A record** for your hostname (e.g. `learn`) → your **VPS IPv4**.
   - E.g. `novaworth.ai  A  1.2.3.4`
   - If using Cloudflare, set the record to **DNS only** (grey cloud) while first testing, to avoid conflicts with Let's Encrypt.
2. Edit the repo **`Caddyfile`**: replace `novaworth.ai` with your actual hostname (e.g. `novaworth.ai`).

### 6.2 One-time VPS setup
```bash
ssh root@<VPS_IP>

# add a normal user (optional but recommended)
adduser deploy && usermod -aG sudo deploy

# install Docker + compose plugin
curl -fsSL https://get.docker.com | sh
docker compose version

# clone the app
sudo mkdir -p /opt/novaworth
sudo chown $USER /opt/novaworth
git clone https://github.com/<you>/novaworth.git /opt/novaworth
cd /opt/novaworth
```

Create `/opt/novaworth/.env` with **production** values (same keys as step 4, but):
```env
DATABASE_URL=postgresql://...  # your Neon pooled URL
REDIS_URL=redis://redis:6379   # internal Docker service name in prod
SESSION_SECRET=<a long random secret>
OPENAI_API_KEY=sk-...
RESEND_API_KEY=re_...
APP_ENV=production
APP_URL=https://novaworth.ai
```

### 6.3 Build the UI (static site)
On the VPS (install the wasm toolchain first), or locally then upload the `dist/` folder:
```bash
cd /opt/novaworth/services/ui
rustup target add wasm32-unknown-unknown
cargo install --locked trunk
trunk build          # produces services/ui/dist/
```
Caddy serves `services/ui/dist` mounted at `/srv/www` (already configured).

### 6.4 Start the stack
```bash
cd /opt/novaworth
docker compose -f docker-compose.prod.yml up -d --build
docker compose -f docker-compose.prod.yml ps
```
This builds + starts **api**, **genai**, **redis**, and **caddy**. Caddy requests and installs the HTTPS certificate automatically.

### 6.5 (Optional) GitHub Actions CI/CD
1. Push the code: `git remote add origin ... ; git push -u origin main`.
2. The workflow `.github/workflows/deploy.yml` runs tests, builds the Docker image, pushes it to **GitHub Container Registry (GHCR)**, and SSHes into the VPS to `docker compose pull && up -d`.
3. Add these **Actions secrets** in GitHub → repo → **Settings → Secrets and variables → Actions**:
   - `HETZNER_IP` — the VPS IPv4
   - `HETZNER_SSH_KEY` — the private key for the `deploy` user (add its `~/.ssh/authorized_keys` on the VPS)
   - (`GITHUB_TOKEN` is auto-provided.)
4. To make `docker compose pull` resolve the image from GHCR, in `docker-compose.prod.yml` change the `api`/`genai` images to `ghcr.io/<you>/novaworth:latest` and remove their `build:` lines; then on the VPS run once: `echo $GHCR_TOKEN | docker login ghcr.io -u <you> --password-stdin`.

> Start with the **manual** path (6.2–6.4) — it's simpler to debug. Add CI once it's stable.

## 7. Post-deploy verification

```bash
# Through Caddy (public): should return HTTP 200 over HTTPS
curl -I https://novaworth.ai/health        # UI static index (200)
curl https://novaworth.ai/health           # routes to API /health via Caddy
curl -I https://novaworth.ai/              # UI HTML (200)
```

Then in a browser:
1. Open `https://novaworth.ai` → the landing page renders.
2. **Register** a user → redirected to the **Dashboard**.
3. **Dashboard** shows your XP/EC wallet + the leaderboard; the **AI tutor** box returns an explanation (requires a valid OpenAI key).

## 8. Troubleshooting & notes

- **Hetzner 'verification' / new account delay** — brand-new accounts can be flagged for manual review (photo ID). If the server won't provision, wait or contact support; the docs team notes this is common for first orders.
- **Neon connection string** — use the **Pooled** URL for the app. If migrations fail, try the **Direct** URL (sqlx `migrate!` runs plain DDL, which works on both, but direct avoids any pooler DDL quirk).
- **Neon free-tier cold start** — the first query after ~5 min of idleness can take a few seconds while compute wakes (`Dataconnector cold start`). This is normal on the free plan.
- **Caddy no certificate** — make sure the domain A record actually resolves to the VPS IP, and that ports **80 and 443** are open in the Hetzner **Firewall** (Hetzner's default firewall is deny-all; create a rule allow 80/443 + 22).
- **Lock down the backend ports** — the compose maps `4000`/`5000` to the host for debugging. For production, remove those `ports:` mappings so `api`/`genai` are only reachable through Caddy, or firewall them to your IP.
- **OpenAI model deprecation** — `gpt-4o-mini` is being deprecated. If you switch to `gpt-4.1-mini`, change `OPENAI_MODEL_DEFAULT` in `crates/common/src/constants.rs` (and `OPENAI_EMBEDDING_MODEL` in `crates/ai/src/embeddings.rs`).
- **CORS** — in production everything is same-origin via Caddy, so no CORS is needed. In dev the API only allows `APP_URL`, so set `APP_URL` to your UI dev origin (`http://localhost:8080`).
- **Session cookies** — the API's `SESSION_COOKIE_NAME` cookie works same-origin out of the box. If you ever host UI and API on *different* origins, configure the cookie `Secure; SameSite=None`.
- **Migrations are automatic** — the API runs `sqlx::migrate!` on startup; rebuild the image after adding a new file under `migrations/`.
- **Secrets** — never commit `.env` (it's already in `.gitignore`). Keep `SESSION_SECRET` and API keys in CI secrets / the server `.env` only.
- **Backups** — Neon takes automatic snapshots; for off-site backups, schedule periodic `pg_dump` to Cloudflare R2 (not yet wired into the repo).

---

**Done.** You now have: a working local dev environment, live accounts for every external service, and a production deployment behind Caddy HTTPS on a Hetzner VPS with Neon Postgres. Re-check pricing on the provider pages before committing to paid tiers (this runbook was verified **November 2026**).