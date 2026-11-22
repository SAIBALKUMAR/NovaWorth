# Core UX Flows

## 1. Purpose

This document specifies the key journeys end to end: first session, daily learning, level-up, failure, redemption, skill switching, post-100, sponsor, and guardian. Each flow includes states, edge cases, and acceptance criteria.

---

## 2. Flow A — First session

```
Landing → Value prop → Start
→ Consent → Identity → Work → Life → Interests
→ Diagnostic → Profile → Recommendations
→ Choose skill → First mission → Feedback
→ "Next: level 1 gate"
```

### 2.1 States
| State | Behavior |
|---|---|
| Drop-off | Save and resume |
| Skip diagnostic | Sampler |
| No experience | Life/interests only |
| Low bandwidth | Text-only |
| Minor | Guardian consent |
| Disagree | Browse all skills |

### 2.2 Acceptance
- First mission completed in ≤10 minutes.
- Learner can explain what they learned.
- One clear next step.

---

## 3. Flow B — Daily learning

```
Today → Next best action
→ Session (review → learn → practice → reflect)
→ Summary → Next action
```

### 3.1 Variants
- 2-minute review.
- 25-minute standard.
- 60-minute deep.
- Offline pack.
- Audio-only.

### 3.2 States
- Fatigue → suggest rest.
- Frustration → scaffold.
- Boredom → harder.
- Missed day → no penalty.

### 3.3 Acceptance
- Session always ends with a win or a clear next step.
- No dead ends.
- No forced video.

---

## 4. Flow C — Level up

```
Level complete → Readiness check
→ Gate brief → Assessment → Score
→ Pass → Award → Credential → Next level
```

### 4.1 Pass screen
- What you proved.
- XP + EC with breakdown.
- Credential.
- Next level preview.
- Share (private by default).

### 4.2 States
- Borderline → oral recheck.
- Integrity hold → review.
- Pool exhausted → queued award.
- Panel → scheduled.

### 4.3 Acceptance
- Award is atomic.
- Breakdown is visible.
- Next step is one tap.

---

## 5. Flow D — Failure and retry

```
Gate fail → Specific feedback
→ Gap analysis → Practice plan
→ Retry window → Retake
```

### 5.1 Copy
- "Not yet" not "failed."
- Show what was missing.
- Show the shortest path.
- Offer human help.

### 5.2 States
- 3 fails → 7-day cooldown.
- 4+ → remediation plan.
- Appeal → review.

### 5.3 Acceptance
- No EC on fail.
- Partial XP for effort.
- Retry plan is specific.
- No shame language.

---

## 6. Flow E — Redemption

```
Wallet → Redeem → Category
→ Item → Confirm → Escrow
→ Fulfillment → Confirm receipt
→ Rate → Done
```

### 6.1 States
- Insufficient EC → savings goal.
- Out of stock → waitlist.
- Address needed → form.
- Minor → guardian approval.
- Dispute → support.

### 6.2 Acceptance
- Price and fees clear before confirm.
- Escrow protects learner.
- Refund path visible.
- No dark patterns.

---

## 7. Flow F — Skill switching

```
Skills → Catalog → Fit check
→ Preview level 1 → Switch
→ Keep old progress → New plan
```

### 7.1 Rules
- No penalty.
- Old XP/EC retained.
- New recommendation.
- Optional sampler.
- Return anytime.

---

## 8. Flow G — Post-100

```
Level 100 → Prestige intro
→ Choose frontier
→ Challenge → Evidence → Panel
→ Prestige award → Next
```

### 8.1 Paths
- Live-fire.
- Constellation.
- Bounty.
- Research.
- Mentoring.
- Teaching.

### 8.2 Acceptance
- Clear requirements.
- Diminishing EC explained.
- Status and access shown.
- No infinite farming.

---

## 9. Flow H — Sponsor cohort

```
Invite → Consent → Onboarding
→ Cohort dashboard → Learn
→ Aggregate outcomes → Audit export
```

### 9.1 Rules
- Individual data not shared without consent.
- Aggregate only by default.
- Sponsor cannot influence scoring.
- Learner can opt out of sharing.

---

## 10. Flow I — Guardian

```
Guardian creates account → Consent
→ Child profile → Limits
→ Progress view → Approval for spend
```

### 10.1 Rules
- No peer transfers.
- Spending limits.
- Content controls.
- Privacy.
- Guardian sees pass/fail, not raw responses.

---

## 11. Flow J — Credential verification

```
Public link → Credential page
→ Skill, level, date, evidence hash
→ Integrity tier → Verify
```

### 11.1 Rules
- No PII beyond consent.
- Revocation visible.
- Portable.
- Employer-friendly.

---

## 12. Requirements

- `FR-UX-101`: Every flow has defined states.
- `FR-UX-102`: No flow ends in a dead end.
- `FR-UX-103`: Failure flows are actionable.
- `FR-UX-104`: Redemption is transparent.
- `FR-UX-105`: Skill switching is penalty-free.
- `FR-UX-106`: Post-100 paths are clear.
- `FR-UX-107`: Sponsor views are aggregate by default.
- `FR-UX-108`: Guardian controls are enforced.
- `FR-UX-109`: Credentials are publicly verifiable.
- `FR-UX-110`: All flows are accessible.
- `NFR-UX-101`: Flow completion <3 taps for core actions.
- `NFR-UX-102`: Error recovery <1 tap.
