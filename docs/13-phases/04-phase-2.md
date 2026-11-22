# Phase 2 — Beta (Weeks 27–48)

## 1. Question

**Does it work on mobile, at breadth, with credible assessments?**

Beta opens registration, ships native mobile, expands to 100 skills, and hardens assessment quality. A limited EC pilot begins with one sponsor and a curated essentials catalog.

---

## 2. Scope

### In scope
- Public registration (waitlist-gated).
- iOS + Android apps with offline levels 1–10.
- 100 skills with levels 1–30; 25 skills complete 1–100.
- Assessment calibration and reliability reporting.
- Role-play at scale (20+ scenarios).
- Two simulators.
- EC pilot: one sponsor pool, essentials only.
- Marketplace pilot: 1 region, 3 partners.
- Localization: 2 languages.
- Support tiers T0–T2.
- Trust & safety: moderation + basic fraud.

### Out of scope
- Full marketplace.
- Post-100.
- Spatial.
- Panels above level 70.
- 10 languages.

---

## 3. Deliverables

| # | Deliverable | Acceptance |
|---|---|---|
| 1 | Public beta | 25,000 registered |
| 2 | Mobile apps | 10,000 WAU, ≥50% sessions |
| 3 | 100 skills | 25 complete, 100 at 1–30 |
| 4 | Assessment calibration | α ≥0.80 on core |
| 5 | Role-play | Human agreement ≥85% |
| 6 | EC pilot | 1 sponsor, essentials |
| 7 | Marketplace pilot | ≥95% redemption success |
| 8 | Localization | 2 languages |
| 9 | Fraud controls | <1% loss in pilot |
| 10 | Sponsor LOIs | 20 |

---

## 4. Workplan

| Weeks | Focus |
|---|---|
| 27–30 | Mobile foundation + sync |
| 29–33 | Offline packs + camera evidence |
| 30–36 | Content expansion to 100 skills |
| 32–38 | Assessment calibration + item analytics |
| 34–40 | Role-play scenarios + human review |
| 36–42 | EC pilot + ledger hardening |
| 38–44 | Marketplace pilot + fulfillment |
| 40–45 | Localization (language 2) |
| 42–46 | Fraud + moderation ops |
| 44–47 | Scale test, accessibility audit |
| 46–48 | Beta review + Phase 3 gate |

---

## 5. Content plan

| Type | Count |
|---|---|
| Skills at 1–30 | 100 |
| Skills at 1–100 | 25 |
| Levels authored | 25×100 + 75×30 = 4,750 |
| Items | 20+ per level |
| Role-play scenarios | 20 |
| Simulators | 2 |
| Concept pages | 3–5 per level |

---

## 6. Assessment quality program

- Pilot every new level with 30–50 learners.
- Item analysis: difficulty, discrimination, distractor.
- Reliability: α ≥0.80 on core assessments.
- Inter-rater: κ ≥0.70.
- AI-human agreement ≥90%.
- DIF analysis for 2 languages.
- Quarterly fairness report.

---

## 7. EC pilot design

| Parameter | Value |
|---|---|
| Sponsor | 1 anchor sponsor |
| Pool | $250k equivalent |
| Eligibility | Verified levels only |
| Catalog | Essentials, 1 region, 3 partners |
| Cap per learner | 2,000 EC/month |
| Redemption | Escrow + confirmation |
| Fraud controls | Device, address, velocity |

Pilot rules:
- EC is real but limited.
- No cash-out.
- Every award backed by reserve.
- Weekly reconciliation.
- Public pilot report at the end.

---

## 8. Exit criteria

| Criterion | Target |
|---|---|
| Registered learners | 25,000 |
| Weekly active learners | 10,000 |
| Skills with 100 levels | 25 |
| Skills with levels 1–30 | 100 |
| D30 retention | ≥35% |
| Level 10 within 30 days | ≥30% |
| Mobile share of sessions | ≥50% |
| Role-play human agreement | ≥85% |
| Assessment α (core) | ≥0.80 |
| Sponsor LOIs | 20 |
| Marketplace redemption success | ≥95% |
| Fraud loss (pilot) | <1% |
| Accessibility | WCAG 2.2 AA verified |
| Languages | 2 |

### 8.1 Gate decision
- **Go:** breadth, mobile, and quality targets met; EC funding secured.
- **Iterate:** mobile or assessment lag → fix before economy.
- **Pause:** fraud or funding fails → do not open economy.

---

## 9. Requirements

- `FR-PHS-301`: Mobile supports offline levels 1–10.
- `FR-PHS-302`: 100 skills are playable to level 30.
- `FR-PHS-303`: 25 skills are complete to level 100.
- `FR-PHS-304`: Core assessments meet α ≥0.80.
- `FR-PHS-305`: Role-play meets human agreement ≥85%.
- `FR-PHS-306`: EC pilot is sponsor-funded and capped.
- `FR-PHS-307`: Marketplace pilot uses escrow.
- `FR-PHS-308`: Fraud loss is <1% in pilot.
- `FR-PHS-309`: WCAG 2.2 AA is verified.
- `FR-PHS-310`: Two languages are fully reviewed.
- `NFR-PHS-301`: Mobile cold start <2 s.
- `NFR-PHS-302`: Offline sync ≥99%.
- `NFR-PHS-303`: Uptime ≥99.9%.

---

## 10. Risks

| Risk | Mitigation |
|---|---|
| Mobile quality | Native modules only where needed; device lab |
| Content breadth vs depth | 1–30 breadth, 1–100 for exemplars |
| Assessment drift | Weekly calibration; retire bad items |
| EC pilot fraud | Caps, curated catalog, manual review |
| Sponsor concentration | 20 LOIs; diversify before Phase 3 |
| Localization quality | In-country review; DIF checks |

---

## 11. Related

- [Phase 1](03-phase-1.md)
- [Phase 3](05-phase-3.md)
- [Mobile](../06-platforms/03-mobile.md)
- [Marketplace overview](../08-marketplace/01-marketplace-overview.md)
