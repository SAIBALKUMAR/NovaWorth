# Assessment Authoring Standard

## 1. Purpose

Assessment is the product's quality moat. If the proof is weak, the points are weak, and the economy collapses. This standard defines how assessments are designed, calibrated, secured, and reviewed.

**Principle:** assess what the learner can *do*, not what they can *recognize*.

---

## 2. Evidence hierarchy

| Tier | Evidence | Weight at level 30 | Weight at level 90 |
|---|---|---|---|
| 1 | Recall / recognition | 20% | 0% |
| 2 | Application to familiar problems | 40% | 10% |
| 3 | Transfer to novel problems | 25% | 30% |
| 4 | Artifact / performance | 15% | 35% |
| 5 | Real-world verified impact | 0% | 25% |

Higher levels shift weight upward. A level 90 credential cannot be earned by quizzes alone.

---

## 3. Item types

| Type | Use | Auto-grade | Notes |
|---|---|---|---|
| Multiple choice | Recall, discrimination | Yes | 4 options; plausible distractors |
| Multi-select | Classification | Yes | Partial credit |
| Numeric | Calculation | Yes | Tolerance bands |
| Short answer | Explanation | AI + rubric | Human sample |
| Essay | Argument | AI + rubric | Human sample |
| Code | Programming | Tests + review | Sandboxed |
| Spreadsheet | Modeling | Formula checks | Versioned |
| Diagram | Systems | Rubric | Image + text |
| Case analysis | Judgment | Rubric | Structured response |
| Oral | Explanation/transfer | Human/AI | Recorded |
| Role-play | Interpersonal | Rubric + AI | Scenario engine |
| Simulation | Operational | Logs + outcome | Deterministic |
| Artifact | Creation | Rubric + panel | Provenance |
| Performance | Physical | Video + rubric | Safety review |

### 3.1 Item quality rules
- One concept per item.
- No trick wording.
- Distractors reflect real misconceptions.
- No "all of the above" except where justified.
- No cultural references that are not taught.
- No item depends on a single device.
- Every item has a source and a difficulty estimate.

---

## 4. Blueprint

### 4.1 Knowledge blueprint
| Band | Items | Time | Pass |
|---|---|---|---|
| 1–10 | 10–15 | 15 min | 80% |
| 11–20 | 15–20 | 25 min | 80% |
| 21–30 | 20–25 | 35 min | 80% |
| 31–40 | 20–30 | 45 min | 80% |
| 41–50 | 25–30 | 60 min | 80% |
| 51–60 | 25–35 | 75 min | 80% |
| 61–70 | 30–40 | 90 min | 80% |
| 71–80 | 30–40 | 90 min | 80% |
| 81–90 | 20–30 + oral | 120 min | 80% + oral |
| 91–100 | Panel | Multi-day | Panel |

### 4.2 Practical blueprint
- 1 primary task + 1 transfer task.
- 4 rubric criteria.
- 4 performance levels per criterion.
- At least one criterion tests judgment/edge cases.
- At least one criterion tests communication or documentation.

### 4.3 Item bank
- Minimum 20 items per level; 40 preferred.
- At least 3 variants per high-stakes item.
- Rotate 30% per cohort.
- Retire items with exposure or drift.

---

## 5. Rubric design

### 5.1 Structure
| Criterion | 1 (Not yet) | 2 (Developing) | 3 (Proficient) | 4 (Exemplary) |
|---|---|---|---|---|
| Correctness | Major errors | Some errors | Accurate | Flawless, robust |
| Method | No method | Partial | Sound | Elegant/efficient |
| Judgment | Ignores context | Basic | Considers tradeoffs | Anticipates edge cases |
| Communication | Unclear | Understandable | Clear | Compelling and precise |

### 5.2 Rules
- Criteria are independent; no double-counting.
- Descriptors are observable, not adjectives.
- Weights are hidden from learners to reduce gaming, but criteria are public.
- Critical criteria (safety, correctness) have a hard floor of 2.
- Rubrics are versioned and calibrated with anchor examples.

### 5.3 Anchor examples
Every rubric has 2–3 scored exemplars per level. These are the reference for human and AI graders. Anchors are reviewed quarterly for drift.

---

## 6. Reliability

| Metric | Target | Method |
|---|---|---|
| Internal consistency | Cronbach α ≥0.80 | Item analysis |
| Inter-rater reliability | Cohen's κ ≥0.70 | Double grading sample |
| Test-retest | r ≥0.80 | Retake sample |
| AI-human agreement | ≥90% within 1 point | Audit sample |
| Item discrimination | ≥0.30 | Point-biserial |
| Item difficulty | 0.30–0.85 | Proportion correct |

### 6.1 Calibration
- Pilot with 30–50 learners per item set.
- Use IRT (2PL or 3PL) for high-stakes items.
- Remove or rewrite items with poor discrimination.
- Flag items with DIF across groups.
- Recalibrate annually or after major content changes.

---

## 7. Validity

| Type | Question | Evidence |
|---|---|---|
| Content validity | Does it cover the level outcome? | Expert mapping |
| Construct validity | Does it measure the intended capability? | Correlation with performance |
| Predictive validity | Does it predict real-world success? | Partner outcome data |
| Consequential validity | Does it cause harm? | Fairness and access review |
| Transfer validity | Does it generalize? | Novel task performance |

A level with no transfer task cannot be rated above band 5.

---

## 8. Fairness

### 8.1 Requirements
- Differential item functioning (DIF) analysis quarterly.
- Adverse-impact ratio ≥0.80 across protected groups.
- Accessible formats for every item.
- Language review for clarity and bias.
- No item requires prior cultural knowledge not taught.
- Accommodations do not change the construct.

### 8.2 Accommodations
- Extended time.
- Screen reader / braille.
- Captions/transcripts.
- Alternative input.
- Separate room / quiet environment.
- Human reader/scribe.
- Oral instead of written where the construct allows.

Accommodations are recorded and do not penalize the learner.

---

## 9. AI grading

### 9.1 Allowed uses
- First-pass scoring of short answers and essays.
- Rubric-anchored feedback.
- Plagiarism/provenance signals.
- Oral response transcription and analysis.
- Simulation log analysis.

### 9.2 Requirements
- AI scores are anchored to human-scored exemplars.
- Confidence score per item.
- Low-confidence items go to human review.
- 5–10% random human audit.
- Learners can appeal any AI score.
- No AI-only decision at level 80+.
- Prompts and model versions are logged.

### 9.3 Prohibited
- AI grading of high-stakes artifacts without human review.
- AI detecting "AI writing" as the sole basis for penalty.
- AI inferring protected traits.
- AI-generated rubric changes without psychometric review.

---

## 10. Security

| Control | Description |
|---|---|
| Item encryption | Items encrypted at rest; decrypted only for delivery |
| Randomized delivery | Different item sets per attempt |
| Watermarking | Learner ID in documents/media |
| Canary items | Detect leaks |
| Secure browser | For proctored gates |
| Time limits | Prevent answer lookup |
| Proctoring | Tiered by level |
| Audit logs | Every item delivery and score change |
| Rotation | 30% per cohort |

---

## 11. Pilot and launch

```
Draft items → Expert review → Cognitive labs (5–10 learners)
→ Pilot (30–50) → Item analysis → Revise → Calibrate
→ Publish → Monitor → Refresh
```

### 11.1 Launch gates
- ≥20 items in bank.
- α ≥0.75 in pilot (0.80 at launch).
- No item with discrimination <0.20.
- Pass rate within ±10 points of band target.
- Median time within ±25% of blueprint.
- Accessibility review complete.
- Safety review complete (high-risk).

---

## 12. Monitoring

| Signal | Action |
|---|---|
| Pass rate >95% | Add difficulty, rotate items, review leakage |
| Pass rate <30% | Add scaffolding, rewrite items, check prerequisites |
| High item exposure | Retire/rotate |
| DIF flag | Review and revise |
| Dispute spike | Calibrate graders, clarify rubric |
| AI-human gap >10% | Retrain/recalibrate |
| Score drift | Recalibrate anchors |

---

## 13. Acceptance criteria

- `FR-ASM-001`: Every level has a blueprint, rubric, and item bank.
- `FR-ASM-002`: Every high-stakes item has a difficulty estimate.
- `FR-ASM-003`: Every rubric has 4 criteria and anchor examples.
- `FR-ASM-004`: Every assessment has an accessibility review.
- `FR-ASM-005`: Every AI-graded item has a human audit path.
- `FR-ASM-006`: Every level 80+ assessment includes human review.
- `FR-ASM-007`: Every item is versioned and traceable.
- `FR-ASM-008`: Fairness analysis runs quarterly.
- `FR-ASM-009`: Retake policy is enforced.
- `FR-ASM-010`: Assessment results are exportable.
- `NFR-ASM-001`: Assessment start time <2 s.
- `NFR-ASM-002`: Autosave every 10 s.
- `NFR-ASM-003`: No data loss on disconnect.
