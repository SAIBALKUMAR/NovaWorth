# Recommendation Engine

## 1. Purpose

The recommender answers: **"Which skill gives this person the most return for the effort they can realistically invest?"**

It is not a popularity engine. It combines aptitude, interest, transferable experience, opportunity, and feasibility — and it explains itself.

**Agency rule:** the learner can choose any of the 500 skills. The recommender advises; it never gates.

---

## 2. Fit score

```
Fit = 0.30 × Aptitude
    + 0.25 × Interest
    + 0.20 × Transferable experience
    + 0.15 × Opportunity
    + 0.10 × Feasibility
```

Each component is normalized to 0–100. Final fit is 0–100.

### 2.1 Component definitions

| Component | Inputs | Notes |
|---|---|---|
| **Aptitude** | Diagnostic scores, learning agility, prior knowledge | Skill-relevant, not global IQ |
| **Interest** | Card sort, goals, free text | Intrinsic motivation predicts persistence |
| **Transferable experience** | Experience graph, evidence | Verified > evidenced > self-reported |
| **Opportunity** | Employer demand, sponsor demand, scarcity, prestige | Regional and temporal |
| **Feasibility** | Time, device, budget, location, accessibility | Constraints reduce fit, not capability |

### 2.2 Weights by situation
| Situation | Adjustment |
|---|---|
| Career switcher | Opportunity +0.05, Feasibility +0.05 |
| Student | Aptitude +0.05, Interest +0.05 |
| Returner | Transferable +0.10, Feasibility +0.05 |
| Senior professional | Aptitude +0.05, Opportunity +0.05 |
| Sponsor cohort | Opportunity +0.10 (within cohort goals) |
| Accessibility-first | Feasibility +0.10 |

Weights are versioned and reviewed quarterly.

---

## 3. Scoring detail

### 3.1 Aptitude
```
aptitude = 0.4 × domain_diagnostic
         + 0.2 × fluid_reasoning
         + 0.2 × learning_agility
         + 0.2 × prior_knowledge
```
- Domain diagnostic is weighted highest because it is the most predictive.
- Missing components are imputed with cohort priors and flagged low-confidence.

### 3.2 Interest
```
interest = 0.5 × explicit_interest
         + 0.3 × goal_alignment
         + 0.2 × energizing_activities
```

### 3.3 Transferable experience
```
transfer = 0.6 × verified_or_evidenced
         + 0.3 × self_reported
         + 0.1 × inferred
```
Cap self-reported contribution at 50% to prevent overclaiming.

### 3.4 Opportunity
```
opportunity = 0.4 × employer_demand
            + 0.3 × sponsor_demand
            + 0.2 × scarcity
            + 0.1 × prestige
```
Regional data; updated quarterly.

### 3.5 Feasibility
```
feasibility = 1 - constraint_penalty
```
Penalties:
- Time <3h/week: −20.
- No device: −40.
- Low bandwidth: −10 (text path available).
- No local access for physical skill: −30.
- Budget <$20: −15.
- Accessibility gap: −50 (until fixed).
- High-risk without supervision: −20.

---

## 4. Diversity and portfolio

A good recommendation set is not four variations of the same skill.

### 4.1 Selection rules
- Top 3–4 must span ≥2 domains.
- Include:
  - **Best fit** (highest score).
  - **Quick win** (fastest to level 10).
  - **Stretch** (high upside, higher difficulty).
  - **Wildcard** (adjacent interest, low overlap).
- No skill appears twice.
- At least one path must be feasible on the learner's device.

### 4.2 Example set
| Type | Skill | Fit | Time to L10 | Why |
|---|---|---|---|---|
| Best fit | AI operations | 88 | 6 weeks | Strong transfer + interest + demand |
| Quick win | Clear writing | 82 | 3 weeks | High aptitude + fast payoff |
| Stretch | Data analysis | 76 | 10 weeks | High demand, needs math ramp |
| Wildcard | UX research | 74 | 8 weeks | Uses empathy, new domain |

---

## 5. Explainability

Every recommendation shows:
- Top 3 factors with weights.
- Confidence level (low/medium/high).
- Expected time to level 10.
- Expected EC at level 10 (range, not promise).
- Difficulty and pass rate.
- One sample level.
- What would change the recommendation.

Example:
> **AI operations — 88 fit (high confidence)**
> Because: your support experience maps to 6 of 10 core skills; you scored 78th percentile in reasoning; AI roles are in high demand in your region.
> Time to level 10: ~6 weeks at 8h/week.
> Watch out: level 20+ requires SQL; we can add a bridge.

---

## 6. Cold start

| Data available | Strategy |
|---|---|
| Nothing | 7-day sampler across 3 domains; interest card sort |
| Interests only | Content-based similarity; popular beginner paths |
| Work history only | Transferable mapping + domain tasters |
| Diagnostic only | Aptitude-led recommendations |
| Sponsor cohort | Cohort goals + individual fit |
| Returning learner | Prior progress + updated interests |

### 6.1 Sampler
- 3 domains × 3 micro-missions (10 min each).
- Measures enjoyment, performance, and persistence.
- Produces a fit estimate without a long survey.

---

## 7. Feedback and learning

### 7.1 Explicit feedback
- "Why this?" expander.
- "Not for me" with reasons.
- "I want this instead" (captures preference).
- Post-level: "Was this a good fit?"

### 7.2 Implicit feedback
- Start/complete rates.
- Time to first attempt.
- Retry persistence.
- Skill switching.
- Satisfaction.

### 7.3 Model updates
- Online updates to interest and feasibility.
- Weekly retraining of opportunity weights.
- Quarterly fairness and bias audit.
- No feedback loop that narrows options unfairly.

---

## 8. Anti-patterns

| Anti-pattern | Why it is banned |
|---|---|
| Recommending only high-demand skills | Ignores agency and fit |
| Recommending only easy skills | Prevents growth |
| Recommending based on demographics | Discriminatory and illegal |
| Hiding the full catalog | Reduces agency |
| Paid placement in recommendations | Corrupts advice |
| Overconfident predictions | Misleads learners |
| Ignoring constraints | Sets people up to fail |
| Recommending a skill the learner cannot access | Wastes effort |

---

## 9. Evaluation

| Metric | Target |
|---|---|
| Recommendation acceptance | ≥45% |
| 30-day retention of accepted path | ≥50% |
| Fit self-report | ≥4/5 |
| Diversity of top 4 | ≥2 domains |
| Calibration error | <10 points |
| Adverse-impact ratio | ≥0.80 |
| Learner-reported regret | <10% |
| Time to first level pass | <7 days |

### 9.1 Offline evaluation
- Holdout cohorts.
- Counterfactual evaluation.
- Calibration curves.
- Fairness slices.

### 9.2 Online evaluation
- A/B tests with guardrails.
- Long-term retention, not just clicks.
- Monitor for filter bubbles.

---

## 10. Requirements

- `FR-ONB-301`: Recommendations show top factors and confidence.
- `FR-ONB-302`: Top 4 span at least 2 domains.
- `FR-ONB-303`: A quick win and a stretch are always included.
- `FR-ONB-304`: The full catalog is always accessible.
- `FR-ONB-305`: Learners can reject a recommendation with a reason.
- `FR-ONB-306`: No paid placement.
- `FR-ONB-307`: No demographic targeting.
- `FR-ONB-308`: Cold start uses a sampler.
- `FR-ONB-309`: Recommendations update with new evidence.
- `FR-ONB-310`: Fairness audit quarterly.
- `NFR-ONB-301`: Recommendations render in <1 s.
- `NFR-ONB-302`: Explanations are available in all supported languages.
