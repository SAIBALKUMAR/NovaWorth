# Requirements Index

## 1. Purpose

Every functional (`FR`), non-functional (`NFR`), and business rule (`BR`) requirement in the baseline, grouped by area. IDs are stable; changing one requires a change request and a doc version bump.

**Baseline totals:** 676 FR · 143 NFR · 18 BR · **837 requirements**.

---

## 2. Conventions

- `FR-<AREA>-NNN` — functional requirement (behavior the system must have).
- `NFR-<AREA>-NNN` — non-functional requirement (quality, performance, reliability).
- `BR-<AREA>-NNN` — business rule (policy that constrains the product or economy).
- Priority: **P0** = required for the phase to ship; **P1** = next; **P2** = later. Default is P0 unless a phase doc says otherwise.

---

## 3. Index by area

| Area | Meaning | FR | NFR | BR | Source |
|---|---|---|---|---|---|
| `ONB` | Onboarding, profiling, recommendation | 001–010, 101–110, 201–210, 301–310, 401–410 (50) | 001–003, 301–302 (5) | 001–003 | [Onboarding](../03-onboarding/01-onboarding-flow.md) |
| `LRN` | Learning engine, tutor, media, quizzes, role-play, sims, spacing, gate | 001–010, 101–110, 201–210, 301–310, 401–410, 501–510, 601–610 (70) | 001–003, 101–103, 201–203, 301–302, 401–402, 501–503, 601–602 (18) | 001–003 | [Learning engine](../04-learning/01-learning-engine.md) |
| `ASM` | Assessment, diagnostics, gates, portfolios, integrity | 001–010, 101–110, 201–210, 301–310, 401–410, 501–510, 601–610 (70) | 001–003, 101–103, 201–203, 301–302, 401–403, 501–502, 601–602 (18) | 001–003 | [Assessment system](../05-assessment/01-assessment-system.md) |
| `ECN` | Economy, ledger, scoring, post-100, marketplace economics, integrity | 001, 060, 101–108, 201–207, 301–306 (23) | 001–003, 201–202 (5) | 001–004 | [Economy model](../01-economy/01-economy-model.md) |
| `SKL` | Taxonomy, catalog, level standard, assessment standard, pipeline | 001–008, 101–107, 201–208 (23) | 001–002, 101, 201–202 (5) | 001–002 | [Taxonomy](../02-skills/01-taxonomy.md) |
| `PRS` | Principles and personas | 001–010 (10) | — | 001–002 | [Principles](../00-foundations/02-principles-personas.md) |
| `PLT` | Architecture, web, mobile, visionOS, metaverse, accessibility | 001–010, 101–110, 201–210, 301–310, 401–410, 501–510 (60) | 001–003, 101–103, 201–203, 301–303, 401–403, 501–503 (18) | 001–002 | [Architecture](../06-platforms/01-architecture.md) |
| `UX` | IA, core flows | 001–010, 101–110 (20) | 001–002, 101–102 (4) | 001–002 | [Information architecture](../07-experience/01-information-architecture.md) |
| `SOC` | Guilds, leagues, mentorship, profile, reputation, notifications | 001–010, 101–110, 201–210 (30) | 001–002, 101–102, 201–202 (6) | 001–002 | [Social & guilds](../07-experience/03-social-guilds.md) |
| `MKT` | Marketplace, catalog, partners, fulfillment | 001–010, 101–110, 201–210, 301–310, 401–410 (50) | 101–103, 201–202, 301–302, 401–402 (9) | 001–004 | [Marketplace overview](../08-marketplace/01-marketplace-overview.md) |
| `TNS` | Integrity, AI safety, privacy, child safety, fraud | 001–010, 101–110, 201–210, 301–310, 401–410, 501–510 (60) | 001–002, 101–102, 201–202, 301–302, 401–402, 501–502 (12) | 001–003 | [Integrity & anti-cheat](../09-trust-safety/01-integrity-anti-cheat.md) |
| `DAT` | Data model | 001–010 (10) | 001–002 (2) | 001 | [Data model](../10-data-ai/01-data-model.md) |
| `AI` | AI architecture | 001–010 (10) | 001–003 (3) | 001–002 | [AI architecture](../10-data-ai/02-ai-architecture.md) |
| `CNT` | Content generation | 001–010, 101–110 (20) | 101–102 (2) | 001–002 | [Content generation](../10-data-ai/03-content-generation.md) |
| `EVL` | Evaluation | 001–010 (10) | 001–002 (2) | 001 | [Evaluation](../10-data-ai/04-evaluation.md) |
| `ANL` | Analytics | 001–010 (10) | 001–002 (2) | 001 | [Analytics](../10-data-ai/05-analytics.md) |
| `OPS` | Content ops, support, reliability, incidents | 101–110, 201–210, 301–310, 401–410 (40) | 101–102, 201–202, 301–303, 401–402 (9) | 001–002 | [Content ops](../11-operations/01-content-ops.md) |
| `LEG` | Regulatory, credentials, IP | 001–010, 101–110, 201–210 (30) | 001–002, 101–102, 201–202 (6) | 001–003 | [Regulatory](../12-legal/01-regulatory.md) |
| `PHS` | Roadmap, phases, release gates | 001–010, 101–110, 201–210, 301–310, 401–410, 501–510, 601–610, 701–710 (80) | 101–102, 201–202, 301–303, 401–403, 501–503, 601–602, 701–702 (17) | 001–003 | [Roadmap](../13-phases/01-roadmap.md) |

---

## 4. Business rules

| ID | Rule | Source |
|---|---|---|
| `BR-ECN-001` | EC has no cash value and cannot be withdrawn in v1. | [Economy model](../01-economy/01-economy-model.md) |
| `BR-ECN-002` | Every EC award must be funded by a sponsor pool before it is posted. | [Points & ledger](../01-economy/02-points-ledger.md) |
| `BR-ECN-003` | XP is non-spendable and never decays. | [Scoring model](../01-economy/03-scoring-model.md) |
| `BR-ECN-004` | Repeat passes decay: 1.0 → 0.2 → 0.05 → 0. | [Scoring model](../01-economy/03-scoring-model.md) |
| `BR-SKL-001` | A skill has exactly 100 levels; level 100 is world-class and panel-assessed. | [Level authoring standard](../02-skills/03-level-authoring-standard.md) |
| `BR-SKL-002` | No level may be gated by hardware; a text path exists for every level. | [Text & video](../04-learning/03-text-video.md) |
| `BR-PRS-001` | No learner is labeled by a global IQ score. | [Profiling](../03-onboarding/03-profiling.md) |
| `BR-PRS-002` | The learner may choose any skill regardless of recommendation. | [Recommendation engine](../03-onboarding/04-recommendation-engine.md) |
| `BR-PLT-001` | No scoring advantage for any platform, including Vision Pro. | [Vision Pro](../06-platforms/04-visionos.md) |
| `BR-PLT-002` | Offline mode never permits high-stakes gates. | [Web](../06-platforms/02-web.md) |
| `BR-UX-001` | Social features never gate learning or earning. | [Social & guilds](../07-experience/03-social-guilds.md) |
| `BR-UX-002` | No dark patterns: no fake urgency, no streak shaming, no forced continuity. | [Notifications & rituals](../07-experience/05-notifications-rituals.md) |
| `BR-SOC-001` | Reputation is not purchasable or transferable. | [Profile & reputation](../07-experience/04-profile-reputation.md) |
| `BR-SOC-002` | Minors are private by default and require guardian approval to spend. | [Child safety](../09-trust-safety/04-child-safety.md) |
| `BR-MKT-001` | Money cannot buy XP, levels, passes, or prestige. | [Marketplace overview](../08-marketplace/01-marketplace-overview.md) |
| `BR-MKT-002` | Essentials are prioritized and capped; no dynamic upward pricing on essentials. | [Catalog & pricing](../08-marketplace/02-catalog-pricing.md) |
| `BR-MKT-003` | No personalized pricing. | [Catalog & pricing](../08-marketplace/02-catalog-pricing.md) |
| `BR-MKT-004` | Escrow holds EC until fulfillment is confirmed. | [Fulfillment & disputes](../08-marketplace/04-fulfillment-disputes.md) |
| `BR-TNS-001` | No award is posted below the integrity threshold. | [Integrity & anti-cheat](../09-trust-safety/01-integrity-anti-cheat.md) |
| `BR-TNS-002` | No model training on learner data without explicit consent. | [Privacy & data](../09-trust-safety/03-privacy-data.md) |
| `BR-TNS-003` | Safety incidents are always SEV1. | [Incident response](../11-operations/04-incident-response.md) |
| `BR-DAT-001` | Ledger entries are immutable; corrections use compensating entries. | [Data model](../10-data-ai/01-data-model.md) |
| `BR-AI-001` | AI never grades high-stakes work alone. | [AI safety](../09-trust-safety/02-ai-safety.md) |
| `BR-AI-002` | AI-generated content is labeled and provenance-tracked. | [IP & content](../12-legal/03-ip-content.md) |
| `BR-CNT-001` | No AI-drafted level publishes without human review. | [Content generation](../10-data-ai/03-content-generation.md) |
| `BR-CNT-002` | Fabricated citations are a publish blocker. | [Content generation](../10-data-ai/03-content-generation.md) |
| `BR-EVL-001` | Economy changes require simulation and a holdout. | [Evaluation](../10-data-ai/04-evaluation.md) |
| `BR-ANL-001` | Analytics contains no PII. | [Analytics](../10-data-ai/05-analytics.md) |
| `BR-OPS-001` | Safety contacts are escalated within 15 minutes. | [Support & moderation](../11-operations/02-support-moderation.md) |
| `BR-OPS-002` | A backup is only trusted after a successful restore drill. | [Reliability](../11-operations/03-reliability.md) |
| `BR-LEG-001` | No degree, license, or accreditation claims. | [Regulatory](../12-legal/01-regulatory.md) |
| `BR-LEG-002` | Every credential states scope, date, and integrity tier. | [Credentials & employment](../12-legal/02-credentials-employment.md) |
| `BR-LEG-003` | Every asset has a rights record. | [IP & content](../12-legal/03-ip-content.md) |
| `BR-PHS-001` | No phase starts before the previous gate passes. | [Roadmap](../13-phases/01-roadmap.md) |
| `BR-PHS-002` | Guardrail breaches block release. | [Release gates](../13-phases/08-release-gates.md) |
| `BR-PHS-003` | Waivers require written risk acceptance and two signatures. | [Release gates](../13-phases/08-release-gates.md) |

---

## 5. Traceability rules

- Every FR maps to at least one acceptance test.
- Every NFR maps to a monitoring signal or test.
- Every BR maps to an enforcement point (code, policy, or contract).
- Phase docs reference the FRs they deliver.
- Orphan requirements (no test, no owner) are flagged quarterly.

---

## 6. Change control

1. Raise a change request with rationale and impact.
2. Update the source doc and this index.
3. Bump the doc version.
4. Notify dependent teams.
5. If the change affects the economy or assessment, run the relevant review.

---

## 7. Related

- [Risk register](02-risk-register.md)
- [Decision log](03-decision-log.md)
- [Open questions](05-open-questions.md)
