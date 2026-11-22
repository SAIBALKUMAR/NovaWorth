# Release Gates

## 1. Purpose

A release gate is a formal go/no-go decision. It protects learners, the currency, and the company from shipping on optimism. Every gate has criteria, evidence, owners, and a decision record.

**Rule:** a gate cannot be waived by a single person. Waivers require a written risk acceptance signed by the accountable executive and the relevant functional lead.

---

## 2. Gate structure

```
GATE REVIEW
├── Criteria (from phase doc)
├── Evidence (dashboards, audits, reports)
├── Guardrails (must not be breached)
├── Risks (open, accepted, mitigated)
├── Decision (go / conditional / no-go)
└── Record (signed, dated, stored)
```

---

## 3. Universal guardrails

These apply to every gate. A breach blocks release regardless of other progress.

| Guardrail | Threshold |
|---|---|
| Accessibility blocking defects | 0 |
| Integrity flag rate | <3% |
| Appeal reversal rate | <5% |
| Fraud loss | <1% of EC |
| AI harmful output | <0.1% |
| Answer leak in gates | 0 |
| Data subject requests overdue | 0 |
| Critical security findings | 0 |
| Ledger reconciliation variance | 0 unexplained |
| Learner burnout self-report | <15% |

---

## 4. Phase gates

### 4.1 Phase 0 → 1
| Criterion | Evidence |
|---|---|
| 40 expert / 60 learner interviews | Research synthesis |
| 50 validated L100 definitions | External review |
| 5 exemplars authored | Content audit |
| Onboarding prototype ≥80% task success | Usability report |
| Scoring model 3 reviews | Review memos |
| Legal/privacy risk register | Signed register |
| Phase 1 plan approved | Exec sign-off |

### 4.2 Phase 1 → 2
| Criterion | Evidence |
|---|---|
| 1,000 invited learners | Analytics |
| Onboarding ≥65% | Funnel dashboard |
| First pass ≤7 days ≥50% | Cohort report |
| D7 ≥40%, D30 ≥25% | Retention dashboard |
| Pass rate 55–75% | Assessment report |
| NPS ≥40 | Survey |
| 25 skills playable | Content audit |

### 4.3 Phase 2 → 3
| Criterion | Evidence |
|---|---|
| 25,000 registered / 10,000 WAU | Analytics |
| 25 skills complete, 100 at 1–30 | Content audit |
| Assessment α ≥0.80 | Psychometric report |
| Role-play agreement ≥85% | QA report |
| 20 sponsor LOIs | Contracts |
| Marketplace pilot ≥95% | Ops report |
| WCAG 2.2 AA verified | Audit |
| 2 languages | Localization QA |

### 4.4 Phase 3 → 4
| Criterion | Evidence |
|---|---|
| 100,000 MAU | Analytics |
| ≥30% earning EC monthly | Economy dashboard |
| Emission ≤80% budget | Finance report |
| Fraud <1% | Trust report |
| Redemption ≥97% | Marketplace report |
| Post-100 ≥1,000 | Product analytics |
| Independent audit 0 critical | Audit letter |
| 3 languages | Localization QA |

### 4.5 Phase 4 → 5
| Criterion | Evidence |
|---|---|
| 1M registered / 250k MAU | Analytics |
| 500 skills complete | Content audit |
| 500k credentials | Credential registry |
| Cost <$1.50/learner/month | Finance report |
| 100 employer partners | Partner list |
| 10 languages | Localization QA |
| Vision Pro 10k MAU (or documented deferral) | Product analytics |
| Transparency report | Published |
| Livelihood pilot plan + ethics review | Signed plan |

### 4.6 Phase 5 scale decision
| Criterion | Evidence |
|---|---|
| 5,000 pilot participants | Pilot report |
| ≥60% households ≥20% essentials | Independent study |
| 25 protocol partners | Partner list |
| Autonomous content ≥70% | Content ops report |
| Learner-majority council seated | Governance record |
| Economic audit published | Public report |
| No ethics violations | Ethics review |

---

## 5. Release types

| Release | Gate | Approval |
|---|---|---|
| Minor (bug fix) | Automated tests + canary | Eng lead |
| Feature | Feature gate + guardrails | Product + eng |
| Assessment change | Psychometric review + pilot | Assessment lead |
| Economy change | Simulation + holdout + finance | Economy council |
| Marketplace partner | Vetting + pilot | Partnerships + trust |
| Jurisdiction launch | Legal checklist | Legal + exec |
| Phase transition | Full gate review | Exec + board |

---

## 6. Decision record

Every gate produces:

```yaml
gate: phase_2_to_3
date: 2026-11-21
decision: conditional_go
criteria:
  met: 12
  missed: 1
  missed_items:
    - criterion: role_play_agreement
      actual: 0.83
      target: 0.85
      remediation: "Recalibrate rubric; re-test by 2026-12-15"
risks_accepted:
  - risk: sponsor_concentration
    owner: CFO
    mitigation: "Diversify to 5 sponsors by Q1"
signatures:
  - ceo
  - cpo
  - cto
  - head_of_trust
```

Records are immutable and stored with the phase docs.

---

## 7. Post-release review

- 7-day health check.
- 30-day outcome review.
- Guardrail monitoring.
- Rollback criteria defined before launch.
- Incident review if thresholds breached.

---

## 8. Requirements

- `FR-PHS-701`: Every phase transition has a written gate review.
- `FR-PHS-702`: Gate decisions require named signatures.
- `FR-PHS-703`: Guardrail breaches block release.
- `FR-PHS-704`: Waivers require written risk acceptance.
- `FR-PHS-705`: Gate evidence is stored and immutable.
- `FR-PHS-706`: Assessment changes require psychometric review.
- `FR-PHS-707`: Economy changes require simulation and holdout.
- `FR-PHS-708`: Jurisdiction launches require a legal checklist.
- `FR-PHS-709`: Rollback criteria are defined pre-launch.
- `FR-PHS-710`: Post-release reviews occur at 7 and 30 days.
- `NFR-PHS-701`: Gate evidence retrievable <1 min.
- `NFR-PHS-702`: Decision record tamper-evident.

---

## 9. Related

- [Roadmap](01-roadmap.md)
- [Success metrics](../00-foundations/04-success-metrics.md)
- [Evaluation](../10-data-ai/04-evaluation.md)
- [Incident response](../11-operations/04-incident-response.md)
