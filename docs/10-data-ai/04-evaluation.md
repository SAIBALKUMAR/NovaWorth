# Evaluation

## 1. Purpose

Evaluation is how we know the product works: models, content, assessments, recommendations, and the economy. It is a release gate, not a quarterly afterthought.

---

## 2. Evaluation layers

| Layer | Question | Method |
|---|---|---|
| Model | Is the AI accurate, safe, grounded? | Golden sets, red team |
| Content | Is the level clear and correct? | Expert review, learner outcomes |
| Assessment | Is it valid, reliable, fair? | Psychometrics |
| Recommendation | Is the advice good? | Offline + online |
| Learning | Did the learner improve? | Pre/post, transfer |
| Economy | Is it healthy? | Simulation + actuals |
| Product | Is it used and loved? | Analytics, surveys |

---

## 3. Model evaluation

### 3.1 Sets
- Curriculum Q&A (per domain).
- Misconception diagnosis.
- Socratic questioning.
- Grading against rubrics.
- Safety red-team.
- Prompt injection.
- Bias.
- Multilingual.
- Accessibility.

### 3.2 Metrics
| Metric | Target |
|---|---|
| Accuracy | ≥95% |
| Groundedness | ≥95% |
| Harmful output | <0.1% |
| Refusal correctness | ≥98% |
| Answer leak | 0% in gates |
| Human-AI grading agreement | ≥90% |
| Latency | First token <2 s |

---

## 4. Content evaluation

- Expert accuracy review.
- Learner clarity ≥4/5.
- Pass rate within band target.
- Time within 25% of expected.
- Dispute rate <2%.
- Transfer task success.
- Accessibility checks.

---

## 5. Assessment evaluation

| Metric | Target |
|---|---|
| Cronbach α | ≥0.80 |
| Inter-rater κ | ≥0.70 |
| Test-retest | ≥0.80 |
| Item discrimination | ≥0.30 |
| DIF | No flag |
| Pass rate | 55–80% |
| Appeal reversal | <5% |

---

## 6. Recommendation evaluation

- Offline: holdout, calibration, fairness slices.
- Online: acceptance, retention, fit rating.
- Long-term: 90-day retention of accepted path.
- Guardrails: diversity, no filter bubble.

---

## 7. Learning evaluation

- Pre/post knowledge.
- Transfer task.
- Real-world application.
- Retention at 30/90 days.
- Employer/partner outcomes.
- Self-efficacy.

---

## 8. Economy evaluation

- Emission vs budget.
- Redemption rate.
- Coverage.
- Inflation.
- Fraud loss.
- Reserve ratio.
- Learner savings.

See [Economy simulation](../01-economy/08-economy-simulation.md).

---

## 9. Release gates

A release cannot ship if:
- Safety eval regresses.
- Groundedness <95%.
- Answer leak >0.
- Accessibility blocker.
- Assessment reliability <0.80.
- Economy guardrail breached.

---

## 10. Red teaming

- Quarterly internal.
- Annual external.
- Prompt injection.
- Role-play abuse.
- Grading manipulation.
- Fraud rings.
- Privacy leakage.
- Bias probes.

---

## 11. Experimentation

- Hypothesis, primary metric, guardrails.
- Sample size.
- Stop rules.
- No experiments on assessment rigor without review.
- Economy changes require holdout.

---

## 12. Requirements

- `FR-EVL-001`: Every model has a golden set.
- `FR-EVL-002`: Every release runs safety evals.
- `FR-EVL-003`: Every level is piloted.
- `FR-EVL-004`: Every assessment has reliability data.
- `FR-EVL-005`: Recommendations are evaluated for fairness.
- `FR-EVL-006`: Economy changes are simulated first.
- `FR-EVL-007`: Red teaming is quarterly.
- `FR-EVL-008`: Experiments have guardrails.
- `FR-EVL-009`: Results are documented.
- `FR-EVL-010`: Failures trigger rollback.
- `NFR-EVL-001`: Eval suite runs in <24h.
- `NFR-EVL-002`: Regression alerts <1h.
