# Risk Register & Infrastructure Contingencies

| Metadata | Value |
|---|---|
| **Document Path** | `docs/14-appendices/02-risk-register.md` |
| **Version** | `2.0.0` |
| **Last Updated** | `2026-11-21` |
| **Author** | Senior Technical Writer & System Architect |
| **Status** | Active / Living Register |
| **Review Cadence** | Bi-weekly by Architecture & Operations; at every Phase Release Gate |
| **Budget Envelope** | Strict adherence to $1,000 Upfront + $100/mo Operating Runway |
| **Related Docs** | [Delivery Roadmap](../13-phases/01-roadmap.md), [Phase 0 Foundations](../13-phases/02-phase-0.md), [Phase 1 Alpha](../13-phases/03-phase-1.md), [Release Gates](../13-phases/08-release-gates.md), [Incident Response](../11-operations/04-incident-response.md) |

---

## Table of Contents

1. [Purpose & Governance Cadence](#1-purpose--governance-cadence)
2. [Scoring & Severity Taxonomy](#2-scoring--severity-taxonomy)
3. [Top Strategic Risks (R-01 through R-20)](#3-top-strategic-risks-r-01-through-r-20)
4. [Strategic Risk Detail (Deep Dives)](#4-strategic-risk-detail-deep-dives)
5. [Risk Treatment Framework](#5-risk-treatment-framework)
6. [Operational Review & Escalation Workflow](#6-operational-review--escalation-workflow)
7. [Governance Requirements](#7-governance-requirements)
8. [Budget & Infrastructure Risks (15 Bootstrap-Specific Risks)](#8-budget--infrastructure-risks-15-bootstrap-specific-risks)
9. [Automated Budget Risk Mitigation Engine](#9-automated-budget-risk-mitigation-engine)
10. [Edge Cases & Incident Recovery Runbooks](#10-edge-cases--incident-recovery-runbooks)
11. [Success Metrics & Audit Protocols](#11-success-metrics--audit-protocols)
12. [Related Documentation](#12-related-documentation)

---

## 1. Purpose & Governance Cadence

This document serves as the comprehensive risk management register for the NovaWorth platform. It catalogs, quantifies, and assigns mitigation protocols to all technical, systemic, economic, legal, and infrastructure failure modes. 

Under the project's strict bootstrapping constraint (**$1,000 upfront cash budget + $100/month maximum operating runway**), infrastructure failure modes that could otherwise be resolved via capital expenditure must be pre-empted through algorithmic fallbacks, zero-cost escape hatches, and strict quota governance.

**Review Cadence:**
- Operational & Budget Risks ($\text{Score} \ge 12$): Reviewed bi-weekly by the Technical Architect and Operations Lead.
- Comprehensive Register Review: Conducted at every formal Phase Gate transition (Gate 0 through Gate 5).
- Quarterly Audit: Executive summary presented to advisory stakeholders and sponsors.

---

## 2. Scoring & Severity Taxonomy

Risks are quantified on a 5-point discrete matrix measuring Likelihood ($L$) and Impact ($I$):

| Rating | Likelihood ($L$) | Quantitative Frequency | Impact ($I$) | Systemic Consequence |
|---|---|---|---|---|
| **5** | Almost Certain | Occurs multiple times per month | Existential | Permanent loss of platform trust, bankruptcy, or irreversible legal shutdown |
| **4** | Likely | Expected to occur within a phase | Severe | System downtime $>48\text{h}$, major security breach, or loss $>50\%$ of monthly runway |
| **3** | Possible | Plausible under specific conditions | Major | Partial service degradation, temporary quota blockage, or customer-facing errors |
| **2** | Unlikely | Rare edge condition | Moderate | Minor operational delay, non-blocking bug, or contained cost spike ($<\$20$) |
| **1** | Rare | Extremely improbable | Minor | Negligible annoyance with immediate workarounds |

$$\text{Composite Risk Score} = L \times I \quad (\text{Range: } 1 \text{ to } 25)$$

- **Critical Tier ($\text{Score} \ge 15$):** Daily monitoring; mandatory automated circuit breakers; immediate executive visibility.
- **High Tier ($12 \le \text{Score} \le 14$):** Bi-weekly tracking; automated threshold alerting; written contingency runbooks.
- **Moderate Tier ($6 \le \text{Score} \le 10$):** Monthly review; standard preventative controls.
- **Low Tier ($\text{Score} \le 5$):** Accepted operational noise; logged and monitored passively.

---

## 3. Top Strategic Risks (R-01 through R-20)

| ID | Strategic Risk Description | L | I | Score | Owner | Mitigation Protocol | Trigger Metric |
|---|---|---|---|---|---|---|---|
| **R-01** | Assessment inflation: gate exams become trivial, devaluing credentials | 3 | 5 | **15** | Head of Assessment | 2PL IRT calibration, answer pool rotation, statistical cheat detection | First-attempt pass rate $>80\%$ for 14 consecutive days |
| **R-02** | Fraud rings collude to farm Educational Currency ($\text{EC}$) at scale | 3 | 5 | **15** | Head of Trust & Safety | Device fingerprinting, IP clustering, velocity limits, 14-day redemption holds | Unexplained spike in identical answer patterns $>1\%$ of cohort |
| **R-03** | Sponsor pool commitments dry up, rendering EC awards unfunded | 3 | 5 | **15** | CFO / Finance Lead | Mandatory $1.0\times$ reserve ratio; automated switch to "XP-Only" non-cash mode | Liquid reserve coverage ratio drops below $1.05$ |
| **R-04** | Authoring costs for 500 skills exceed available capital | 4 | 4 | **16** | Content Architect | Modular 12-field templates, AI-assisted drafting, community SME reviews | Average cost per authored level exceeds $\$15.00$ |
| **R-05** | AI Tutor outputs harmful, offensive, or hallucinated gate exam leaks | 3 | 5 | **15** | Technical Architect | RAG grounding against verified curriculum, regex safety wrappers, kill switch | Evaluated groundedness drops below $95\%$; any verified leak |
| **R-06** | Adverse regulatory enforcement (child safety, sweepstakes/securities) | 3 | 4 | **12** | General Counsel | Closed-loop promotional credits; no fiat cash-out in Alpha; parental consent | Any regulatory inquiry letter or jurisdiction statutory change |
| **R-07** | Learner burnout from streak mechanics and progress pressure | 3 | 4 | **12** | Product Lead | Momentum factor $M \in [1.0, 1.25]$ instead of punitive zero-out streaks | Self-reported learner burnout $>15\%$ in weekly surveys |
| **R-08** | Accessibility defects invite ADA/WCAG compliance lawsuits | 2 | 5 | **10** | Head of Design | Automated Pa11y CI checks, keyboard-first navigation, WCAG 2.2 AA audit | Any blocking accessibility defect reported on production |
| **R-09** | Marketplace supply shortages cause redemption delivery failures | 3 | 4 | **12** | Marketplace Lead | Pre-allocated digital voucher inventory; escrow hold until receipt confirmation | Supplier fulfillment latency $>14$ days |
| **R-10** | Recommendation algorithms induce demographic bias or filter bubbles | 3 | 4 | **12** | Data Lead | Multi-objective scoring; strict exclusion of demographic traits from model | Adverse impact ratio drops below $0.80$ in recommendation audits |
| **R-11** | Data breach exposes learner personally identifiable information (PII) | 2 | 5 | **10** | CISO | Neon database encryption at rest, tokenized credentials, zero-PII logging | Any unauthorized database access alert or secret leak |
| **R-12** | Industry employers reject platform credentials as unverified | 3 | 4 | **12** | Partnerships Lead | W3C Verifiable Credentials with cryptographic item-level audit trails | Employer credential verification bounce rate $>50\%$ |
| **R-13** | AI inference token consumption exceeds monthly unit economics | 3 | 4 | **12** | System Architect | Redis semantic caching, context truncation, rate limiting to 25 turns/day | Average AI spend per monthly active learner exceeds $\$0.05$ |
| **R-14** | Critical vendor dependency failure (Cloudflare or OpenAI suspension) | 2 | 4 | **8** | System Architect | PWA architecture, multi-provider LLM gateway (Groq, Ollama fallbacks) | Vendor API availability drops below $99.0\%$ over 7 days |
| **R-15** | Insider tampering with assessment scores or token ledger entries | 2 | 5 | **10** | COO / Operations | Immutable ledger hashing, strict separation of duties, four-eyes approval | Cryptographic ledger hash verification mismatch |
| **R-16** | Child safety violation involving underage learner interaction | 2 | 5 | **10** | Head of Trust & Safety | COPPA age-gating; zero direct peer messaging for learners $<16$ years | Any safety report flagged as involving a minor |
| **R-17** | Single-person engineering bottleneck paralyzes delivery | 3 | 3 | **9** | CEO / Founder | Comprehensive markdown documentation, standard CI/CD, modular code | Single developer unavailable for $>7$ consecutive days |
| **R-18** | High-end spatial learning interfaces fail to justify ROI | 3 | 3 | **9** | Product Lead | Strict Phase 4 gating; 2D Web PWA parity guaranteed for all curricula | Spatial modality learning lift $<15\%$ in controlled pilot |
| **R-19** | Translation and localization defects distort assessment fairness | 3 | 3 | **9** | Content Architect | Professional native SME reviews; Differential Item Functioning (DIF) checks | Level dispute rate in localized language exceeds $2.5\%$ |
| **R-20** | Reputational public smear labeling model as "crypto grift" or "gambling" | 3 | 4 | **12** | CEO / Communications | Radical transparency, verified learning science papers, zero token speculation | Viral negative press cycle or coordinated social campaign |

---

## 4. Strategic Risk Detail (Deep Dives)

### R-01: Assessment Inflation & Credential Devaluation
- **Root Cause:** Inadequate question pool sizes, static item leakage to online forums, or grading drift from LLM evaluators.
- **Direct Impact:** Employers lose confidence in certified skill levels; credentials become worthless signaling mechanisms; platform value collapses.
- **Operational Controls:**
  - Item Response Theory (IRT) difficulty tracking: Items with pass rates $>85\%$ automatically relegated to non-credit practice.
  - Dynamic parameter randomization: Numerical variables and code test cases randomized per attempt.
  - Human psychometric spot-checks on $5\%$ of all passed level gates.
- **Early Warning Trigger:** Aggregate cohort first-attempt gate pass rate exceeding $75\%$ over any 14-day rolling window.

### R-02: Fraud Rings & Sybil Attacks on the Token Ledger
- **Root Cause:** Educational Currency ($\text{EC}$) holds real-world purchasing power; malicious actors deploy bot farms or multi-account scripts to harvest rewards.
- **Direct Impact:** Rapid depletion of sponsor liquidity reserves; platform insolvency; regulatory scrutiny for money laundering.
- **Operational Controls:**
  - Power-law progression friction: Earning velocity naturally throttled by mathematical difficulty:
    $$\text{EC}(L) = 10 \times L^{1.2} \times m_d \times Q \times E \times M \times R$$
  - Mandatory Level-Gate Integrity Tiers ($E$ multiplier capped at $1.0$ for open sessions; requires proctoring for high-value milestone levels).
  - 14-day escrow hold on first marketplace redemptions for newly certified accounts.
- **Early Warning Trigger:** More than 3 accounts completing level gates from identical subnet IP ranges within $<10$ minutes.

### R-03: Sponsor Liquidity Exhaustion
- **Root Cause:** Macroeconomic downturn or sponsor churn leading to unfulfilled funding commitments while learners accumulate redeemable $\text{EC}$.
- **Direct Impact:** Platform default; inability to honor marketplace orders; catastrophic loss of learner trust.
- **Operational Controls:**
  - Liquid Reserve Mandate: Platform maintains a strict $\ge 1.05\times$ cash-to-outstanding-EC reserve ratio.
  - Automated Fallback Protocol: If the reserve ratio breaches $1.0$, the platform automatically toggles into "Gamified XP Mode," awarding non-monetary badges and pausing EC issuance until pool replenishment.
- **Early Warning Trigger:** Committed corporate sponsor pipeline drops below $120\%$ of projected 90-day EC issuance.

---

## 5. Risk Treatment Framework

All identified risks are assigned one of four structured treatment protocols:

```mermaid
flowchart TD
    EVAL{"Evaluate Risk Score\n(L x I)"}
    
    EVAL -->|Score >= 15| AVOID["AVOID\nEliminate high-risk functionality\n(e.g., No real cash-out in Alpha)"]
    EVAL -->|10 <= Score < 15| REDUCE["REDUCE\nImplement automated technical controls\n(e.g., Caching, Rate limits, IRT)"]
    EVAL -->|6 <= Score < 10| TRANSFER["TRANSFER\nShift liability via contract/insurance\n(e.g., Cloudflare DDoS, Terms of Service)"]
    EVAL -->|Score < 6| ACCEPT["ACCEPT\nLog in register and monitor passively\n(e.g., Minor UI glitches)"]
```

---

## 6. Operational Review & Escalation Workflow

1. **Automated Continuous Polling:** Telemetry monitors (PostHog, Neon console, Uptime Kuma) evaluate risk metrics every 60 seconds.
2. **Severity Escalation:**
   - Any threshold breach automatically triggers a high-priority PagerDuty / Slack webhook to the designated Risk Owner.
   - The Risk Owner must acknowledge the incident within 30 minutes for Critical risks, or within 4 hours for High risks.
3. **Phase Gate Enforcement:**
   - No Phase Release Gate (e.g., Gate 0 or Gate 1) can be signed off if any Critical risk has an open, unmitigated action item.
   - Any High risk missing a validated contingency plan requires a formal exception waiver signed by the Technical Architect.

---

## 7. Governance Requirements

- `FR-PHS-801`: Every cataloged risk must maintain an assigned single-individual owner and documented mitigation runbook.
- `FR-PHS-802`: Composite risks with scores $\ge 12$ must be reviewed bi-weekly during operational sprints.
- `FR-PHS-803`: Newly identified technical or financial vulnerabilities must be logged in this register within 5 business days.
- `FR-PHS-804`: Phase gate transitions must perform a full review of all active risk scores against current telemetry.
- `FR-PHS-805`: Risk records may only be marked as "Closed" following verified implementation of architectural controls.
- `FR-PHS-806`: Automated alerts must notify the engineering team whenever any resource metric approaches $80\%$ of free-tier caps.
- `NFR-PHS-801`: All automated contingency failovers must restore essential learner services within $<15$ minutes of primary failure.

---

## 8. Budget & Infrastructure Risks (15 Bootstrap-Specific Risks)

The following 15 risks specifically address vulnerabilities arising from the **$1,000 upfront cash budget + $100/month ongoing operational runway** constraint, detailing technical controls and zero-cost escape hatches.

| Risk ID | Failure Scenario & Description | L | I | Score | Owner | Prevention & Mitigation Protocol | Emergency Contingency Runbook | Review Cadence |
|---|---|---|---|---|---|---|---|---|
| **RSK-BGT-01** | **Free tier limit hit unexpectedly:** A SaaS vendor secretly enforces hard usage ceilings, terminating service without grace period. | 3 | 4 | **12** | System Architect | Continuous automated telemetry meters tracking quota percentages; threshold alerts at $80\%$ of monthly free caps. | Instant migration to pre-tested open source containerized fallback (MinIO, Plunk, self-hosted Postgres). | Weekly |
| **RSK-BGT-02** | **OpenAI API price increase or token billing surge:** AI consumption exceeds the \$30/month Alpha budget ceiling. | 2 | 3 | **6** | System Architect | Strict sliding-window history (8 turns max); prompt compression; Redis semantic caching targeting $\ge 60\%$ hit rate. | Automatic circuit-breaker toggles AI requests to Groq Free Tier (Llama 3.3 70B) or local Ollama on VPS. | Weekly |
| **RSK-BGT-03** | **Hetzner VPS hardware failure:** The CAX21 ARM virtual machine encounters underlying hypervisor or NVMe drive failure. | 2 | 4 | **8** | DevOps Lead | Daily automated Hetzner Cloud VM snapshots (€0.0006/GB, ~$0.05/mo); state decoupled to Neon and Cloudflare R2. | Provision replacement CAX21 instance in alternative datacenter; restore from snapshot via CLI in $<20$ mins. | Monthly |
| **RSK-BGT-04** | **Neon Postgres connectivity interruption:** Serverless cold-start latency spikes or regional network outage disconnects app. | 2 | 5 | **10** | Database Lead | Connection pooling enabled via `pgBouncer`; connection retry logic with exponential jitter; health check probing. | Restore latest automated WAL-G backup into containerized PostgreSQL 16 on Hetzner VPS volume. | Bi-weekly |
| **RSK-BGT-05** | **Cloudflare R2 regional storage outage:** Temporary S3 API error halts artifact uploading or concept image rendering. | 2 | 3 | **6** | Full-Stack Lead | Aggressive Cloudflare CDN caching on static media assets; client-side exponential retry on image loads. | Failover write path to local VPS persistent Docker volume; queue asynchronous background sync to R2. | Monthly |
| **RSK-BGT-06** | **PostHog self-hosted memory OOM crash:** Attempting to self-host PostHog on low-RAM CAX21 causes server OOM crash. | 4 | 3 | **12** | System Architect | Avoid self-hosting PostHog on Alpha VPS. Utilize **PostHog Cloud Free Tier** (1,000,000 events/month free). | If PostHog Cloud free tier approaches 900k events, sample client telemetry events at $50\%$ rate. | Bi-weekly |
| **RSK-BGT-07** | **Single VPS becomes Single Point of Failure (SPOF):** Nginx crash or OS lockup brings down all platform web interfaces. | 2 | 5 | **10** | DevOps Lead | Docker containers configured with `restart: always`; systemd auto-restart watchdog; external Uptime Kuma monitoring. | Cloudflare custom maintenance page served automatically; reboot or re-deploy container via SSH in $<5$ mins. | Weekly |
| **RSK-BGT-08** | **Resend domain reputation drops:** Verification emails marked as spam; 3,000 free email limit exceeded. | 3 | 3 | **9** | Backend Lead | DKIM, SPF, and DMARC strictly configured; rate limit signup attempts; suppress test emails in non-production. | Switch transactional email transport to Amazon SES ($0.10/1\text{k}$ emails) or self-hosted Plunk via SMTP. | Bi-weekly |
| **RSK-BGT-09** | **OpenAI account suspended or API key compromised:** Secret leak or policy violation blocks LLM API key. | 1 | 5 | **5** | Technical Architect | Automated git-secrets scanning in CI/CD; OpenAI spending limits hard-capped at \$30; isolated service account. | Immediate environment variable flip to Groq API key (Llama 3.3 70B) with zero downtime. | Monthly |
| **RSK-BGT-10** | **GitHub repository compromised or deleted:** Loss of codebase or malicious commit injected into main branch. | 1 | 5 | **5** | Lead Engineer | Enforce 2FA on all contributor accounts; branch protection requiring signed commits; automated daily offsite clone. | Restore codebase from local developer clones or secondary GitLab private backup mirror. | Monthly |
| **RSK-BGT-11** | **Neon 0.5GB free storage exceeded during staging:** Fast integration test runs flood database with test records. | 4 | 3 | **12** | Database Lead | Automated teardown scripts in CI; test suites run against ephemeral local SQLite/Docker Postgres, not Neon. | Automated cron job truncates logs and test branches; upgrade to Neon Launch plan ($19/mo) if required. | Weekly |
| **RSK-BGT-12** | **BullMQ Redis OOM crash:** Unbounded dead-letter jobs accumulate in Redis on Hetzner VPS, consuming 1GB memory. | 3 | 4 | **12** | Backend Lead | Redis `maxmemory 1gb` with `volatile-lru` eviction; BullMQ jobs configured with `removeOnComplete: 100`. | Automated Redis memory alert at $80\%$; script purges failed queue jobs older than 48 hours. | Weekly |
| **RSK-BGT-13** | **Cloudflare R2 unexpected operation billing:** High-frequency Class A operations exceed free 1M ops/month limit. | 2 | 3 | **6** | Frontend Lead | Client-side artifact bundle aggregation; batch upload metadata; cache presigned URLs in Redis for 1 hour. | Throttle artifact uploads to 5 files per gate attempt; enforce 5MB maximum file size ceiling. | Monthly |
| **RSK-BGT-14** | **GitHub Actions 2,000 monthly free CI minutes exhausted:** Excessive CI runs on every commit consume free tier. | 3 | 2 | **6** | DevOps Lead | Turborepo remote caching enabled; path filters skip CI for documentation and asset changes; lint locally via Husky. | Shift CI test runs to Hetzner VPS via self-hosted GitHub Actions Runner container ($0 additional cost). | Bi-weekly |
| **RSK-BGT-15** | **Meilisearch memory leak on low-memory CAX21 VPS:** Search indexing consumes $>1.5\text{GB}$ RAM, starving Next.js. | 3 | 4 | **12** | DevOps Lead | Docker compose enforces hard memory limit: `mem_limit: 512m`; `MEILI_MAX_INDEXING_MEMORY=256MB`. | Linux cgroup kills only Meilisearch; systemd restarts container; fallback search cascades to Postgres ILIKE. | Weekly |

---

## 9. Automated Budget Risk Mitigation Engine

To ensure that resource exhaustion never causes unbudgeted financial charges or unrecoverable service crashes, the platform deploys an automated supervisor process.

```mermaid
flowchart TD
    POLL["Resource Quota Monitor\n(Cron Job Every 15 Mins)"] --> CHK_AI{"AI Spend\n> $1.00 / day?"}
    
    CHK_AI -->|YES| FLIP_GROQ["Trigger AI Circuit Breaker:\nSwitch Next.js LLM Provider to Groq Llama 3.3"]
    CHK_AI -->|NO| CHK_RAM{"VPS RAM\n> 85% Utilization?"}
    
    CHK_RAM -->|YES| PURGE_REDIS["Evict Volatile Redis Keys\n& Restart Background Workers"]
    CHK_RAM -->|NO| CHK_DB{"Neon DB Storage\n> 400MB (80%)?"}
    
    CHK_DB -->|YES| PURGE_TEST["Drop Staging DB Branches\n& Prune Audit Logs to R2 Archive"]
    CHK_DB -->|NO| OK["Status Green: System Operating Within Budget"]
```

### 9.1 Automated Quota Safeguard Controller (TypeScript Specification)
```typescript
/**
 * Automated Infrastructure & Budget Quota Safeguard Controller
 * Runs periodically to evaluate free-tier consumption and activate failovers.
 */

export interface SystemResourceMetrics {
  hetznerRamUsedMb: number;
  hetznerRamTotalMb: number;
  neonStorageUsedMb: number;
  monthlyAiSpendUsd: number;
  resendEmailsSentThisMonth: number;
}

export interface SafeguardActionResult {
  timestamp: string;
  actionsTriggered: string[];
  systemStatus: 'NOMINAL' | 'DEGRADED_FAILOVER' | 'CRITICAL_INTERVENTION';
}

export class QuotaSafeguardController {
  private readonly AI_BUDGET_CEILING_USD = 28.00;
  private readonly NEON_STORAGE_WARN_MB = 400.0; // 80% of 500MB free tier
  private readonly RESEND_EMAIL_LIMIT = 2800;   // 93% of 3,000 free tier
  private readonly RAM_USAGE_RATIO_WARN = 0.85;

  public evaluateThresholds(metrics: SystemResourceMetrics): SafeguardActionResult {
    const actions: string[] = [];
    let status: SafeguardActionResult['systemStatus'] = 'NOMINAL';

    // 1. Evaluate AI Monthly Spend
    if (metrics.monthlyAiSpendUsd >= this.AI_BUDGET_CEILING_USD) {
      actions.push('ACTIVATE_GROQ_AI_FALLBACK: OpenAI monthly budget ceiling reached.');
      status = 'DEGRADED_FAILOVER';
    }

    // 2. Evaluate Neon Postgres Free Storage (500MB Cap)
    if (metrics.neonStorageUsedMb >= this.NEON_STORAGE_WARN_MB) {
      actions.push('TRIGGER_DB_PRUNE: Neon storage >80%. Dropping ephemeral staging branches & vacuuming.');
      status = 'DEGRADED_FAILOVER';
    }

    // 3. Evaluate Resend Monthly Free Email Quota (3,000 Cap)
    if (metrics.resendEmailsSentThisMonth >= this.RESEND_EMAIL_LIMIT) {
      actions.push('SWITCH_EMAIL_SES: Resend 3k quota exhausted. Cascading outbound email to Amazon SES.');
      status = 'DEGRADED_FAILOVER';
    }

    // 4. Evaluate Hetzner CAX21 RAM Pressure (8GB Total)
    const ramRatio = metrics.hetznerRamUsedMb / metrics.hetznerRamTotalMb;
    if (ramRatio >= this.RAM_USAGE_RATIO_WARN) {
      actions.push('RESTART_BULLMQ_WORKER: Host RAM >85%. Purging completed Redis queues & reclaiming page cache.');
      status = 'CRITICAL_INTERVENTION';
    }

    return {
      timestamp: new Date().toISOString(),
      actionsTriggered: actions,
      systemStatus: status,
    };
  }
}
```

---

## 10. Edge Cases & Incident Recovery Runbooks

### 10.1 Runbook: Rapid Database Failover from Neon to VPS Self-Hosted Postgres
- **Scenario:** Neon serverless experiences a catastrophic outage or sudden unannounced pricing changes.
- **Estimated Recovery Time:** $<30$ minutes.
- **Execution Steps:**
  1. SSH into Hetzner VPS: `ssh deploy@server.educonomy.org`.
  2. Launch local containerized Postgres instance:
     ```bash
     docker compose up -d postgres-fallback
     ```
  3. Pull latest nightly backup dump from Cloudflare R2:
     ```bash
     aws s3 cp s3://educonomy-backups/latest-db.sql.gz ./ --endpoint-url https://<account_id>.r2.cloudflarestorage.com
     gunzip latest-db.sql.gz && docker exec -i educonomy-postgres psql -U postgres -d educonomy < latest-db.sql
     ```
  4. Update `.env.production` in Next.js app directory:
     ```bash
     DATABASE_URL="postgresql://postgres:secure_password@postgres-fallback:5432/educonomy"
     ```
  5. Restart Next.js container: `docker compose restart next-app`.
  6. Verify API health check: `curl -I https://api.educonomy.org/v1/health`.

### 10.2 Runbook: OpenAI Budget Lockout & Groq Free-Tier Activation
- **Scenario:** AI token bills hit $\$30.00$ or OpenAI experiences global 500 errors.
- **Estimated Recovery Time:** $<60$ seconds (Automated zero-downtime toggle).
- **Execution Steps:**
  1. The `QuotaSafeguardController` detects the quota breach or HTTP 429 response from OpenAI.
  2. The application's `AiGatewayOptimizer` dynamically sets `USE_GROQ_FALLBACK=true` in Redis.
  3. All subsequent learner Socratic queries route to Meta Llama 3.3 70B hosted on Groq Cloud Free Tier via its OpenAI-compatible SDK endpoint.
  4. System emits a low-priority notification to the Technical Architect.

---

## 11. Success Metrics & Audit Protocols

| Risk Dimension | Metric Target | Monitoring Tool | Verification Frequency |
|---|---|---|---|
| **Capital Preservation** | Total Spend $\le \$47.00$ (Phase 0), $\le \$350.00$ (Phase 1) | Bank Ledger & Invoices | Weekly sprint retro |
| **Operational Burn Rate** | Infrastructure Run-Rate $\le \$60.00/\text{month}$ | Hetzner & Neon Invoices | 1st of every calendar month |
| **Free-Tier Safety Margin** | All resources remain $<85\%$ of vendor free limits | PostHog & Cloudflare Metrics | Daily automated cron check |
| **Platform Availability** | System Uptime $\ge 99.5\%$ | External Uptime Kuma Probes | Continuous (1-minute intervals) |
| **Credential Integrity** | Pass Rate calibrated to $55\%–75\%$ | SQL Audit Queries on Neon | Weekly psychometric calibration |
| **Mean Time to Recover (MTTR)**| Critical failure recovery $<30$ minutes | Incident Post-Mortem Logs | Post-incident review |

---

## 12. Related Documentation

- [Delivery Roadmap & Phase Sequencing](../13-phases/01-roadmap.md)
- [Phase 0 — Foundations Specification](../13-phases/02-phase-0.md)
- [Phase 1 — Alpha Engineering Master Plan](../13-phases/03-phase-1.md)
- [Release Gates & Verification Protocols](../13-phases/08-release-gates.md)
- [API & Event Architecture Specification](04-api-events.md)
- [Incident Response & Operational Runbooks](../11-operations/04-incident-response.md)
