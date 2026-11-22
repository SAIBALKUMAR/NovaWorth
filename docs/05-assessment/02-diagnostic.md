# Diagnostic Assessment

## 1. Purpose

The diagnostic does three jobs:
1. **Place** the learner at the right starting level.
2. **Profile** strengths and gaps.
3. **Recommend** a path with evidence.

It is not a pass/fail exam and never awards XP or EC.

---

## 2. Design principles

- **Adaptive:** items adjust to the learner.
- **Short:** 10–20 minutes for placement; 25 minutes for full profile.
- **Domain-specific:** measures the skill being considered.
- **Low-stakes:** no penalty; can retake.
- **Transparent:** learner sees what was measured and why.
- **Fair:** accessible, culturally reviewed, no protected traits.

---

## 3. Components

| Component | Purpose | Items | Time |
|---|---|---|---|
| Domain knowledge | Prior knowledge | 8–15 | 8 min |
| Applied reasoning | Transfer | 3–5 | 5 min |
| Aptitude (optional) | Readiness | 10–20 | 10 min |
| Effort task (optional) | Persistence | 1 | 3 min |
| Self-report | Confidence/goals | 5 | 2 min |

---

## 4. Adaptive algorithm

### 4.1 Item selection
- Start at medium difficulty (θ = 0).
- Select the item with maximum information at current θ.
- Update θ after each response.
- Stop when standard error <0.3 or max items reached.

### 4.2 Scoring
- IRT 2PL for knowledge items.
- Rubric for applied tasks.
- Weighted composite:
```
diagnostic_score = 0.7 × knowledge
                 + 0.2 × applied
                 + 0.1 × self_report
```

### 4.3 Placement
| Score | Recommended start | Requirement |
|---|---|---|
| <40 | Level 1 | None |
| 40–59 | Level 5 | None |
| 60–74 | Level 10 | Practical task |
| 75–84 | Level 15 | Practical + review |
| 85–92 | Level 20 | Proctored practical |
| >92 | Level 25–30 | Proctored + panel |

Placement is capped at 30 in v1.

---

## 5. Report

The learner sees:
- Estimated level range.
- Strengths (top 3).
- Gaps (top 3).
- Recommended start.
- A sample level.
- What would change the estimate.
- Confidence.

Example:
> **Computing & Software:** estimated level 12–16 (medium confidence).
> Strengths: file systems, basic Python, debugging.
> Gaps: data structures, testing, Git workflows.
> Recommended start: level 10, with a bridge to 11–13.

---

## 6. Placement rules

- Placement never awards XP/EC.
- Placement above level 10 requires a practical task.
- Placement above level 20 requires proctoring.
- Placement expires after 12 months.
- Learner can always start lower.
- Learner can challenge placement with a gate attempt.
- Misplacement is corrected after the first 3 levels.

---

## 7. Cold start

| Data | Strategy |
|---|---|
| No data | Interest + 3-domain sampler |
| Work history | Transferable mapping + domain diagnostic |
| Prior credentials | Verify and place |
| Sponsor cohort | Cohort baseline + individual diagnostic |
| Returning | Prior mastery + decay adjustment |

---

## 8. Fairness

- DIF analysis.
- Language review.
- Cultural review.
- Accessibility accommodations.
- No demographic inference.
- Adverse-impact monitoring.

---

## 9. Requirements

- `FR-ASM-301`: Diagnostic is adaptive.
- `FR-ASM-302`: Diagnostic never awards XP/EC.
- `FR-ASM-303`: Placement above level 10 requires evidence.
- `FR-ASM-304`: Learner can accept a lower placement.
- `FR-ASM-305`: Report shows confidence and gaps.
- `FR-ASM-306`: Diagnostic can be retaken after 30 days.
- `FR-ASM-307`: Diagnostic is accessible.
- `FR-ASM-308`: Diagnostic items are calibrated.
- `FR-ASM-309`: No protected traits are inferred.
- `FR-ASM-310`: Placement is reviewed if the learner fails 3 levels.
- `NFR-ASM-301`: Diagnostic completion ≤25 min.
- `NFR-ASM-302`: Results in <5 s.
