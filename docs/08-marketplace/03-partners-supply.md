# Partners & Supply

## 1. Purpose

Supply is the hard part of the marketplace. This document defines partner types, onboarding, contracts, settlement, quality, and capacity planning.

---

## 2. Partner types

| Type | Role | Settlement |
|---|---|---|
| Merchant | Goods | Fiat after fulfillment |
| Service provider | Services | Fiat after completion |
| Employer | Bounties, sponsorship | Fiat/EC |
| Sponsor | Funds pools | Fiat to treasury |
| AI provider | Model/tool credits | Usage-based |
| Logistics | Delivery | Per shipment |
| Payment partner | Payouts | Per transaction |

---

## 3. Onboarding

```
Application → Vetting → Contract → Catalog setup
→ Pilot → Review → Scale
```

### 3.1 Vetting
- Legal entity and tax ID.
- Licenses (where required).
- Insurance.
- References.
- Financial stability.
- Quality history.
- Sanctions/PEP screening.
- Data protection agreement.

### 3.2 Contract terms
- Pricing and margin.
- Settlement schedule.
- Fulfillment SLA.
- Returns/refunds.
- Quality thresholds.
- Fraud liability.
- Data use.
- Termination.
- Dispute resolution.

---

## 4. Settlement

| Event | Timing |
|---|---|
| Order fulfilled | Merchant payable created |
| Return window closes | Payable released |
| Payout run | Weekly |
| Dispute | Hold until resolved |
| Chargeback | Deducted/claimed |

- Settlement in local fiat.
- No EC cash-out by partners.
- Tax reporting.
- Statements and reconciliation.

---

## 5. Quality management

| Metric | Threshold |
|---|---|
| Rating | ≥4.2 |
| Fulfillment failure | <2% |
| Dispute rate | <3% |
| Return rate | <5% |
| Response time | <24h |
| Stock accuracy | ≥95% |

### 5.1 Remediation
1. Warning + improvement plan.
2. Reduced visibility.
3. Suspension.
4. Termination.

---

## 6. Capacity planning

```
Capacity_EC = Σ(partner_category_capacity_fiat) / anchor_rate
```

- Commitments for essentials.
- Forecast by category/region.
- 20% buffer for essentials.
- Waitlists when demand exceeds capacity.
- Onboarding pipeline for gaps.

---

## 7. Sponsor partners

### 7.1 Pool setup
- Purpose and eligibility.
- Budget and duration.
- Region/skill/cohort.
- Reporting requirements.
- Unused-fund policy.
- Branding rules.

### 7.2 Rules
- No influence on scoring.
- No individual learner data without consent.
- Aggregate reporting.
- Audit rights.
- Public disclosure of sponsorship.

---

## 8. AI service partners

- Usage-based credits.
- Cost caps.
- Rate limits.
- Data processing terms.
- Model version pinning.
- Fallback providers.
- No training on learner data without consent.

---

## 9. Requirements

- `FR-MKT-301`: Every partner is vetted and contracted.
- `FR-MKT-302`: Settlement is on a fixed schedule.
- `FR-MKT-303`: Quality thresholds are enforced.
- `FR-MKT-304`: Essentials have capacity commitments.
- `FR-MKT-305`: Sponsor pools have clear rules.
- `FR-MKT-306`: Sponsors cannot influence scoring.
- `FR-MKT-307`: AI partners have data terms.
- `FR-MKT-308`: Capacity is forecast weekly.
- `FR-MKT-309`: Partner data is auditable.
- `FR-MKT-310`: Termination protects learners.
- `NFR-MKT-301`: Settlement accuracy 100%.
- `NFR-MKT-302`: Payout within 7 days.
