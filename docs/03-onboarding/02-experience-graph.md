# Experience Graph

## 1. Purpose

Most people undervalue what they already know. A caregiver has operations, health, and conflict skills. A weekend footballer has coaching, physiology, and teamwork. A gamer has systems thinking, rapid learning, and coordination.

The **experience graph** turns work history, life experience, interests, and constraints into structured signal for recommendations, placement, and content adaptation.

---

## 2. Graph model

### 2.1 Nodes
| Type | Examples |
|---|---|
| `Role` | Support lead, line cook, student, parent |
| `Project` | Migrated a CRM, renovated a kitchen, ran a fundraiser |
| `Tool` | Excel, Python, espresso machine, loom |
| `Domain` | Health, computing, food, finance |
| `Skill` | Active listening, knife skills, budgeting |
| `Experience` | Travel, sport, caregiving, volunteering |
| `Interest` | AI, design, cooking, climate |
| `Constraint` | 5h/week, low bandwidth, no car |
| `Evidence` | Resume, portfolio, certificate, video |
| `Outcome` | Reduced costs, won a match, cared for a parent |

### 2.2 Edges
| Edge | Meaning | Example |
|---|---|---|
| `used_in` | Tool used in a role | Excel → support lead |
| `demonstrated` | Evidence of a skill | Project → project management |
| `learned_from` | Experience teaches a skill | Caregiving → health advocacy |
| `interested_in` | Motivation | Interest → AI |
| `constrained_by` | Feasibility | Constraint → no lab access |
| `requires` | Prerequisite | Skill → skill |
| `transferable_to` | Adjacent capability | Cooking → operations |
| `evidenced_by` | Proof | Skill → certificate |

---

## 3. Capture

### 3.1 Structured prompts
Learners pick from cards and add free text. Examples:

| Prompt | Extracted signal |
|---|---|
| "What did you do at work?" | Roles, tools, outcomes |
| "What are you proud of?" | Projects, strengths |
| "What do you do outside work?" | Sports, caregiving, hobbies |
| "What have you fixed or built?" | Craft, systems, persistence |
| "What do you teach others?" | Communication, mastery |
| "What do you avoid?" | Disinterests, constraints |
| "What would you do for free?" | Intrinsic motivation |

### 3.2 Free-text extraction
- LLM extracts candidate skills with confidence.
- Every extraction is shown to the learner for confirmation.
- No skill is added without consent.
- Ambiguous claims are marked "self-reported" until evidenced.

### 3.3 Evidence upload
- Resume/CV.
- Portfolio links.
- Certificates.
- Photos/videos of work.
- Employer or client references.
- Optional verification.

### 3.4 Progressive capture
- Day 1: roles, life domains, interests.
- Day 3: one project detail.
- Day 7: one outcome.
- Day 14: one piece of evidence.
- Ongoing: each completed level adds evidence automatically.

---

## 4. Skill inference

### 4.1 Transferable skill mapping
| Experience | Candidate skills | Confidence |
|---|---|---|
| Caregiving for a parent | Health advocacy, medication literacy, elder care, conflict resolution, scheduling | High |
| Line cook | Knife skills, food hygiene, timing, inventory, teamwork | High |
| Weekend football | Teamwork, coaching, injury prevention, sports nutrition | Medium |
| Travel in 5 countries | Cross-cultural communication, navigation, budgeting, adaptability | Medium |
| Home renovation | Measurement, plumbing basics, project management, budgeting | Medium |
| Gaming guild leader | Coordination, conflict resolution, planning, mentoring | Medium |
| Volunteering | Community organizing, facilitation, empathy | Medium |
| Self-taught coding | Programming fundamentals, debugging, self-directed learning | Medium |
| Parenting | Child development, patience, negotiation, logistics | Medium |

### 4.2 Confidence levels
| Level | Meaning | Use |
|---|---|---|
| `verified` | Evidence + verification | Placement, credentials |
| `evidenced` | Artifact provided | Placement, recommendations |
| `self_reported` | Learner claim | Recommendations only |
| `inferred` | Model inference | Prompts, not assertions |

### 4.3 Anti-flattery rule
Do not tell someone they are "level 40" based on a hobby. Recommend a **placement assessment** instead.

---

## 5. Graph queries

### 5.1 For recommendations
```
candidate_skills = skills where:
  transferable_from(experience) OR interest OR diagnostic_strength
  AND NOT constrained_out
rank by fit_score
```

### 5.2 For placement
```
placement_level = highest level where:
  prerequisite skills are verified
  diagnostic score ≥ threshold
  learner opts in
```

### 5.3 For content adaptation
- Use familiar examples (e.g., cooking examples for a chef learning chemistry).
- Adjust reading level.
- Suggest practice contexts.

### 5.4 For social
- Suggest guilds with similar experience.
- Avoid matching solely on demographics.

---

## 6. Example graph

**Maya, 34, support lead, parent, 8h/week, mobile-only.**

```
Role: Support lead ──used_in──> CRM, Excel, ticketing
Role: Support lead ──demonstrated──> Conflict resolution, clear writing
Experience: Parenting ──learned_from──> Patience, logistics, health literacy
Interest: AI ──interested_in──> AI literacy, prompt engineering
Constraint: 8h/week, mobile ──constrained_by──> no lab-heavy paths
Project: Reduced response time 30% ──demonstrated──> Process improvement, analytics
```

**Recommendations:**
1. AI operations (CMP/AID + COM) — 88 fit.
2. Customer success engineering (CMP + COM) — 84 fit.
3. UX research (ART + COM) — 79 fit.
4. Data analysis (MTH + AID) — 76 fit.

**Why:** transferable communication and process experience, strong interest in AI, feasible on mobile, high demand.

---

## 7. Privacy and consent

- Experience data is sensitive. Opt-in for recommendation use.
- Learners can view, edit, export, and delete every node.
- Employers/sponsors see aggregates, not raw life experience.
- No inference of protected traits (health, religion, sexuality, ethnicity).
- No use of experience data for advertising.
- Minors: guardian consent; limited collection.

---

## 8. Data quality

| Risk | Control |
|---|---|
| Overclaiming | Confidence levels; verification prompts |
| Underclaiming | Prompt cards; examples; "no experience" normalization |
| Model hallucination | Learner confirmation; no auto-assertion |
| Stale data | Re-confirm every 12 months |
| Bias in extraction | Fairness review; multilingual testing |
| Privacy leak | Encryption; access controls; deletion |

---

## 9. Requirements

- `FR-ONB-101`: Learners can add, edit, and delete experience nodes.
- `FR-ONB-102`: Every inferred skill is confirmed before use.
- `FR-ONB-103`: Confidence is stored and displayed.
- `FR-ONB-104`: Experience data is excluded from sponsor views.
- `FR-ONB-105`: The graph supports free text, cards, and uploads.
- `FR-ONB-106`: Transferable mappings are reviewed by domain experts.
- `FR-ONB-107`: The graph can be exported as JSON.
- `FR-ONB-108`: Deleting a node removes derived recommendations within 24h.
- `FR-ONB-109`: No protected traits are inferred.
- `FR-ONB-110`: Minors have restricted collection.

---

## 10. Success metrics

| Metric | Target |
|---|---|
| Learners who add ≥3 experience nodes | ≥70% |
| Inferred skills confirmed | ≥60% |
| Recommendation relevance (self-report) | ≥4/5 |
| Graph edit rate | <10% |
| Deletion requests completed | 100% within 30 days |
| Bias audit findings | 0 critical |
