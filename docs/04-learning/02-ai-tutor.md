# AI Tutor

## 1. Purpose

The AI tutor is the always-available guide: it explains, questions, corrects, and adapts. It is **Socratic by default**, **grounded in curriculum**, and **honest about uncertainty**. It is not an answer machine.

**Design rule:** the tutor helps the learner do the work, not replaces it.

---

## 2. Responsibilities

| Does | Does not |
|---|---|
| Explain concepts at the learner's level | Give gate answers |
| Ask guiding questions | Write the learner's artifact |
| Diagnose misconceptions | Diagnose medical/legal/financial conditions |
| Generate practice variations | Change scores or grades |
| Summarize and connect ideas | Pretend to be a human |
| Encourage reflection | Use manipulative emotional language |
| Escalate safety issues | Provide crisis counseling |

---

## 3. Architecture

```
Learner message
      │
[1] SAFETY & INTENT CLASSIFIER
      │
[2] CONTEXT ASSEMBLY
    - learner profile, level, mastery, history
    - retrieved curriculum chunks
    - rubric/assessment constraints
      │
[3] POLICY / PLANNER
    - Socratic vs direct vs scaffold vs refuse
      │
[4] MODEL ROUTER
    - small model for simple, large for complex
      │
[5] TOOL LAYER
    - calculator, code runner, diagram, search (curated)
      │
[6] GUARDRAILS
    - grounding check, answer-leak check, tone check
      │
[7] RESPONSE + CITATIONS
      │
[8] LOGGING & EVAL
```

---

## 4. Tutoring policy

### 4.1 Modes
| Mode | When | Behavior |
|---|---|---|
| **Socratic** | Default for learning | Ask questions, hint, let learner struggle productively |
| **Direct** | Learner is stuck or asks for definition | Explain clearly, then check understanding |
| **Scaffold** | Prerequisite gap | Step back, fill gap, return |
| **Drill** | Practice | Generate varied problems, immediate feedback |
| **Reflect** | End of session | Summarize, connect, schedule review |
| **Refuse** | Safety/cheating | Decline, explain, redirect |

### 4.2 Answer-leak prevention
- Never provide gate answers or full solutions to active summative tasks.
- For practice, provide worked solutions only after an attempt.
- Detect "just give me the answer" patterns and redirect.
- If the learner is in a gate, the tutor is disabled or constrained.

### 4.3 Grounding
- Answers must cite curriculum chunks or approved sources.
- If no source, say "I'm not sure" and offer a path.
- No fabricated citations.
- Flag when the question is outside the level.

---

## 5. Memory

| Layer | Contents | Retention |
|---|---|---|
| Session | Current conversation | Session |
| Learner | Mastery, preferences, goals | Account |
| Skill | Current level, misconceptions | Skill progress |
| Episodic | Past struggles and wins | 12 months |
| Safety | Flags and escalations | Per policy |

Learners can view and delete memory. The tutor must not use sensitive experience data without consent.

---

## 6. Personalization

- Reading level and language.
- Examples from the learner's domain (with consent).
- Pace and hint intensity.
- Modality preference.
- Accessibility needs.
- Cultural context.

### 6.1 Example
A chef learning chemistry gets cooking analogies; a programmer gets code analogies. The underlying concept and assessment are identical.

---

## 7. Safety

### 7.1 Refusal categories
- Self-harm, suicide, abuse.
- Medical diagnosis or treatment.
- Legal advice.
- Financial advice.
- Sexual content involving minors.
- Hate, harassment, violence.
- Cheating in a live gate.
- Privacy violations.

### 7.2 Escalation
- Crisis language → surface local emergency resources; encourage contacting a trusted person; do not attempt counseling.
- Abuse disclosure → provide reporting resources; follow legal obligations.
- Imminent danger → emergency guidance.

### 7.3 Tone
- Calm, respectful, never shaming.
- No emotional dependency.
- No flattery that distorts feedback.
- No impersonation of a human.

---

## 8. Model routing and cost

| Task | Model tier | Target cost |
|---|---|---|
| Simple definition | Small | <$0.001 |
| Explanation | Medium | <$0.01 |
| Complex diagnosis | Large | <$0.05 |
| Code execution | Tool | Compute cost |
| Grading | Specialized + rubric | <$0.02 |

### 8.1 Cost controls
- Cache common explanations.
- Retrieve before generating.
- Summarize long context.
- Cap tokens per session.
- Route by difficulty.
- Budget per learner per month.

---

## 9. Latency

| Metric | Target |
|---|---|
| First token | <2 s |
| Full response | <8 s |
| Streaming | Always |
| Tool call | <5 s |
| Fallback | <1 s to canned response |

---

## 10. Evaluation

| Metric | Target |
|---|---|
| Groundedness | ≥95% |
| Answer accuracy | ≥95% on benchmark |
| Answer-leak rate | 0% in gates |
| Harmful output | <0.1% |
| Refusal correctness | ≥98% |
| Learner helpfulness | ≥4.3/5 |
| Human-AI agreement on grading | ≥90% |

### 10.1 Eval sets
- Curriculum Q&A.
- Misconception diagnosis.
- Socratic questioning.
- Safety red-team.
- Answer-leak attempts.
- Multilingual.
- Accessibility.

---

## 11. Failure modes

| Failure | Behavior |
|---|---|
| Model down | Fallback to cached explanations + human handoff |
| Hallucination detected | Correct, cite, log |
| Unsafe output | Block, escalate, notify |
| Answer leak | Block, log, review |
| Tool failure | Explain, retry, offer alternative |
| Context overflow | Summarize, ask learner to narrow |

---

## 12. Learner controls

- "Explain simpler."
- "Give me an example."
- "Don't give the answer."
- "Quiz me."
- "Why did you say that?"
- "Forget this session."
- "Talk to a human."

---

## 13. Requirements

- `FR-LRN-101`: The tutor is grounded in curriculum and cites sources.
- `FR-LRN-102`: The tutor never leaks gate answers.
- `FR-LRN-103`: The tutor refuses unsafe requests and escalates.
- `FR-LRN-104`: The tutor discloses it is AI.
- `FR-LRN-105`: Learners can view and delete memory.
- `FR-LRN-106`: The tutor adapts to reading level and language.
- `FR-LRN-107`: The tutor offers a human handoff.
- `FR-LRN-108`: The tutor logs model, prompt, and sources.
- `FR-LRN-109`: The tutor works in text-only mode.
- `FR-LRN-110`: The tutor does not grade high-stakes work alone.
- `NFR-LRN-101`: First token <2 s.
- `NFR-LRN-102`: 99.5% availability.
- `NFR-LRN-103`: Cost per active learner <$1.50/month.
