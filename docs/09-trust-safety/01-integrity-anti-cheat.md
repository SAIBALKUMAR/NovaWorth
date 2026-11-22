# Integrity & Anti-Cheat (Cross-Cutting)

## 1. Purpose

This is the cross-cutting integrity view. Detailed controls live in:
- [Economy integrity](../01-economy/07-integrity.md)
- [Assessment integrity & appeals](../05-assessment/05-integrity-rubrics-appeals.md)

Here we define the threat model, the control stack, and the operating rhythm.

---

## 2. Threat model

| Actor | Goal | Example |
|---|---|---|
| Individual cheater | Pass fast | Search answers, undeclared AI |
| Organized farm | Extract EC | Many accounts, scripts |
| Account seller | Profit | Sell level-60 accounts |
| Collusion ring | Mutual help | Answer sharing, peer-review rings |
| Content leaker | Status/profit | Publish item banks |
| Malicious insider | Money/ideology | Alter scores, approve awards |
| AI jailbreaker | Curiosity/profit | Manipulate tutor/grader |
| Merchant fraudster | Launder EC | Fake orders, self-redemption |
| Sponsor manipulator | Influence | Pressure scoring |

---

## 3. Control stack

```
Identity → Device → Content → Assessment → Behavior
→ Artifact → Post-hoc → Human review → Appeals
```

| Layer | Controls |
|---|---|
| Identity | Verified account, ID at high stakes, liveness, duplicate detection |
| Device | Binding, emulator/root detection, session anomalies |
| Content | Randomization, watermarking, canary items, rotation |
| Assessment | Adaptive items, oral defense, unique tasks |
| Behavior | Timing, paste, tab, typing, eye/head (consented) |
| Artifact | Provenance, version history, process evidence |
| Post-hoc | Collusion graph, anomaly detection, audits |
| Human | Review, panels, investigations |
| Appeals | New reviewer, evidence, timelines |

---

## 4. Integrity score

- 0–1 composite.
- Weighted: identity 0.15, device 0.15, behavior 0.20, leakage 0.15, provenance 0.20, human 0.15.
- Thresholds by level:
  - ≥0.85 normal.
  - 0.70–0.84 award + sample.
  - 0.50–0.69 hold + review.
  - <0.50 invalid.

---

## 5. Penalties

| Finding | First | Repeat | Severe |
|---|---|---|---|
| Undeclared AI (practice) | Warning + redo | Invalidation | Suspension |
| Answer sharing | Invalidation | 30-day suspension | Ban |
| Account sharing | Invalidation | 90-day suspension | Ban |
| Organized fraud | Clawback + ban | Legal | Legal |
| Merchant fraud | Termination | Legal | Legal |
| Insider misconduct | Termination + legal | Legal | Legal |

Proportionate, documented, appealable. Minors have a rehabilitative process.

---

## 6. Red teaming

- Quarterly internal.
- Annual external.
- Bug bounty for integrity issues.
- Adversarial content tests.
- Simulated fraud rings.
- Tabletop exercises.

---

## 7. Metrics

| Metric | Target |
|---|---|
| Integrity flag rate | <3% |
| Confirmed fraud | <1% of awards |
| False positive | <0.5% |
| Appeal rate | <2% |
| Reversal rate | <5% |
| Time to resolve | <7 days |
| Clawback recovery | >80% |
| Content leaks | 0/quarter |
| Insider incidents | 0/year |

---

## 8. Requirements

- `FR-TNS-101`: Every high-stakes attempt has an integrity score.
- `FR-TNS-102`: Scores below threshold cannot auto-award.
- `FR-TNS-103`: Every invalidation is appealable.
- `FR-TNS-104`: Clawbacks use compensating entries.
- `FR-TNS-105`: Credentials can be revoked.
- `FR-TNS-106`: Item banks are randomized and watermarked.
- `FR-TNS-107`: Collusion detection runs weekly.
- `FR-TNS-108`: Insider access is least-privilege.
- `FR-TNS-109`: Red teaming is quarterly.
- `FR-TNS-110`: Fraud metrics go to the Economy Council monthly.
- `NFR-TNS-101`: Integrity checks add <2 min.
- `NFR-TNS-102`: False positives resolved <24h.
