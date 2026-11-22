# Phase 4 — Scale (Weeks 73–108)

## 1. Question

**Can the product run at scale with trust, breadth, and spatial depth?**

Phase 4 completes the 500-skill catalog, launches Vision Pro, scales guilds and localization, and proves unit economics at 1M registered learners.

---

## 2. Scope

### In scope
- 500 skills fully authored (100 levels each).
- Vision Pro app with 10 spatial experiences.
- Guilds and leagues at scale.
- 10 languages.
- Employer verification partners (100).
- Panels at level 80+.
- Cost optimization to <$1.50/learner/month.
- Public transparency report.
- Optional metaverse go/no-go.

### Out of scope
- Livelihood pilots (Phase 5).
- Open protocol.
- Learner-majority governance.

---

## 3. Deliverables

| # | Deliverable | Acceptance |
|---|---|---|
| 1 | 500 skills | 50,000 levels authored |
| 2 | Vision Pro | 10,000 MAU |
| 3 | Guilds | ≥25% of MAU |
| 4 | Localization | 10 languages |
| 5 | Employer partners | 100 verifying |
| 6 | Credentials | 500,000 issued |
| 7 | Cost | <$1.50/learner/month |
| 8 | Accessibility | WCAG 2.2 AA verified |
| 9 | Transparency report | Published |
| 10 | Metaverse decision | Go/no-go |

---

## 4. Workplan

| Weeks | Focus |
|---|---|
| 73–80 | Content factory scale-up |
| 76–84 | 500-skill authoring waves |
| 80–88 | Vision Pro build + 10 experiences |
| 84–92 | Guilds, leagues, mentorship at scale |
| 88–96 | Localization waves (to 10 languages) |
| 92–100 | Employer partnerships + verification API |
| 96–104 | Cost optimization + capacity |
| 100–106 | Accessibility + security audits |
| 104–107 | Transparency report |
| 106–108 | Phase 5 gate |

---

## 5. Content factory at scale

| Metric | Target |
|---|---|
| Levels authored | 50,000 |
| Cost per level | <$150 |
| Review SLA | <5 days |
| Pilot per level | 30–50 learners |
| Provenance completeness | 100% |
| Localized with in-country review | 100% |

Process:
- Template reuse by archetype.
- AI drafting with retrieval grounding.
- Six-reviewer gate.
- Batch pilots.
- Continuous refresh.

---

## 6. Spatial (Vision Pro)

- 10 experiences across craft, design, role-play, sim.
- 2D fallback for every experience.
- No scoring advantage.
- Seated mode, captions, accessibility.
- Performance: 90 fps target, 72 minimum.
- Go/no-go based on outcome lift ≥15% and cost.

See [Vision Pro](../06-platforms/04-visionos.md).

---

## 7. Social at scale

- Guilds 5–50 members.
- 12-week leagues, opt-in.
- Mentor matching and compensation.
- Peer review with calibration.
- Moderation 24/7.
- Safeguarding for minors.

---

## 8. Unit economics

| Cost | Target |
|---|---|
| AI inference | <$1.50/learner/month total |
| Media | <$0.35 |
| Compute | <$0.60 |
| Storage | <$0.15 |
| Support | <$0.40 |
| **Total** | **<$3.50** |

Levers: routing, caching, transcript-first media, sim instancing, regional compute.

---

## 9. Exit criteria

| Criterion | Target |
|---|---|
| Registered learners | 1,000,000 |
| Monthly active learners | 250,000 |
| Skills fully authored | 500 |
| Verifiable credentials issued | 500,000 |
| Vision Pro MAU | 10,000 |
| Guild participation | ≥25% of MAU |
| Localization | 10 languages |
| Cost per active learner | <$1.50/month |
| Employer verification partners | 100 |
| Accessibility audit | WCAG 2.2 AA verified |
| Independent security audit | 0 critical |
| Transparency report | Published |

### 9.1 Gate decision
- **Go:** scale and trust targets met; livelihood pilot feasible.
- **Consolidate:** cost or quality lag → optimize before Phase 5.
- **Defer spatial:** outcome lift insufficient → keep 2D only.

---

## 10. Requirements

- `FR-PHS-501`: 500 skills are complete to level 100.
- `FR-PHS-502`: Every spatial experience has a 2D fallback.
- `FR-PHS-503`: Guilds are moderated 24/7.
- `FR-PHS-504`: 10 languages are reviewed in-country.
- `FR-PHS-505`: Employer verification API is available.
- `FR-PHS-506`: Cost per learner is <$1.50/month.
- `FR-PHS-507`: Credentials are verifiable at scale.
- `FR-PHS-508`: WCAG 2.2 AA is verified.
- `FR-PHS-509`: Transparency report is public.
- `FR-PHS-510`: Metaverse decision is evidence-based.
- `NFR-PHS-501`: 250k MAU with p95 <400 ms.
- `NFR-PHS-502`: 99.95% availability.
- `NFR-PHS-503`: Credential verification <1 s.

---

## 11. Risks

| Risk | Mitigation |
|---|---|
| Content quality at volume | Factory gates, pilots, audits |
| Spatial cost | Cap investment; evidence gate |
| Localization drift | In-country review, DIF |
| Cost overrun | Routing, caching, budget caps |
| Moderation load | AI triage + human escalation |
| Employer trust | Independent audit, transparency |

---

## 12. Related

- [Phase 3](05-phase-3.md)
- [Phase 5](07-phase-5.md)
- [Catalog (500)](../02-skills/02-catalog-500.md)
- [Reliability](../11-operations/03-reliability.md)
