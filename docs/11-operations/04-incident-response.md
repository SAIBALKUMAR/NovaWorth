# Incident Response

## 1. Purpose

Incidents are inevitable. The measure of the system is how fast it detects, contains, communicates, and learns. This document defines severity, roles, process, communication, and postmortems.

---

## 2. Severity levels

| Sev | Definition | Examples | Response |
|---|---|---|---|
| **SEV1** | Critical outage or safety/security harm | Ledger corruption, data breach, assessment leak, platform down | Immediate, all hands |
| **SEV2** | Major degradation | AI tutor down, payments failing, high error rate | <15 min |
| **SEV3** | Partial degradation | One region slow, one feature broken | <1 h |
| **SEV4** | Minor | Cosmetic, single-user issue | Next business day |

### 2.1 Special classes
- **Safety incident:** harm or imminent risk to a person → SEV1 regardless of scale.
- **Integrity incident:** credential fraud, item leak → SEV1/2.
- **Financial incident:** ledger imbalance, unauthorized payout → SEV1.
- **Privacy incident:** data exposure → SEV1 + legal notification path.

---

## 3. Roles

| Role | Responsibility |
|---|---|
| Incident commander (IC) | Owns the incident, decisions, and timeline |
| Ops lead | Technical coordination |
| Comms lead | Internal and external updates |
| Scribe | Timeline, decisions, actions |
| Subject experts | Diagnose and fix |
| Executive sponsor | Escalation, external statements |
| Legal/privacy | Notification obligations |
| Trust & Safety | User safety actions |

The IC does not debug. The IC coordinates.

---

## 4. Process

```
DETECT → DECLARE → ASSEMBLE → CONTAIN
→ DIAGNOSE → MITIGATE → RECOVER → COMMUNICATE
→ POSTMORTEM → FIX → VERIFY
```

### 4.1 Detect
- Alerts, synthetic checks, reports, staff observation.
- Any team member can declare.

### 4.2 Declare
- Open incident channel.
- Assign IC.
- Set severity.
- Start timeline.

### 4.3 Contain
- Stop the bleeding: rollback, feature flag off, isolate service, freeze payouts.
- Protect people first, data second, money third, availability fourth.

### 4.4 Mitigate
- Restore service with the smallest safe change.
- Prefer rollback over forward-fix under pressure.
- Verify with real user flows.

### 4.5 Recover
- Confirm SLOs.
- Reconcile data (ledger, awards).
- Notify affected users.
- Lift freezes gradually.

---

## 5. Communication

### 5.1 Internal cadence
| Sev | Update frequency |
|---|---|
| SEV1 | Every 30 min |
| SEV2 | Every 60 min |
| SEV3 | Every 4 h |
| SEV4 | Daily |

### 5.2 External
- Status page updated within 30 min of SEV1/2.
- Plain-language impact and ETA.
- No speculation on cause.
- Follow-up when resolved.
- Direct notice for affected users (email/in-app).

### 5.3 Regulatory
- Privacy breach: assess notification duty; 72-hour GDPR clock.
- Financial: notify payment partners and regulators as required.
- Safety: follow child-safety and law-enforcement obligations.

---

## 6. Postmortem

### 6.1 Required for
- All SEV1.
- SEV2 with user impact >1h.
- Any safety, privacy, or integrity incident.

### 6.2 Template
1. Summary.
2. Impact (users, data, money, trust).
3. Timeline.
4. Root cause(s).
5. Contributing factors.
6. What went well.
7. What went poorly.
8. Action items (owner, due date, priority).
9. Detection and prevention improvements.

### 6.3 Rules
- Blameless.
- Specific and verifiable.
- Actions tracked to completion.
- Published internally within 5 business days.
- External summary for major incidents.

---

## 7. Playbooks

| Scenario | First actions |
|---|---|
| Ledger imbalance | Freeze writes, snapshot, reconcile, identify source |
| AI tutor unsafe output | Kill switch, preserve logs, assess exposure, notify safety |
| Assessment leak | Rotate items, invalidate affected attempts, notify |
| Data breach | Contain, preserve evidence, engage legal, assess notification |
| Payment outage | Fail closed, queue orders, communicate |
| Marketplace fraud ring | Freeze accounts, preserve evidence, investigate |
| Region outage | Failover, communicate, monitor |
| Credential forgery | Revoke, notify verifiers, investigate |

Each playbook has an owner, a checklist, and a drill schedule.

---

## 8. Drills

- Monthly tabletop.
- Quarterly live game-day.
- Annual cross-functional crisis simulation.
- New IC training and shadowing.
- Post-drill report with fixes.

---

## 9. Metrics

| Metric | Target |
|---|---|
| MTTD (detect) | <5 min |
| MTTA (acknowledge) | <15 min |
| MTTR (mitigate) | <45 min |
| SEV1 per quarter | <2 |
| Postmortem on time | 100% |
| Action items closed on time | ≥90% |
| Status page first update | <30 min |
| Repeat incidents | <10% |

---

## 10. Requirements

- `FR-OPS-401`: Any team member can declare an incident.
- `FR-OPS-402`: Every incident has an IC and a timeline.
- `FR-OPS-403`: Safety incidents are always SEV1.
- `FR-OPS-404`: Status page updates within 30 minutes.
- `FR-OPS-405`: Postmortems are blameless and tracked.
- `FR-OPS-406`: Ledger incidents freeze writes first.
- `FR-OPS-407`: AI incidents have a kill switch.
- `FR-OPS-408`: Privacy incidents route to legal within 1 hour.
- `FR-OPS-409`: Playbooks are drilled.
- `FR-OPS-410`: Action items have owners and dates.
- `NFR-OPS-401`: Page delivery <60 s.
- `NFR-OPS-402`: Incident channel created <2 min.

---

## 11. Related

- [Reliability](03-reliability.md)
- [Privacy & data](../09-trust-safety/03-privacy-data.md)
- [Integrity & anti-cheat](../09-trust-safety/01-integrity-anti-cheat.md)
