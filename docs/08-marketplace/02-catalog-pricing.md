# Catalog & Pricing

## 1. Purpose

This document defines how items enter the catalog, how prices are set, and how the catalog stays trustworthy across regions.

---

## 2. Catalog governance

### 2.1 Item lifecycle
```
Proposal → Vetting → Pricing → Legal/safety → Pilot
→ Publish → Monitor → Refresh/Retire
```

### 2.2 Vetting
- Partner legitimacy.
- Product safety.
- Regulatory compliance.
- Fulfillment capacity.
- Return policy.
- Quality rating.

### 2.3 Item metadata
- Title, description, images.
- Category, tags.
- Region availability.
- Price and fees.
- ETA.
- Warranty/returns.
- Restrictions.
- Carbon/sustainability notes (optional).

---

## 3. Pricing formula

```
EC_price = round( (fiat_cost + fulfillment_cost + partner_margin) / anchor_rate )
```

| Component | Notes |
|---|---|
| `fiat_cost` | Wholesale/service cost |
| `fulfillment_cost` | Shipping, handling, payment, support |
| `partner_margin` | Agreed margin |
| `anchor_rate` | Regional EC value |

### 3.1 Example
A grocery basket costs $40 wholesale, $5 fulfillment, 10% margin, anchor $0.05:
```
(40 + 5 + 4) / 0.05 = 980 EC
```

### 3.2 Price bands
| Band | EC | Examples |
|---|---|---|
| Micro | 10–100 | Coffee, snack, transit |
| Small | 100–500 | Groceries, hygiene, book |
| Medium | 500–2,000 | Repair, headphones |
| Large | 2,000–10,000 | Phone, appliance, trip |
| Major | 10,000–50,000 | Laptop, e-bike, rent |
| Transformational | 50,000+ | Vehicle, home repair |

---

## 4. Essentials

- Maximum price cap.
- Minimum supply commitment.
- Priority fulfillment.
- Sponsor subsidies allowed.
- No dynamic upward pricing.
- Waitlist if scarce.
- Local basket definition.

### 4.1 Essential baskets
| Region | Basket | Target EC |
|---|---|---|
| US/EU | 2 weeks groceries | 1,200 |
| India | 2 weeks staples | 400 |
| Brazil | 2 weeks groceries | 700 |
| Kenya | 2 weeks staples | 350 |
| Philippines | 2 weeks groceries | 450 |

---

## 5. Dynamic pricing

- Only for experiences and seasonal goods.
- Bounded ±20%.
- Never personalized.
- Labeled "seasonal price."
- Not for essentials.
- Logged and auditable.

---

## 6. Discounts and subsidies

| Type | Who pays | Rules |
|---|---|---|
| Sponsor subsidy | Sponsor pool | Clearly labeled |
| Partner discount | Partner | Passed to learner |
| Promotional EC | Platform | Expiry allowed |
| Bulk discount | Partner | Transparent |

No fake "was" prices. No countdown timers.

---

## 7. Regional pricing

- Anchor rate by region.
- Local costs and taxes.
- Currency of account is EC.
- No arbitrage: region-locked fulfillment.
- Price parity reviews quarterly.

---

## 8. Catalog quality

| Metric | Target |
|---|---|
| Item rating | ≥4.2 |
| Fulfillment failure | <2% |
| Return rate | <5% |
| Dispute rate | <3% |
| Description accuracy | ≥95% |
| Stock accuracy | ≥95% |

Items below thresholds are reviewed and may be removed.

---

## 9. Requirements

- `FR-MKT-201`: Every item has complete metadata.
- `FR-MKT-202`: Prices follow the formula and are versioned.
- `FR-MKT-203`: Essentials have caps and commitments.
- `FR-MKT-204`: Dynamic pricing is bounded and labeled.
- `FR-MKT-205`: No personalized pricing.
- `FR-MKT-206`: Subsidies are labeled.
- `FR-MKT-207`: Regional prices are reviewed quarterly.
- `FR-MKT-208`: Items below quality thresholds are removed.
- `FR-MKT-209`: Catalog changes are logged.
- `FR-MKT-210`: Restricted items are blocked.
- `NFR-MKT-201`: Price calculation <100 ms.
- `NFR-MKT-202`: Catalog sync <5 min.
