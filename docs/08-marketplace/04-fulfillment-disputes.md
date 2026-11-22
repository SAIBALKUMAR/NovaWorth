# Fulfillment & Disputes

## 1. Purpose

A redemption promise is only as good as its fulfillment. This document defines order lifecycle, SLAs, delivery, returns, disputes, and fraud handling.

---

## 2. Order lifecycle

```
CREATED → PAID (escrow) → CONFIRMED → PREPARING
→ SHIPPED/IN SERVICE → DELIVERED → CONFIRMED BY LEARNER
→ SETTLED
```

### 2.1 States
| State | Meaning |
|---|---|
| `created` | Item selected, not paid |
| `paid` | EC in escrow |
| `confirmed` | Partner accepted |
| `preparing` | Being prepared |
| `shipped` | In transit |
| `delivered` | At destination |
| `completed` | Learner confirmed |
| `cancelled` | Cancelled |
| `refunded` | EC returned |
| `disputed` | Under review |

---

## 3. SLAs

| Category | Confirm | Fulfill | Deliver |
|---|---|---|---|
| Essentials | <2h | <24h | <48h |
| Tools | <24h | <48h | <7 days |
| Services | <24h | Scheduled | <14 days |
| Experiences | <48h | Scheduled | Event date |
| Learning | Instant | Instant | Instant |

Breaches trigger escalation and possible compensation.

---

## 4. Delivery

- Address validation.
- Tracking.
- Delivery confirmation.
- Proof of delivery (photo/signature).
- Failed delivery retry.
- Lost package replacement.

---

## 5. Returns and refunds

| Situation | Resolution |
|---|---|
| Wrong item | Replace or refund |
| Damaged | Replace or refund |
| Not as described | Refund |
| Change of mind | Per partner policy |
| Service not delivered | Refund |
| Event cancelled | Refund |

- Refunds return EC to wallet.
- Cash refunds only where law requires.
- Return window: 14–30 days.
- Refund time: <5 days after approval.

---

## 6. Disputes

### 6.1 Flow
```
Learner opens dispute
→ Evidence requested
→ Partner responds
→ Mediation
→ Decision
→ Refund / release / partial
→ Appeal (once)
```

### 6.2 Evidence
- Order details.
- Photos.
- Tracking.
- Messages.
- Partner records.

### 6.3 Timelines
| Step | Target |
|---|---|
| Acknowledge | <24h |
| Partner response | <48h |
| Decision | <7 days |
| Refund | <5 days after decision |

### 6.4 Outcomes
- Full refund.
- Partial refund.
- Replacement.
- Release to partner.
- Account review.

---

## 7. Fraud

| Pattern | Control |
|---|---|
| Fake delivery | Proof of delivery, tracking |
| Empty box | Weight checks, photos |
| Return fraud | Inspection, serial numbers |
| Self-dealing | Merchant-employee checks |
| Collusion | Graph analysis |
| Account takeover | MFA, anomaly detection |

Fraud is investigated; penalties include suspension and clawback.

---

## 8. Support

- In-app help.
- Order status.
- Chat/email.
- Human escalation.
- Accessibility support.
- Language support.

---

## 9. Metrics

| Metric | Target |
|---|---|
| Fulfillment success | ≥97% |
| On-time delivery | ≥90% |
| Dispute rate | <3% |
| Dispute resolution | <7 days |
| Refund time | <5 days |
| CSAT | ≥4.3/5 |
| Fraud loss | <1% |

---

## 10. Requirements

- `FR-MKT-401`: Every order has a status timeline.
- `FR-MKT-402`: SLAs are enforced and monitored.
- `FR-MKT-403`: Delivery has proof.
- `FR-MKT-404`: Returns follow policy.
- `FR-MKT-405`: Disputes have a structured flow.
- `FR-MKT-406`: Refunds return EC.
- `FR-MKT-407`: Fraud is investigated.
- `FR-MKT-408`: Support is accessible.
- `FR-MKT-409`: Compensation exists for SLA breaches.
- `FR-MKT-410`: Disputes are appealable.
- `NFR-MKT-401`: Status updates <5 min.
- `NFR-MKT-402`: Dispute acknowledgment <24h.
