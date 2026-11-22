# Assessment System

## 1. Purpose

Assessment is how the product converts activity into credible capability. It answers:
- Did the learner actually learn it?
- Can they apply it in a new context?
- Can they do it under real constraints?
- Is the evidence attributable to them?

The system is designed for **validity, reliability, fairness, and resistance to gaming**.

---

## 2. Evidence model

### 2.1 Evidence tiers
| Tier | Type | Examples | Weight |
|---|---|---|---|
| 1 | Recognition | MCQ, matching | Low |
| 2 | Application | Numeric, short answer, code | Medium |
| 3 | Transfer | Novel case, oral defense | High |
| 4 | Performance | Artifact, role-play, sim | High |
| 5 | Real-world impact | Partner-verified outcome | Highest |

### 2.2 Evidence record
```json
{
  "evidence_id": "ev_...",
  "learner_id": "usr_...",
  "skill_id": "CMP-25",
  "level": 40,
  "type": "artifact",
  "artifact_uri": "s3://...",
  "artifact_hash": "sha256:...",
  "rubric_version": "rub-2026.1",
  "scores": { "correctness": 3, "method": 3, "judgment": 2, "communication": 4 },
  "integrity_score": 0.91,
  "reviewer_ids": ["rev_..."],
  "provenance": { "ai_assisted": false, "tools": ["git"] },
  "created_at": "2026-11-21T10:00:00Z"
}
```

### 2.3 Rules
- Evidence must be attributable, dated, and versioned.
- AI assistance must be declared.
- Evidence is retained for the credential lifetime.
- Learners can export evidence.
- Evidence is encrypted and access-controlled.

---

## 3. Assessment types

| Type | Purpose | When | Scored by |
|---|---|---|---|
| Diagnostic | Placement | Onboarding | Auto + IRT |
| Formative | Practice | Every session | Auto/AI |
| Summative gate | Level pass | End of level | Auto + AI + human |
| Practical | Applied skill | Bands 3+ | Rubric + human |
| Oral | Explanation/transfer | Bands 4+ | Human/AI |
| Role-play | Interpersonal | Bands 3+ | Rubric + human |
| Simulation | Judgment | Bands 4+ | Rules + rubric |
| Portfolio | Body of work | Bands 7+ | Panel |
| Real-world | Impact | Bands 9+ | Partner + panel |

---

## 4. Lifecycle

```
DESIGN → REVIEW → PILOT → CALIBRATE → PUBLISH
   → DELIVER → SCORE → AUDIT → APPEAL → REFRESH
```

### 4.1 Design
- Start from the level outcome.
- Build a blueprint.
- Write items/tasks.
- Define rubrics and anchors.
- Plan integrity and accessibility.

### 4.2 Review
- SME accuracy.
- ID alignment.
- Psychometric quality.
- Safety/legal.
- Accessibility.

### 4.3 Pilot
- 30–50 learners.
- Item analysis.
- Time analysis.
- Fairness review.

### 4.4 Calibrate
- IRT parameters.
- Pass thresholds.
- Rubric anchors.
- AI grader calibration.

### 4.5 Publish
- Versioned.
- Randomized.
- Monitored.

---

## 5. Roles

| Role | Responsibility |
|---|---|
| Assessment lead | Owns the system and standards |
| Psychometrician | Calibration, reliability, fairness |
| SME | Content accuracy |
| ID | Construct alignment |
| Grader | Rubric scoring |
| Examiner | High-stakes panels |
| Integrity analyst | Fraud detection |
| Accessibility reviewer | Accommodations |
| Learner advocate | Appeals and fairness |

---

## 6. AI grading

### 6.1 Use
- First-pass scoring.
- Feedback generation.
- Plagiarism/provenance signals.
- Transcript analysis.

### 6.2 Controls
- Rubric-anchored prompts.
- Confidence score.
- Human audit sample (5–10%).
- Low-confidence → human.
- No AI-only at level 80+.
- Model/prompt version logged.
- Bias testing.

### 6.3 Prohibited
- AI detecting "AI writing" as sole penalty.
- AI inferring protected traits.
- AI changing rubrics without review.
- AI grading without appeal path.

---

## 7. Quality metrics

| Metric | Target |
|---|---|
| Internal consistency | α ≥0.80 |
| Inter-rater reliability | κ ≥0.70 |
| AI-human agreement | ≥90% |
| Test-retest | r ≥0.80 |
| Item discrimination | ≥0.30 |
| Pass rate | 55–80% |
| Dispute rate | <2% |
| Appeal reversal | <5% |
| Accessibility blockers | 0 |

---

## 8. Fairness

- DIF analysis quarterly.
- Adverse-impact ratio ≥0.80.
- Language and cultural review.
- Accommodations without construct change.
- Public fairness report.
- Human review of group gaps >5 points.

---

## 9. Security

| Layer | Control |
|---|---|
| Item security | Encryption, randomization, watermarking |
| Delivery | Secure browser, time limits |
| Identity | ID verification, liveness |
| Behavior | Tab/paste/typing signals |
| Artifacts | Provenance, version history |
| Post-hoc | Collusion graphs, anomaly detection |
| Human | Review, appeals, audits |

---

## 10. Accessibility

- Every assessment has an accessible variant.
- Extended time.
- Screen reader/braille.
- Captions/transcripts.
- Alternative input.
- Human reader/scribe.
- Separate room.
- Accommodations recorded, not penalized.

---

## 11. Data model

| Entity | Key fields |
|---|---|
| `assessment` | id, level_id, blueprint, version |
| `item` | id, type, difficulty, discrimination, content_hash |
| `attempt` | id, learner_id, assessment_id, started_at, submitted_at |
| `response` | attempt_id, item_id, answer, score, confidence |
| `artifact` | id, attempt_id, uri, hash, provenance |
| `rubric_score` | attempt_id, criterion, score, evidence |
| `review` | attempt_id, reviewer_id, decision, notes |
| `appeal` | attempt_id, reason, status, decision |

---

## 12. Requirements

- `FR-ASM-201`: Every assessment maps to a level outcome.
- `FR-ASM-202`: Every assessment has a blueprint and rubric.
- `FR-ASM-203`: Every item is calibrated and versioned.
- `FR-ASM-204`: Every attempt has an integrity score.
- `FR-ASM-205`: Every AI score has a human audit path.
- `FR-ASM-206`: Every learner can appeal.
- `FR-ASM-207`: Every assessment is accessible.
- `FR-ASM-208`: Every credential links to evidence.
- `FR-ASM-209`: Every assessment is piloted before launch.
- `FR-ASM-210`: Fairness analysis runs quarterly.
- `NFR-ASM-201`: Assessment start <2 s.
- `NFR-ASM-202`: Autosave every 10 s.
- `NFR-ASM-203`: No data loss on disconnect.
