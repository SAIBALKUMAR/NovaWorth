# Support & Moderation

## 1. Purpose

Support resolves learning, billing, marketplace, accessibility, and safety problems. Moderation keeps social spaces usable. Both are staffed by humans with clear authority, SLAs, and escalation paths.

**Rule:** a learner in crisis never waits in a general queue.

---

## 2. Support tiers

| Tier | Scope | Channel | Target response |
|---|---|---|---|
| T0 | Self-serve help center, AI assistant | In-app, web | Instant |
| T1 | Account, learning, wallet, orders | Chat, email | <4h |
| T2 | Assessment disputes, refunds, complex orders | Ticket | <24h |
| T3 | Safety, legal, integrity, data rights | Escalation | <1h (safety) |
| T4 | Executive/regulator/legal | Named owner | Per obligation |

---

## 3. Channels

- In-app help and chat.
- Email.
- Web form.
- Phone (accessibility and safety).
- SMS (safety only).
- Guardian line (minors).
- Partner support portal.

Every channel creates a ticket with a stable ID. No channel is a dead end.

---

## 4. SLAs

| Category | First response | Resolution |
|---|---|---|
| Safety (self-harm, abuse, threat) | <15 min | Immediate action |
| Account access | <1h | <24h |
| Payment/EC discrepancy | <4h | <3 days |
| Order issue | <4h | <7 days |
| Assessment dispute | <24h | <7 days |
| Accessibility blocker | <4h | <48h |
| Data rights request | <72h | Legal timeline |
| General learning | <8h | <3 days |

SLA clock pauses only while waiting on the learner.

---

## 5. AI assistant

### 5.1 Scope
- Answer product questions from the help center.
- Check order status.
- Explain wallet entries.
- Guide to the right form.
- Detect safety language and escalate.

### 5.2 Limits
- No account changes without verification.
- No refunds or awards.
- No legal, medical, or financial advice.
- No access to other learners' data.
- Always offers human handoff.

---

## 6. Moderation

### 6.1 Surfaces
- Guild discussions.
- Peer review comments.
- Public profiles.
- Mentorship sessions.
- Marketplace reviews.
- Metaverse spaces.

### 6.2 Actions
| Action | When | Appeal |
|---|---|---|
| Warning | First minor breach | Yes |
| Content removal | Policy violation | Yes |
| Mute | Repeated minor | Yes |
| Restrict | Serious | Yes |
| Ban | Severe/illegal | Yes (once) |
| Report to authorities | Illegal content | N/A |

### 6.3 Coverage
- 24/7 for public spaces.
- Human review of AI flags above threshold.
- Two-adult rule for minor-related cases.
- Moderator wellbeing support.

---

## 7. Safety escalation

```
Detect (AI flag, report, staff)
   │
[1] IMMEDIATE SAFETY CHECK
   │
[2] SURFACE LOCAL RESOURCES (crisis lines, emergency)
   │
[3] NOTIFY SAFETY LEAD (<15 min)
   │
[4] PRESERVE EVIDENCE (legal hold)
   │
[5] ACT (contact, restrict, report)
   │
[6] DOCUMENT + DEBRIEF
```

Rules:
- Never leave a crisis disclosure unanswered.
- Do not counsel; connect to professionals.
- Notify guardians unless the guardian is the risk.
- Preserve evidence before deletion requests.

---

## 8. Quality assurance

| Check | Frequency |
|---|---|
| Ticket QA sample | 5% weekly |
| CSAT review | Weekly |
| SLA breach review | Daily |
| Moderation calibration | Monthly |
| Safety drill | Quarterly |

---

## 9. Metrics

| Metric | Target |
|---|---|
| CSAT | ≥4.4/5 |
| First-contact resolution | ≥65% |
| Safety response | <15 min |
| SLA adherence | ≥95% |
| Reopen rate | <5% |
| Moderation precision | ≥90% |
| Moderation recall (harmful content) | ≥95% |
| Accessibility tickets resolved <48h | 100% |
| AI handoff satisfaction | ≥4/5 |

---

## 10. Requirements

- `FR-OPS-201`: Every contact creates a ticket.
- `FR-OPS-202`: Safety contacts are escalated within 15 minutes.
- `FR-OPS-203`: AI support discloses it is AI and offers human handoff.
- `FR-OPS-204`: Moderation actions are appealable.
- `FR-OPS-205`: Public spaces are moderated 24/7.
- `FR-OPS-206`: Minors' cases follow the two-adult rule.
- `FR-OPS-207`: Accessibility blockers are treated as S1.
- `FR-OPS-208`: Data rights requests route to the DPO.
- `FR-OPS-209`: Evidence is preserved on legal hold.
- `FR-OPS-210`: Support is available in every supported language.
- `NFR-OPS-201`: Chat first response <60 s (bot), <4h (human).
- `NFR-OPS-202`: Ticket system availability 99.9%.

---

## 11. Related

- [Child safety](../09-trust-safety/04-child-safety.md)
- [Fraud & abuse](../09-trust-safety/05-fraud-abuse.md)
- [Incident response](04-incident-response.md)
