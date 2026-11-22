# Phase 3 — Economy (Weeks 49–72)

## 1. Question

**Does earning and redeeming work safely at meaningful scale?**

Phase 3 turns EC from a pilot into a real economy: multiple sponsors, a broader marketplace, post-100 prestige, live-fire sims, and mature fraud controls.

---

## 2. Scope

### In scope
- Public launch in 3 regions.
- EC live for all eligible learners.
- Marketplace: 5 categories, 30+ partners.
- Post-100 prestige and frontier challenges.
- 5 live-fire simulators.
- Expert panels at level 70+.
- Fraud operations and integrity analytics.
- 3 languages.
- Sponsor dashboard and reporting.
- Independent security audit.

### Out of scope
- 500 skills (Phase 4).
- Vision Pro.
- Metaverse.
- Livelihood pilots.
- 10 languages.

---

## 3. Deliverables

| # | Deliverable | Acceptance |
|---|---|---|
| 1 | Public launch | 100,000 MAU |
| 2 | EC live | ≥30% earn monthly |
| 3 | Marketplace | 30+ partners, 5 categories |
| 4 | Post-100 | 1,000 participants |
| 5 | Sims | 5 live-fire |
| 6 | Panels | Level 70+ |
| 7 | Fraud ops | <1% loss |
| 8 | Sponsor dashboard | Aggregate reporting |
| 9 | Localization | 3 languages |
| 10 | Independent audit | 0 critical findings |

---

## 4. Workplan

| Weeks | Focus |
|---|---|
| 49–52 | Ledger hardening + reserve model |
| 50–55 | Sponsor onboarding + pools |
| 52–58 | Marketplace expansion |
| 54–60 | Post-100 design + prestige |
| 56–62 | Live-fire sims |
| 58–64 | Panel operations |
| 60–66 | Fraud ops + integrity analytics |
| 62–68 | Localization (language 3) |
| 64–70 | Scale test + cost tuning |
| 66–71 | Independent audit |
| 70–72 | Phase 4 gate |

---

## 5. Economy design

### 5.1 Emission
- Sponsor-funded pools.
- Monthly budget per pool.
- Emission ≤80% of budget.
- Reserve ratio ≥1.0 for outstanding EC.
- Weekly reconciliation.

### 5.2 Sinks
- Redemption (primary).
- Optional sinks: exam fees, mentorship, AI credits, experiences.
- No forced sinks; savings encouraged.

### 5.3 Controls
- Per-learner monthly cap.
- Essentials priority.
- Dynamic pricing only where allowed.
- Fraud reserves.
- Public economy report quarterly.

See [Marketplace economics](../01-economy/06-marketplace-economics.md).

---

## 6. Marketplace scope

| Category | Partners | Notes |
|---|---|---|
| Essentials | 12 | Groceries, hygiene, transit |
| Tools | 6 | Devices, equipment |
| Services | 6 | Repairs, tutoring, care |
| Experiences | 4 | Events, travel, sport |
| Learning | 4 | Exams, mentorship, AI credits |

Rules:
- Curated, not open marketplace.
- Escrow on every order.
- Dispute path on every order.
- Quality thresholds enforced.
- No restricted categories.

---

## 7. Post-100

- Prestige levels with diminishing EC.
- Frontier challenges (research, live ops, build).
- Constellations (multi-skill missions).
- Bounties from employers/sponsors.
- Mentoring and teaching.
- Panel review at 90+.

---

## 8. Fraud and integrity

| Control | Target |
|---|---|
| Device binding | High-stakes gates |
| Behavior analytics | All gates |
| Collusion graph | Weekly |
| Marketplace velocity | Real-time |
| Insider controls | Four-eyes |
| Clawback | Compensating entries |
| Audit sample | 5% of gates |

---

## 9. Exit criteria

| Criterion | Target |
|---|---|
| Monthly active learners | 100,000 |
| Learners earning EC monthly | ≥30% |
| EC emission vs budget | ≤80% |
| Fraud loss | <1% of EC |
| Redemption success | ≥97% |
| Fulfillment ≤7 days | ≥90% |
| Marketplace CSAT | ≥4.3/5 |
| Post-100 participants | ≥1,000 |
| Independent audit critical findings | 0 |
| Uptime | 99.9% |
| Languages | 3 |
| Panels operating | Level 70+ |

### 9.1 Gate decision
- **Go:** economy healthy, fraud controlled, audit clean.
- **Hold:** fraud or reserve issues → freeze expansion.
- **Redesign:** redemption coverage fails → revisit supply model.

---

## 10. Requirements

- `FR-PHS-401`: EC is live only when reserves cover outstanding balances.
- `FR-PHS-402`: Every award is sponsor-funded.
- `FR-PHS-403`: Emission ≤80% of pool budget.
- `FR-PHS-404`: Marketplace uses escrow on every order.
- `FR-PHS-405`: Fraud loss is <1% of EC.
- `FR-PHS-406`: Post-100 paths are live.
- `FR-PHS-407`: Panels operate at level 70+.
- `FR-PHS-408`: Sponsor reporting is aggregate only.
- `FR-PHS-409`: Independent audit is completed.
- `FR-PHS-410`: Quarterly economy report is published.
- `NFR-PHS-401`: Ledger p99 <150 ms.
- `NFR-PHS-402`: Redemption API 99.9%.
- `NFR-PHS-403`: Reconciliation daily, zero unexplained variance.

---

## 11. Risks

| Risk | Mitigation |
|---|---|
| Inflation | Emission caps, sinks, monitoring |
| Fraud ring | Graph analysis, caps, manual review |
| Supply shortage | Partner pipeline, waitlists |
| Sponsor dependence | Diversify; XP-only fallback |
| Post-100 farming | Diminishing EC, panel gates |
| Audit findings | Pre-audit readiness review |

---

## 12. Related

- [Phase 2](04-phase-2.md)
- [Phase 4](06-phase-4.md)
- [Economy model](../01-economy/01-economy-model.md)
- [Post-100 prestige](../01-economy/05-post-100-prestige.md)
