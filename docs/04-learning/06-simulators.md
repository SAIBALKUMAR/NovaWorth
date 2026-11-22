# Simulators & Live-Fire

## 1. Purpose

Simulators are where knowledge becomes judgment. They compress years of operational experience into hours: a production outage, a dinner rush, a clinic emergency, a market crash, a construction delay.

**Rule:** a simulation must have consequences, incomplete information, and a debrief.

---

## 2. Simulation types

| Type | Description | Example |
|---|---|---|
| **Deterministic sim** | Rules-based system with measurable outcomes | Load test, budget model |
| **Branching scenario** | Choices lead to different states | Incident response |
| **Live-fire** | Timed, high-pressure, randomized failures | 3 a.m. outage |
| **Role-play sim** | Human/AI characters + state | Crisis communication |
| **Sandbox** | Open-ended build/test environment | Code, 3D, kitchen |
| **Tabletop** | Discussion-based exercise | Disaster response |
| **Constellation** | Multi-domain, multi-artifact | City resilience |

---

## 3. Scenario engine

```
World state
├── entities (servers, people, inventory, money)
├── rules (physics, economics, biology, law)
├── events (scheduled + random)
├── objectives (primary, secondary, hidden)
├── constraints (time, budget, authority)
└── scoring (outcome + process + ethics)
```

### 3.1 Tick model
- Turn-based or real-time.
- Each tick updates state.
- Events inject failures.
- Learner actions have costs and delays.
- The sim ends on success, failure, or time.

### 3.2 Determinism
- Seeded randomness for reproducibility.
- Replayable logs.
- Same seed + same actions = same outcome.
- Randomization for anti-memorization.

---

## 4. Live-fire design

### 4.1 Principles
- **Incomplete information.** The learner never has all the data.
- **Time pressure.** Decisions have deadlines.
- **Cascading effects.** Fixing one thing breaks another.
- **Stakeholders.** People demand updates.
- **Ethics.** Shortcuts have costs.
- **Debrief.** Every run is reviewed.

### 4.2 Example: production outage (CMP-25, level 88)
| Phase | Event |
|---|---|
| 0 | Alert: p99 latency 10× |
| 1 | Dashboard shows DB CPU 100% |
| 2 | A deploy happened 20 min ago |
| 3 | Rollback fixes latency but breaks checkout |
| 4 | Stakeholder demands ETA |
| 5 | Root cause: a missing index + traffic spike |
| 6 | Fix, verify, communicate, postmortem |

Scoring: detection, mitigation, communication, root cause, prevention, ethics.

### 4.3 Example: dinner rush (COK-04, level 58)
- 40 covers in 90 minutes.
- One burner fails.
- A server drops a plate.
- A guest has an allergy.
- A supplier delivered wrong fish.

Scoring: timing, safety, quality, communication, recovery.

### 4.4 Example: clinic emergency (HLT-10, level 50)
- 5 casualties.
- Limited supplies.
- Conflicting information.
- A distressed family member.

Scoring: triage, safety, communication, documentation, ethics.

---

## 5. After-action review (AAR)

Every simulation ends with:
1. **Timeline** of decisions.
2. **Outcome** vs objectives.
3. **What went well.**
4. **What went wrong.**
5. **Root causes.**
6. **What to practice next.**
7. **Score and evidence.**

The AAR is scored too: insight, honesty, specificity.

---

## 6. Scoring

| Dimension | Weight | Evidence |
|---|---|---|
| Outcome | 30% | Objectives met |
| Process | 25% | Decision quality |
| Safety/ethics | 20% | No harmful shortcuts |
| Communication | 15% | Stakeholder updates |
| Learning | 10% | AAR quality |

### 6.1 Failure is allowed
- A failed sim can still earn partial XP for legitimate process.
- EC only for accepted outcomes at high levels.
- The debrief determines what was learned.

---

## 7. Safety

- High-risk sims are clearly labeled.
- No real people are harmed.
- Medical/legal/financial sims include scope limits.
- Crisis sims include escalation resources.
- Learners can stop at any time.
- Content is reviewed by domain experts.

---

## 8. Technical architecture

| Component | Approach |
|---|---|
| Engine | Deterministic rules + event scheduler |
| State | Event-sourced; replayable |
| UI | WebGL/Canvas for 2D; WebGPU/Unity for 3D |
| AI | NPCs, hints, debrief, not the rules |
| Scoring | Rules + rubric + human |
| Storage | Run logs, artifacts, recordings |
| Scale | Stateless workers; per-run sandbox |

### 8.1 Performance
- 60 fps for 3D on mid-range devices.
- <100 ms input latency.
- Graceful degradation to 2D.
- Offline mode for low-stakes sims.

---

## 9. Authoring

A simulation spec includes:
- Learning objectives.
- World model.
- Events and triggers.
- Success/failure conditions.
- Rubric.
- AAR prompts.
- Safety review.
- Playtest data.

Authoring time: 40–120 hours per sim. Reuse engines and scenario templates.

---

## 10. Requirements

- `FR-LRN-501`: Every simulation has objectives, rules, and a rubric.
- `FR-LRN-502`: Every simulation ends with an AAR.
- `FR-LRN-503`: Runs are replayable and logged.
- `FR-LRN-504`: Randomization prevents memorization.
- `FR-LRN-505`: High-risk sims have safety review.
- `FR-LRN-506`: Learners can pause/exit.
- `FR-LRN-507`: Sims work on mid-range devices.
- `FR-LRN-508`: 2D fallback exists for 3D sims.
- `FR-LRN-509`: Scoring includes process and ethics.
- `FR-LRN-510`: Sim data feeds mastery estimates.
- `NFR-LRN-501`: Input latency <100 ms.
- `NFR-LRN-502`: 60 fps target; 30 fps minimum.
- `NFR-LRN-503`: Run recovery after disconnect.
