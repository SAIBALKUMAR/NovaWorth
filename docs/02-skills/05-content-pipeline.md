# Content Pipeline

## 1. Purpose

The content pipeline turns expert knowledge into 50,000 assessed levels without collapsing under cost or inconsistency. It is a **human-AI factory**: AI accelerates drafting and variation; humans own accuracy, pedagogy, safety, and final sign-off.

**Target:** cost per accepted level <$150; throughput 25–50 levels/week at Phase 2; 500 skills fully authored by Phase 4.

---

## 2. Roles

| Role | Responsibility | Ratio |
|---|---|---|
| **Domain lead** | Owns the 25-skill domain map and level-100 standards | 1 per domain |
| **Subject-matter expert (SME)** | Writes/validates technical content | 1 per 3–5 skills |
| **Instructional designer (ID)** | Structures outcomes, practice, and scaffolds | 1 per 10 skills |
| **Assessment specialist** | Builds blueprints, items, rubrics, calibration | 1 per 15 skills |
| **Editor** | Clarity, consistency, style, localization readiness | 1 per 20 skills |
| **Safety/ethics reviewer** | High-risk content and scope limits | 1 per high-risk domain |
| **AI content engineer** | Prompts, pipelines, provenance, evals | 1 per 50 skills |
| **QA reviewer** | Functional, accessibility, link, media checks | 1 per 30 skills |
| **Learner panel** | Pilot feedback and clarity | 10–30 per skill |

---

## 3. Pipeline stages

```
[0] DEMAND & PRIORITIZATION
        │
[1] SKILL BRIEF (level-100 outcome, archetype, prerequisites)
        │
[2] BAND OUTLINE (10 bands × 10 levels, outcomes)
        │
[3] LEVEL DRAFT (SME + AI, 12 fields)
        │
[4] PEDAGOGY REVIEW (ID: outcomes, practice, scaffolds)
        │
[5] ASSESSMENT BUILD (blueprint, items, rubric, anchors)
        │
[6] SAFETY & ACCESSIBILITY REVIEW
        │
[7] PILOT (10–50 learners)
        │
[8] CALIBRATION & REVISION
        │
[9] PUBLISH (versioned, localized)
        │
[10] MONITOR & REFRESH (quarterly)
```

### 3.1 Stage details

| Stage | Input | Output | Owner | Gate |
|---|---|---|---|---|
| 0 | Learner demand, sponsor demand, gaps | Prioritized backlog | Product | Council approval |
| 1 | Skill name | Skill brief (1 page) | Domain lead | Level-100 validated |
| 2 | Brief | 100 outcomes grouped in bands | SME + ID | Coherence review |
| 3 | Outcomes | Level specs (12 fields) | SME + AI | Completeness |
| 4 | Specs | Pedagogical review notes | ID | Learning quality |
| 5 | Specs | Items, rubrics, anchors | Assessment | Psychometric quality |
| 6 | All | Safety/accessibility sign-off | Reviewers | No blockers |
| 7 | Level | Pilot data | Research | 10+ learners |
| 8 | Pilot data | Revised level | ID + Assessment | Pass rate target |
| 9 | Approved level | Published version | Content ops | Checklist |
| 10 | Live data | Refresh plan | Domain lead | Quality thresholds |

---

## 4. AI use policy

### 4.1 Allowed
- Drafting explanations, examples, and practice variations.
- Generating item candidates from a blueprint.
- Adapting reading level and locale.
- Creating alt text, captions, and summaries.
- Translating with human review.
- Building simulation scenarios from a spec.
- Flagging inconsistencies and gaps.

### 4.2 Required
- Human SME review of every factual claim.
- Human ID review of every learning sequence.
- Human assessment review of every item and rubric.
- Provenance label on all AI-assisted content.
- Model/version/prompt logged.
- No AI-generated level published without pilot.

### 4.3 Prohibited
- AI inventing citations or facts.
- AI writing the level-100 standard without expert input.
- AI grading high-stakes work alone.
- AI generating safety-critical instructions without expert review.
- Using learner data to train models without consent.

---

## 5. Level brief template

```md
# Skill Brief: [Skill name]
ID: [DOMAIN-NN]
Archetype: [ ]
Level-100 outcome: [world-class proof]
Who this is for: [learner + context]
Prerequisites: [skills/levels]
Core concepts (10-band arc): [list]
Signature assessments: [types]
Safety classification: [standard/caution/high-risk]
Localization notes: [units, law, culture]
Tools: [required/optional]
Low-cost path: [how to learn with minimal equipment]
```

---

## 6. Authoring workflow

### 6.1 Level spec
1. SME writes the outcome and "why it matters."
2. AI drafts concepts and examples from a vetted source pack.
3. SME corrects and adds counterexamples.
4. ID sequences practice from guided to independent.
5. Assessment specialist builds blueprint and items.
6. Editor tightens language and accessibility.
7. Safety reviewer checks scope.
8. Pilot and calibrate.
9. Publish with version and provenance.

### 6.2 Source pack
Every skill has a source pack:
- 3–5 authoritative references.
- 2–3 real-world examples.
- 1–2 common misconceptions.
- Safety notes.
- Localization notes.
- Expert contacts.

No source pack, no authoring.

---

## 7. Tooling

| Tool | Purpose |
|---|---|
| **Curriculum CMS** | Level specs, versions, workflow states |
| **Item bank** | Items, metadata, calibration, exposure |
| **Rubric studio** | Criteria, anchors, calibration |
| **Simulation builder** | Scenario graphs, state, scoring |
| **Media pipeline** | Video, audio, captions, transcripts |
| **AI drafting service** | Source-grounded generation |
| **Review queue** | Assignments, comments, approvals |
| **Provenance ledger** | Who/what/which model produced each asset |
| **Localization manager** | Translation, review, layout |
| **Analytics** | Level health, pass rates, disputes |

---

## 8. Quality gates

### 8.1 Completeness
- 12 fields present.
- 3+ practice exercises, 1 applied.
- 20+ items.
- 4-criterion rubric.
- Safety/accessibility sign-off.

### 8.2 Learning quality
- Outcome is observable.
- Prerequisites are correct.
- Examples and counterexamples are accurate.
- Practice is sequenced.
- Cognitive load is appropriate.

### 8.3 Assessment quality
- Blueprint matches band.
- Items are unambiguous.
- Distractors are plausible.
- Rubric anchors exist.
- Pilot reliability ≥0.75.

### 8.4 Safety
- Scope limits stated.
- Warnings present.
- Escalation guidance.
- No licensure claims.
- Legal review for high-risk.

### 8.5 Accessibility
- Captions/transcripts.
- Alt text.
- Keyboard navigation.
- Screen-reader labels.
- Color contrast.
- Plain-language option.

---

## 9. Cost and throughput

| Item | Target |
|---|---|
| SME time per level | 4–8h |
| AI drafting cost per level | $5–15 |
| Assessment build per level | 3–5h |
| Review per level | 2–3h |
| Pilot cost per level | $50–100 |
| Total cost per accepted level | <$150 |
| Levels per author-week | 2–3 |
| Throughput at Phase 2 | 25–50/week |
| Throughput at Phase 4 | 150–250/week |

### 9.1 Scaling levers
- Reuse patterns across skills (not content).
- Template libraries by archetype.
- AI-assisted variation with human review.
- Offshore/local author networks.
- Learner-generated practice (reviewed).
- Modular media reuse.

---

## 10. Localization

1. Source in English (or launch language).
2. Machine translation with glossary.
3. In-country reviewer.
4. Cultural adaptation of examples.
5. Assessment review for fairness.
6. Layout/QA.
7. Publish as locale variant.

Localization is not just translation: units, law, food, money, and social norms change.

---

## 11. Versioning and refresh

| Change | Version bump | Re-pilot? | Learner notice |
|---|---|---|---|
| Typo | Patch | No | No |
| New example | Minor | No | No |
| New assessment item | Minor | Yes (small) | No |
| Outcome change | Major | Yes | Yes, 30 days |
| Level split/merge | Major | Yes | Yes, 90 days |
| Safety change | Major | Yes | Immediate |

Refresh triggers:
- Pass rate outside 55–95%.
- Dispute rate >2%.
- Content older than 18 months in fast-moving fields.
- New evidence or regulation.
- Learner clarity <4/5.

---

## 12. RACI

| Activity | Domain lead | SME | ID | Assessment | Safety | Content ops |
|---|---|---|---|---|---|---|
| Skill brief | A | R | C | C | C | I |
| Band outline | A | R | R | C | C | I |
| Level draft | C | R | C | C | I | I |
| Pedagogy review | I | C | A | C | I | I |
| Assessment build | I | C | C | A/R | C | I |
| Safety review | C | C | I | I | A/R | I |
| Pilot | I | C | C | R | I | A |
| Publish | I | I | I | I | C | A/R |
| Refresh | A | R | C | C | C | R |

R = responsible, A = accountable, C = consulted, I = informed.

---

## 13. Metrics

| Metric | Target |
|---|---|
| Cost per accepted level | <$150 |
| First-pass review acceptance | ≥70% |
| Pilot reliability | ≥0.75 |
| Pass rate within target | ≥80% of levels |
| Dispute rate | <2% |
| Accessibility blockers | 0 |
| Time from brief to publish | ≤6 weeks |
| Refresh backlog age | <90 days |
| Learner clarity | ≥4/5 |

---

## 14. Acceptance criteria

- `FR-CNT-001`: No level publishes without all quality gates.
- `FR-CNT-002`: Every asset has provenance metadata.
- `FR-CNT-003`: Every level has a source pack.
- `FR-CNT-004`: Every AI-assisted asset is human-reviewed.
- `FR-CNT-005`: Every level is piloted before full release.
- `FR-CNT-006`: Every level has a refresh date.
- `FR-CNT-007`: Localization follows the 7-step process.
- `FR-CNT-008`: Cost per level is tracked and reported.
- `FR-CNT-009`: Author attribution and payment are recorded.
- `FR-CNT-010`: Learner-facing changelogs exist for major changes.
