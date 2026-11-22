# Quizzes & Practice

## 1. Purpose

Quizzes are the fastest, cheapest way to practice retrieval and diagnose misconceptions. They are **formative by default** and **summative only at gates**. They must be hard to game and useful when wrong.

---

## 2. Practice vs gate

| | Practice | Gate |
|---|---|---|
| Purpose | Learn | Prove |
| Attempts | Unlimited | 3/day, cooldown |
| Feedback | Immediate, detailed | After submission |
| Items | Adaptive | Blueprint + randomized |
| Score | Informs mastery | Determines award |
| Integrity | Light | Tiered proctoring |

---

## 3. Item types

| Type | Best for | Auto-grade | Notes |
|---|---|---|---|
| Multiple choice | Recall, discrimination | Yes | 4 options |
| Multi-select | Classification | Yes | Partial credit |
| True/false | Quick checks | Yes | Avoid overuse |
| Numeric | Calculation | Yes | Tolerance |
| Fill-in | Terminology | Yes | Fuzzy match |
| Short answer | Explanation | AI + rubric | Human sample |
| Ordering | Processes | Yes | Drag or keyboard |
| Matching | Relationships | Yes | Accessible alternative |
| Code | Programming | Tests | Sandbox |
| Case | Judgment | Rubric | Structured |
| Confidence | Calibration | Yes | "How sure?" |

---

## 4. Adaptive practice

### 4.1 Target
70–85% success. Too easy is boring; too hard is demoralizing.

### 4.2 Selection
```
next_item = argmax( information_gain × relevance × freshness )
```
- Information gain from IRT/BKT.
- Relevance to current objective.
- Freshness to avoid repetition.

### 4.3 Stopping
- Mastery ≥0.85 with confidence.
- 10–20 items.
- Or learner ends session.

---

## 5. Feedback

### 5.1 Correct
- Confirm.
- Explain why (brief).
- Connect to concept.
- Offer a harder variant.

### 5.2 Incorrect
- Do not shame.
- Show the correct answer.
- Explain the misconception.
- Offer a similar item.
- Link to the concept.
- Track the misconception.

### 5.3 "I don't know"
- Valid response.
- Show answer + explanation.
- Schedule review sooner.
- No penalty beyond mastery update.

---

## 6. Confidence and calibration

- Ask "How confident are you?" on high-stakes practice.
- Compare confidence to correctness.
- Overconfidence triggers a calibration lesson.
- Underconfidence triggers evidence of competence.

---

## 7. Spaced repetition integration

- Every item has a stability/difficulty estimate (FSRS).
- Correct + confident → longer interval.
- Wrong or slow → shorter interval.
- Review queue prioritizes weak, due, and prerequisite items.
- Review counts as practice, not gate.

---

## 8. Anti-gaming

| Threat | Control |
|---|---|
| Answer sharing | Randomized banks, variants, watermarking |
| Screenshot leakage | Canary items, rotation |
| AI answering | Oral checks, process evidence, proctored gates |
| Guess farming | Confidence weighting, item exposure limits |
| Retry brute force | Cooldowns, variant rotation |
| Pattern guessing | Distractor balance, item analysis |

---

## 9. Analytics

| Metric | Use |
|---|---|
| Item difficulty | Calibration |
| Discrimination | Item quality |
| Distractor analysis | Misconception detection |
| Time per item | Pacing |
| Confidence gap | Calibration |
| Retry rate | Difficulty |
| Hint use | Scaffolding need |

---

## 10. Accessibility

- Keyboard-only.
- Screen-reader labels.
- No drag-only interactions.
- Extended time.
- Plain language.
- High contrast.
- Alternative formats for image-based items.

---

## 11. Requirements

- `FR-LRN-301`: Practice is unlimited and low-stakes.
- `FR-LRN-302`: Gates use randomized item sets.
- `FR-LRN-303`: Every wrong answer gets an explanation.
- `FR-LRN-304`: Items are calibrated and versioned.
- `FR-LRN-305`: Confidence is captured on high-stakes practice.
- `FR-LRN-306`: Review intervals adapt to performance.
- `FR-LRN-307`: No item is repeated within a session.
- `FR-LRN-308`: Item exposure is monitored.
- `FR-LRN-309`: All item types are accessible.
- `FR-LRN-310`: Quiz data feeds mastery estimates.
- `NFR-LRN-301`: Item load <200 ms.
- `NFR-LRN-302`: Autosave every answer.
