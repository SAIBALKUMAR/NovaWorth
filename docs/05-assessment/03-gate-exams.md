# Gate Exams

## 1. Purpose

This document specifies how summative gate exams are designed and administered. The learner-facing flow is in [Level gate](../04-learning/08-level-gate.md); this is the assessment design and operations view.

---

## 2. Design principles

- **Aligned:** every item maps to a level outcome.
- **Representative:** covers the domain, not a corner.
- **Fair:** accessible, culturally reviewed, calibrated.
- **Secure:** randomized, proctored by tier.
- **Decisive:** clear pass/fail with actionable feedback.
- **Appealable:** evidence and review path.

---

## 3. Blueprint

### 3.1 Knowledge
| Element | Target |
|---|---|
| Items | 15–40 depending on band |
| Cognitive levels | 40% apply, 40% analyze, 20% evaluate |
| Item types | 50% selected response, 30% constructed, 20% applied |
| Time | 1.5 min/item average |
| Pass | 80% |
| Distinction | 95% |

### 3.2 Applied
- 1 primary task.
- 1 transfer task.
- 4 rubric criteria.
- 60–120 minutes.
- Pass: 2.5/4 average, no criterion <2.

### 3.3 Oral
- 15–30 minutes.
- 3 questions: explain, transfer, defend.
- Pass: 3/4 average.

### 3.4 Artifact
- Submitted 48h before panel.
- Provenance verified.
- Panel reviews 30–60 minutes.
- Pass: 3/4 on all critical criteria.

---

## 4. Item selection

- Randomize from a calibrated bank.
- Stratify by concept and difficulty.
- Include 1–2 anchor items for comparability.
- Avoid items seen in the last 90 days.
- Use variants for high-exposure items.

### 4.1 Bank rules
- ≥20 items per level; 40 preferred.
- ≥3 variants per high-stakes item.
- 30% rotation per cohort.
- Retire items with exposure >20%.

---

## 5. Administration

### 5.1 Tiers
| Tier | Levels | Controls |
|---|---|---|
| Open | 1–20 | Randomized, time limit, analytics |
| Monitored | 21–50 | Webcam/screen check, behavior |
| Proctored | 51–80 | Live/recorded proctor, ID, secure browser |
| Panel | 81–100 | Human panel, oral, artifact |

### 5.2 Environment
- Quiet space recommended.
- Stable connection.
- Camera/mic for monitored+.
- No second monitor (where detectable).
- No external help.

### 5.3 Accommodations
- Extended time (1.5× or 2×).
- Screen reader/braille.
- Captions/transcripts.
- Alternative input.
- Human reader/scribe.
- Separate room.
- Oral instead of written where construct allows.

---

## 6. Scoring

### 6.1 Auto
- MCQ, numeric, code tests, structured items.
- Immediate score.
- Item-level feedback after submission.

### 6.2 AI
- Short answer, essay, case.
- Rubric-anchored.
- Confidence score.
- Human sample.

### 6.3 Human
- Practical, oral, artifact, panel.
- Double grading for high stakes.
- Calibration sessions.

### 6.4 Borderline
- 75–79% knowledge or 2.4 applied.
- Optional oral recheck within 72h.
- Human decides.

---

## 7. Integrity

- Identity verification at monitored+.
- Device/session checks.
- Behavioral signals.
- Content watermarking.
- Artifact provenance.
- Collusion detection.
- Integrity score thresholds.

---

## 8. Results

| Outcome | Learner sees | System does |
|---|---|---|
| Pass | Score, rubric, evidence, next level | Award XP/EC, credential |
| Near pass | Gap analysis, oral option | Hold award |
| Fail | Specific gaps, plan, retake window | No EC; partial XP |
| Invalid | Reason, appeal path | Hold; investigate |

Results within:
- Auto: immediate.
- AI: <1 hour.
- Human: <72 hours.
- Panel: <7 days.

---

## 9. Retakes and appeals

- 3 attempts/24h; 7-day cooldown after 3 fails.
- Different items each attempt.
- Appeals within 14 days; new reviewer; decision in 7 days.
- Second appeal for level 80+.

---

## 10. Panel exams (80–100)

### 10.1 Composition
- 3–5 experts.
- At least one external.
- Conflict-of-interest declarations.
- Calibration training.

### 10.2 Format
- Artifact review (48h prior).
- 60–90 minute oral defense.
- Live task or simulation.
- Deliberation and written decision.

### 10.3 Scoring
| Dimension | Weight |
|---|---|
| Technical depth | 25% |
| Judgment | 25% |
| Impact | 20% |
| Communication | 15% |
| Ethics | 15% |

Pass: 4/5 on every dimension. No compensation.

---

## 11. Requirements

- `FR-ASM-401`: Every gate has a blueprint.
- `FR-ASM-402`: Every gate uses a calibrated bank.
- `FR-ASM-403`: Every gate is randomized.
- `FR-ASM-404`: Every gate has an integrity tier.
- `FR-ASM-405`: Every gate has an appeal path.
- `FR-ASM-406`: Panel exams have external experts.
- `FR-ASM-407`: Accommodations are available.
- `FR-ASM-408`: Results meet the time targets.
- `FR-ASM-409`: Borderline cases get human review.
- `FR-ASM-410`: Panel decisions are documented.
- `NFR-ASM-401`: Autosave every 10 s.
- `NFR-ASM-402`: Resume within 15 min.
- `NFR-ASM-403`: No score loss on disconnect.
