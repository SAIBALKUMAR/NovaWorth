# Economy Model

## 1. Purpose

The economy exists to make learning **consequential**. It converts verified capability into a claim on goods, services, tools, and experiences — without becoming a free-money scheme, a speculative token, or a pay-to-win game.

This document defines the actors, currencies, flows, funding model, and governance rules. The exact scoring math lives in [Scoring model](03-scoring-model.md); the ledger implementation lives in [Points & ledger](02-points-ledger.md).

---

## 2. Design goals

| Goal | How the model achieves it |
|---|---|
| Reward verified learning, not engagement | Awards require a passed gate or verified evidence |
| Make hard levels worth more | Power-law XP/EC curves |
| Keep high levels rare | Difficulty, time, pass rates, expert panels |
| Prevent runaway inflation | Sponsor-funded emission, caps, sinks, treasury reserves |
| Protect the vulnerable | Essentials catalog, spending limits, no cash-out, wellbeing caps |
| Preserve learner agency | Any skill, any level attempt, no forced path |
| Survive audits | Double-entry ledger, rule versioning, evidence hashes |
| Align sponsors with outcomes | Cohort pools, verified-level reporting, unused-fund rules |

---

## 3. Actors

| Actor | Role in the economy |
|---|---|
| **Learner** | Earns XP/EC, redeems, builds reputation |
| **Sponsor** | Funds EC pools for cohorts, regions, skills, or populations |
| **Merchant/partner** | Supplies goods/services; receives settlement |
| **Employer** | Verifies credentials, may sponsor employees, posts bounties |
| **Examiner** | Validates high-level evidence; earns review fees/reputation |
| **Content author** | Creates levels; paid per accepted level and quality |
| **Platform/treasury** | Operates ledger, marketplace, risk, and emission controls |
| **Regulator/auditor** | Reviews consumer protection, privacy, employment, financial claims |
| **Economy Council** | Sets emission, pricing bands, and rule changes; learner-majority in Phase 5 |

---

## 4. Currencies and stores of value

### 4.1 XP — Mastery Points
- **Purpose:** Measure progress and mastery.
- **Earned by:** Passing levels, distinctions, prestige challenges, teaching.
- **Properties:** Non-transferable, non-spendable, never expires, not affected by marketplace activity.
- **Use:** Rank, level progress, eligibility for advanced content, prestige, leaderboards.

### 4.2 EC — Education Credits
- **Purpose:** Spendable claim on the marketplace.
- **Earned by:** Verified level passes, band bonuses, streaks/missions (capped), mentoring, bounties, real-world impact.
- **Properties:** Transferable only through approved gifts/family pools (Phase 3+), expires only if promotional, subject to clawback.
- **Use:** Goods, services, tools, experiences, AI services, exam fees, mentorship.
- **Backing:** Each EC in circulation is matched by a reserved sponsor/treasury fund at the regional anchor rate.

### 4.3 Reputation — Trust Score
- **Purpose:** Signal reliability and unlock high-stakes roles.
- **Earned by:** Verified evidence, peer review quality, mentoring, consistent integrity, successful bounties.
- **Properties:** Non-transferable, decays slowly with inactivity, recoverable through good behavior.
- **Use:** Examiner eligibility, guild leadership, marketplace trust, higher redemption limits.

### 4.4 Prestige — Post-100 Status
- **Purpose:** Recognize world-class and frontier contribution.
- **Earned by:** Prestige levels, frontier challenges, constellations, research, mentoring.
- **Properties:** Non-transferable, non-purchasable, permanent.
- **Use:** Status, access, governance weight, invitation-only opportunities.

### 4.5 What is *not* a currency
- Streaks, badges, followers, likes, and leaderboard rank are engagement signals, not stores of value.
- No internal token that can be traded, speculated on, or converted to cash in v1.

---

## 5. The economic loop

```
Sponsors / employers / governments
        │  fund pools (fiat)
        ▼
   TREASURY ──► reserves ──► EC emission rules
        │                         │
        │                         ▼
        │                    LEARNER EC
        │                    (earned only by proof)
        │                         │
        ▼                         ▼
   MARKETPLACE ◄──────────── redemption
        │
        ├── merchant settlement (fiat)
        ├── platform fee
        └── treasury replenishment
```

### 5.1 Emission
- EC is created only when an award rule fires after a verified event.
- Every emission references a **funding pool** and a **rule version**.
- Emission cannot exceed the pool's available balance. If the pool is exhausted, awards queue or pay at a reduced rate — never create unbacked EC.
- Base level awards are funded first; bonuses and bounties are funded from separate pools.

### 5.2 Circulation
- Learners hold EC in a wallet.
- EC can be spent, saved, gifted (approved cases), or allocated to a family pool.
- EC cannot be sold, traded, or converted to cash.

### 5.3 Sinks
| Sink | Share of redemption (target) | Notes |
|---|---|---|
| Essential goods | 35% | Food, hygiene, transport, utilities |
| Tools & devices | 20% | Laptops, phones, instruments, equipment |
| Services | 20% | Repairs, tutoring, care, home services |
| Experiences | 10% | Events, travel, museums, sports |
| Learning upgrades | 8% | Exam fees, mentorship, premium sims |
| Status/cosmetic | 5% | Profile themes, guild banners (never power) |
| Fees | 2% | Marketplace/transaction fees |

### 5.4 Savings
Learners may save EC for larger goals. Savings are encouraged; a 20–30% savings rate is healthy and signals trust in the currency. The treasury must hold reserves against all outstanding EC.

---

## 6. Funding model

### 6.1 Sponsor pools
A sponsor pool has:
- `pool_id`, sponsor, purpose, region, skill/domain eligibility, cohort eligibility.
- `budget_fiat`, `anchor_rate`, `reserved_ec`, `emitted_ec`, `redeemed_ec`, `expires_at`.
- Rules for unused funds: roll over, return, or convert to a new pool.

### 6.2 Funding sources
| Source | Share target (Phase 3) | Notes |
|---|---|---|
| Government/workforce grants | 35% | Outcomes-based contracts |
| Employers | 25% | Upskilling, hiring pipelines |
| Foundations/NGOs | 15% | Equity and access programs |
| Marketplace margin | 15% | Platform fee recycled into treasury |
| Brand/sponsor campaigns | 7% | Clearly labeled, no influence on scoring |
| Learner payments | 3% | Optional premium features; never required to earn |

### 6.3 Anchor rate
- `1 EC = $0.05` baseline subsidy value (regional index 0.6–1.8).
- The anchor is a **redemption value**, not a cash exchange rate.
- Regional baskets adjust prices so a "coffee" or "groceries" basket feels comparable across markets.

### 6.4 Reserves
- Treasury holds fiat reserves equal to at least **100% of outstanding EC** at anchor.
- A separate **fraud reserve** of 2–5% covers clawbacks and disputes.
- Reserves are audited monthly and published in aggregate.

---

## 7. Business rules

| ID | Rule |
|---|---|
| BR-ECN-001 | No EC without a verified assessment event or approved real-world evidence. |
| BR-ECN-002 | Every EC emission must reference a funded pool and a rule version. |
| BR-ECN-003 | Emission cannot exceed available pool balance. |
| BR-ECN-004 | Base level awards are paid before bonuses. |
| BR-ECN-005 | Repeating the same level yields diminishing EC (1.0 → 0.2 → 0.05 → 0). |
| BR-ECN-006 | EC cannot be converted to cash, crypto, or transferable value in v1. |
| BR-ECN-007 | Minors have guardian-set spending limits and no peer transfers. |
| BR-ECN-008 | Fraudulent awards are clawed back; clawbacks are appealable. |
| BR-ECN-009 | Sponsor branding cannot influence level scores, recommendations, or gate outcomes. |
| BR-ECN-010 | Marketplace prices are public and versioned; personalized pricing is banned. |
| BR-ECN-011 | Essentials have price caps and priority supply. |
| BR-ECN-012 | A learner can always see the provenance of every EC balance. |
| BR-ECN-013 | Economy rule changes require a 30-day notice, impact analysis, and grandfathering. |
| BR-ECN-014 | No pay-to-win: money cannot buy XP, levels, gate passes, or prestige. |
| BR-ECN-015 | Sponsored content is labeled and cannot replace core curriculum. |

---

## 8. Governance

### 8.1 Economy Council
- **Phase 1–2:** internal council (Product, Learning, Finance, Trust, Legal).
- **Phase 3:** adds sponsor and learner representatives.
- **Phase 5:** learner-majority, independent chair, public minutes.

### 8.2 Responsibilities
- Set emission targets and bonus pools.
- Approve domain multipliers.
- Review inflation, redemption coverage, and fraud.
- Approve marketplace category caps.
- Publish quarterly economy report.

### 8.3 Change process
1. Proposal with data and impact model.
2. Fairness and legal review.
3. 30-day learner notice.
4. Grandfather existing awards.
5. Monitor for 90 days; rollback if guardrails breach.

---

## 9. Regional design

- **Currency of account:** EC is global; prices are local.
- **Anchor index:** adjusted for purchasing power.
- **Essentials basket:** defined per region with local partners.
- **Sponsor pools:** can be region-locked.
- **Payout rails:** local payment partners; no crypto.
- **Language and content:** localized; assessments culturally reviewed.

---

## 10. Risks and mitigations

| Risk | Impact | Mitigation |
|---|---|---|
| Inflation from over-emission | EC loses meaning | Pool caps, reserves, sink ratio monitoring |
| Sponsor withdrawal | Awards stop | Diversified funding, 6-month runway, base-rate priority |
| Fraud/farming | Treasury loss, trust damage | Layered integrity, clawbacks, audits |
| Marketplace shortage | Redemption promise breaks | Supply contracts, essentials priority, waitlists |
| Overpromising income | Regulatory/consumer harm | "Supplemental dividend" messaging, no cash claims |
| Elite capture | Inequity | Need-based pools, floor multipliers, access programs |
| Complexity | Learners don't understand | Simple wallet UX, explainers, one number per level |

---

## 11. Acceptance criteria

- A learner can trace every EC to a specific award and funding pool.
- A sponsor can see budget, emission, redemption, and outcomes for their pool.
- The ledger balances to zero at every moment.
- Emission never exceeds funded reserves in any region.
- A fraudulent award can be reversed within 24 hours of confirmation.
- A learner can redeem an essential item within 7 days in a supported region.
- Economy rules are versioned and visible in the app.

---

## 12. Related requirements

`FR-ECN-001` … `FR-ECN-060` (see [Requirements index](../14-appendices/01-requirements-index.md)).
