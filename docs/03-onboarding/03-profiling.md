# Profiling: Aptitude, Effort, Interests & Constraints

## 1. Purpose

Profiling answers four questions:
1. **What are you ready to learn?** (aptitude and prior knowledge)
2. **How do you learn best?** (pace, support, modality)
3. **What do you want?** (interests and goals)
4. **What is feasible?** (time, device, money, access)

It is not a judgment of human worth. It is a set of signals to calibrate challenge, recommendation, and support.

---

## 2. Principles

- **Skill-relevant, not global.** Measure abilities that predict performance in the target domain.
- **No single score.** Show a profile with confidence intervals, not an IQ number.
- **No protected traits.** Do not infer race, gender, religion, sexuality, disability, or health status.
- **No direct payout.** Aptitude affects readiness and support; EC depends on demonstrated performance.
- **Learner-visible.** The learner can see and contest their profile.
- **Actionable.** Every result maps to a concrete recommendation.

---

## 3. Aptitude battery

### 3.1 Components
| Component | What it measures | Items | Time |
|---|---|---|---|
| Fluid reasoning | Novel pattern and rule discovery | 8–12 | 4 min |
| Working memory | Holding and manipulating information | 6–10 | 3 min |
| Numeracy | Quantitative reasoning | 8–10 | 4 min |
| Verbal reasoning | Inference, argument, vocabulary in context | 8–10 | 4 min |
| Spatial reasoning | Mental rotation, navigation | 6–8 | 3 min |
| Learning agility | Improvement across repeated novel tasks | 2 rounds | 4 min |
| Effort profile | Persistence, strategy, help-seeking | 1 task | 3 min |

Total: ~25 minutes if all components are taken. Onboarding uses a short form (~10 minutes); the full battery is optional and spread over the first week.

### 3.2 Adaptive delivery
- Start at medium difficulty.
- Adjust after each item (IRT-based).
- Stop when confidence interval is narrow enough.
- No time pressure except where the construct requires it.

### 3.3 Output
```
Profile:
  Fluid reasoning: 62nd percentile (CI 52–71)
  Working memory: 48th percentile (CI 38–58)
  Numeracy: 71st percentile (CI 61–80)
  Verbal: 55th percentile (CI 45–65)
  Spatial: 40th percentile (CI 30–50)
  Learning agility: high (improved 35% across rounds)
  Effort: persistent, seeks help appropriately
```

The learner sees strengths, growth areas, and suggested support — not a rank.

---

## 4. Effort profile

Effort is not "hours spent." It is **how** the learner engages.

### 4.1 Signals
| Signal | What it indicates |
|---|---|
| Persistence after failure | Resilience |
| Strategy change | Metacognition |
| Help-seeking quality | Resourcefulness |
| Reflection specificity | Learning depth |
| Revision behavior | Craft |
| Time to first attempt | Confidence/avoidance |
| Session regularity | Habit |
| Error correction | Feedback use |

### 4.2 Effort task
A short, hard puzzle with no obvious solution. Observe:
- Does the learner try multiple strategies?
- Do they ask for a hint? When?
- Do they give up, persist, or reflect?
- Do they explain their reasoning?

### 4.3 Effort score
Used for:
- Pacing and hint intensity.
- Recommending study systems.
- EC effort multiplier (only for demonstrated deliberate practice).
- Never for labeling the learner.

---

## 5. Interests

### 5.1 Capture
- Card sort: energizing vs draining.
- Free text: "What would you do for free?"
- Ranking: impact, status, security, creativity, autonomy, service.
- Optional: dream project.

### 5.2 Interest model
| Dimension | Low | High |
|---|---|---|
| People vs things | Solitary | Social |
| Structure vs ambiguity | Clear rules | Open-ended |
| Depth vs breadth | Specialist | Generalist |
| Risk | Stability | Adventure |
| Impact | Self | Society |

Interests affect recommendation weights and content examples, not gate standards.

---

## 6. Constraints

| Constraint | Why it matters | Adaptation |
|---|---|---|
| Time per week | Pace and plan | Adjust expected timeline |
| Device | Modality | Text-first, mobile-first |
| Bandwidth | Content | Offline packs, low-res |
| Budget | Tools | Free/low-cost path |
| Physical environment | Practice | Home alternatives |
| Accessibility | Access | Accommodations, formats |
| Location | Physical skills | Local partners, sims |
| Caregiving | Schedule | Micro-sessions |
| Health | Safety | Scope limits, pacing |
| Language | Comprehension | Translation, plain language |

Constraints are **not** deficits. They change the path, not the standard.

---

## 7. Diagnostic placement

### 7.1 Purpose
Avoid wasting a learner's time on what they already know.

### 7.2 Process
1. Learner selects a skill.
2. Adaptive diagnostic (10–20 items) across bands 1–3.
3. Optional practical task.
4. Placement recommendation: start at level 1, 5, 10, or 15.
5. Learner can accept or start lower.
6. Placement never awards XP/EC; it only sets the start.

### 7.3 Rules
- Placement above level 10 requires a practical task.
- Placement above level 20 requires a proctored assessment.
- Placement is capped at level 30 in v1.
- Placement expires after 12 months.
- A learner can always choose to start at level 1.

---

## 8. Fairness

| Risk | Control |
|---|---|
| Cultural bias in items | DIF analysis; diverse pilot |
| Language bias | Plain language; translation; oral option |
| Disability bias | Accommodations; construct review |
| Socioeconomic bias | No hardware-dependent items |
| Stereotype threat | Neutral framing; no demographic priming |
| Misuse of profile | No employer access; no direct pay |

### 8.1 Adverse impact
- Monitor selection rates by group.
- Ratio ≥0.80; investigate gaps.
- Publish annual fairness report.
- Human review of any high-stakes use.

---

## 9. Learner-facing language

| Avoid | Use |
|---|---|
| "Your IQ is..." | "Your reasoning profile..." |
| "You are low aptitude" | "You may benefit from more scaffolding in..." |
| "You are gifted" | "You show strong readiness in..." |
| "This is your limit" | "This is your current starting point" |
| "You failed" | "Not yet — here is the gap" |

---

## 10. Data and retention

- Raw responses retained 24 months for calibration, then de-identified.
- Profile retained while account is active; deletable.
- Sponsors see aggregate distributions only.
- No sale of profile data.
- Model versions logged.

---

## 11. Requirements

- `FR-ONB-201`: Profiling uses validated, skill-relevant measures.
- `FR-ONB-202`: No single global score is displayed.
- `FR-ONB-203`: Confidence intervals are shown.
- `FR-ONB-204`: Learners can skip and retake.
- `FR-ONB-205`: Accommodations are available for every component.
- `FR-ONB-206`: No protected traits are inferred.
- `FR-ONB-207`: Profile data is not shared with sponsors at individual level.
- `FR-ONB-208`: Fairness analysis runs quarterly.
- `FR-ONB-209`: Placement is capped and requires evidence above level 10.
- `FR-ONB-210`: Profile influences support, not direct EC.

---

## 12. Success metrics

| Metric | Target |
|---|---|
| Profile completion | ≥60% |
| Learner finds profile useful | ≥4/5 |
| Placement accuracy (level 10 retention) | ≥75% |
| Adverse-impact ratio | ≥0.80 |
| Accommodation requests fulfilled | 100% |
| Profile deletion requests completed | 100% within 30 days |
