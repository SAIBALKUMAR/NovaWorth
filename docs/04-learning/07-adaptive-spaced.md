# Adaptive Path & Spaced Repetition

## 1. Purpose

Two systems keep learners moving efficiently:
1. **Adaptive pathing** — what to learn next, at what difficulty.
2. **Spaced repetition** — when to review so it sticks.

Together they prevent boredom, frustration, and forgetting.

---

## 2. Knowledge graph

### 2.1 Nodes
- **Skill** (500).
- **Level** (100 per skill).
- **Concept** (5–15 per level).
- **Item** (assessment).
- **Content** (explanation, example, practice).

### 2.2 Edges
| Edge | Meaning |
|---|---|
| `requires` | Must know before |
| `part_of` | Concept belongs to level |
| `assesses` | Item measures concept |
| `explains` | Content teaches concept |
| `transfer` | Applies in another context |
| `misconception` | Common wrong model |

### 2.3 Graph rules
- No cycles in `requires`.
- Every level has ≥3 concepts.
- Every concept has ≥2 content assets and ≥5 items.
- Orphan concepts are flagged.

---

## 3. Mastery estimation

### 3.1 BKT per concept
```
P(mastery | correct) = P(correct | mastered) × P(mastered) / P(correct)
P(mastery | wrong)   = P(wrong | mastered) × P(mastered) / P(wrong)
```
- Slip: 0.1.
- Guess: 0.2.
- Learn rate: 0.15 (adjusted per concept).
- Decay: 0.01/day after 30 days without review.

### 3.2 IRT per level
- 2PL model for item difficulty and discrimination.
- Ability estimate θ with standard error.
- Used for gate readiness and item selection.

### 3.3 Confidence
- Show a range, not a false precision.
- "Ready" requires lower bound ≥0.7.

---

## 4. Next-best-action

```
candidates = [
  due_reviews,
  weak_prerequisites,
  next_concepts,
  fragile_skills,
  applied_projects,
  gate_attempts
]

score(c) = 0.30 × urgency
         + 0.25 × prerequisite_value
         + 0.20 × goal_alignment
         + 0.15 × readiness
         + 0.10 × preference
         - fatigue
```

### 4.1 Rules
- Never start a new concept if a prerequisite is <0.6 mastery.
- Never gate if predicted pass <0.75.
- Always offer a 2-minute review option.
- If fatigue is high, recommend rest.

---

## 5. Difficulty targeting

| Learner state | Target success | Strategy |
|---|---|---|
| New | 80–85% | Guided practice |
| Developing | 75–80% | Mixed |
| Proficient | 70–75% | Transfer |
| Advanced | 65–70% | Novel contexts |
| Fatigued | 85%+ | Review only |

### 5.1 Desirable difficulty
- Struggle is productive, but not endless.
- If 3 consecutive failures, step back.
- If 5 consecutive successes, step up.

---

## 6. Spaced repetition

### 6.1 Scheduler
Use FSRS (Free Spaced Repetition Scheduler) or equivalent:
- Stability: how long memory lasts.
- Difficulty: how hard the item is for this learner.
- Retrievability: probability of recall now.
- Review when retrievability ≈0.9.

### 6.2 Review queue
Priority:
1. Overdue and weak.
2. Prerequisites for current level.
3. Items with low confidence.
4. Mixed interleaving.
5. New items (limited).

### 6.3 Daily limits
- 20–40 reviews/day (configurable).
- 10 new items/day.
- Stop at fatigue.
- No penalty for missed reviews.

### 6.4 Interleaving
- Mix related concepts.
- Avoid blocked practice after early bands.
- Use varied contexts.

---

## 7. Forgetting and decay

- Mastery decays slowly after 30 days of no review.
- A decayed skill is not lost; it is "rusty."
- Rusty skills are refreshed with a short review.
- High-stakes gates require current mastery.
- Credentials do not expire, but "current" badges do.

---

## 8. Transfer

### 8.1 Transfer tasks
- Apply a concept in a new context.
- Explain to a novice.
- Debug a novel case.
- Combine two skills.

### 8.2 Transfer score
```
transfer = 0.4 × novel_problem
         + 0.3 × explanation
         + 0.2 × debugging
         + 0.1 × combination
```
Transfer is required for high mastery multipliers and band 5+.

---

## 9. Cold start

| Data | Strategy |
|---|---|
| None | Diagnostic + sampler |
| Some | Prior-based BKT |
| Many | Personalized FSRS |
| Skill switch | Transfer priors from related skills |

---

## 10. Metrics

| Metric | Target |
|---|---|
| Review adherence | ≥60% |
| Retention at 30 days | ≥80% |
| Time to mastery | Within 25% of expected |
| Gate readiness accuracy | ≥80% |
| Frustration rate | <10% |
| Boredom rate | <10% |
| Transfer success | ≥60% at band 5+ |

---

## 11. Requirements

- `FR-LRN-601`: The engine models mastery per concept.
- `FR-LRN-602`: Prerequisites block new concepts below threshold.
- `FR-LRN-603`: Review scheduling uses FSRS or equivalent.
- `FR-LRN-604`: Learners can see why an item is due.
- `FR-LRN-605`: Daily limits are configurable.
- `FR-LRN-606`: Missed reviews do not punish.
- `FR-LRN-607`: Transfer tasks are required at band 5+.
- `FR-LRN-608`: Mastery decay is visible.
- `FR-LRN-609`: The engine explains next-best-action.
- `FR-LRN-610`: Cold start uses diagnostic priors.
- `NFR-LRN-601`: Review queue generation <200 ms.
- `NFR-LRN-602`: Mastery update <100 ms.
