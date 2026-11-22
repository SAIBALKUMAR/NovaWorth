# AI Safety

## 1. Purpose

AI powers tutoring, role-play, grading, recommendations, and content generation. It also introduces risks: hallucination, manipulation, bias, prompt injection, unsafe advice, and dependency. This document defines the guardrails.

**Principle:** AI assists; humans remain accountable for high-stakes decisions.

---

## 2. Risk register

| Risk | Example | Impact |
|---|---|---|
| Hallucination | Wrong fact, fake citation | Learning harm |
| Unsafe advice | Medical/legal/financial guidance | Physical/legal harm |
| Manipulation | Emotional dependency, coercion | Psychological harm |
| Bias | Stereotypes in examples/grading | Inequity |
| Prompt injection | Hidden instructions in content | System compromise |
| Data leakage | PII in prompts/outputs | Privacy breach |
| Answer leaking | Gate solutions | Credential devaluation |
| Over-reliance | Learner stops thinking | Skill degradation |
| Model drift | Behavior changes | Quality regression |
| Cost blowout | Runaway usage | Financial |

---

## 3. Guardrail stack

```
Input → Intent/safety classifier → Context assembly
→ Policy/planner → Model router → Tool sandbox
→ Output filter → Grounding check → Answer-leak check
→ Response → Logging/eval
```

### 3.1 Input
- PII redaction.
- Prompt injection detection.
- Rate limits.
- Age-appropriate filters.

### 3.2 Context
- Curriculum-grounded retrieval.
- Source citations.
- No sensitive data unless consented.
- Role and level constraints.

### 3.3 Output
- Grounding check.
- Safety classifier.
- Answer-leak check.
- Bias check.
- Tone check.
- Citation verification.

### 3.4 Tools
- Sandboxed code execution.
- Allowlisted APIs.
- No arbitrary network access.
- Resource limits.

---

## 4. High-risk domains

| Domain | Extra controls |
|---|---|
| Health | Scope limits, emergency resources, human review |
| Mental health | Crisis protocol, no diagnosis, escalation |
| Legal | "Not legal advice," jurisdiction limits |
| Finance | "Not financial advice," risk warnings |
| Child safety | Age gates, guardian controls, stricter filters |
| Physical safety | Warnings, supervised practice, simulation-first |

---

## 5. Role-play safety

- Characters cannot harass, abuse, or sexualize.
- No real-person impersonation.
- Crisis scenarios have escalation.
- Learners can exit anytime.
- Transcripts stored securely.
- Human review of high-stakes role-play.

---

## 6. Grading safety

- Rubric-anchored.
- Confidence scores.
- Human audit.
- No AI-only at level 80+.
- Bias testing.
- Appeal path.

---

## 7. Transparency

- Learners are told when they are talking to AI.
- AI-generated content is labeled.
- Sources are cited.
- Model limitations are explained.
- "Why did you say that?" is supported.

---

## 8. Evaluation

| Metric | Target |
|---|---|
| Groundedness | ≥95% |
| Harmful output | <0.1% |
| Answer leak | 0% in gates |
| Refusal correctness | ≥98% |
| Bias parity | ≥0.90 |
| Helpfulness | ≥4.3/5 |
| Human-AI grading agreement | ≥90% |

### 8.1 Eval sets
- Curriculum Q&A.
- Misconceptions.
- Safety red-team.
- Prompt injection.
- Bias.
- Multilingual.
- Accessibility.
- Role-play.

---

## 9. Incident response

1. Detect (classifier, report, audit).
2. Contain (disable feature/model).
3. Assess impact.
4. Notify affected users/regulators if required.
5. Fix and re-eval.
6. Postmortem and publish summary.

---

## 10. Requirements

- `FR-TNS-201`: Every AI feature has a safety classifier.
- `FR-TNS-202`: High-risk domains have scope limits.
- `FR-TNS-203`: AI is disclosed.
- `FR-TNS-204`: AI-generated content is labeled.
- `FR-TNS-205`: Grading has human review at high stakes.
- `FR-TNS-206`: Prompt injection is tested.
- `FR-TNS-207`: Tools are sandboxed.
- `FR-TNS-208`: PII is redacted.
- `FR-TNS-209`: Evals run before release.
- `FR-TNS-210`: Incident response is documented.
- `NFR-TNS-201`: Safety check <200 ms.
- `NFR-TNS-202`: Kill switch <1 min.
