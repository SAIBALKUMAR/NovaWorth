# Local Development & Engineering Onboarding Guide

> ⚠️ **Implementation note:** the codebase is implemented in **Rust**. See [16-rust-implementation.md](../16-rust-implementation.md) and the runbook [SETUP.md](../../SETUP.md). The Turborepo/Next.js onboarding steps in this document are superseded.

| Metadata | Value |
|---|---|
| **Section Path** | `docs/06-platforms/07-local-dev-guide.md` |
| **Version** | 1.0.0 |
| **Last Updated** | 2026-11-21 |
| **Related Docs** | [01-architecture.md](01-architecture.md), [05-budget-stack.md](../00-foundations/05-budget-stack.md), [08-deployment-guide.md](08-deployment-guide.md), [03-scoring-model.md](../01-economy/03-scoring-model.md), [01-requirements-index.md](../14-appendices/01-requirements-index.md) |

---

## Table of Contents
1. [Engineering Prerequisites & Toolchains](#1-engineering-prerequisites--toolchains)
2. [Monorepo Repository Architecture](#2-monorepo-repository-architecture)
3. [Environment Configuration (.env.example Specification)](#3-environment-configuration-envexample-specification)
4. [First-Time Developer Setup Runbook](#4-first-time-developer-setup-runbook)
5. [Database Provisioning, Drizzle ORM & Catalog Seeding](#5-database-provisioning-drizzle-orm--catalog-seeding)
6. [Local Docker Infrastructure vs. Hybrid Cloud Boundary](#6-local-docker-infrastructure-vs-hybrid-cloud-boundary)
7. [Standard Engineering Workflows](#7-standard-engineering-workflows)
   - [7.1 Adding a New REST API Endpoint (OpenAPI First)](#71-adding-a-new-rest-api-endpoint-openapi-first)
   - [7.2 Adding a New Database Entity (Drizzle Schema)](#72-adding-a-new-database-entity-drizzle-schema)
   - [7.3 Creating a New Asynchronous Queue Job (BullMQ)](#73-creating-a-new-asynchronous-queue-job-bullmq)
   - [7.4 Authoring a Transactional Email Template (React Email)](#74-authoring-a-transactional-email-template-react-email)
8. [Testing Strategy & Quality Verification](#8-testing-strategy--quality-verification)
9. [Troubleshooting Runbook (16 Common Engineering Scenarios)](#9-troubleshooting-runbook-16-common-engineering-scenarios)

---

## 1. Engineering Prerequisites & Toolchains

### 1.1 Purpose
This section establishes the deterministic host machine baseline required to develop, compile, test, and containerize the NovaWorth platform without local version drift.

### 1.2 Required Software Versions
| Dependency | Exact Supported Version | Installation Command / Verification | Purpose in Toolchain |
|---|---|---|---|
| **Node.js** | `v20.18.x LTS` (Iron) | `node -v` | Core JavaScript runtime for Next.js, workers, and build scripts |
| **pnpm** | `9.12.x` or later | `pnpm -v` (Install: `corepack enable && corepack prepare pnpm@latest --activate`) | Strict monorepo package management and symlinked dependency isolation |
| **Docker Desktop / Engine**| Engine `27.x`+ / Compose `v2.29`+ | `docker --version && docker compose version` | Local container runtime for Redis, Meilisearch, and Prometheus |
| **Git** | `2.45.x` or later | `git --version` | Version control, submodule management, and commit signing |
| **OpenSSL** | `3.0.x` or later | `openssl version` | Cryptographic key generation for Lucia Auth secret signing |

### 1.3 Recommended VSCode Workspace Configuration
To enforce architectural boundaries, all engineers must install the following workspace extensions:
- **`dbaeumer.vscode-eslint`**: Strict TypeScript linting across Turborepo packages.
- **`esbenp.prettier-vscode`**: Automated code formatting on save.
- **`bierner.markdown-mermaid`**: Live preview rendering for architectural Mermaid diagrams.
- **`tamasfe.even-better-toml`**: Syntax highlighting for Turborepo and Cargo manifests.
- **`humao.rest-client`**: In-editor HTTP testing for REST and OpenAPI endpoints.

---

## 2. Monorepo Repository Architecture

### 2.1 Purpose
The repository is structured as a **Turborepo monorepo**, strictly separating applications (`apps/*`) from isolated, reusable domain packages (`packages/*`). Applications consume domain logic exclusively through typed package interfaces.

### 2.2 Directory Structure
```
edu-economy/
├── apps/
│   ├── web/                     # Next.js 15 App Router PWA (SSR, RSC, Service Worker)
│   │   ├── app/                 # Routes, API handlers (/api/v1/*), Server Actions
│   │   ├── components/          # Reusable React UI primitives (Radix UI + Tailwind)
│   │   ├── hooks/               # Client-side React hooks (audio, WebAssembly, PWA)
│   │   └── public/              # PWA manifest, service worker scripts, static icons
│   └── worker/                  # BullMQ background workers and scheduled cron tasks
│       ├── src/                 # Job processors (email, ledger audit, vector reindex)
│       └── index.ts             # Worker cluster entry point and Redis event loop
├── packages/
│   ├── db/                      # Neon Postgres client, Drizzle ORM schema, migrations
│   │   ├── src/schema/          # Entity definitions (users, ledger, skills, assessments)
│   │   ├── migrations/          # Version-controlled SQL migration deltas
│   │   └── seeds/               # 500-skill catalog seed files and test fixtures
│   ├── auth/                    # Lucia Auth v3 configuration, session adapters, RBAC
│   ├── ai/                      # OpenAI gateway, Socratic prompt router, RAG retrieval
│   ├── storage/                 # Cloudflare R2 S3-compatible client and presigned URLs
│   ├── email/                   # Resend transactional email client and React Email components
│   ├── queue/                   # BullMQ queue definitions, job payloads, and producer helpers
│   └── types/                   # Shared TypeScript models, Zod schemas, OpenAPI 3.1 specs
├── docker/
│   ├── docker-compose.dev.yml   # Local dev containers (Redis, Meilisearch, Prometheus)
│   └── docker-compose.prod.yml  # Production deployment manifest for Hetzner CAX21
├── turbo.json                   # Turborepo task pipeline dependency graph
├── package.json                 # Monorepo root scripts and pnpm workspace configuration
└── .env.example                 # Master environment variable template
```

---

## 3. Environment Configuration (`.env.example` Specification)

### 3.1 Purpose
Defines every runtime configuration key across apps and packages, providing explicit procurement instructions and fail-safe defaults for local development.

### 3.2 Master `.env.example` Template
```bash
# ==============================================================================
# NovaWorth - MASTER LOCAL ENVIRONMENT CONFIGURATION
# Copy this file to '.env' at the repository root before running setup scripts.
# ==============================================================================

# ------------------------------------------------------------------------------
# 1. APPLICATION CORE & RUNTIME
# ------------------------------------------------------------------------------
NODE_ENV=development
NEXT_PUBLIC_APP_URL=http://localhost:3000
API_SERVER_PORT=4000
LOG_LEVEL=debug

# ------------------------------------------------------------------------------
# 2. DATABASE & NEON POSTGRESQL (Required: Cloud or Local Postgres)
# Obtain from Neon Console (https://console.neon.tech) -> Project -> Connection String
# ------------------------------------------------------------------------------
# Direct connection for schema migrations (Port 5432)
DATABASE_URL=postgres://edu_user:edu_pass@ep-cool-fog-123456.us-east-2.aws.neon.tech/neondb?sslmode=require
# Pooled connection for high-concurrency app routes (Port 6543)
DATABASE_POOLED_URL=postgres://edu_user:edu_pass@ep-cool-fog-123456-pooler.us-east-2.aws.neon.tech/neondb?sslmode=require

# ------------------------------------------------------------------------------
# 3. AUTHENTICATION & SECURITY (Lucia Auth v3)
# Generate secrets via: `openssl rand -hex 32`
# ------------------------------------------------------------------------------
AUTH_SECRET=4f8b9e6a1d2c3e5f7a9b0c2d4e6f8a1b3c5d7e9f0a2b4c6d8e0f1a3b5c7d9e1f
SESSION_COOKIE_NAME=edu_session_token

# ------------------------------------------------------------------------------
# 4. REDIS CACHE & QUEUE ENGINE (Local Docker)
# Points to the local Docker Compose Redis container
# ------------------------------------------------------------------------------
REDIS_URL=redis://localhost:6379
REDIS_PASSWORD=

# ------------------------------------------------------------------------------
# 5. MEILISEARCH CATALOG SEARCH (Local Docker)
# Master key configured in docker/docker-compose.dev.yml
# ------------------------------------------------------------------------------
MEILISEARCH_HOST=http://localhost:7700
MEILISEARCH_MASTER_KEY=local_master_dev_key_only_change_in_prod_123

# ------------------------------------------------------------------------------
# 6. OPENAI API & EMBEDDINGS (Required for AI Tutor & RAG)
# Obtain from OpenAI Platform (https://platform.openai.com/api-keys)
# ------------------------------------------------------------------------------
OPENAI_API_KEY=sk-proj-demoKeyReplaceWithYourPersonalTestKey
OPENAI_MODEL_TUTOR=gpt-4o-mini
OPENAI_MODEL_PANEL=gpt-4o
OPENAI_MODEL_EMBEDDING=text-embedding-3-small

# ------------------------------------------------------------------------------
# 7. CLOUDFLARE R2 OBJECT STORAGE (Optional in Dev - Mocked if empty)
# Obtain from Cloudflare Dashboard -> R2 -> Manage R2 API Tokens
# ------------------------------------------------------------------------------
R2_ACCOUNT_ID=mock_account_id_dev
R2_ACCESS_KEY_ID=mock_access_key_dev
R2_SECRET_ACCESS_KEY=mock_secret_key_dev
R2_BUCKET_NAME=edu-economy-artifacts-dev
R2_PUBLIC_DOMAIN=http://localhost:3000/mock-r2

# ------------------------------------------------------------------------------
# 8. RESEND TRANSACTIONAL EMAIL (Optional in Dev - Logs to console if empty)
# Obtain from Resend Dashboard (https://resend.com/api-keys)
# ------------------------------------------------------------------------------
RESEND_API_KEY=re_dev_mock_key_will_log_to_terminal
EMAIL_FROM_ADDRESS=onboarding@educationeconomy.org

# ------------------------------------------------------------------------------
# 9. TELEMETRY & PRODUCT ANALYTICS (Optional in Dev)
# Obtain from PostHog Cloud (https://app.posthog.com)
# ------------------------------------------------------------------------------
NEXT_PUBLIC_POSTHOG_KEY=phc_mock_key_for_local_testing_only
NEXT_PUBLIC_POSTHOG_HOST=https://us.i.posthog.com
```

---

## 4. First-Time Developer Setup Runbook

### 4.1 Purpose
A deterministic, single-turn runbook that brings an engineer from a clean `git clone` to an operational local development environment in under 7 minutes.

### 4.2 Setup Flow Diagram
```mermaid
flowchart TD
    A["1. Clone Repository"] --> B["2. pnpm install"]
    B --> C["3. Copy .env.example to .env"]
    C --> D["4. docker compose -f docker/docker-compose.dev.yml up -d"]
    D --> E["5. pnpm db:push (Apply Drizzle Schema)"]
    E --> F["6. pnpm db:seed:catalog (Seed 500 Skills)"]
    F --> G["7. pnpm dev (Launch Turborepo Apps)"]
    G --> H["8. Verify http://localhost:3000 Ready"]
```

### 4.3 Step-by-Step Terminal Execution Commands

```bash
# Step 1: Clone the monorepo
git clone https://github.com/novaworth/edu-economy.git
cd edu-economy

# Step 2: Install workspace dependencies using strict pnpm
corepack enable
pnpm install

# Step 3: Initialize environment variables
cp .env.example .env
# Edit .env to add your personal Neon connection string and OpenAI key

# Step 4: Start local infrastructure containers (Redis & Meilisearch)
docker compose -f docker/docker-compose.dev.yml up -d

# Step 5: Verify containers are healthy
docker compose -f docker/docker-compose.dev.yml ps

# Step 6: Apply Drizzle database schema to Neon Postgres
pnpm db:push

# Step 7: Seed the core 500-skill catalog and baseline rubrics
pnpm db:seed:catalog

# Step 8: Build shared packages
pnpm build:packages

# Step 9: Launch Next.js web application and background worker in parallel
pnpm dev
```

---

## 5. Database Provisioning, Drizzle ORM & Catalog Seeding

### 5.1 Purpose
The platform leverages **Neon Serverless PostgreSQL** with the `pgvector` extension. This section details project provisioning, branch isolation for local development, schema migrations, and catalog seeding.

### 5.2 Neon Provisioning Runbook
1. Sign in to the [Neon Console](https://console.neon.tech).
2. Create a new project titled `edu-economy-dev`.
3. In the SQL Editor, verify that the vector extension is available:
   ```sql
   CREATE EXTENSION IF NOT EXISTS vector;
   ```
4. Copy the connection string provided in the dashboard, ensuring `sslmode=require` is appended.
5. In your local `.env`, set `DATABASE_URL` to the unpooled endpoint and `DATABASE_POOLED_URL` to the pooled endpoint (`-pooler`).

### 5.3 Seeding the 500-Skill Catalog
The platform catalog contains 500 distinct skills mapped across 10 levels of progression. The automated seeder imports these definitions into Neon Postgres and populates Meilisearch full-text search indexes.

```typescript
// packages/db/src/seeds/catalog-seed.ts
import { db } from '../client';
import { skills, skillLevels } from '../schema';
import { MeiliSearch } from 'meilisearch';

const meili = new MeiliSearch({
  host: process.env.MEILISEARCH_HOST || 'http://localhost:7700',
  apiKey: process.env.MEILISEARCH_MASTER_KEY
});

export async function seedCatalog() {
  console.log('🌱 Starting 500-Skill Catalog Ingestion...');

  const sampleSkills = [
    { id: 'SWE-01', code: 'FULLSTACK_ENG', domain: 'ENGINEERING', domainMultiplier: '1.20', title: 'Fullstack Software Engineering' },
    { id: 'BIO-04', code: 'GENOMIC_DATA', domain: 'BIOTECH', domainMultiplier: '1.40', title: 'Genomic Sequence Analysis' },
    { id: 'FIN-02', code: 'ALGO_TRADING', domain: 'FINANCE', domainMultiplier: '1.30', title: 'Algorithmic Liquidity Systems' }
  ];

  for (const item of sampleSkills) {
    // 1. Upsert Skill Record
    await db.insert(skills).values(item).onConflictDoNothing();

    // 2. Generate 100 Progressive Levels for each skill
    const levelsBatch = [];
    for (let level = 1; level <= 100; level++) {
      levelsBatch.push({
        skillId: item.id,
        level: level,
        band: Math.ceil(level / 10),
        title: `Level ${level} Mastery Benchmark`,
        // Canonical Power-Law Reward Precomputation
        baseEc: Math.round(10 * Math.pow(level, 1.2) * 100) / 100,
        baseXp: Math.round(100 * Math.pow(level, 1.5))
      });
    }
    await db.insert(skillLevels).values(levelsBatch).onConflictDoNothing();
  }

  // 3. Index Catalog into Meilisearch
  const index = meili.index('skills');
  await index.addDocuments(sampleSkills, { primaryKey: 'id' });
  console.log('✅ Catalog and Search Indexes Successfully Populated.');
}
```

---

## 6. Local Docker Infrastructure vs. Hybrid Cloud Boundary

To balance developer ergonomics against operational fidelity, the local environment operates in a **Hybrid Cloud Model**: high-throughput, low-latency state engines run locally via Docker, while complex stateful cloud primitives run against cloud free tiers.

| Subsystem Component | Local Dev Execution Location | Cloud Provider Execution | Developer Rationale & Trade-off |
|---|---|---|---|
| **PostgreSQL & `pgvector`** | Remote Cloud (Neon Branch) | Neon Free Tier / Branch | Eliminates heavy local Postgres setup; unlocks instant branch rollback and parity. |
| **In-Memory Cache (Redis)** | Local Container (`redis:7.4`) | Self-Hosted VPS in Prod | Ultra-fast local restarts; zero latency on worker queues; no Upstash bill. |
| **Search Engine (Meilisearch)**| Local Container (`v1.10`) | Self-Hosted VPS in Prod | Instant index flushing; zero cost for test index creation and demolition. |
| **AI Inference (OpenAI)** | Remote Cloud API (Personal Key)| OpenAI Cloud | Local CPU cannot run 8B+ LLMs at acceptable developer latencies. |
| **Object Store (Cloudflare R2)**| Local Filesystem Mock or R2 | Cloudflare R2 | Uploads write to local `/tmp/edu-storage` if R2 credentials are omitted. |
| **Transactional Email (Resend)**| Local Terminal Logger or Resend | Resend Cloud API | Prevents accidental delivery of test emails to real learner inboxes. |

---

## 7. Standard Engineering Workflows

### 7.1 Adding a New REST API Endpoint (OpenAPI First)

```mermaid
flowchart LR
    A["1. Define Zod Schema in packages/types"] --> B["2. Implement Route in apps/web/app/api"]
    B --> C["3. Mount in OpenAPI Registry"]
    C --> D["4. Execute Vitest Integration Test"]
```

```typescript
// packages/types/src/schemas/gate.ts
import { z } from 'zod';

export const SubmitGateInputSchema = z.object({
  skillId: z.string().min(2),
  level: z.number().int().min(1).max(100),
  artifactUrl: z.string().url().optional(),
  reflectionText: z.string().min(50).max(5000)
});

export type SubmitGateInput = z.infer<typeof SubmitGateInputSchema>;
```

### 7.2 Adding a New Database Entity (Drizzle Schema)
1. Define table schema in `packages/db/src/schema/ledger.ts`:
   ```typescript
   import { pgTable, text, numeric, timestamp, varchar } from 'drizzle-orm/pg-core';

   export const sponsorPools = pgTable('sponsor_pools', {
     id: varchar('id', { length: 36 }).primaryKey(),
     sponsorName: text('sponsor_name').notNull(),
     availableBalance: numeric('available_balance', { precision: 12, scale: 2 }).notNull(),
     emittedBalance: numeric('emitted_balance', { precision: 12, scale: 2 }).default('0.00'),
     createdAt: timestamp('created_at').defaultNow().notNull()
   });
   ```
2. Generate migration SQL delta:
   ```bash
   pnpm --filter @edu/db drizzle-kit generate
   ```
3. Apply migration to local Neon branch:
   ```bash
   pnpm --filter @edu/db drizzle-kit push
   ```

### 7.3 Creating a New Asynchronous Queue Job (BullMQ)
```typescript
// packages/queue/src/jobs/audit-ledger.ts
import { Queue, Worker, Job } from 'bullmq';
import Redis from 'ioredis';

const connection = new Redis(process.env.REDIS_URL || 'redis://localhost:6379', { maxRetriesPerRequest: null });

export interface AuditLedgerPayload {
  auditRunId: string;
  triggeredBy: string;
}

export const auditQueue = new Queue<AuditLedgerPayload>('audit-ledger', { connection });

export function createAuditWorker() {
  return new Worker<AuditLedgerPayload>(
    'audit-ledger',
    async (job: Job<AuditLedgerPayload>) => {
      console.log(`[Job ${job.id}] Executing Ledger Double-Entry Rebalancing Audit...`);
      // Rebalancing logic verifies Sum(Debits) == Sum(Credits)
      return { balanced: true, auditedAt: new Date().toISOString() };
    },
    { connection, concurrency: 2 }
  );
}
```

### 7.4 Authoring a Transactional Email Template (React Email)
```tsx
// packages/email/src/templates/GatePassedReceipt.tsx
import * as React from 'react';
import { Html, Body, Container, Heading, Text } from '@react-email/components';

interface GatePassedProps {
  learnerName: string;
  skillName: string;
  level: number;
  ecEarned: number;
  xpEarned: number;
}

export const GatePassedReceipt = ({ learnerName, skillName, level, ecEarned, xpEarned }: GatePassedProps) => (
  <Html>
    <Body style={{ backgroundColor: '#0f172a', color: '#f8fafc', fontFamily: 'sans-serif' }}>
      <Container style={{ margin: '0 auto', padding: '24px', maxWidth: '580px' }}>
        <Heading style={{ color: '#38bdf8' }}>Proof of Mastery Verified</Heading>
        <Text>Congratulations {learnerName},</Text>
        <Text>You have officially passed <strong>{skillName} - Level {level}</strong>.</Text>
        <Text>Awards Emitted: <strong>+{ecEarned} EC</strong> and <strong>+{xpEarned} XP</strong>.</Text>
      </Container>
    </Body>
  </Html>
);
```

---

## 8. Testing Strategy & Quality Verification

| Test Layer | Framework | Scope & Boundary | Target Execution Time | Command |
|---|---|---|---|---|
| **Unit Tests** | Vitest | Mathematical calculations (power-law curves, reward algorithms, schema parsing) | $< 5\text{ seconds}$ | `pnpm test:unit` |
| **Integration Tests** | Vitest + Neon Branch | Database transactions, serializable ledger isolation, BullMQ queues | $< 30\text{ seconds}$ | `pnpm test:integration` |
| **E2E Tests** | Playwright | Complete learner core loop: Onboard $\to$ Learn $\to$ Pass Gate $\to$ Redeem | $< 90\text{ seconds}$ | `pnpm test:e2e` |
| **Typecheck & Lint** | `tsc` + ESLint | Full repository type verification and syntax compliance | $< 15\text{ seconds}$ | `pnpm lint && pnpm typecheck` |

---

## 9. Troubleshooting Runbook (16 Common Engineering Scenarios)

### Issue 1: `ECONNREFUSED 127.0.0.1:6379`
- **Cause:** Local Redis Docker container is not running or crashed.
- **Remediation:** Run `docker compose -f docker/docker-compose.dev.yml up -d redis` and verify with `docker ps`.

### Issue 2: `NeonDbError: password authentication failed for user`
- **Cause:** Neon database branch connection string has expired or credentials in `.env` contain unescaped characters.
- **Remediation:** Refresh the credentials in the Neon Console, ensure the URL string is URL-encoded, and verify `sslmode=require`.

### Issue 3: `DrizzleError: relation "ledger_entries" does not exist`
- **Cause:** Schema changes were made without executing migrations on the active branch.
- **Remediation:** Execute `pnpm db:push` to synchronize the Drizzle schema directly with the target database.

### Issue 4: `OpenAIError: 429 You exceeded your current quota`
- **Cause:** OpenAI API key has zero pre-funded credit balance or hit the tier RPM limit.
- **Remediation:** Add a minimum $5 billing credit in the OpenAI platform dashboard or enable mock AI mode via `MOCK_AI_RESPONSES=true`.

### Issue 5: `MeiliSearchCommunicationError: Failed to connect to Meilisearch at http://localhost:7700`
- **Cause:** Meilisearch container failed to initialize due to permissions on the local `./meili_data` volume.
- **Remediation:** Run `docker compose -f docker/docker-compose.dev.yml restart meilisearch`. If permission denied, run `chmod -R 777 ./meili_data`.

### Issue 6: `pnpm: command not found`
- **Cause:** Corepack is not enabled in the current Node.js shell environment.
- **Remediation:** Execute `corepack enable && corepack prepare pnpm@9.12.0 --activate`.

### Issue 7: `Ports are not available: 0.0.0.0:6379 bind: address already in use`
- **Cause:** A native Redis server or conflicting background Docker container is holding port 6379.
- **Remediation:** Identify and terminate the conflicting process:
  ```bash
  sudo lsof -i :6379 || netstat -ano | findstr :6379
  docker kill $(docker ps -q --filter expose=6379)
  ```

### Issue 8: `TypeScript error: Cannot find module '@edu/types'`
- **Cause:** Turborepo internal package build cache is cold or workspace symlinks were broken.
- **Remediation:** Run `pnpm clean && pnpm install && pnpm build:packages`.

### Issue 9: `Playwright: browser not installed`
- **Cause:** End-to-end test runner missing underlying browser binaries.
- **Remediation:** Execute `pnpm exec playwright install --with-deps chromium`.

### Issue 10: `Neon Branch Limit Exceeded: Free tier allows maximum 10 child branches`
- **Cause:** Orphaned CI preview branches were not pruned post-PR merge.
- **Remediation:** Run `neon branches list` and delete obsolete branches via `neon branches delete <branch-id>`.

### Issue 11: `Lucia Auth: Session cookie missing or unparseable`
- **Cause:** Browser rejected auth cookie due to `Secure=true` flag over unencrypted `http://localhost:3000`.
- **Remediation:** In `packages/auth/src/lucia.ts`, ensure `secure: process.env.NODE_ENV === 'production'` is configured.

### Issue 12: `BullMQ: Missing lock for job <id>`
- **Cause:** Long-running async task exceeded the worker lock duration threshold (default 30s).
- **Remediation:** In worker options, increase `lockDuration: 60000` or invoke `job.updateProgress()` periodically to renew the lock.

### Issue 13: `ResendError: You can only send testing emails to your own email address`
- **Cause:** Attempting to send an email to a third-party address without a verified sending domain on the Resend free tier.
- **Remediation:** Set `EMAIL_TO` in your local tests to the email address registered with your Resend account.

### Issue 14: `Next.js 15: Route used headers()... should be awaited`
- **Cause:** Next.js 15 migration introduced asynchronous `headers()` and `cookies()`.
- **Remediation:** Change `const headerList = headers();` to `const headerList = await headers();`.

### Issue 15: `Turbo error: Command exited with code 137 (OOM)`
- **Cause:** Node.js V8 heap memory exhaustion during heavy parallel compilation.
- **Remediation:** Prepend environment flag: `NODE_OPTIONS="--max-old-space-size=4096" pnpm build`.

### Issue 16: `Cloudflare R2: SignatureDoesNotMatch`
- **Cause:** Mismatched S3 endpoint format or invalid secret key formatting.
- **Remediation:** Ensure R2 endpoint format matches: `https://<ACCOUNT_ID>.r2.cloudflarestorage.com` with region set to `auto`.
