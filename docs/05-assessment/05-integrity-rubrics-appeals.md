# Integrity, Rubrics & Appeals

## 1. Purpose

This document connects three things that determine trust:
1. **Integrity** — is the evidence real?
2. **Rubrics** — is the scoring fair and consistent?
3. **Appeals** — can a learner challenge a wrong decision?

A failure in any one collapses confidence in the credential and the currency.

---

## 2. Integrity controls

### 2.1 Layers
| Layer | Controls |
|---|---|
| Identity | Verified account, ID at high stakes, liveness |
| Device | Binding, emulator/root detection, session anomalies |
| Content | Randomization, watermarking, canary items, rotation |
| Assessment | Adaptive items, oral defense, unique tasks |
| Behavior | Timing, paste, tab, typing, eye/head (consented) |
| Artifact | Provenance, version history, process evidence |
| Post-hoc | Collusion graph, anomaly detection, audits |
| Human | Review, panels, investigations |

### 2.2 Integrity score
- 0–1 composite.
- Thresholds by level.
- <0.5 invalid; 0.5–0.69 hold; 0.7–0.84 sample; ≥0.85 normal.

### 2.3 Penalties
| Finding | Response |
|---|---|
| Undeclared AI in practice | Warning + redo |
| Answer sharing | Level invalidation |
| Account sharing | Suspension |
| Organized fraud | Clawback + ban |
| Insider misconduct | Termination + legal |

Proportionate, documented, appealable.

---

## 3. Rubric quality

### 3.1 Design
- 4 criteria.
- 4 performance levels.
- Observable descriptors.
- Anchor examples.
- Critical criteria identified.
- Safety floor where needed.

### 3.2 Calibration
- Grader training.
- Anchor scoring.
- Double grading sample.
- κ ≥0.70.
- Drift monitoring.
- Recalibration quarterly.

### 3.3 Common failures
| Failure | Fix |
|---|---|
| Vague descriptors | Use observable behavior |
| Double-counting | Separate criteria |
| Halo effect | Score criteria independently |
| Leniency/severity | Anchor calibration |
| Cultural bias | Diverse anchors, review |
| AI drift | Re-anchor, audit |

---

## 4. Appeals

### 4.1 Grounds
- Scoring error.
- Rubric misapplication.
- Technical failure.
- Integrity false positive.
- Accommodation not applied.
- New evidence.

### 4.2 Process
```
Appeal filed (14 days)
      │
[1] ACKNOWLEDGE (24h)
      │
[2] EVIDENCE PACKET
      │
[3] NEW REVIEWER
      │
[4] DECISION (7 days; 21 complex)
      │
[5] OUTCOME
   ├── Upheld → award restored
   └── Rejected → rationale + next steps
```

### 4.3 Rules
- Reviewer is not the original grader.
- Learner sees the evidence used.
- No retaliation.
- Second appeal for level 80+ or bans.
- Decisions are logged and audited.

---

## 5. Audit

### 5.1 Internal
- Monthly sample of 5% of gates.
- Weekly integrity review.
- Quarterly rubric calibration.
- Quarterly fairness analysis.

### 5.2 External
- Annual independent audit.
- Public summary.
- Remediation plan.

### 5.3 Metrics
| Metric | Target |
|---|---|
| Integrity flag rate | <3% |
| Confirmed fraud | <1% |
| False positive | <0.5% |
| Appeal rate | <2% |
| Reversal rate | <5% |
| Grader agreement | κ ≥0.70 |
| AI-human agreement | ≥90% |
| Time to appeal decision | <7 days |

---

## 6. Learner communication

- Explain the decision.
- Show the rubric.
- Show the evidence.
- Offer next steps.
- Offer human help.
- No shaming language.

---

## 7. Requirements

- `FR-ASM-601`: Every gate has an integrity score.
- `FR-ASM-602`: Every rubric has anchors.
- `FR-ASM-603`: Every grader is calibrated.
- `FR-ASM-604`: Every appeal gets a new reviewer.
- `FR-ASM-605`: Every appeal decision is documented.
- `FR-ASM-606`: Every penalty is appealable.
- `FR-ASM-607`: Audit samples run monthly.
- `FR-ASM-608`: Fairness analysis runs quarterly.
- `FR-ASM-609`: Learners see evidence used.
- `FR-ASM-610`: No retaliation for appeals.
- `NFR-ASM-601`: Appeal acknowledgment <24h.
- `NFR-ASM-602`: Appeal decision <7 days.
