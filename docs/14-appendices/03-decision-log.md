# Decision Log

## 1. Purpose

A record of the decisions that shape the product, why they were made, and what they rule out. Future teams should not have to rediscover the reasoning.

**Format:** ID · decision · date · status · rationale · consequences.

---

## 2. Product decisions

### D-001 — Capability-backed points, not engagement points
- **Date:** 2026-11-21 · **Status:** Accepted
- **Decision:** XP and EC require verified evidence from a gate or accepted artifact. Logins, watching, and streaks earn nothing.
- **Why:** The product's premise is verified capability. Engagement points would recreate the attention economy it is meant to replace.
- **Consequences:** Higher assessment cost; slower early earnings; stronger long-term trust.

### D-002 — 500 skills × 100 levels
- **Date:** 2026-11-21 · **Status:** Accepted
- **Decision:** 20 domains × 25 skills, each with 100 authored levels (10 bands × 10).
- **Why:** Breadth for matching, depth for credibility, a fixed scope for the content factory.
- **Consequences:** 50,000 level specs; requires a pipeline, not hand-authoring.

### D-003 — Power-law difficulty and rewards
- **Date:** 2026-11-21 · **Status:** Accepted
- **Decision:** `XP = 100 × L^1.5 × m_d`; `EC = 10 × L^1.2 × m_d × Q × E × M × R`; hours `1.5 × L^1.1`; difficulty `10 × L^1.35`.
- **Why:** Higher levels should feel rarer and be worth more, without becoming a grind.
- **Consequences:** Level 100 is a multi-year commitment; EC must be reserve-funded.

### D-004 — "IQ" as demonstrated performance, not a global score
- **Date:** 2026-11-21 · **Status:** Accepted
- **Decision:** Aptitude is measured per domain and shown as a profile with confidence intervals. It affects support and recommendations, never direct EC. A +0.10 growth dividend applies for levels 1–30 to learners with lower prior aptitude.
- **Why:** A global IQ payout would be unfair, legally risky, and demotivating. Effort and mastery should drive rewards.
- **Consequences:** No "IQ score" marketing; fairness audits required.

### D-005 — Post-100 continues with diminishing returns
- **Date:** 2026-11-21 · **Status:** Accepted
- **Decision:** After level 100, prestige levels pay `EC(100) × 0.5 / n^0.7` and push learners toward specialties, constellations, and bounties.
- **Why:** Prevent infinite farming while keeping mastery meaningful.
- **Consequences:** Prestige is status and access, not a salary.

### D-006 — EC is closed-loop and not cash-redeemable in v1
- **Date:** 2026-11-21 · **Status:** Accepted
- **Decision:** EC can be spent in the marketplace only. No cash-out, no P2P transfer (except approved mentor/bounty flows).
- **Why:** Avoids money-transmission regulation, fraud, and inflation.
- **Consequences:** Redemption supply is the binding constraint; essentials must be prioritized.

### D-007 — Learner agency over recommendation
- **Date:** 2026-11-21 · **Status:** Accepted
- **Decision:** The recommender advises; the learner can choose any of the 500 skills.
- **Why:** Autonomy predicts persistence; recommendations can be wrong.
- **Consequences:** The full catalog is always visible; fit scores are explained.

### D-008 — Text path for every level
- **Date:** 2026-11-21 · **Status:** Accepted
- **Decision:** No level requires video, VR, or a specific device.
- **Why:** Access and fairness; low-bandwidth learners are core, not edge.
- **Consequences:** Content parity is a publish gate.

---

## 3. Economy decisions

### D-010 — Double-entry ledger from day one
- **Date:** 2026-11-21 · **Status:** Accepted
- **Decision:** All XP/EC movements are ledger entries with hash-chained accounts and compensating corrections.
- **Why:** Auditability, fraud detection, sponsor reporting.
- **Consequences:** No direct balance updates; reconciliation is mandatory.

### D-011 — Sponsor-funded emission
- **Date:** 2026-11-21 · **Status:** Accepted
- **Decision:** Every EC award is backed by a funded pool before posting.
- **Why:** Prevents unfunded promises and inflation.
- **Consequences:** Awards may queue when a pool is exhausted; sponsors get reporting.

### D-012 — Repeat decay
- **Date:** 2026-11-21 · **Status:** Accepted
- **Decision:** Re-passing the same level pays EC at 1.0 → 0.2 → 0.05 → 0; XP always pays once.
- **Why:** Allow refreshers without farming.
- **Consequences:** Retakes are for learning, not income.

### D-013 — Regional anchor rates
- **Date:** 2026-11-21 · **Status:** Accepted
- **Decision:** EC is global; purchasing power is regional via anchor rates (e.g., $0.05 US/EU, $0.02 India/Kenya).
- **Why:** Fairness across economies; local supply.
- **Consequences:** Region-locked fulfillment to prevent arbitrage.

---

## 4. Assessment decisions

### D-020 — Knowledge 80%, practical 2.5/4
- **Date:** 2026-11-21 · **Status:** Accepted
- **Decision:** Knowledge pass ≥80%; practical average ≥2.5/4 with no criterion <2; safety floor 3 for high-risk.
- **Why:** Clear, defensible thresholds aligned to competence.
- **Consequences:** No compensation across critical criteria.

### D-021 — Integrity tiers by level
- **Date:** 2026-11-21 · **Status:** Accepted
- **Decision:** Open (1–20), Monitored (21–50), Proctored (51–80), Panel (81–100).
- **Why:** Proportional controls; high stakes get human review.
- **Consequences:** Cost rises with level; panels need scheduling.

### D-022 — Human review above level 40
- **Date:** 2026-11-21 · **Status:** Accepted
- **Decision:** AI may propose scores, but humans review at 40+ and decide at 80+.
- **Why:** Accountability and fairness at high stakes.
- **Consequences:** Latency and cost; requires examiner capacity.

---

## 5. Platform decisions

### D-030 — PWA first, native where needed
- **Date:** 2026-11-21 · **Status:** Accepted
- **Decision:** Web is a PWA; mobile is React Native with native modules for camera/offline; visionOS is native SwiftUI/RealityKit.
- **Why:** Reach and speed, with native quality where it matters.
- **Consequences:** Shared design system and API contracts are mandatory.

### D-031 — Vision Pro is an enhancement
- **Date:** 2026-11-21 · **Status:** Accepted
- **Decision:** Every spatial experience has a 2D fallback and no scoring advantage.
- **Why:** Fairness and cost control.
- **Consequences:** Spatial investment must prove outcome lift.

### D-032 — Metaverse is deferred
- **Date:** 2026-11-21 · **Status:** Accepted
- **Decision:** Build WebXR/metaverse only after Phase 3, gated by evidence.
- **Why:** Unproven learning value; high cost and moderation burden.
- **Consequences:** 2D multiplayer is the default for shared sims.

---

## 6. Trust & data decisions

### D-040 — AI discloses itself
- **Date:** 2026-11-21 · **Status:** Accepted
- **Decision:** Learners are always told when interacting with AI; AI-generated media is labeled.
- **Why:** Honesty and informed consent.
- **Consequences:** Some UX friction; brand trust gain.

### D-041 — No training on learner data without consent
- **Date:** 2026-11-21 · **Status:** Accepted
- **Decision:** Learner content is never used for model training without explicit, revocable consent.
- **Why:** Privacy and trust.
- **Consequences:** Model improvement relies on licensed and synthetic data.

### D-042 — Minors: guardian consent and restricted design
- **Date:** 2026-11-21 · **Status:** Accepted
- **Decision:** Under-16 requires guardian consent; no peer transfers; private by default; spending approval.
- **Why:** Safety and legal compliance.
- **Consequences:** Reduced feature set for minors; safeguarding operations.

---

## 7. Process decisions

### D-050 — Phased gates with written exit criteria
- **Date:** 2026-11-21 · **Status:** Accepted
- **Decision:** Six phases, each with metrics, guardrails, and a signed gate.
- **Why:** Prevent premature scaling and unfunded promises.
- **Consequences:** Slower launches; fewer catastrophic mistakes.

### D-051 — Requirements IDs are stable
- **Date:** 2026-11-21 · **Status:** Accepted
- **Decision:** FR/NFR/BR IDs never get reused; changes create new IDs or version bumps.
- **Why:** Traceability across docs, tests, and tickets.
- **Consequences:** Some IDs will be retired, not renumbered.

### D-052 — Catalog generated, not hand-edited
- **Date:** 2026-11-21 · **Status:** Accepted
- **Decision:** The 500-skill catalog is produced by `tools/build-catalog.mjs`; edits go to the script.
- **Why:** Enforce 25 per domain, 500 total, unique IDs.
- **Consequences:** Regeneration required after any taxonomy change.

---

## 8. Superseded / rejected

| ID | Proposal | Outcome | Why |
|---|---|---|---|
| X-01 | Direct cash-out of EC | Rejected | Money transmission, fraud, inflation |
| X-02 | Global IQ score with payout | Rejected | Unfair, legally risky, demotivating |
| X-03 | Engagement-based points | Rejected | Contradicts capability premise |
| X-04 | Open marketplace at launch | Deferred | Quality and fraud risk |
| X-05 | Metaverse-first launch | Rejected | Unproven value, high cost |
| X-06 | Streak-based rewards | Rejected | Anxiety and churn; replaced by momentum |
| X-07 | Paid placement in recommendations | Banned | Corrupts advice |
| X-08 | AI-only grading at level 80+ | Banned | Accountability |

---

## 9. How to add a decision

1. Write the decision, date, and status.
2. State the rationale and the alternatives considered.
3. List consequences and what it rules out.
4. Link affected docs and requirements.
5. Announce in the weekly product review.

---

## 10. Related

- [Open questions](05-open-questions.md)
- [Risk register](02-risk-register.md)
- [Requirements index](01-requirements-index.md)
