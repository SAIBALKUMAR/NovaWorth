# Fraud & Abuse

## 1. Purpose

Fraud is an economic attack on the product. This document covers account takeover, collusion, marketplace fraud, insider risk, and the operating model for detection and response.

---

## 2. Fraud taxonomy

| Type | Example | Impact |
|---|---|---|
| Account takeover | Stolen credentials | Theft, data loss |
| Multi-accounting | One person, many accounts | EC farming |
| Collusion | Groups share answers/reviews | Credential devaluation |
| Marketplace fraud | Fake orders, self-redemption | Treasury loss |
| Refund abuse | Repeated false claims | Margin loss |
| Insider fraud | Score changes, payout approval | Systemic trust |
| Sponsor fraud | Fake cohorts | Funding misuse |
| AI fraud | Synthetic identities | Scale abuse |

---

## 3. Detection

### 3.1 Signals
- Device fingerprints.
- IP/geo anomalies.
- Behavioral patterns.
- Payment instruments.
- Graph analysis (shared devices, addresses, reviewers).
- Velocity (awards, redemptions).
- Content similarity.
- Biometric mismatch.
- Whistleblower reports.

### 3.2 Models
- Rules for known patterns.
- Anomaly detection for new patterns.
- Graph clustering for rings.
- Human review for high-value cases.

---

## 4. Response

```
DETECT → TRIAGE → CONTAIN → INVESTIGATE
→ DECIDE → RECOVER → LEARN
```

| Stage | Actions |
|---|---|
| Detect | Alert, score, queue |
| Triage | Severity, impact, false positive |
| Contain | Freeze account/order, block device |
| Investigate | Evidence, interviews, records |
| Decide | Warning, clawback, suspension, ban, legal |
| Recover | Reverse awards, refund, reserve |
| Learn | Update rules, models, controls |

---

## 5. Penalties

| Severity | Example | Response |
|---|---|---|
| Low | Minor rule breach | Warning |
| Medium | Answer sharing | Invalidation |
| High | Organized farming | Clawback + ban |
| Critical | Insider/fraud ring | Legal + ban |

All penalties are documented and appealable (except emergency containment).

---

## 6. Marketplace fraud

- Address verification.
- Device and payment checks.
- Merchant-employee screening.
- Proof of delivery.
- Serial numbers for high-value items.
- Refund pattern analysis.
- Escrow and reserves.

---

## 7. Insider risk

- Least privilege.
- Separation of duties.
- Four-eyes approvals.
- Access reviews.
- Audit logs.
- Whistleblower channel.
- Background checks.
- No single person can approve high-value payouts.

---

## 8. Metrics

| Metric | Target |
|---|---|
| Fraud loss | <1% of EC |
| Account takeover rate | <0.1% |
| False positive | <0.5% |
| Time to contain | <24h |
| Recovery rate | >80% |
| Insider incidents | 0/year |
| Ring detection | <2 weeks |

---

## 9. Requirements

- `FR-TNS-501`: Fraud signals are monitored continuously.
- `FR-TNS-502`: High-value orders require extra verification.
- `FR-TNS-503`: Collusion detection runs weekly.
- `FR-TNS-504`: Insider access is least-privilege.
- `FR-TNS-505`: Penalties are documented and appealable.
- `FR-TNS-506`: Clawbacks use compensating entries.
- `FR-TNS-507`: Fraud reserve covers losses.
- `FR-TNS-508`: Whistleblower channel exists.
- `FR-TNS-509`: Fraud metrics go to the council monthly.
- `FR-TNS-510`: Law enforcement is engaged where required.
- `NFR-TNS-501`: Containment <24h.
- `NFR-TNS-502`: Audit trail 100%.
