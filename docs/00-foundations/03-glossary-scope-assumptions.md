# Glossary, Scope & Assumptions

## 1. Glossary

| Term | Definition |
|---|---|
| **Skill** | A durable human capability with a defined 100-level ladder, assessment standard, and evidence model. One of 500. |
| **Domain** | A group of 25 related skills (e.g., Computing & Software). 20 domains total. |
| **Level** | A bounded unit of capability within a skill, from 1 to 100. Has objectives, content, practice, assessment, and rewards. |
| **Band** | A group of 10 levels with a shared developmental theme (e.g., levels 41–50 = Specialist). |
| **XP** | Mastery points. Permanent, non-transferable, non-spendable. Determines rank and progress. |
| **EC (Education Credit)** | Spendable learning credit, funded by sponsors and marketplace margin. Not cash in v1. |
| **Reputation** | Trust score derived from verified evidence, peer review, mentoring, and integrity history. Unlocks high-stakes roles. |
| **Prestige** | Post-100 status earned through frontier challenges, constellations, research, and mentoring. Not purchasable. |
| **Gate** | The summative assessment that must be passed to advance a level. |
| **Evidence** | Any artifact or observation used to prove capability: test responses, code, video, rubric scores, real-world outcomes. |
| **Artifact** | A learner-created object submitted as evidence (project, document, recording, build, repair). |
| **Experience graph** | A structured representation of a learner's work, life experience, interests, constraints, and evidence. |
| **Fit score** | A 0–100 recommendation score combining aptitude, interest, transferable experience, opportunity, and feasibility. |
| **Constellation** | A cross-skill mission that requires 3+ skills (e.g., software + health + communication). |
| **Frontier challenge** | A post-100, novel or real-world problem with no known answer, scored by experts and impact. |
| **Live-fire simulation** | A timed, high-pressure scenario (e.g., production incident, kitchen dinner service) with branching consequences. |
| **Emission** | The creation of new EC. Capped by funded sponsor pools. |
| **Sink** | A mechanism that removes EC from circulation (redemptions, fees, expired promotional credits). |
| **Sponsor pool** | A ring-fenced budget that funds EC for a cohort, region, skill, or population. |
| **Treasury** | The account that holds unallocated sponsor funds and platform margin. |
| **Clawback** | Reversal of an award after fraud or assessment invalidation. |
| **Verifiable credential (VC)** | A tamper-evident, portable proof of a level or skill, signed by the platform. |
| **Mastery estimate** | The system's current belief about a learner's ability, updated after every interaction. |
| **FSRS** | Free Spaced Repetition Scheduler; the algorithm used for review timing. |
| **IRT** | Item Response Theory; used to calibrate item difficulty and estimate ability. |
| **BKT** | Bayesian Knowledge Tracing; used for fine-grained skill mastery. |
| **Provenance** | The origin and edit history of content or evidence, including AI involvement. |
| **LMS** | Learning Management System (legacy category we are not building). |
| **PWA** | Progressive Web App. |
| **visionOS** | Apple's spatial operating system for Vision Pro. |
| **WebXR** | Browser standard for immersive experiences. |
| **Guardian** | A verified adult responsible for a minor's account. |
| **Cohort** | A group of learners funded or tracked together (sponsor, employer, region, class). |
| **Season** | A 12-week competitive cycle for leagues and leaderboards. Mastery never resets. |

---

## 2. Scope

### 2.1 In scope (v1 → Phase 4)

- 500 skills across 20 domains, with 100 levels each.
- Onboarding: work history, life experience, interests, constraints, diagnostic, recommendation.
- Learning modalities: text, AI tutor, video, quizzes, role-play, simulators, projects.
- Assessments: diagnostic, formative, gate exams, practical artifacts, expert panels at high levels.
- XP/EC economy with double-entry ledger, sponsor pools, marketplace, and integrity controls.
- Marketplace for curated goods, services, tools, experiences, and AI services.
- Web PWA, iOS/Android, Apple Vision Pro; optional WebXR/metaverse.
- Social: guilds, leagues, mentoring, peer review.
- Credentials: verifiable credentials and a public skill wallet.
- Sponsor/employer dashboards with consent-based, aggregate data.
- Trust & safety: anti-cheat, fraud, AI safety, child safety, privacy.
- Operations: content factory, support, moderation, reliability, incident response.
- Analytics and experimentation.

### 2.2 Out of scope (v1)

- Cash withdrawal, trading EC, or speculative markets.
- A full accredited degree or professional licensure.
- A general-purpose social network.
- A job board with guaranteed placement (Phase 5 explores integrations).
- Building our own foundation models.
- Building our own payment rails or physical logistics network.
- Full metaverse world with persistent avatars (Phase 4 pilot only).
- Medical, legal, or financial advice delivered as a professional service.
- Hardware manufacturing.

### 2.3 Explicitly deferred

- Livelihood-scale payouts (Phase 5 pilot).
- Employer-paid micro-credentials at scale (Phase 4).
- Open protocol for third-party skill providers (Phase 5).
- Autonomous content generation without human review (Phase 5, gated).
- Cross-border EC settlement (Phase 5).

---

## 3. Assumptions

| ID | Assumption | Impact if wrong | Validation |
|---|---|---|---|
| A1 | People will do hard learning for a mix of status, material reward, and mastery. | Core motivation fails. | Phase 0 interviews + Phase 1 retention. |
| A2 | Sponsors will fund verified learning outcomes. | EC has no backing. | 10 sponsor LOIs before Phase 3. |
| A3 | A 100-level ladder can be authored at consistent quality. | Content cost explodes or quality collapses. | 5 exemplars + 25-skill alpha. |
| A4 | AI can grade practice reliably with human audit. | Assessment cost too high or trust too low. | Eval harness; human agreement >90%. |
| A5 | Learners accept AI role-play and simulation as credible proof. | High-level gates lack validity. | Compare with expert panels. |
| A6 | Mobile is the primary device for the core audience. | Web-first investment misallocated. | Phase 1 device analytics. |
| A7 | Spatial learning improves outcomes for physical/interpersonal skills. | Vision Pro investment is premature. | Phase 4 controlled pilot. |
| A8 | Fraud can be held below 1% of awards with layered controls. | Economy becomes untrustworthy. | Red-team + Phase 3 audit. |
| A9 | A curated marketplace can fulfill reliably across regions. | Redemption promise breaks. | Partner pilots in 3 regions. |
| A10 | Learners will accept "supplemental dividend" positioning in early phases. | Overpromising causes churn/regulatory risk. | Messaging tests; legal review. |
| A11 | 20 domains × 25 skills is a useful map, not an arbitrary grid. | Catalog feels padded. | Domain expert review; usage data. |
| A12 | A small team can operate the content factory with AI assistance. | Unit economics fail. | Cost per accepted level <$150. |

---

## 4. Constraints

### 4.1 Product constraints
- No skill may require a specific hardware device to complete.
- No level may be passed by watching alone.
- No EC may be earned without a verifiable assessment event.
- No high-stakes decision may be fully automated.
- No minor may spend EC without guardian-configured limits.

### 4.2 Technical constraints
- Web must work on mid-range Android devices and slow networks.
- Mobile must support offline learning packs and queued evidence upload.
- Vision Pro app must degrade gracefully to a 2D window.
- AI costs must stay below $1.50 per active learner per month at scale.
- P95 API latency <300 ms; AI tutor first token <4 s.
- 99.9% monthly availability for core learning and ledger.

### 4.3 Legal/regulatory constraints
- GDPR/CCPA-style privacy rights, including deletion and portability.
- COPPA/GDPR-K for minors; minimum age 13 with guardian consent, 16+ standalone (region-dependent).
- Consumer protection rules for marketplace and credits.
- Accessibility: WCAG 2.2 AA, ADA, European Accessibility Act.
- Employment law risk if credentials are used in hiring; adverse-impact monitoring.
- Education regulation if we offer regulated professions; avoid licensure claims.
- Financial regulation if EC becomes redeemable for cash; v1 avoids this.

### 4.4 Content constraints
- Every level must have a named subject-matter expert reviewer.
- Every assessment item must have a source, difficulty calibration, and fairness review.
- Every AI-generated asset must be labeled and reviewed.
- Safety-critical skills (health, electrical, climbing, etc.) must include explicit risk warnings and cannot substitute for certification.

---

## 5. Dependencies

| Dependency | Owner | Needed by |
|---|---|---|
| Identity provider / SSO | Platform | Phase 1 |
| Payment and payout rails | Partnerships/Finance | Phase 3 |
| Marketplace partners (3 regions) | Business development | Phase 3 |
| Sponsor commitments | Partnerships | Phase 2 |
| Expert examiner network | Learning | Phase 2 |
| Proctoring vendor or in-house | Trust & Safety | Phase 2 |
| Content licensing | Legal | Phase 2 |
| Vision Pro hardware and developer account | Platform | Phase 4 |
| Localization vendors | Content Ops | Phase 4 |
| Independent audit firm | Finance/Legal | Phase 3 |

---

## 6. Key product boundaries

1. **NovaWorth is not a school.** It is a capability economy with learning as the engine.
2. **It is not a test prep company.** Assessment exists to protect the value of the credential and the economy.
3. **It is not a marketplace first.** The marketplace exists to make learning consequential; it is curated and capped.
4. **It is not a metaverse.** Spatial is one modality among many.
5. **It is not a universal score of human worth.** It measures specific, demonstrated capabilities, with context and uncertainty.

---

## 7. Open decisions (tracked in appendices)

- Minimum age and guardian model by region.
- Whether EC ever becomes cash-redeemable.
- Launch regions and languages.
- Sponsor concentration limits.
- Whether level 100 requires real-world impact in every skill.
- How much of the marketplace is owned vs partnered.
- Whether to open the skill protocol to third parties.
