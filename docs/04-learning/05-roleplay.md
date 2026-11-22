# AI Role-Play

## 1. Purpose

Role-play is how we assess interpersonal skills credibly: listening, conflict, negotiation, care, leadership, sales, teaching, and crisis communication. It is a **scenario engine** with AI-controlled characters, a rubric, and a debrief — not a free chat.

**Rule:** role-play is scored on observable behavior, not on how likable the learner is.

---

## 2. When to use

| Skill type | Example |
|---|---|
| Communication | Delivering bad news |
| Conflict | Mediating a dispute |
| Negotiation | Salary or vendor negotiation |
| Care | Supporting a grieving colleague |
| Leadership | Giving difficult feedback |
| Sales | Discovery call |
| Teaching | Explaining a concept |
| Crisis | Customer escalation |
| Cross-cultural | Working across norms |
| Clinical communication | Taking a history (within scope) |

---

## 3. Scenario model

```
scenario
├── context (setting, stakes, time)
├── characters (persona, goals, secrets, triggers)
├── learner_role (goal, constraints, authority)
├── beats (opening, complication, climax, close)
├── success criteria (rubric)
├── failure conditions (safety, ethics)
└── debrief (questions, evidence, next steps)
```

### 3.1 Persona sheet
| Field | Example |
|---|---|
| Name | "Priya, 42, operations manager" |
| Goal | Keep the project on track |
| Fear | Being blamed for the delay |
| Communication style | Direct, fast, slightly defensive |
| Secrets | Knows the vendor warning was ignored |
| Triggers | Being interrupted, vague answers |
| Arc | Defensive → collaborative if heard |

### 3.2 Learner brief
- Your role and authority.
- Your goal.
- Time limit.
- Constraints (no budget, must consult legal).
- What success looks like.

---

## 4. Interaction loop

```
Learner speaks
      │
[1] TRANSCRIBE / PARSE
      │
[2] STATE UPDATE
    - trust, stress, disclosure, progress
      │
[3] PERSONA RESPONSE
    - in character, consistent, bounded
      │
[4] SAFETY CHECK
    - no abuse, no manipulation, no crisis mishandling
      │
[5] RUBRIC SIGNAL
    - track evidence for each criterion
      │
[6] BEAT CONTROL
    - advance, complicate, or close
```

### 4.1 State variables
| Variable | Range | Effect |
|---|---|---|
| Trust | 0–100 | Willingness to disclose |
| Stress | 0–100 | Reactivity, brevity |
| Disclosure | 0–100 | Reveals secrets |
| Progress | 0–100 | Moves toward resolution |
| Safety | 0–100 | Flags harmful behavior |

### 4.2 Guardrails
- Characters stay in role.
- No real person impersonation.
- No sexual content, harassment, or abuse.
- No medical/legal/financial advice from the character.
- Crisis scenarios have escalation paths.
- Learner can pause or exit at any time.

---

## 5. Scoring

### 5.1 Rubric
| Criterion | 1 | 2 | 3 | 4 |
|---|---|---|---|---|
| Presence | Distracted | Inconsistent | Attentive | Deeply present |
| Inquiry | No questions | Surface | Good questions | Reveals what matters |
| Empathy | Cold | Formulaic | Warm | Precise validation |
| Judgment | Poor | Rule-bound | Sound | Excellent under pressure |
| Outcome | Worsened | Neutral | Resolved | Durable agreement |

Weights vary by skill. Safety is a hard gate.

### 5.2 Evidence
- Transcript.
- State trajectory.
- Key moments (timestamped).
- Rubric scores.
- Learner reflection.
- Optional recording (with consent).

### 5.3 AI + human
- AI proposes scores and evidence.
- Human reviews at level 40+.
- Human final at level 80+.
- 10% random audit at all levels.

---

## 6. Debrief

The debrief is where learning happens.

1. **What happened?** Learner summarizes.
2. **What worked?** Specific moments.
3. **What missed?** Specific moments.
4. **What would you do differently?** Plan.
5. **What will you practice?** Next step.
6. **Score and evidence.** Transparent.

No debrief, no credit.

---

## 7. Scenario examples

### 7.1 Difficult feedback (WRK-12, level 35)
- Learner is a team lead.
- Character is a defensive senior engineer who missed a deadline.
- Goal: clear feedback + agreement on next steps.
- Rubric: directness, listening, specificity, follow-through.

### 7.2 Grief support (MND-09, level 30)
- Learner is a colleague.
- Character lost a parent.
- Goal: presence and support, not advice.
- Safety: no therapy; escalate if crisis language.
- Rubric: empathy, boundaries, practical support.

### 7.3 Vendor negotiation (COM-05, level 45)
- Learner buys services.
- Character is a vendor with a hidden discount.
- Goal: 15% reduction with a 2-year contract.
- Rubric: preparation, questioning, anchoring, tradeoffs, close.

### 7.4 Clinical history (HLT-13, level 40)
- Learner takes a medication history.
- Character is an older adult with polypharmacy.
- Goal: accurate list, interactions flagged, handover.
- Safety: scope limits; no prescribing.

---

## 8. Personalization

- Persona difficulty adapts (cooperative → resistant).
- Cultural context adapts.
- Language adapts.
- Accessibility: text-only, voice, avatar, or VR.
- Learner can replay with a different persona.

---

## 9. Integrity

- Randomized personas and secrets.
- No scripted "right answer" to memorize.
- State trajectories vary.
- Oral reflection.
- AI detection for written prep.
- High-stakes role-play is live or recorded.

---

## 10. Technical

| Component | Approach |
|---|---|
| Dialogue | LLM with persona constraints |
| State | Deterministic state machine + LLM |
| Scoring | Rubric-anchored LLM + human |
| Voice | STT/TTS optional |
| Avatar | 2D/3D optional; Vision Pro later |
| Storage | Transcript + evidence, encrypted |
| Latency | <2 s response |

---

## 11. Requirements

- `FR-LRN-401`: Every role-play has a scenario, persona sheet, and rubric.
- `FR-LRN-402`: Characters stay in role and respect safety.
- `FR-LRN-403`: Safety is a hard gate.
- `FR-LRN-404`: Every role-play has a debrief.
- `FR-LRN-405`: Scoring cites specific evidence.
- `FR-LRN-406`: Human review at level 40+.
- `FR-LRN-407`: Learners can pause/exit anytime.
- `FR-LRN-408`: Transcripts are stored securely and deletable.
- `FR-LRN-409`: Role-play works text-only.
- `FR-LRN-410`: Personas are randomized.
- `NFR-LRN-401`: Response <2 s.
- `NFR-LRN-402`: Transcript accuracy ≥95%.
