# Marketplace Economics

## 1. Purpose

The marketplace is where learning becomes consequential. It must feel fair, useful, and trustworthy — while remaining economically sustainable and resistant to fraud.

This document covers pricing, supply, demand, subsidies, inflation control, and unit economics. Fulfillment operations are in [Fulfillment & disputes](../08-marketplace/04-fulfillment-disputes.md).

---

## 2. Principles

1. **Essentials first.** Food, hygiene, transport, utilities, and basic tools are prioritized.
2. **No pay-to-win.** Money cannot buy XP, levels, gate passes, or prestige.
3. **Transparent prices.** Same price for everyone in a region; no personalized pricing.
4. **Funded promises.** Every EC is backed by a reserve; no unbacked rewards.
5. **Curated, not infinite.** A smaller catalog that fulfills reliably beats a huge catalog that disappoints.
6. **Local first.** Regional partners and baskets; global only where logistics work.
7. **Dignity.** Redemption is private; no public "charity" labeling.

---

## 3. Catalog categories

| Category | Examples | Cap per learner/month | Notes |
|---|---|---|---|
| **Essentials** | Groceries, hygiene, transport passes, utility credit | Region-specific | Price caps; priority supply |
| **Tools & devices** | Laptop, phone, headphones, tools, instruments | 1 major item/quarter | Fraud checks; warranty |
| **Services** | Repairs, tutoring, home services, care | 2–4/month | Vetted providers |
| **Experiences** | Museums, events, travel, sports | 1–2/month | Local partners |
| **Learning upgrades** | Exam fees, mentorship, premium sims | Uncapped within reason | Directly supports learning |
| **AI services** | Tutoring, coding assistant credits, agent runs | Usage-based | Cost-controlled |
| **Status/cosmetic** | Profile themes, guild banners | Cosmetic only | No power |
| **Bounties** | Paid real-world work | N/A | Earned, not redeemed |

### 3.1 Restricted categories
- Alcohol, tobacco, gambling, weapons, adult content, crypto, cash equivalents, prescription drugs, and speculative financial products are prohibited.
- Health/medical items require regulatory review.
- Items that could undermine learning (e.g., exam answer services) are banned.

---

## 4. Pricing model

### 4.1 Price formula
```
EC_price = round( (fiat_cost + fulfillment_cost + partner_margin) / anchor_rate )
```
- `fiat_cost`: wholesale or service cost.
- `fulfillment_cost`: shipping, handling, payment, support.
- `partner_margin`: agreed margin.
- `anchor_rate`: regional EC value (default $0.05).

### 4.2 Price bands
| Band | EC range | Examples |
|---|---|---|
| Micro | 10–100 | Coffee, snack, transit ride |
| Small | 100–500 | Groceries, hygiene pack, book |
| Medium | 500–2,000 | Repair, course fee, headphones |
| Large | 2,000–10,000 | Phone, appliance, short trip |
| Major | 10,000–50,000 | Laptop, e-bike, rent subsidy |
| Transformational | 50,000+ | Vehicle, home repair, education fund |

### 4.3 Essentials pricing
- Essentials have a maximum EC price and a minimum supply commitment.
- Sponsors can subsidize essentials directly (e.g., 50% of a grocery basket).
- Essentials cannot be dynamically priced upward during scarcity; they go to waitlist.

### 4.4 Dynamic pricing (non-essentials)
- Allowed only for experiences and seasonal goods.
- Bounded to ±20% of the base price.
- Never personalized.
- Must be labeled "seasonal price."

### 4.5 No cash-out
- EC cannot be converted to cash, crypto, or stored-value instruments.
- Refunds return EC to the wallet, not cash (except where consumer law requires otherwise).
- Promotional EC may expire; earned EC does not.

---

## 5. Supply model

### 5.1 Partner types
| Type | Role | Settlement |
|---|---|---|
| **Merchant** | Sells goods | Fiat payout after fulfillment |
| **Service provider** | Delivers services | Fiat payout after completion |
| **Employer** | Posts bounties, sponsors employees | Fiat or EC |
| **Sponsor** | Funds pools | Fiat into treasury |
| **AI provider** | Supplies model/tool credits | Usage-based settlement |

### 5.2 Supply commitments
- Essentials: minimum monthly capacity per region.
- Tools: guaranteed stock for top 20 items.
- Services: vetted provider bench with SLA.
- Experiences: seasonal calendar.

### 5.3 Partner quality
- Rating ≥4.2 to remain in good standing.
- Fulfillment failure <2%.
- Dispute rate <3%.
- Fraud incidents trigger review.
- Removal for repeated violations.

---

## 6. Demand model

### 6.1 Demand drivers
- EC earned per learner.
- Redemption propensity (target 60–80%).
- Catalog attractiveness.
- Seasonality (holidays, back-to-school).
- Regional price index.

### 6.2 Demand forecasting
- Weekly forecast by category and region.
- Inputs: active learners, EC balances, level completions, historical redemption, seasonality, partner capacity.
- Buffer: 20% for essentials, 10% for others.

### 6.3 Waitlists
- If demand exceeds supply, learners join a waitlist with an estimated date.
- Essentials are prioritized by need, not by level.
- No auctioning of scarce goods.

---

## 7. Inflation and emission control

### 7.1 Key ratios
| Ratio | Definition | Target |
|---|---|---|
| **Emission utilization** | EC emitted / funded budget | 60–80% |
| **Redemption rate** | EC redeemed / EC earned | 60–80% |
| **Sink ratio** | EC removed / EC emitted | 55–75% |
| **Coverage** | Marketplace capacity / redemption demand | ≥1.1× |
| **Reserve ratio** | Fiat reserves / outstanding EC | ≥1.0× |
| **Velocity** | EC spent per average balance per month | 0.5–0.8 |

### 7.2 Levers
| Lever | When to use | Lag |
|---|---|---|
| Band bonuses | Boost motivation | Immediate |
| Domain multiplier | Shift supply to scarce skills | 1 quarter |
| Price adjustments | Restore coverage | 2–4 weeks |
| Catalog expansion | Absorb demand | 4–8 weeks |
| Sponsor campaigns | Add funded supply | 1–2 weeks |
| Savings incentives | Reduce velocity | 1–2 weeks |
| Promotional expiry | Remove stale promo EC | 30–90 days |

### 7.3 Rules
- Never reduce base level awards to fix inflation; use bonuses and prices first.
- Never change historical awards.
- Never create unbacked EC.
- Communicate changes 30 days ahead.
- Publish a quarterly economy report.

---

## 8. Unit economics

### 8.1 Per learner (Phase 3 target)
| Item | Monthly |
|---|---|
| EC earned | 400 |
| EC redeemed | 260 |
| Fiat value redeemed (@$0.05) | $13.00 |
| Platform fee (8%) | $1.04 |
| AI/content cost | $1.20 |
| Support/ops | $0.40 |
| **Contribution** | **-$0.56** |

Early phases are subsidy-funded. Path to positive contribution:
- Increase sponsor funding per verified outcome.
- Improve content reuse.
- Reduce AI cost per interaction.
- Grow marketplace margin on non-essentials.
- Add employer-paid bounties.

### 8.2 Per level
| Item | Cost |
|---|---|
| Authoring (amortized) | $80–150 |
| Review/QA | $20–40 |
| Media | $10–50 |
| Assessment maintenance | $10/level/year |
| Expected learners per level | 500+ |
| Cost per learner-level | <$0.50 |

### 8.3 Marketplace
- Platform fee: 8–12% (essentials 5%).
- Payment processing: 2–3%.
- Fulfillment support: 1–2%.
- Fraud reserve: 2–5%.
- Net margin: 0–4% (recycled into treasury).

---

## 9. Worked example: 10,000 active learners

| Metric | Value |
|---|---|
| EC earned/month | 4.08M |
| Fiat value (@$0.05) | $204,000 |
| Sponsor budget/month | $250,000 |
| Utilization | 82% |
| EC redeemed/month (65%) | 2.65M |
| Fiat redeemed | $132,500 |
| Marketplace capacity | 3.0M EC |
| Coverage | 1.13× |
| Merchant settlement | $121,900 |
| Platform fee (8%) | $10,600 |
| Outstanding EC liability | ~$71,500/month + prior |
| Reserve requirement | 100% of outstanding |

If fraud is 3% of awards, monthly loss is ~$6,100. Layered controls target <1% (~$2,000).

---

## 10. Regional baskets

| Region | Anchor | Coffee | Grocery basket | Transit month | Phone |
|---|---|---|---|---|---|
| US/EU | $0.05 | 60 EC | 1,200 EC | 1,600 EC | 20,000 EC |
| India | $0.02 | 20 EC | 400 EC | 300 EC | 8,000 EC |
| Brazil | $0.03 | 30 EC | 700 EC | 500 EC | 12,000 EC |
| Kenya | $0.02 | 20 EC | 350 EC | 250 EC | 7,000 EC |
| Philippines | $0.02 | 25 EC | 450 EC | 300 EC | 8,000 EC |

Baskets are reviewed quarterly with local partners.

---

## 11. Risks

| Risk | Impact | Mitigation |
|---|---|---|
| Sponsor concentration | Awards stop | Diversify; 6-month runway |
| Supply shortage | Trust damage | Commitments; waitlists; essentials priority |
| Inflation | EC devalues | Emission caps; reserves; price index |
| Fraud | Treasury loss | Integrity controls; clawbacks |
| Partner failure | Fulfillment gaps | Bench; SLAs; replacement |
| Regulatory | Fines/stop | Legal review; no cash-out; consumer protections |
| Overpromising | Churn/regulatory | Clear messaging; realistic baskets |

---

## 12. Acceptance criteria

- `FR-MKT-001`: Every price is public and versioned.
- `FR-MKT-002`: Essentials have caps and priority supply.
- `FR-MKT-003`: No personalized pricing.
- `FR-MKT-004`: EC cannot be cashed out.
- `FR-MKT-005`: Every redemption is backed by reserves.
- `FR-MKT-006`: Waitlists show estimated dates.
- `FR-MKT-007`: Partners meet quality thresholds.
- `FR-MKT-008`: Emission utilization and coverage are monitored weekly.
- `FR-MKT-009`: Economy changes are communicated 30 days ahead.
- `FR-MKT-010`: A quarterly economy report is published.
