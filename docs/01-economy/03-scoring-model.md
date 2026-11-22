# Scoring Model

## 1. Purpose

This is the exact, versioned scoring specification. It defines how difficulty, XP, and EC are computed, how multipliers work, and how fairness and anti-gaming are enforced.

**Design stance:** the user asked for higher IQ/effort to earn more and lower IQ/effort to earn less. We implement that as **demonstrated performance and deliberate effort at the level**, not as a payout based on a global IQ label. This is more predictive, more fair, and legally safer. A learner with high aptitude will naturally earn higher quality/mastery multipliers and finish faster; a learner with lower aptitude still gets a guaranteed base award and a growth dividend.

---

## 2. Core variables

| Symbol | Meaning | Range / default |
|---|---|---|
| `L` | Level number | 1–100 (prestige 101+) |
| `m_d` | Domain multiplier | 0.9–1.3 |
| `D(L)` | Difficulty index | `10 × L^1.35` |
| `H(L)` | Expected hours for the level | `1.5 × L^1.1` |
| `XP(L)` | Mastery points | `100 × L^1.5 × m_d` |
| `EC_base(L)` | Base credit award | `10 × L^1.2 × m_d` |
| `Q` | Quality multiplier | 0.8–1.25 |
| `E` | Effort/efficiency multiplier | 0.9–1.2 |
| `M` | Mastery/transfer multiplier | 0.85–1.5 |
| `R` | Repeat decay | 1.0 / 0.2 / 0.05 / 0 |
| `Cap` | Total multiplier cap | 2.0 |
| `A` | Aptitude signal (diagnostic) | 0–1; used for readiness, not direct pay |

### 2.1 Final formulas

```
XP_award = round( XP(L) × Q × M )
EC_award = round( EC_base(L) × Q × E × M × R )
```

- `Q` and `M` apply to XP; `E` and `R` apply only to EC.
- Total EC multiplier `Q × E × M` is capped at `2.0`; floor at `0.6`.
- XP is never reduced by repeat decay; mastery is permanent. Repeating a level yields 10% XP (capped at 1 repeat) and 0 EC after the third pass.

### 2.2 Domain multipliers

| Domain | `m_d` | Rationale |
|---|---|---|
| Health & Body | 1.30 | Safety-critical, high demand |
| Mental Health & Mind | 1.25 | Scarce, high impact |
| Home & Practical Trades | 1.20 | Physical scarcity, aging infrastructure |
| Computing & Software | 1.20 | Economic leverage |
| AI & Data | 1.20 | Frontier demand |
| Science & Systems | 1.15 | Long-horizon value |
| Mathematics | 1.15 | Foundation for other skills |
| Communication & Language | 1.10 | Universal leverage |
| Finance & Economics | 1.10 | High consequence |
| Civics & Law | 1.10 | Institutional navigation |
| Work & Career | 1.05 | Broad but commoditized |
| Business & Entrepreneurship | 1.05 | High variance |
| Cooking & Nutrition | 1.05 | Daily necessity |
| Relationships & Parenting | 1.05 | High impact, hard to assess |
| Learning & Meta-Skills | 1.00 | Multiplier on everything |
| Outdoors & Survival | 1.00 | Niche, safety-sensitive |
| Sports & Movement | 0.95 | Access varies |
| Arts & Design | 0.95 | Subjective, abundant supply |
| Music & Performance | 0.95 | Subjective, abundant supply |
| Media & Storytelling | 0.95 | Fast-changing, abundant supply |

Domain multipliers are reviewed annually by the Economy Council. Changes are grandfathered for existing levels.

---

## 3. Difficulty and time curve

### 3.1 Difficulty
`D(L) = 10 × L^1.35`

| Level | D | Relative to L1 |
|---|---|---|
| 1 | 10 | 1× |
| 10 | 225 | 22× |
| 25 | 812 | 81× |
| 50 | 1,977 | 198× |
| 75 | 3,414 | 341× |
| 100 | 5,012 | 501× |

Difficulty drives matchmaking, readiness warnings, and simulation complexity. It is not directly paid; pay uses XP/EC formulas.

### 3.2 Expected hours
`H(L) = 1.5 × L^1.1`

| Level | Hours for level | Cumulative to level |
|---|---|---|
| 1 | 1.5 | 1.5 |
| 10 | 18.8 | 87 |
| 20 | 40.4 | 316 |
| 30 | 63.8 | 700 |
| 40 | 88.6 | 1,230 |
| 50 | 114.5 | 1,900 |
| 60 | 141.4 | 2,700 |
| 70 | 169.2 | 3,630 |
| 80 | 198.0 | 4,700 |
| 90 | 227.6 | 5,900 |
| 100 | 237.7 | 11,320 |

The cumulative curve is deliberately super-linear. Reaching level 100 in one skill is roughly **5.4 years at 40 hours/week** of deliberate practice — comparable to the "10,000-hour" mastery heuristic, with structured assessment.

### 3.3 Target first-attempt pass rates

| Band | Levels | Target pass (prepared learner) |
|---|---|---|
| 1 | 1–10 | 90% |
| 2 | 11–20 | 85% |
| 3 | 21–30 | 78% |
| 4 | 31–40 | 70% |
| 5 | 41–50 | 62% |
| 6 | 51–60 | 55% |
| 7 | 61–70 | 48% |
| 8 | 71–80 | 40% |
| 9 | 81–90 | 30% |
| 10 | 91–100 | 20% (panel: 5–10%) |

These are targets, not quotas. Calibration uses real attempt data; items are adjusted, not learners.

---

## 4. Multipliers

### 4.1 Quality `Q`
Based on the gate result and evidence quality.

| Result | `Q` | Criteria |
|---|---|---|
| Pass | 0.80 | Meets minimum |
| Solid | 1.00 | Meets all criteria comfortably |
| Distinction | 1.15 | 95%+ knowledge or 3.5/4 practical |
| Outstanding | 1.25 | 98%+ and exemplary artifact/transfer |

### 4.2 Effort `E`
Based on deliberate-practice signals relative to `H(L)`:
- Active practice time (not idle video).
- Retrieval attempts, revisions, and spaced reviews.
- Persistence after failure.
- Quality of notes/artifacts.
- No credit for passive watching, tab-open time, or >6h/day.

| Signal | `E` |
|---|---|
| Minimal but passed | 0.90 |
| Expected engagement | 1.00 |
| High deliberate practice | 1.10 |
| Exceptional, sustained, efficient | 1.20 |

`E` is capped and cannot compensate for a low `Q`. It rewards process, not suffering.

### 4.3 Mastery `M`
Based on evidence of transfer and above-level performance:
- Correctly solves unseen, above-level problems.
- Explains the underlying principle.
- Transfers to a new context.
- Teaches it back accurately.
- Produces a novel, correct artifact.

| Evidence | `M` |
|---|---|
| Pass at level only | 0.85 |
| Solid level performance | 1.00 |
| Transfer task solved | 1.15 |
| Above-level challenge solved | 1.30 |
| Novel contribution / expert commendation | 1.50 |

`M` is the main channel through which high aptitude earns more. It requires evidence; it cannot be claimed.

### 4.4 Repeat decay `R`
| Attempt | EC multiplier | XP multiplier |
|---|---|---|
| 1st pass | 1.00 | 1.00 |
| 2nd pass | 0.20 | 0.10 |
| 3rd pass | 0.05 | 0.00 |
| 4th+ | 0.00 | 0.00 |

Repeats are allowed for learning but not for farming. A "repeat" means passing the same level again after already passing it. Failed attempts do not count as repeats.

### 4.5 Growth dividend
To avoid a rich-get-richer spiral:
- Learners in the bottom 25% of diagnostic aptitude receive a **+0.10 EC multiplier** for levels 1–30 ("growth dividend").
- Learners who improve their diagnostic score by ≥15% receive a one-time bonus.
- Need-based sponsor pools can add a flat stipend, separate from scoring.
- No learner can earn below the base award for a valid pass.

### 4.6 Cap and floor
```
multiplier = clamp(Q × E × M, 0.6, 2.0)
```
This prevents extreme outliers and protects against gaming one dimension.

---

## 5. Worked examples

Assume `m_d = 1.1` (Computing) unless noted.

### Example A — steady learner, level 10
- Q=0.8, E=1.0, M=0.85, R=1.0.
- XP = 100 × 10^1.5 × 1.1 = 3,478.
- EC_base = 10 × 10^1.2 × 1.1 = 174.
- EC = 174 × 0.8 × 1.0 × 0.85 = 118.

### Example B — strong learner, level 10
- Q=1.15, E=1.1, M=1.3, R=1.0.
- XP = 3,478 × 1.15 × 1.3 = 5,199.
- EC = 174 × 1.15 × 1.1 × 1.3 = 286.

### Example C — level 50, distinction
- Q=1.15, E=1.1, M=1.15, R=1.0.
- XP = 100 × 50^1.5 × 1.1 = 38,890.
- EC_base = 10 × 50^1.2 × 1.1 = 1,200.
- EC = 1,200 × 1.15 × 1.1 × 1.15 = 1,746.

### Example D — level 100, world-class
- Q=1.25, E=1.15, M=1.5, R=1.0; cap applies at 2.0.
- XP = 100 × 100^1.5 × 1.1 = 110,000 × 1.25 × 1.5 = 206,250.
- EC_base = 10 × 100^1.2 × 1.1 = 2,763.
- EC = 2,763 × 2.0 = 5,526.

### Example E — repeat of level 30
- Base EC 685; R=0.2; Q=1.0, E=1.0, M=1.0 → 137 EC.
- Third pass: 34 EC. Fourth: 0.

### Example F — low-aptitude learner with growth dividend
- Level 5, Q=0.8, E=1.0, M=0.85, growth +0.10.
- EC_base = 10 × 5^1.2 × 1.0 = 69.
- EC = 69 × 0.8 × 1.0 × 0.85 × 1.10 = 52.

---

## 6. Aptitude and effort profiling

### 6.1 What we measure
- **Fluid reasoning:** pattern completion, novel problem solving.
- **Working memory:** sequence and manipulation tasks.
- **Numeracy:** quantitative reasoning.
- **Verbal reasoning:** inference and argument.
- **Spatial reasoning:** mental rotation, navigation.
- **Learning agility:** improvement across repeated novel tasks.
- **Effort profile:** persistence, error correction, help-seeking, reflection.

### 6.2 What we do not do
- No single "IQ score" displayed to learners or sponsors.
- No use of protected traits.
- No direct EC payout based on diagnostic score.
- No labeling of children.
- No hiring recommendation based solely on aptitude.

### 6.3 How aptitude affects the experience
| Use | Effect |
|---|---|
| Readiness | Recommended starting level and pace |
| Content difficulty | Adaptive challenge selection |
| Recommendations | Fit score weighting |
| Mastery multiplier | Only via demonstrated transfer at the level |
| Support | Scaffolds, hints, tutoring intensity |

### 6.4 Fairness checks
- Adverse-impact ratio by region, gender, age, disability, and language ≥0.80.
- Differential item functioning (DIF) analysis each quarter.
- Human review of any group gap >5 percentage points.
- Public fairness report annually.

---

## 7. Anti-gaming rules

| Threat | Control |
|---|---|
| Answer sharing | Randomized item banks, per-attempt variants, watermarking |
| AI answering | Oral defense, artifact provenance, in-person/proctored gates, process evidence |
| Account sharing | Device binding, behavioral signals, liveness checks at high stakes |
| Grinding | Repeat decay, daily caps, quality gates |
| Collusion | Graph analysis, pair/group anomaly detection |
| Content leakage | Item rotation, canary items, leak detection |
| Rubric gaming | Hidden criteria weights, human sampling, adversarial artifacts |
| Time fraud | Active-time measurement, not wall-clock |
| Sponsor pressure | Score isolation, audit logs, conflict-of-interest rules |

---

## 8. Rule versioning

- Every score references `rule_version` (e.g., `score-2026.1`).
- Rules are immutable once published.
- Changes require: impact simulation, fairness review, 30-day notice, grandfathering.
- Historical awards retain their original rule version.
- A learner can view the rule used for any award.

---

## 9. Pseudocode

```ts
function computeAward(input: AwardInput): Award {
  const { level, domainMultiplier, quality, effort, mastery, repeatIndex, aptitudePercentile } = input;

  const baseXp = 100 * Math.pow(level, 1.5) * domainMultiplier;
  const baseEc = 10 * Math.pow(level, 1.2) * domainMultiplier;

  const repeatEc = [1, 0.2, 0.05, 0][Math.min(repeatIndex, 3)];
  const repeatXp = [1, 0.1, 0, 0][Math.min(repeatIndex, 3)];

  const growth = aptitudePercentile <= 25 && level <= 30 ? 1.1 : 1.0;
  const multiplier = clamp(quality * effort * mastery * growth, 0.6, 2.0);

  return {
    xp: Math.round(baseXp * quality * mastery * repeatXp),
    ec: Math.round(baseEc * multiplier * repeatEc),
    ruleVersion: "score-2026.1",
  };
}
```

---

## 10. Acceptance criteria

- `FR-ECN-201`: The same attempt always produces the same award for the same rule version.
- `FR-ECN-202`: Every award displays its multipliers and rule version to the learner.
- `FR-ECN-203`: Repeats decay exactly as specified.
- `FR-ECN-204`: Multipliers are capped and floored.
- `FR-ECN-205`: Aptitude never directly sets EC; only readiness and evidence-based mastery.
- `FR-ECN-206`: Fairness report is generated quarterly.
- `FR-ECN-207`: A rule change cannot alter historical awards.
- `NFR-ECN-201`: Award computation <50 ms.
- `NFR-ECN-202`: 100% of awards have a rule version and evidence link.
