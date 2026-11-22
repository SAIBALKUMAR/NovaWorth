# Success Metrics

## 1. North star

**Verified Mastery Gains per Weekly Active Learner (VMG-WAL)**

> The number of quality-adjusted levels passed, with evidence, per weekly active learner per month.

Why this metric:
- It cannot be inflated by logins or video views.
- It rewards both reach and rigor.
- It balances volume (more learners) with depth (harder levels).
- It is auditable from the ledger and assessment records.

**Quality adjustment:** each pass is weighted by assessment confidence and integrity score. A pass with a failed integrity check counts as 0; a pass with strong evidence counts up to 1.2.

---

## 2. Metric tree

```
NORTH STAR: Verified Mastery Gains per WAL
├── Acquisition
│   ├── Qualified visits
│   ├── Onboarding starts
│   └── Onboarding completion rate
├── Activation
│   ├── First diagnostic completed
│   ├── First skill chosen
│   ├── First practice session
│   └── First level passed (D0–D7)
├── Engagement
│   ├── Weekly active learners
│   ├── Sessions per week
│   ├── Minutes of deliberate practice
│   ├── Modality mix (text/video/quiz/roleplay/sim)
│   └── Review queue adherence
├── Progression
│   ├── Levels passed per learner
│   ├── Band completion rate
│   ├── Time per level vs expected
│   ├── First-attempt pass rate
│   ├── Retry rate
│   └── Skill switching rate
├── Quality
│   ├── Assessment reliability
│   ├── Human-AI grader agreement
│   ├── Artifact rubric variance
│   ├── Appeal rate / reversal rate
│   └── Integrity flag rate
├── Economy
│   ├── EC earned per learner
│   ├── EC redeemed per learner
│   ├── Redemption coverage (supply/demand)
│   ├── Sponsor pool utilization
│   ├── Emission vs budget
│   ├── EC velocity and sink ratio
│   └── Fraud loss rate
├── Marketplace
│   ├── Redemption success rate
│   ├── Fulfillment time
│   ├── CSAT
│   ├── Dispute rate
│   └── Repeat redemption rate
├── Retention
│   ├── D1 / D7 / D30
│   ├── W4 / W12
│   ├── Season return rate
│   └── Post-level-30 retention
└── Trust & wellbeing
    ├── Learner-reported burnout
    ├── Sleep/study balance
    ├── Safety incidents
    ├── Accessibility defects
    └── Complaint volume
```

---

## 3. Targets by phase

### Phase 0 — Foundations
| Metric | Target |
|---|---|
| Expert interviews completed | 40 |
| Learner interviews completed | 60 |
| Skills with validated level-100 definition | 50 |
| Exemplar skills fully authored | 5 |
| Onboarding prototype task success | ≥80% |
| Scoring model simulation reviewed | 3 independent reviews |
| Legal/privacy risk register | Complete |

### Phase 1 — Alpha
| Metric | Target |
|---|---|
| Invited learners | 1,000 |
| Onboarding completion | ≥65% |
| First level passed within 7 days | ≥50% |
| D7 retention | ≥40% |
| D30 retention | ≥25% |
| First-attempt gate pass rate | 55–75% |
| Integrity flag rate | <3% |
| Appeal reversal rate | <5% |
| Accessibility blocking defects | 0 |
| NPS | ≥40 |

### Phase 2 — Beta
| Metric | Target |
|---|---|
| Registered learners | 25,000 |
| Weekly active learners | 10,000 |
| Skills with full 100 levels | 25 |
| Skills with levels 1–30 | 100 |
| D30 retention | ≥35% |
| Level 10 completion within 30 days | ≥30% |
| Mobile share of sessions | ≥50% |
| Role-play rubric-human agreement | ≥85% |
| Sponsor LOIs | 20 |
| Marketplace pilot redemption success | ≥95% |

### Phase 3 — Economy
| Metric | Target |
|---|---|
| Monthly active learners | 100,000 |
| Learners earning EC monthly | ≥30% |
| EC emission vs sponsor budget | ≤80% |
| Fraud loss rate | <1% of EC |
| Redemption success | ≥97% |
| Fulfillment within 7 days | ≥90% |
| Marketplace CSAT | ≥4.3/5 |
| Post-100 participants | ≥1,000 |
| Independent audit findings | 0 critical |
| Uptime | 99.9% |

### Phase 4 — Scale
| Metric | Target |
|---|---|
| Registered learners | 1M |
| Monthly active learners | 250,000 |
| Skills fully authored | 500 |
| Verifiable credentials issued | 500,000 |
| Vision Pro MAU | 10,000 |
| Guild participation | ≥25% of MAU |
| Localization | 10 languages |
| Cost per active learner | <$1.50/month |
| Employer verification partners | 100 |
| Accessibility audit | WCAG 2.2 AA verified |

### Phase 5 — Frontier
| Metric | Target |
|---|---|
| Livelihood pilot participants | 5,000 |
| Pilot households with EC covering ≥20% of essentials | ≥60% |
| Open protocol partners | 25 |
| Autonomous content acceptance rate | ≥70% with human review |
| Governance council seats | Learner-majority |
| Independent economic audit | Annual, public |

---

## 4. Guardrail metrics

These must not degrade as growth metrics improve.

| Guardrail | Threshold | Action if breached |
|---|---|---|
| First-attempt pass rate | 55–80% | Review difficulty calibration |
| Assessment reliability (Cronbach α) | ≥0.80 | Re-author items |
| Human-AI grader agreement | ≥90% | Retrain/recalibrate |
| Integrity flag rate | <3% | Investigate content or proctoring |
| Appeal reversal rate | <5% | Audit grader and rubric |
| Fraud loss | <1% of EC | Freeze payouts, investigate |
| Learner burnout (self-report) | <15% | Reduce pressure, adjust notifications |
| Study after midnight | <10% of sessions | Wellbeing nudges |
| Accessibility blocking defects | 0 | Stop-ship |
| Support first response | <4h | Staff up |
| AI harmful-output rate | <0.1% | Tighten guardrails |
| Data subject requests overdue | 0 | Compliance escalation |

---

## 5. Instrumentation requirements

### 5.1 Event standards
- Every event has `event_id`, `occurred_at`, `user_id` (pseudonymous), `session_id`, `platform`, `app_version`, `schema_version`.
- Learning events include `skill_id`, `level`, `content_id`, `content_version`, `modality`.
- Assessment events include `attempt_id`, `item_ids`, `score`, `rubric_version`, `integrity_score`.
- Economy events include `ledger_entry_id`, `award_id`, `amount`, `currency`, `rule_version`.
- No PII in event payloads; use pseudonymous IDs and a separate identity map.

### 5.2 Dashboards
1. Executive: north star, WAL, EC emission, fraud, redemption.
2. Learning: funnel by skill/level, difficulty, time, pass rate, retries.
3. Content: level health, item statistics, dispute rate, refresh backlog.
4. Economy: ledger balance, sponsor utilization, velocity, sink ratio, inflation.
5. Marketplace: catalog coverage, fulfillment, CSAT, disputes.
6. Trust: flags, appeals, reversals, safety incidents.
7. Accessibility: defect count, assistive-tech sessions, completion parity.
8. AI: cost per interaction, latency, refusal rate, eval scores, drift.

### 5.3 Experimentation
- All experiments have a hypothesis, primary metric, guardrails, sample size, and stop rule.
- No experiment may reduce assessment rigor without a quality guardrail.
- Economy changes are not A/B tested on users without a holdout and a fairness review.

---

## 6. Anti-metrics

We explicitly do not optimize for:

- Time in app.
- Videos watched.
- Streak length for its own sake.
- Number of badges issued.
- Number of courses published.
- Social followers.
- EC spent (we want healthy savings too).
- Pass rate (we want calibrated difficulty, not easy tests).

---

## 7. Reporting cadence

| Report | Audience | Cadence |
|---|---|---|
| North star & guardrails | All hands | Weekly |
| Learning quality | Product/Learning | Weekly |
| Economy health | Finance/Leadership | Monthly |
| Fraud & integrity | Trust & Safety | Weekly |
| Marketplace ops | Partnerships | Weekly |
| Accessibility | Product/QA | Monthly |
| Independent audit | Board/Public | Annual |
