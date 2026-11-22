# Content Operations

## 1. Purpose

Content ops keeps 50,000 level specs and their assessments accurate, current, accessible, and localized after launch. Publishing is a factory; operations is the maintenance yard.

**Core loop:** monitor → triage → fix → re-review → republish → measure.

---

## 2. Scope

| Asset | Count at full scale | Refresh cadence |
|---|---|---|
| Skills | 500 | 24 months |
| Levels | 50,000 | 18 months |
| Concepts | 250,000–750,000 | 18 months |
| Items | 1,000,000+ | Continuous rotation |
| Media assets | 100,000+ | 18 months |
| Rubrics | 50,000 | 12 months |
| Simulations | 2,000+ | 24 months |

---

## 3. Team structure

| Role | Ratio | Responsibility |
|---|---|---|
| Content ops lead | 1 per 5,000 levels | Queue health, SLAs |
| Domain editor | 1 per 2 domains | Accuracy, scope |
| Instructional designer | 1 per 4 domains | Alignment, load |
| Assessment specialist | 1 per 5 domains | Blueprints, items |
| Localization manager | 1 per region | Language, culture |
| Accessibility reviewer | Shared pool | WCAG, alt text |
| Media producer | Shared pool | Video, audio, 3D |
| Provenance auditor | 1 per 10,000 assets | Source, AI use |

---

## 4. Intake and triage

### 4.1 Signals
- Learner reports (error, confusion, outdated).
- Assessment anomalies (pass rate drift, item misfit).
- Expert review findings.
- Regulatory or safety changes.
- Partner/employer feedback.
- AI eval regressions.

### 4.2 Severity
| Sev | Example | SLA (ack → fix) |
|---|---|---|
| S1 | Safety error, harmful advice | 4h → 24h |
| S2 | Factual error affecting a gate | 24h → 5 days |
| S3 | Confusing content, broken media | 3 days → 15 days |
| S4 | Style, minor improvement | 10 days → next cycle |

### 4.3 Routing
```
Report → Classify → Assign owner → Fix draft
→ SME/ID review → Accessibility check → Publish
→ Notify reporter → Close
```

---

## 5. Review queues

| Queue | Owner | SLA |
|---|---|---|
| Safety | Trust & Safety + SME | 4h |
| Factual | Domain editor | 5 days |
| Assessment | Assessment specialist | 7 days |
| Localization | Localization manager | 10 days |
| Accessibility | Accessibility reviewer | 10 days |
| Provenance | Provenance auditor | 15 days |

Queues are visible, prioritized, and capacity-planned weekly.

---

## 6. Refresh program

### 6.1 Triggers
- Age >18 months.
- Pass rate outside 55–80%.
- Dispute rate >2%.
- New research or standard.
- Tool/version change (software, equipment).
- Learner clarity <4/5.

### 6.2 Process
1. Impact scan (usage, revenue, risk).
2. Assign reviewer.
3. Update content, items, rubric.
4. Pilot with 30–50 learners.
5. Republish with version bump.
6. Notify affected learners if the change is material.

### 6.3 Backward compatibility
- Historical attempts keep their version.
- Credentials are not retroactively invalidated by content refresh.
- Material change creates a new "current" badge requirement.

---

## 7. Localization

| Step | Owner | Output |
|---|---|---|
| 1. Glossary | Localization manager | Term base |
| 2. Translation | Translator | Draft |
| 3. In-country review | Local SME | Corrections |
| 4. Cultural adaptation | Editor | Examples, images |
| 5. Assessment fairness | Psychometrician | DIF check |
| 6. Layout QA | QA | Screenshots |
| 7. Publish | Ops | Localized pack |

Rules:
- No machine translation publishes without review.
- Idioms and examples are adapted, not transliterated.
- Currency, units, and law references are localized.
- Right-to-left and CJK layouts are tested.
- Voice/captions are re-recorded, not subtitled over English.

---

## 8. Media operations

- Asset naming and versioning standard.
- Master files retained.
- Derivative renditions (480p, 720p, 1080p, audio-only).
- Caption and transcript pipeline.
- Alt text and audio description.
- Rights and license tracking.
- Expiry and re-recording schedule.

---

## 9. Provenance and audit

Every asset stores:
- Author and reviewer IDs.
- AI involvement and model version.
- Prompt/template version.
- Source pack and citations.
- Review dates.
- Change history.

Quarterly audit samples 2% of assets for provenance completeness. Missing provenance blocks republication.

---

## 10. Metrics

| Metric | Target |
|---|---|
| S1 fix within 24h | 100% |
| S2 fix within 5 days | ≥95% |
| Refresh coverage (18 months) | ≥95% |
| Levels with complete provenance | 100% |
| Localized levels with in-country review | 100% |
| Accessibility defects at publish | 0 |
| Learner clarity | ≥4/5 |
| Content dispute rate | <2% |
| Reviewer throughput | Within 15% of plan |

---

## 11. Requirements

- `FR-OPS-101`: Every content report is triaged within SLA.
- `FR-OPS-102`: Every asset has an owner and version.
- `FR-OPS-103`: Safety issues have a 4-hour acknowledgment SLA.
- `FR-OPS-104`: Every refresh is piloted before publish.
- `FR-OPS-105`: Localization includes in-country review.
- `FR-OPS-106`: Every asset has complete provenance.
- `FR-OPS-107`: Accessibility review precedes publish.
- `FR-OPS-108`: Historical attempts retain their version.
- `FR-OPS-109`: Material changes notify affected learners.
- `FR-OPS-110`: Queues and SLAs are visible to the team.
- `NFR-OPS-101`: Publish pipeline <30 min.
- `NFR-OPS-102`: Asset retrieval <1 s.

---

## 12. Related

- [Content pipeline](../02-skills/05-content-pipeline.md)
- [Content generation](../10-data-ai/03-content-generation.md)
- [Support & moderation](02-support-moderation.md)
