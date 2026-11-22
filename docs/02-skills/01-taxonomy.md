# Skill Taxonomy

## 1. Purpose

The taxonomy is the map of human capability the product commits to. It must be:
- **Broad enough** to cover real life, not just employability.
- **Deep enough** that every skill can support 100 meaningful levels.
- **Assessable** with credible evidence.
- **AI-resilient** — skills that remain valuable when AI automates routine work.
- **Equitable** — accessible across regions, devices, and abilities.

**Baseline:** 20 domains × 25 skills = **500 skills**. Each skill has 100 levels = **50,000 level specifications**.

The full list is in [Catalog 500](02-catalog-500.md) and [`data/skills-500.csv`](data/skills-500.csv).

---

## 2. The 20 domains

| # | Domain | Code | Why it belongs |
|---|---|---|---|
| 1 | Communication & Language | COM | Every human system runs on communication |
| 2 | Mathematics & Quantitative Reasoning | MTH | Foundation for science, finance, and decisions |
| 3 | Science & Systems Thinking | SCI | Understanding the physical and living world |
| 4 | Computing & Software | CMP | Leverage in an AI-native economy |
| 5 | AI & Data | AID | The new literacy of production and judgment |
| 6 | Health & Body | HLT | Survival, dignity, and care |
| 7 | Mental Health & Mind | MND | Inner life and sustainable performance |
| 8 | Cooking & Nutrition | COK | Daily necessity and cultural identity |
| 9 | Home & Practical Trades | HOM | Physical world maintenance and repair |
| 10 | Finance & Economics | FIN | Navigating money, risk, and institutions |
| 11 | Civics & Law | CIV | Rights, obligations, and collective decisions |
| 12 | Work & Career | WRK | Turning capability into opportunity |
| 13 | Business & Entrepreneurship | BIZ | Creating value and organizations |
| 14 | Arts & Design | ART | Meaning, beauty, and communication |
| 15 | Music & Performance | MUS | Expression, presence, and joy |
| 16 | Media & Storytelling | MED | Information, narrative, and public trust |
| 17 | Sports & Movement | SPT | Physical health and embodied skill |
| 18 | Outdoors & Survival | OUT | Resilience in the physical world |
| 19 | Relationships & Parenting | REL | The core of human life |
| 20 | Learning & Meta-Skills | LRN | The multiplier on everything else |

---

## 3. Selection criteria

A skill is admitted only if it passes all seven tests:

| Criterion | Question | Evidence |
|---|---|---|
| **Human need** | Does it improve safety, health, autonomy, connection, or opportunity? | Expert review, learner demand |
| **AI resilience** | Does it remain valuable when routine tasks are automated? | Task analysis |
| **Depth** | Can it support 100 levels without padding? | Level-100 definition + band outline |
| **Assessability** | Can we verify it credibly? | Evidence model |
| **Teachable** | Can it be learned through available modalities? | Content prototype |
| **Economic demand** | Will sponsors, employers, or learners pay for it? | Market signal |
| **Equity** | Can it be learned without rare hardware or privilege? | Access review |

### 3.1 Red flags (rejected)
- Pseudoscience or unfalsifiable claims.
- Skills that require harming others.
- Gambling, speculation, or exploitation.
- Content that cannot be assessed beyond self-report.
- Skills that duplicate another skill without a distinct level-100 outcome.
- Skills that are purely a single tool (tools change; capabilities endure).

---

## 4. Archetypes

Every skill is assigned an archetype that determines its band template, assessment style, and typical evidence.

| Archetype | Description | Example skills | Dominant evidence |
|---|---|---|---|
| **Foundation** | Daily-life literacy and self-care | Sleep, hygiene, money basics | Quiz + observed task |
| **Craft** | Physical making and skilled handwork | Knife skills, plumbing, woodworking | Artifact + practical rubric |
| **Analytical** | Reasoning, modeling, evidence | Statistics, physics, logic | Problems + oral defense |
| **Technical** | Building and operating systems | Backend, networking, ML | Code/system + sim |
| **Creative** | Making meaning and beauty | Writing, design, music | Portfolio + critique |
| **Relational** | Working with people and oneself | Listening, conflict, care | Role-play + reflective artifact |
| **Institutional** | Navigating rules and organizations | Contracts, taxes, policy | Case + artifact |
| **Entrepreneurial** | Creating value under uncertainty | Sales, fundraising, operations | Venture artifact + sim |
| **Physical** | Embodied performance and safety | Running, climbing, first aid | Observed performance + sim |
| **Meta** | Learning and thinking about thinking | Deliberate practice, bias | Artifact + transfer task |

### 4.1 Archetype mix
| Archetype | Skills |
|---|---:|
| Foundation | 25 |
| Craft | 50 |
| Analytical | 50 |
| Technical | 50 |
| Creative | 75 |
| Relational | 75 |
| Institutional | 75 |
| Entrepreneurial | 25 |
| Physical | 50 |
| Meta | 25 |
| **Total** | **500** |

---

## 5. Level bands

All skills use the same 10-band structure:

| Band | Levels | Developmental task |
|---|---|---|
| 1 | 1–10 | Operate safely; know the vocabulary |
| 2 | 11–20 | Perform independently with guidance |
| 3 | 21–30 | Produce repeatable quality |
| 4 | 31–40 | Build real things end to end |
| 5 | 41–50 | Diagnose, optimize, handle edge cases |
| 6 | 51–60 | Deliver under real constraints |
| 7 | 61–70 | Design systems; lead small teams |
| 8 | 71–80 | Design under uncertainty; handle crises |
| 9 | 81–90 | Lead, teach, innovate, set standards |
| 10 | 91–100 | Advance the field; verified world-class impact |

The archetype determines *what* the developmental task looks like, not whether it exists. See [Level authoring standard](03-level-authoring-standard.md) for per-archetype templates.

---

## 6. Cross-domain dependencies

Skills are not silos. The graph has three edge types:

| Edge | Meaning | Example |
|---|---|---|
| `requires` | Hard prerequisite | DSA requires Programming fundamentals |
| `amplifies` | Improves performance | Statistics amplifies Data literacy |
| `combines` | Enables a constellation | Health + AI + Communication |

### 6.1 Core prerequisite spine
```
Number sense → Arithmetic → Fractions → Algebra → Statistics → Modeling
Computer literacy → OS → Internet → Programming → DSA → Software design
Communication → Feedback → Conflict → Leadership
Health basics → First aid → Chronic care → Public health
Learning how to learn → Deliberate practice → Transfer → Mastery
```

### 6.2 Constellation examples
| Constellation | Skills |
|---|---|
| AI-native clinic | HLT + AID + COM + BIZ |
| Resilient city | CIV + SCI + HOM + FIN |
| Zero-waste restaurant | COK + BIZ + HOM + MED |
| Learning product | LRN + CMP + ART + BIZ |
| Crisis newsroom | MED + CIV + COM + AID |

---

## 7. Skill identity

Each skill has:
- **ID:** `DOMAIN-NN` (e.g., `CMP-06`).
- **Name:** short, concrete, tool-agnostic.
- **One-line promise:** what the learner can do at level 10.
- **Level-100 outcome:** world-class proof.
- **Archetype.**
- **Prerequisites.**
- **Assessment modes.**
- **Domain multiplier.**
- **Safety classification:** `standard`, `caution`, `high-risk`.
- **Localization notes.**
- **Version.**

---

## 8. Governance

### 8.1 Domain councils
Each domain has a council of 3–5 experts:
- 1 practitioner at level 80+ equivalent.
- 1 instructional designer.
- 1 assessment specialist.
- 1 learner advocate.
- 1 safety/ethics reviewer (for high-risk domains).

### 8.2 Change process
1. Proposal with rationale and evidence.
2. Impact analysis (content, assessments, credentials, economy).
3. Council review.
4. 30-day learner notice for removals or level-100 changes.
5. Version bump; grandfather existing credentials.
6. Publish changelog.

### 8.3 What can change
| Change | Allowed? | Notes |
|---|---|---|
| Add a skill | Yes | Must pass all seven criteria |
| Remove a skill | Rare | Grandfather credentials; 90-day notice |
| Rename a skill | Yes | Keep ID; update aliases |
| Change level 100 | Rare | Requires panel and notice |
| Reorder levels | Rare | Requires mapping and migration |
| Change domain multiplier | Yes | Annual review; grandfather awards |

---

## 9. Equity and access

- Every skill must have at least one low-cost, low-bandwidth learning path.
- Physical skills must offer home/low-equipment variants where possible.
- High-risk skills must include safety warnings and cannot claim licensure.
- Language and cultural examples must be reviewed for local fit.
- No skill may require a paid subscription to attempt level 1.

---

## 10. Excluded and deferred skills

### 10.1 Excluded
- Pseudoscientific health practices.
- Gambling and speculative trading strategies.
- Manipulation, coercion, and dark patterns.
- Weapons use (except regulated safety awareness).
- Any skill whose level 100 requires harming a person or animal.

### 10.2 Deferred
- Regulated professions requiring licensure (medicine, law, aviation).
- Highly local trades with no scalable assessment.
- Emerging skills without a stable body of practice.
- Skills that depend on a single proprietary tool.

---

## 11. Requirements

- `FR-SKL-001`: The catalog contains exactly 500 active skills across 20 domains.
- `FR-SKL-002`: Each skill has a unique ID, archetype, level-100 outcome, and assessment modes.
- `FR-SKL-003`: Each skill has 100 level specifications before it is marked "complete."
- `FR-SKL-004`: Cross-domain prerequisites are modeled and visible.
- `FR-SKL-005`: Domain councils review changes quarterly.
- `FR-SKL-006`: Skill changes are versioned and learner-visible.
- `FR-SKL-007`: High-risk skills carry safety classification and warnings.
- `FR-SKL-008`: Every skill has a low-cost learning path.
- `NFR-SKL-001`: Catalog search returns in <200 ms.
- `NFR-SKL-002`: Catalog data is available offline on mobile.

---

## 12. Success metrics

| Metric | Target |
|---|---|
| Skills with full 100 levels | 500 by Phase 4 |
| Skills with 3+ modalities | ≥90% by Phase 3 |
| Skills with validated level-100 panel | 100% |
| Cross-domain prerequisite coverage | 100% of skills |
| Learner-reported catalog relevance | ≥4/5 |
| Underrepresented-domain enrollment | ≥30% of learners |
| Skill change disputes | <1% |
