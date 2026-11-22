# Privacy & Data Protection

## 1. Purpose

Learners share sensitive data: work history, life experience, health constraints, accessibility needs, assessment responses, and financial activity. Privacy is a condition of trust.

**Principle:** collect the minimum, explain the purpose, give control, protect it.

---

## 2. Data map

| Category | Examples | Sensitivity | Purpose |
|---|---|---|---|
| Identity | Name, email, phone | PII | Account |
| Work | Roles, employers, projects | Sensitive | Recommendations |
| Life | Caregiving, health, travel | Highly sensitive | Recommendations |
| Accessibility | Screen reader, accommodations | Highly sensitive | Access |
| Assessment | Responses, scores, artifacts | Sensitive | Credentials |
| Economy | EC balance, awards, orders | Financial | Ledger |
| Behavior | Sessions, clicks, time | Sensitive | Product |
| Device | IP, device ID | PII | Security |
| Biometric | Face/voice (high stakes) | Highly sensitive | Integrity |
| Guardian | Consent, limits | Sensitive | Minors |

---

## 3. Legal bases

| Purpose | Basis |
|---|---|
| Core learning | Contract |
| Recommendations | Consent |
| Research | Consent / legitimate interest |
| Marketing | Consent |
| Safety/fraud | Legitimate interest / legal obligation |
| Legal compliance | Legal obligation |

Consent is granular, revocable, and logged.

---

## 4. Rights

| Right | Implementation |
|---|---|
| Access | Export in 30 days |
| Correction | Edit profile |
| Deletion | Delete account + data |
| Portability | JSON/PDF export |
| Restriction | Pause processing |
| Objection | Opt out of research/marketing |
| Withdraw consent | Toggle anytime |
| Complaint | DPO contact |

Requests are verified and completed within legal timelines.

---

## 5. Retention

| Data | Retention |
|---|---|
| Account | While active + 90 days |
| Assessment evidence | Credential lifetime |
| Ledger | 7 years (legal) |
| Support tickets | 3 years |
| Logs | 90 days |
| Biometric | Deleted after verification (unless legal hold) |
| Research data | De-identified after 24 months |

---

## 6. Sensitive data

- Health, disability, caregiving, sexuality, religion, ethnicity: opt-in only.
- Never used for advertising.
- Never shared with sponsors at individual level.
- Encrypted at rest with field-level encryption.
- Access logged and reviewed.
- No inference of protected traits.

---

## 7. AI and data

- No training on learner data without explicit consent.
- PII redacted before external model calls.
- Model providers under data processing agreements.
- No retention by providers beyond processing.
- Prompt/output logs retained 90 days, then de-identified.
- Learners can opt out of AI personalization.

---

## 8. Cross-border

- Data residency where required.
- Standard contractual clauses.
- Transfer impact assessments.
- Regional storage for sensitive data.
- No transfer to high-risk jurisdictions without safeguards.

---

## 9. Security

| Control | Description |
|---|---|
| Encryption | TLS 1.3, AES-256 at rest |
| Access | Least privilege, MFA, review |
| Monitoring | SIEM, anomaly detection |
| Testing | Pen tests, bug bounty |
| Incident | 72-hour breach notification |
| Vendors | Security review, DPAs |

---

## 10. Minors

- Guardian consent under 16 (region-dependent).
- Restricted data collection.
- No behavioral advertising.
- Private by default.
- No peer transfers.
- Deletion on request.

See [Child safety](04-child-safety.md).

---

## 11. Transparency

- Plain-language privacy notice.
- Data dashboard: what we have, why.
- Annual transparency report.
- Change notices 30 days ahead.
- No dark patterns in consent.

---

## 12. Requirements

- `FR-TNS-301`: Consent is granular and revocable.
- `FR-TNS-302`: Data export within 30 days.
- `FR-TNS-303`: Deletion within 30 days.
- `FR-TNS-304`: Sensitive data is opt-in.
- `FR-TNS-305`: No sale of personal data.
- `FR-TNS-306`: No training without consent.
- `FR-TNS-307`: PII is redacted before AI calls.
- `FR-TNS-308`: Access is logged.
- `FR-TNS-309`: Breach notification within 72h.
- `FR-TNS-310`: Minors have restricted collection.
- `NFR-TNS-301`: Encryption in transit and at rest.
- `NFR-TNS-302`: DSR completion ≥99% on time.
