# Economy Simulation Model

## 1. Purpose

This document provides a worked, tunable model of the economy so the team can test assumptions before committing to real money. It is not a forecast; it is a set of levers and sensitivities.

Use it to answer:
- How much EC will a cohort earn?
- Can the sponsor budget support it?
- Will the marketplace have enough supply?
- What happens if fraud rises or funding falls?
- Where should we tune the curve?

---

## 2. Base scenario: 10,000 monthly active learners

### 2.1 Assumptions
- 10,000 monthly active learners (MAL).
- Distribution skewed to early levels (typical learning product).
- Average EC per level from the scoring model, domain multiplier 1.05.
- 65% redemption rate; 35% saved.
- Sponsor budget $250,000/month.
- Anchor $0.05/EC.
- Marketplace capacity 3.0M EC/month.

### 2.2 Level distribution and earning

| Band | Share | Learners | Levels/month | Avg EC/level | EC/month |
|---|---|---|---|---|---|
| 1–10 | 45% | 4,500 | 2.5 | 80 | 900,000 |
| 11–20 | 27% | 2,700 | 1.8 | 250 | 1,215,000 |
| 21–30 | 15% | 1,500 | 1.2 | 500 | 900,000 |
| 31–40 | 7% | 700 | 0.8 | 750 | 420,000 |
| 41–50 | 3.5% | 350 | 0.5 | 1,000 | 175,000 |
| 51–60 | 1.5% | 150 | 0.35 | 1,250 | 65,625 |
| 61–70 | 0.6% | 60 | 0.25 | 1,500 | 22,500 |
| 71–80 | 0.25% | 25 | 0.18 | 1,800 | 8,100 |
| 81–90 | 0.1% | 10 | 0.12 | 2,100 | 2,520 |
| 91–100 | 0.05% | 5 | 0.08 | 2,400 | 960 |
| **Base total** | | **10,000** | | | **3,709,705** |

Add band bonuses and missions at 10% of base: **370,971 EC**.
**Total emission: 4,080,676 EC/month.**

### 2.3 Funding and redemption

| Metric | Value |
|---|---|
| Emission | 4.08M EC |
| Fiat value @ $0.05 | $204,034 |
| Sponsor budget | $250,000 |
| Utilization | 81.6% |
| Redeemed (65%) | 2.65M EC |
| Saved (35%) | 1.43M EC |
| Fiat redeemed | $132,622 |
| Marketplace capacity | 3.0M EC |
| Coverage | 1.13× |
| Merchant settlement (92%) | $122,012 |
| Platform fee (8%) | $10,610 |
| Outstanding liability added | $71,412 |
| Reserve requirement | 100% of outstanding |

### 2.4 Interpretation
- The sponsor budget supports the cohort with ~18% headroom.
- Marketplace capacity is slightly above demand; essentials should be prioritized.
- The platform fee does not cover AI/content/support costs at this stage; the model relies on sponsor funding and scale.
- Outstanding EC grows if savings remain high; reserves must grow accordingly.

---

## 3. Sensitivity analysis

### 3.1 Fraud rate
| Fraud % of awards | Monthly loss | Annual loss |
|---|---|---|
| 0.5% | $1,020 | $12,242 |
| 1.0% | $2,040 | $24,484 |
| 3.0% | $6,121 | $73,452 |
| 5.0% | $10,202 | $122,420 |

Controls target <1%. At 3%, the fraud reserve and sponsor budget are stressed.

### 3.2 Redemption rate
| Redemption | EC redeemed | Fiat | Coverage |
|---|---|---|---|
| 50% | 2.04M | $102,017 | 1.47× |
| 65% | 2.65M | $132,622 | 1.13× |
| 80% | 3.26M | $163,227 | 0.92× |
| 95% | 3.88M | $193,832 | 0.77× |

Above 80%, supply becomes the binding constraint. Levers: expand catalog, raise prices modestly, or add savings incentives.

### 3.3 Sponsor budget cut
| Budget | Utilization | Action |
|---|---|---|
| $250k | 82% | Normal |
| $200k | 102% | Cut bonuses, queue awards |
| $150k | 136% | Freeze bonuses, reduce domain multipliers |
| $100k | 204% | Emergency: base awards only, waitlist |

Never create unbacked EC. Base awards are protected before bonuses.

### 3.4 Level mix shift
If the cohort skews senior (more high-level learners), emission rises faster than learner count because EC per level is higher. A 10% shift from band 1–3 to band 4–6 increases emission by roughly 25–35%. Plan sponsor pools by level mix, not just headcount.

### 3.5 Pass rate
If pass rates rise 10 points across the board, emission rises ~10–15% (more awards, fewer retries). If pass rates fall 10 points, emission falls but motivation and retention may suffer. Calibrate difficulty; do not use pass rate as a budget lever.

---

## 4. Cohort lifecycle

| Month | Active learners | Avg level | Emission | Notes |
|---|---|---|---|---|
| 1 | 2,000 | 3 | 0.35M | Onboarding wave |
| 3 | 5,000 | 8 | 1.4M | Habit forming |
| 6 | 8,000 | 14 | 2.9M | First band completions |
| 12 | 10,000 | 22 | 4.1M | Stable |
| 24 | 9,000 | 31 | 4.6M | Senior skew |
| 36 | 8,000 | 38 | 5.1M | Plateau; prestige begins |

At 36 months, emission per learner is higher even with fewer learners. Sponsor pools must scale with level mix.

---

## 5. Marketplace capacity model

```
Capacity_EC = Σ(partner_category_capacity_fiat) / anchor_rate
```

Example for 10k learners:
| Category | Monthly fiat capacity | EC capacity |
|---|---|---|
| Essentials | $60,000 | 1.2M |
| Tools | $30,000 | 0.6M |
| Services | $25,000 | 0.5M |
| Experiences | $15,000 | 0.3M |
| Learning | $20,000 | 0.4M |
| **Total** | **$150,000** | **3.0M** |

If demand exceeds capacity, prioritize essentials and learning, then waitlist the rest.

---

## 6. Reserve and liability model

```
Outstanding_EC(t) = Outstanding_EC(t-1) + Emission(t) - Redemption(t) - Expiry(t)
Required_Reserve(t) = Outstanding_EC(t) × anchor_rate
```

With 35% savings and 4.08M emission:
- Month 1 liability: 1.43M EC = $71.4k.
- Month 12 liability (approx): $71.4k × 12 = $857k if no prior reserve.
- Reserve must be funded upfront or built from sponsor contributions.

**Rule:** sponsors fund reserves, not just emission. A pool that funds 1M EC must reserve $50k at $0.05.

---

## 7. Tuning playbook

| Problem | First lever | Second lever | Avoid |
|---|---|---|---|
| Emission > budget | Cut bonuses | Reduce domain multipliers | Cutting base awards |
| Redemption < target | Add desirable catalog | Savings goals | Forcing spending |
| Redemption > supply | Expand partners | Modest price increase | Auctioning essentials |
| Fraud rising | Tighten proctoring | Clawbacks | Punishing all learners |
| Senior skew costly | Prestige EC curve | Sponsor senior pools | Blocking progression |
| Low motivation | Band bonuses | Social leagues | Inflating pass rates |
| High dispute rate | Rubric clarity | Grader calibration | Ignoring appeals |

---

## 8. Scenario: fraud ring

- 200 accounts, 5% of cohort.
- Each farms 500 EC/month = 100,000 EC = $5,000/month.
- Detection after 2 months: $10,000 loss.
- Clawback recovery: 40% ($4,000).
- Net loss: $6,000.
- Controls: device binding, collusion graph, velocity limits, manual review.
- Target: detect within 2 weeks, recover >60%.

---

## 9. Scenario: sponsor withdrawal

- Sponsor providing 40% of budget exits with 30 days' notice.
- Emission must drop by 40%.
- Order of cuts:
  1. Pause bonus pool (10%).
  2. Pause bounties (5%).
  3. Reduce domain multipliers for oversupplied skills (10%).
  4. Queue non-essential awards; protect base awards (15%).
- Communicate transparently; offer savings and alternative pools.
- Do not silently reduce already-earned balances.

---

## 10. Model maintenance

- Re-run monthly with actuals.
- Update distribution, pass rates, redemption, and costs.
- Publish a summary to the Economy Council.
- Version the model alongside scoring rules.
- Independent review annually.

---

## 11. Open parameters

| Parameter | Current | Owner | Review |
|---|---|---|---|
| Anchor rate | $0.05 | Finance | Quarterly |
| Redemption target | 65% | Economy Council | Quarterly |
| Savings target | 35% | Economy Council | Quarterly |
| Fraud reserve | 3% | Trust/Finance | Monthly |
| Bonus share | 10% | Product | Quarterly |
| Domain multipliers | 0.9–1.3 | Council | Annual |
| Reserve ratio | 100% | Finance | Monthly |

---

## 12. Acceptance criteria

- `FR-ECN-301`: The model can be re-run with actuals monthly.
- `FR-ECN-302`: Every scenario has a documented action plan.
- `FR-ECN-303`: Reserve requirements are computed automatically.
- `FR-ECN-304`: Emission cannot exceed funded reserves.
- `FR-ECN-305`: The council reviews the model quarterly.
- `FR-ECN-306`: Model assumptions are public in aggregate.
