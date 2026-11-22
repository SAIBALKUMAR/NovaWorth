# Level Curve

## 1. Why 100 levels

A 100-level ladder solves four problems at once:

1. **Granularity.** Learners always know the next step; progress is visible every few hours, not every semester.
2. **Rarity.** High levels are scarce because the curve is super-linear. Level 100 means something.
3. **Narrative.** Ten bands give a clear identity: Foundation → World-Class.
4. **Economy.** Power-law rewards make the climb worth it without paying beginners too little.

The curve is intentionally brutal at the top and generous enough at the bottom to keep people in the game.

---

## 2. The ten bands

| Band | Levels | Identity | Learner can... | Typical proof |
|---|---|---|---|---|
| 1 | 1–10 | **Foundation** | Operate safely and use the basics | Quiz + simple task |
| 2 | 11–20 | **Literacy** | Work independently with guidance | Applied task |
| 3 | 21–30 | **Practitioner** | Produce repeatable quality | Project + rubric |
| 4 | 31–40 | **Builder** | Build real things end to end | Artifact + review |
| 5 | 41–50 | **Specialist** | Diagnose, optimize, handle edge cases | Case study + oral |
| 6 | 51–60 | **Professional** | Deliver under real constraints | Client-style brief |
| 7 | 61–70 | **Senior** | Design systems and lead small teams | System design + sim |
| 8 | 71–80 | **Architect** | Design under uncertainty; handle crises | Live-fire sim + panel |
| 9 | 81–90 | **Master** | Lead, teach, innovate, set standards | Original work + impact |
| 10 | 91–100 | **World-Class** | Advance the field; verified global impact | Panel + portfolio + impact |

---

## 3. Time, difficulty, and reward by band

| Band | Levels | Hours/level (range) | Cumulative hours | Target pass | XP/level (m=1) | EC/level (m=1) |
|---|---|---|---|---|---|---|
| 1 | 1–10 | 1.5–19 | 87 | 90% | 100–3,162 | 10–159 |
| 2 | 11–20 | 22–40 | 316 | 85% | 3,650–8,944 | 175–364 |
| 3 | 21–30 | 43–64 | 700 | 78% | 9,600–16,432 | 400–593 |
| 4 | 31–40 | 67–89 | 1,230 | 70% | 17,500–25,298 | 640–839 |
| 5 | 41–50 | 92–115 | 1,900 | 62% | 26,500–35,355 | 890–1,091 |
| 6 | 51–60 | 118–141 | 2,700 | 55% | 36,700–46,476 | 1,150–1,358 |
| 7 | 61–70 | 145–169 | 3,630 | 48% | 47,900–58,548 | 1,420–1,638 |
| 8 | 71–80 | 173–198 | 4,700 | 40% | 60,100–71,554 | 1,700–1,930 |
| 9 | 81–90 | 202–228 | 5,900 | 30% | 73,200–85,381 | 2,000–2,233 |
| 10 | 91–100 | 231–238 | 11,320 | 20% | 87,100–100,000 | 2,300–2,512 |

> Hours are **deliberate practice** estimates for a prepared learner, not elapsed time. A learner with strong transfer can move faster; a learner starting from zero may need more.

### 3.1 Cumulative reward to level 100 (m=1, Q=E=M=R=1)

| Component | XP | EC |
|---|---|---|
| Level awards | ~4,000,000 | ~114,000 |
| Band bonuses | — | 38,500 |
| **Total** | **~4,000,000** | **~152,500** |

At the baseline anchor of $0.05/EC, that is roughly **$7,600 of redemption value** over ~11,300 hours. It is a meaningful dividend, not a salary. Phase 5 explores livelihood-scale funding.

---

## 4. Why power law

### 4.1 Difficulty
`D(L) = 10 × L^1.35` grows faster than linear. Each level adds new constraints, ambiguity, and integration with prior levels. Level 90 is not "level 9 ten times"; it is a different kind of problem.

### 4.2 Rewards
`XP(L) = 100 × L^1.5` and `EC(L) = 10 × L^1.2` reward the climb. The XP curve is steeper than EC because mastery status should be the strongest signal; EC is capped by funding reality.

### 4.3 Diminishing returns after 100
Prestige rewards decline per level (`EC ∝ 1/n^0.7`), but status and access rise. This prevents infinite farming while preserving the frontier.

### 4.4 Why not exponential?
Exponential curves make mid-levels feel pointless and top levels impossible to fund. Power laws are steep but survivable: level 50 is ~1/8 of level 100's XP, not 1/1,000.

---

## 5. What a level contains

Every level specification has the same 12 fields:

1. **Level ID and title.**
2. **One-sentence outcome:** "After this level, you can..."
3. **Prerequisites:** prior levels, co-requisites, tools.
4. **Why it matters:** real-world consequence.
5. **Learn:** 2–5 concepts, each with an explanation, example, and counterexample.
6. **Practice:** 3–8 exercises, including at least one applied task.
7. **Common mistakes:** misconceptions and how to detect them.
8. **Assessment:** item blueprint + practical rubric + pass criteria.
9. **Evidence:** what the learner submits.
10. **Time and difficulty:** expected hours, D index.
11. **Rewards:** XP, EC, band bonus.
12. **Safety/ethics:** warnings, constraints, escalation.

See [Level authoring standard](../02-skills/03-level-authoring-standard.md).

---

## 6. Level examples (Computing & Software)

| Level | Title | Outcome |
|---|---|---|
| 1 | Computer information | Identify hardware, OS, and basic components |
| 2 | Laptop operations | Manage files, settings, updates, backups |
| 10 | Productivity suites | Build a spreadsheet, document, and slideshow |
| 20 | DSA in Python | Solve 100 graded problems with tests |
| 30 | Go development | Ship a tested, deployed service |
| 40 | Data science & ML | Train, evaluate, and explain a model |
| 50 | LLM agent flows | Build a reliable multi-step agent |
| 60 | Mass execution | Ship a product with agent teams |
| 70 | Advanced security | Threat-model and harden a system |
| 80 | Hyperscale design | Design a social graph at billions of edges |
| 90 | Crisis leadership | Resolve a live multi-service outage |
| 100 | World-class | Original contribution with verified impact |

Full 100-level breakdown: [CS exemplar](../02-skills/06-exemplar-cs.md).

---

## 7. Calibration loop

```
Author level → Pilot with 20–50 learners → Item analysis
      → Adjust difficulty/hints → Re-pilot → Publish
      → Monitor pass rate, time, disputes → Refresh quarterly
```

### 7.1 Healthy signals
- Pass rate within ±10 points of band target.
- Median time within ±25% of `H(L)`.
- Dispute rate <2%.
- Learner-reported clarity ≥4/5.
- Transfer task success correlates with gate score (r >0.5).

### 7.2 Unhealthy signals and fixes
| Signal | Likely cause | Fix |
|---|---|---|
| Pass rate >95% | Too easy / leaked answers | Add constraints, rotate items |
| Pass rate <30% | Unclear prerequisites / bad items | Add scaffolding, rewrite items |
| Time >> expected | Content gap / UX friction | Add examples, split level |
| High disputes | Ambiguous rubric | Clarify criteria, calibrate graders |
| Low transfer | Teaching to the test | Add applied tasks, oral defense |

---

## 8. Post-100 curve

| Prestige level | EC (relative to L100) | XP | Status |
|---|---|---|---|
| P1 | 50% | 10,000 | Frontier initiate |
| P2 | 31% | 14,100 | — |
| P3 | 23% | 17,300 | — |
| P5 | 16% | 22,400 | — |
| P10 | 10% | 31,600 | Frontier fellow |
| P25 | 5% | 50,000 | — |
| P50 | 3% | 70,700 | Grandmaster circle |
| P100 | 2% | 100,000 | Legend |

Formula: `EC(Pn) = EC(100) × 0.5 / n^0.7`; `XP(Pn) = 10,000 × n^0.5`.
See [Post-100 prestige](05-post-100-prestige.md).

---

## 9. Tuning levers

| Lever | Effect | Guardrail |
|---|---|---|
| Exponent on XP | Steeper top-end status | Don't make mid-levels feel worthless |
| Exponent on EC | Steeper material reward | Must stay within sponsor budget |
| Band bonuses | Smooth motivation | Cap total bonus share at 30% |
| Pass thresholds | Rigor | Keep first-attempt pass 55–80% |
| Expected hours | Pace | Don't reward speed over depth |
| Domain multiplier | Supply/demand | Review annually; cap 0.9–1.3 |
| Repeat decay | Anti-farm | Never punish legitimate review |
| Daily cap | Wellbeing | No loss of XP for rest |

---

## 10. Acceptance criteria

- `FR-SKL-101`: Every level has all 12 required fields.
- `FR-SKL-102`: Every level has a calibrated difficulty and expected time.
- `FR-SKL-103`: Every level has at least one applied assessment.
- `FR-SKL-104`: Pass rates are monitored by band and skill.
- `FR-SKL-105`: Levels with pass rate outside 55–95% are reviewed within 14 days.
- `FR-SKL-106`: Level 100 requires panel review in every skill.
- `FR-SKL-107`: The curve constants are configurable per rule version.
- `NFR-SKL-101`: Level metadata loads in <200 ms.
