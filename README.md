# NovaWorth

**A learning-to-earn economy where verified human capability is the scarce asset.**

NovaWorth is a multi-platform learning product for a world where AI systems produce most goods and AI robotic flows deliver most services. When material abundance stops being the bottleneck, the thing society still needs is capable, adaptive, trustworthy humans. NovaWorth turns that idea into a product loop:

> **Learn → Prove → Earn → Redeem → Learn again.**

Learners pick real skills, climb 100 authored levels per skill, prove mastery through adaptive assessments and practical work, and earn points that can be redeemed for goods, services, tools, and experiences. Higher demonstrated ability and effort earn more. Higher levels are harder, rarer, and worth more. After level 100, learners enter prestige, frontier simulations, cross-skill missions, and mentoring.

This repository is the requirements baseline for design, engineering, content, operations, legal, and investors. It is intentionally implementation-aware but not a codebase.

---

## 1. Product in one page

| Element | Definition |
|---|---|
| **Core promise** | "Become measurably better at something real, and the economy notices." |
| **Learner loop** | Onboarding → skill match → learn → practice → gate assessment → earn XP + Education Credits (EC) → redeem → next level |
| **Supply** | 500 human skills × 100 levels, authored to a common standard; 10 bands of 10 levels |
| **Demand** | Sponsored marketplace of goods, services, tools, experiences, and AI services |
| **Scoring** | Power-law difficulty and rewards; performance, effort, and mastery multipliers; anti-grind decay |
| **Proof** | Adaptive knowledge checks, practical artifacts, AI role-play, simulators, expert panels at the top |
| **Platforms** | Web (PWA), iOS/Android, Apple Vision Pro (spatial), optional metaverse/WebXR |
| **North star** | Verified mastery gains per weekly active learner, with healthy redemption and low fraud |

**What makes it different**

1. **Capability-backed points, not engagement points.** Points require verified evidence.
2. **A real 100-level ladder.** Level 100 means world-class, not "finished a course."
3. **A recommendation engine that finds your unfair advantage.** Work history, life experience, interests, and aptitude produce 3–4 high-yield paths — while every learner can still choose any skill.
4. **A post-100 frontier.** Prestige, combined-skill "constellations," live incident simulations, and real bounties.
5. **An economy with guardrails.** Sponsored emission, double-entry ledger, sinks, caps, fraud controls, and transparent rules.

---

## 2. Documentation map

Read in this order for the full picture.

### Foundations
| Doc | What it answers |
|---|---|
| [Vision](docs/00-foundations/01-vision.md) | Why this product exists and what world it creates |
| [Principles & personas](docs/00-foundations/02-principles-personas.md) | Non-negotiable rules and who we build for |
| [Glossary, scope & assumptions](docs/00-foundations/03-glossary-scope-assumptions.md) | Shared language, in/out of scope, constraints |
| [Success metrics](docs/00-foundations/04-success-metrics.md) | North star, metric tree, phase targets, guardrails |

### Economy
| Doc | What it answers |
|---|---|
| [Economy model](docs/01-economy/01-economy-model.md) | Currencies, flows, sponsors, sinks, governance |
| [Points & ledger](docs/01-economy/02-points-ledger.md) | XP vs EC, double-entry ledger, award lifecycle |
| [Scoring model](docs/01-economy/03-scoring-model.md) | Exact formulas for difficulty, XP, EC, multipliers |
| [Level curve](docs/01-economy/04-level-curve.md) | Why 100 levels feel the way they do; hours, pass rates, rewards |
| [Post-100 prestige](docs/01-economy/05-post-100-prestige.md) | Diminishing returns, frontier challenges, constellations |
| [Marketplace economics](docs/01-economy/06-marketplace-economics.md) | Pricing, subsidies, emission control, inflation |
| [Integrity](docs/01-economy/07-integrity.md) | Anti-cheat, clawbacks, collusion, audit |
| [Simulation model](docs/01-economy/08-economy-simulation.md) | Worked cohort numbers and tuning levers |

### Skills & curriculum
| Doc | What it answers |
|---|---|
| [Taxonomy](docs/02-skills/01-taxonomy.md) | 20 domains × 25 skills; why these skills |
| [Catalog (500)](docs/02-skills/02-catalog-500.md) | The full scope baseline |
| [Level authoring standard](docs/02-skills/03-level-authoring-standard.md) | How every one of 50,000 level specs is written |
| [Assessment authoring standard](docs/02-skills/04-assessment-authoring-standard.md) | Item types, rubrics, evidence, reliability |
| [Content pipeline](docs/02-skills/05-content-pipeline.md) | Research → author → AI draft → review → publish |
| [CS exemplar](docs/02-skills/06-exemplar-cs.md) | All 100 CS levels, end to end |
| [Cooking exemplar](docs/02-skills/07-exemplar-cooking.md) | 10 bands and gate design |
| [Health exemplar](docs/02-skills/08-exemplar-health.md) | Safety-critical skill design |
| [Communication exemplar](docs/02-skills/09-exemplar-communication.md) | Soft-skill assessment design |
| [Reasoning exemplar](docs/02-skills/10-exemplar-reasoning.md) | Transfer and abstraction design |

### Learner flows
| Doc | What it answers |
|---|---|
| [Onboarding flow](docs/03-onboarding/01-onboarding-flow.md) | Step-by-step first session |
| [Experience graph](docs/03-onboarding/02-experience-graph.md) | How work and life experience become signal |
| [Profiling](docs/03-onboarding/03-profiling.md) | Aptitude, effort, interests, constraints |
| [Recommendation engine](docs/03-onboarding/04-recommendation-engine.md) | Fit formula, explainability, diversity, override |
| [First 14 days](docs/03-onboarding/05-first-14-days.md) | Activation and early habit design |

### Learning & assessment
| Doc | What it answers |
|---|---|
| [Learning engine](docs/04-learning/01-learning-engine.md) | The orchestration model across formats |
| [AI tutor](docs/04-learning/02-ai-tutor.md) | Socratic tutoring, memory, safety, cost |
| [Text & video](docs/04-learning/03-text-video.md) | Reading, interactive video, captions, provenance |
| [Quizzes](docs/04-learning/04-quizzes.md) | Item types, adaptive practice, feedback |
| [Role-play](docs/04-learning/05-roleplay.md) | Scenario engine, NPCs, rubric scoring |
| [Simulators](docs/04-learning/06-simulators.md) | Deterministic sims, live-fire incidents, after-action review |
| [Adaptive path & spacing](docs/04-learning/07-adaptive-spaced.md) | Mastery estimation, prerequisites, review queue |
| [Level gate](docs/04-learning/08-level-gate.md) | The exact promotion flow |
| [Assessment system](docs/05-assessment/01-assessment-system.md) | Assessment architecture and evidence model |
| [Diagnostic](docs/05-assessment/02-diagnostic.md) | Cold-start and placement testing |
| [Gate exams](docs/05-assessment/03-gate-exams.md) | Summative design and retake policy |
| [Practical & portfolio](docs/05-assessment/04-practical-portfolio.md) | Artifacts, peer review, expert panels |
| [Integrity, rubrics & appeals](docs/05-assessment/05-integrity-rubrics-appeals.md) | Proctoring, grading quality, disputes |

### Platforms & experience
| Doc | What it answers |
|---|---|
| [Architecture](docs/06-platforms/01-architecture.md) | System shape, services, data stores, AI gateway |
| [Web](docs/06-platforms/02-web.md) | PWA, performance, browser support |
| [Mobile](docs/06-platforms/03-mobile.md) | Offline, camera evidence, notifications |
| [Vision Pro](docs/06-platforms/04-visionos.md) | Spatial learning, immersive sims, accessibility |
| [Metaverse](docs/06-platforms/05-metaverse.md) | Optional WebXR/Unity strategy |
| [Accessibility & offline](docs/06-platforms/06-accessibility-offline.md) | WCAG 2.2 AA, low-bandwidth, assistive tech |
| [Information architecture](docs/07-experience/01-information-architecture.md) | Navigation and screen map |
| [Core UX flows](docs/07-experience/02-core-ux-flows.md) | Key journeys and states |
| [Social & guilds](docs/07-experience/03-social-guilds.md) | Leagues, teams, mentorship, moderation |
| [Profile & reputation](docs/07-experience/04-profile-reputation.md) | Skill wallet, trust, public proof |
| [Notifications & rituals](docs/07-experience/05-notifications-rituals.md) | Daily loops without dark patterns |

### Marketplace, trust, data, operations, legal
| Doc | What it answers |
|---|---|
| [Marketplace overview](docs/08-marketplace/01-marketplace-overview.md) | Catalog, redemption, escrow, restrictions |
| [Catalog & pricing](docs/08-marketplace/02-catalog-pricing.md) | Price bands, essentials, dynamic pricing |
| [Partners & supply](docs/08-marketplace/03-partners-supply.md) | Sponsor and merchant onboarding |
| [Fulfillment & disputes](docs/08-marketplace/04-fulfillment-disputes.md) | Orders, delivery, refunds, arbitration |
| [Integrity & anti-cheat](docs/09-trust-safety/01-integrity-anti-cheat.md) | Threat model and controls |
| [AI safety](docs/09-trust-safety/02-ai-safety.md) | Tutor and role-play guardrails |
| [Privacy & data](docs/09-trust-safety/03-privacy-data.md) | Consent, retention, sensitive data |
| [Child safety](docs/09-trust-safety/04-child-safety.md) | Minors, guardians, age gates |
| [Fraud & abuse](docs/09-trust-safety/05-fraud-abuse.md) | Collusion, account takeover, marketplace fraud |
| [Data model](docs/10-data-ai/01-data-model.md) | Core entities and relationships |
| [AI architecture](docs/10-data-ai/02-ai-architecture.md) | Routing, RAG, evals, guardrails, cost |
| [Content generation](docs/10-data-ai/03-content-generation.md) | Human-AI authoring factory |
| [Evaluation](docs/10-data-ai/04-evaluation.md) | Model, content, and assessment quality |
| [Analytics](docs/10-data-ai/05-analytics.md) | Events, warehouse, dashboards, experiments |
| [Content ops](docs/11-operations/01-content-ops.md) | Review queues, localization, refresh |
| [Support & moderation](docs/11-operations/02-support-moderation.md) | Tiers, SLAs, safety escalation |
| [Reliability](docs/11-operations/03-reliability.md) | SLOs, capacity, cost controls |
| [Incident response](docs/11-operations/04-incident-response.md) | Sev levels, on-call, comms |
| [Regulatory](docs/12-legal/01-regulatory.md) | Education, employment, consumer, data law |
| [Credentials & employment](docs/12-legal/02-credentials-employment.md) | Verifiable credentials, hiring claims |
| [IP & content](docs/12-legal/03-ip-content.md) | Licensing, AI provenance, copyright |

### Delivery
| Doc | What it answers |
|---|---|
| [Roadmap](docs/13-phases/01-roadmap.md) | Phase overview, cost-aware milestones, infrastructure evolution |
| [Phase 0 — Foundations](docs/13-phases/02-phase-0.md) | Research, standards, prototypes; exact $47 cost breakdown |
| [Phase 1 — Alpha](docs/13-phases/03-phase-1.md) | Web + onboarding + 25 skills; ~$58/mo cost breakdown |
| [Phase 2 — Beta](docs/13-phases/04-phase-2.md) | 100 skills, mobile, assessments |
| [Phase 3 — Economy](docs/13-phases/05-phase-3.md) | Real marketplace, post-100, sims |
| [Phase 4 — Scale](docs/13-phases/06-phase-4.md) | 500 skills, spatial, guilds |
| [Phase 5 — Frontier](docs/13-phases/07-phase-5.md) | Autonomous content, livelihood pilots |
| [Release gates](docs/13-phases/08-release-gates.md) | Go/no-go criteria per phase |
| [Requirements index](docs/14-appendices/01-requirements-index.md) | All FR/NFR IDs |
| [Risk register](docs/14-appendices/02-risk-register.md) | Risks, owners, mitigations, budget/infra risks |
| [Decision log](docs/14-appendices/03-decision-log.md) | Key product decisions |
| [API & events](docs/14-appendices/04-api-events.md) | OpenAPI fragments, event schemas, API conventions |
| [Open questions](docs/14-appendices/05-open-questions.md) | What still needs a decision |

### Technology Stack & Infrastructure
| Doc | What it answers |
|---|---|
| [Budget & stack](docs/00-foundations/05-budget-stack.md) | $1,000 upfront + $100/mo tool selection, cost model, free-tier limits |
| [Architecture](docs/06-platforms/01-architecture.md) | C4 diagrams, Hetzner VPS topology, service responsibilities, sequence diagrams |
| [Local dev guide](docs/06-platforms/07-local-dev-guide.md) | Turborepo monorepo setup, env vars, database seeding, dev workflows |
| [Deployment guide](docs/06-platforms/08-deployment-guide.md) | Hetzner VPS, Docker Compose prod, CI/CD, backups, rollback procedure |
| [Rust implementation](docs/16-rust-implementation.md) | **Authoritative map of the Rust codebase** (crates, services, migrations, infra, edge cases) — supersedes the TS-stack sections |
| [Setup & deploy runbook](SETUP.md) | Step-by-step install, account creation, run locally, deploy to Hetzner |
| [Data flows & state machines](docs/16-data-flows.md) | Mermaid mind-map walkthroughs of every flow (auth, gate, ledger, marketplace, AI) + state diagrams |
| [Entities, ER & state diagrams](docs/16-entities.md) | Entity-relationship diagram + per-entity schema and state machines for every migration table |

### Low-Level Design (LLD) — `docs/15-lld/`
Detailed implementation designs for every major subsystem. See [LLD index](docs/15-lld/00-lld-index.md) for reading order by role.

| Doc | What it covers |
|---|---|
| [LLD Index](docs/15-lld/00-lld-index.md) | Reading order by role, cross-references, contribution guide |
| [Identity & Auth](docs/15-lld/01-identity-auth.md) | Lucia Auth v3, 7 auth flows, MFA, guardian consent, RBAC/ABAC matrix |
| [Economy Ledger](docs/15-lld/02-ledger.md) | Drizzle DDL (7 tables), SHA-256 hash-chain, reconciliation, double-spend |
| [Learning Engine](docs/15-lld/03-learning-engine.md) | BKT mastery model, SM-2 spacing, session state machine, adaptive difficulty |
| [Gate Flow](docs/15-lld/04-gate-flow.md) | Saga pattern (10-step), integrity scoring, resume/disconnect, panel scheduling |
| [AI Gateway](docs/15-lld/05-ai-gateway.md) | RAG pipeline, model routing, guardrails, cost tracking (<$1.50/learner/mo) |
| [Onboarding State](docs/15-lld/06-onboarding-state.md) | 13-state FSM, HMAC resume tokens, Rasch IRT aptitude sampler, accessibility |
| [Recommendation](docs/15-lld/07-recommendation.md) | 7-factor fit algorithm, anti-bias hardcoded rules, cold-start, update loop |
| [Marketplace & Escrow](docs/15-lld/08-marketplace-escrow.md) | Escrow saga, regional EC anchors, dispute resolution, merchant onboarding |
| [Notifications](docs/15-lld/09-notifications.md) | Delivery pipeline, rate limits, Resend + Web Push, GDPR opt-out |
| [Content CMS](docs/15-lld/10-content-cms.md) | 17-state workflow, Drizzle schema, BullMQ AI drafting jobs, versioning |
| [Social & Guilds](docs/15-lld/11-social-guilds.md) | Redis sorted-set leagues, mentor flow, moderation, reputation scoring |
| [Analytics Pipeline](docs/15-lld/12-analytics-pipeline.md) | 30+ event catalog, PostHog Cloud, Neon warehouse, 8 dashboard specs |
| [Edge Cases Master](docs/15-lld/13-edge-cases-master.md) | 190+ edge cases across 10 domains with detection, handling, recovery |


---

## 3. Canonical constants (single source of truth)

These numbers are used across every doc. Change them in one place and update the docs together.

| Constant | Value | Notes |
|---|---|---|
| Skills | 500 | 20 domains × 25 |
| Levels per skill | 100 | 10 bands × 10 levels |
| XP for level `L` | `100 × L^1.5 × m_d` | Mastery score, non-spendable |
| EC for level `L` | `10 × L^1.2 × m_d × Q × E × M × R` | Spendable, sponsored |
| Expected hours `H(L)` | `1.5 × L^1.1` | Cumulative ≈ 11,300h to L100 |
| Difficulty `D(L)` | `10 × L^1.35` | Used for matchmaking and pay |
| Domain multiplier `m_d` | 0.9–1.3 | Scarcity/risk weighting |
| Knowledge pass | 80% | Distinction ≥95% |
| Practical pass | 2.5/4 average, no criterion <2 | Distinction ≥3.5/4 |
| EC anchor | 1 EC ≈ $0.05 subsidy value (US/EU baseline); regional anchors $0.02–$0.05 | Regional index; not cash |
| Repeat decay `R` | 1.0 → 0.2 → 0.05 → 0 | EC only |
| Post-100 EC | `EC(100) × 0.5 / n^0.7` | Prestige level `n` |
| Season | 12 weeks | Leaderboards reset; mastery does not |
| Daily counted effort | 6h soft cap | Wellbeing guardrail |

---

## 4. Requirement conventions

- `FR-<AREA>-NNN` = functional requirement.
- `NFR-<AREA>-NNN` = non-functional requirement.
- `BR-<AREA>-NNN` = business rule.
- **P0** = required for the phase to ship; **P1** = next; **P2** = later.
- Every phase doc has **exit criteria**; a phase cannot start until the previous gate passes.
- Every flow doc includes actors, states, edge cases, acceptance criteria, and telemetry.
- "Must" means contractually required; "should" means strongly expected; "may" means optional.

---

## 5. Status

| Item | State |
|---|---|
| Concept | Defined |
| Requirements | Baseline v1.0 (837 tracked IDs) |
| Documentation | 100+ docs + README + Rust implementation reference; links verified |
| 500-skill catalog | Defined (scope baseline) |
| Level specs | 5 exemplars detailed; 495 in authoring backlog |
| Rust implementation | **Complete** — 11 crates, 3 services, 7 migrations, Leptos UI, Docker/Caddy deploy |
| Seed data | Not seeded yet (500-skill CSV available) |
| Next step | Seed skill/level data; wire onboarding FSM + analytics pipeline; harden CI/CD |

**Owner:** Product & Learning Science  
**Version:** 1.0  
**Last updated:** 2026-11-21
