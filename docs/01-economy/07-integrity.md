# Integrity & Anti-Cheat

## 1. Purpose

The economy is only as valuable as the credibility of the proof behind it. If learners can fake levels, the currency becomes worthless and the mission fails.

This document defines the threat model, controls, penalties, and audit mechanisms. It covers both **assessment integrity** (did the learner actually do it?) and **economic integrity** (is the award legitimate?).

---

## 2. Threat model

| Threat actor | Motivation | Example |
|---|---|---|
| **Individual cheater** | Pass quickly | Search answers, use AI, share account |
| **Organized farm** | Extract EC | Many accounts, scripted attempts, collusion |
| **Account seller** | Profit | Sell high-level accounts |
| **Collusion ring** | Mutual help | Group answers, peer-review exchange |
| **Content leaker** | Status/profit | Publish item banks |
| **Malicious insider** | Money/ideology | Alter scores, approve fraudulent awards |
| **AI jailbreaker** | Curiosity/profit | Manipulate tutor/grader |
| **Merchant fraudster** | Launder EC | Fake orders, self-redemption |
| **Sponsor manipulator** | Influence | Pressure scoring or recommendations |

---

## 3. Defense layers

```
Layer 1: Identity & account
Layer 2: Device & session
Layer 3: Content & item security
Layer 4: Assessment design
Layer 5: Behavioral signals
Layer 6: Artifact provenance
Layer 7: Post-hoc analytics
Layer 8: Human review & appeals
```

No single layer is trusted. The goal is to make fraud expensive, detectable, and reversible.

---

## 4. Layer details

### 4.1 Identity and account
- Verified email/phone; optional government ID for high-stakes gates.
- Device binding for high-level attempts.
- Liveness check at level 60+ and for payouts.
- One account per person; duplicate detection via device, payment, and behavioral similarity.
- Guardian accounts for minors.

### 4.2 Device and session
- Session anomaly detection: new device, impossible travel, emulator, rooted device, screen sharing.
- Concurrent session limits.
- High-stakes attempts require a clean session.
- Screen-share and remote-control detection where legally permitted.

### 4.3 Content and item security
- Large item banks with randomized selection.
- Per-attempt variants and parameterized items.
- Watermarking of PDFs/videos with learner ID.
- Canary items to detect leaks.
- Item rotation and retirement.
- No full answer keys in client apps.

### 4.4 Assessment design
- Adaptive testing reduces answer sharing.
- Oral defense and "explain your work" steps.
- Practical tasks with unique inputs.
- Process evidence (commits, drafts, recordings).
- In-person/proctored gates at high levels.
- Panel review at 90–100.

### 4.5 Behavioral signals
- Response time patterns (too fast, too uniform).
- Answer-change patterns.
- Typing cadence and paste detection.
- Navigation anomalies (tab switching, second monitor).
- Eye/head movement where permitted and consented.
- Consistency across sessions.

### 4.6 Artifact provenance
- Version history and timestamps.
- Code commit graphs and authorship.
- Screen recordings for practical tasks.
- Draft-to-final progression.
- AI-use declaration and detection.
- Reproducibility checks.

### 4.7 Post-hoc analytics
- Collusion graph: who answers similarly, who reviews whom.
- Score distributions by cohort and device.
- Award-to-redemption velocity anomalies.
- Merchant self-dealing detection.
- Insider access anomaly detection.

### 4.8 Human review
- High-stakes and flagged cases.
- Appeals.
- Panel decisions.
- Fraud investigations.

---

## 5. Proctoring tiers

| Tier | Used for | Controls |
|---|---|---|
| **Open** | Levels 1–20 | Randomized items, time limits, basic analytics |
| **Monitored** | Levels 21–50 | Webcam/screen check, behavioral signals, ID optional |
| **Proctored** | Levels 51–80 | Live or recorded proctor, ID verification, secure browser |
| **Panel** | Levels 81–100 | Human panel, oral defense, artifact review, in-person option |
| **Frontier** | Prestige | Expert panel, real-world verification, reproducibility |

Learners can always choose a higher-integrity option (e.g., in-person) to strengthen their credential.

---

## 6. Integrity score

Each attempt gets an `integrity_score` from 0 to 1.

| Signal | Weight |
|---|---|
| Identity confidence | 0.15 |
| Device/session cleanliness | 0.15 |
| Behavioral consistency | 0.20 |
| Content leakage match | 0.15 |
| Artifact provenance | 0.20 |
| Human review | 0.15 |

### 6.1 Thresholds
| Score | Action |
|---|---|
| ≥0.85 | Normal award |
| 0.70–0.84 | Award, flagged for sampling |
| 0.50–0.69 | Award held; review required |
| <0.50 | Invalid; no award; investigation |

Thresholds vary by level and stakes.

---

## 7. Penalties

| Finding | First offense | Repeat | Severe |
|---|---|---|---|
| Minor rule breach (e.g., undeclared AI use in practice) | Warning + redo | Level invalidation | Suspension |
| Answer sharing | Level invalidation | 30-day suspension | Ban |
| Account sharing | Level invalidation | 90-day suspension | Ban |
| Organized fraud | Clawback + ban | Legal action | Legal action |
| Merchant fraud | Contract termination | Legal action | Legal action |
| Insider misconduct | Termination + legal | Legal action | Legal action |

Penalties are proportionate, documented, and appealable. Minors have a separate, rehabilitative process.

---

## 8. Clawbacks

- Triggered by invalidated assessment, fraud confirmation, or duplicate award.
- Reverses EC first from available balance, then fraud reserve, then future awards.
- XP is reversed with a compensating entry; level status may be revoked.
- Credentials are revoked and published in a revocation list.
- The learner is notified with reasons and appeal instructions.
- All clawbacks are logged and audited.

---

## 9. Appeals

1. Learner submits an appeal within 14 days.
2. Automated evidence packet is assembled.
3. A human reviewer (not the original grader) reviews.
4. Decision within 7 days; complex cases within 21 days.
5. If upheld, award is restored with a note.
6. If rejected, the learner receives a clear explanation and next steps.
7. A second appeal is allowed for level 80+ or bans.

---

## 10. Insider and sponsor controls

- Separation of duties: content, scoring, and payouts are separate teams.
- No single person can approve a high-value award or payout.
- Sponsor employees cannot access individual learner scores without consent.
- Conflict-of-interest declarations for examiners and council members.
- Access reviews quarterly.
- Whistleblower channel.

---

## 11. Red teaming

- Quarterly internal red team.
- Annual external red team.
- Bug bounty for integrity issues.
- Adversarial content tests (prompt injection, jailbreaks).
- Simulated fraud rings.
- Tabletop incident exercises.

---

## 12. Metrics

| Metric | Target |
|---|---|
| Integrity flag rate | <3% of attempts |
| Confirmed fraud rate | <1% of awards |
| False positive rate | <0.5% |
| Appeal rate | <2% |
| Appeal reversal rate | <5% |
| Time to resolve appeal | <7 days |
| Clawback recovery rate | >80% |
| Content leak incidents | 0 per quarter |
| Insider incidents | 0 per year |

---

## 13. Requirements

- `FR-TNS-001`: Every high-stakes attempt has an integrity score.
- `FR-TNS-002`: Scores below threshold cannot auto-award.
- `FR-TNS-003`: Every invalidation is appealable.
- `FR-TNS-004`: Clawbacks use compensating ledger entries.
- `FR-TNS-005`: Credentials can be revoked and checked publicly.
- `FR-TNS-006`: Item banks are randomized and watermarked.
- `FR-TNS-007`: Collusion detection runs weekly.
- `FR-TNS-008`: Insider access is least-privilege and audited.
- `FR-TNS-009`: Red teaming occurs quarterly.
- `FR-TNS-010`: Fraud metrics are reported to the Economy Council monthly.
- `NFR-TNS-001`: Integrity checks add <2 minutes to a gate.
- `NFR-TNS-002`: False positives do not block legitimate learners for >24 hours without review.
