# Regulatory Landscape

## 1. Purpose

NovaWorth touches education, employment, consumer finance, data protection, and child safety. This document maps the regulatory surface, the product's posture, and the controls that keep it compliant.

**Posture:** the product is a learning and rewards platform, not an accredited school, a lender, a bank, or an employment agency. Claims and design must stay consistent with that.

> This is a requirements-level summary, not legal advice. Jurisdiction-specific counsel signs off before launch in each market.

---

## 2. Regulatory map

| Area | Examples | Why it applies | Posture |
|---|---|---|---|
| Education | Accreditation rules, diploma mills, distance learning | Levels and credentials could be mistaken for degrees | No degree claims; credentials are skill attestations |
| Employment | Hiring discrimination, background checks, job-listing rules | Recommendations and employer matching | No guaranteed jobs; no protected-trait use |
| Consumer protection | Unfair/deceptive practices, dark patterns | Points, marketplace, subscriptions | Transparent terms, no fake urgency |
| Payments/e-money | Money transmission, stored value, prepaid rules | EC balances | EC is not cash; no cash-out; closed-loop |
| Tax | Income from rewards, VAT/GST on goods | Redemptions, partner payouts | Tax reporting; EC treated as discount/reward per jurisdiction |
| Data protection | GDPR, UK GDPR, CCPA/CPRA, LGPD, PIPL | Profiles, assessments, biometrics | Consent, minimization, rights |
| Child safety | COPPA, GDPR-K, AADC, state laws | Minors | Guardian consent, restricted design |
| Accessibility | ADA, EAA, EN 301 549 | Web/mobile/spatial | WCAG 2.2 AA |
| AI | EU AI Act, state AI laws, FTC guidance | Tutoring, grading, recommendations | Transparency, human oversight, bias testing |
| Financial promotion | Investment/credit claims | Marketplace categories | Restricted categories blocked |
| Gambling/loot boxes | Randomized rewards | Leagues, prestige | No paid randomization; no cash value |
| Sanctions/AML | Partner and payout screening | Marketplace partners | KYC/KYB, screening |

---

## 3. Education posture

### 3.1 What we are
- A skills assessment and practice platform.
- A verifiable credential issuer for specific, scoped competencies.
- A rewards program for demonstrated learning.

### 3.2 What we are not
- An accredited institution.
- A degree-granting body.
- A professional licensure body.
- A substitute for regulated training (medicine, law, aviation, electrical licensure).

### 3.3 Rules
- Never call a level a "degree," "diploma," or "license."
- Always show scope, date, and evidence link.
- High-risk domains carry explicit scope disclaimers.
- Partner with accredited bodies where credit matters.
- Comply with local rules on the use of "certified" and "accredited."

### 3.4 Recognition strategy
- Seek articulation agreements with employers and institutions.
- Map levels to national qualification frameworks where possible.
- Independent third-party audit of assessment quality.
- Publish methodology.

---

## 4. Employment posture

- No job guarantees.
- Employer views require learner consent.
- No use of protected traits in recommendations or matching.
- Adverse-impact monitoring.
- Background checks (where required) are done by employers, not the platform.
- Job listings follow local law and are moderated.
- Bounties are contracts with clear scope and payment terms.

---

## 5. Consumer protection

| Risk | Control |
|---|---|
| Misleading earnings claims | Ranges, not promises; disclosed assumptions |
| Dark patterns | No forced continuity, no fake countdowns |
| Hidden fees | All fees shown before confirmation |
| Unfair terms | Plain-language terms; unfair clauses removed |
| Subscription traps | Easy cancel; renewal reminders |
| Points expiry | Disclosed; reasonable notice |
| Price changes | Advance notice for material changes |

---

## 6. Payments and stored value

- EC is a closed-loop reward credit, not legal tender.
- No cash-out in v1.
- No person-to-person transfer of EC (except approved mentor/bounty flows).
- Balances are backed by reserves (see [Marketplace economics](../01-economy/06-marketplace-economics.md)).
- Terms state EC has no cash value and is not a deposit.
- Where local law treats stored value as regulated, obtain advice and adapt (e.g., limit balances, use partner-issued instruments).

### 6.1 Tax
- Redemptions may be taxable benefits in some jurisdictions.
- Provide annual statements where required.
- Partner payouts include tax reporting.
- Do not provide tax advice to learners.

---

## 7. AI regulation

| Requirement | Implementation |
|---|---|
| Transparency | Disclose AI use in tutoring, grading, recommendations |
| Human oversight | Human review at high-stakes gates |
| Risk management | AI risk register, evals, incident response |
| Data governance | Consent, minimization, no training without consent |
| Bias monitoring | Fairness audits, DIF, adverse impact |
| Record-keeping | Model/prompt/version logs |
| Prohibited practices | No social scoring, no manipulative design, no exploitation of vulnerability |

High-risk classification (e.g., education access) triggers stricter documentation and conformity assessment in some jurisdictions.

---

## 8. Jurisdictional launch gates

Before launching in a country:

1. Legal entity and tax registration.
2. Data protection registration/representative if required.
3. Consumer terms localized.
4. Payment and rewards model reviewed.
5. Child-safety settings configured.
6. Accessibility conformance.
7. Marketplace partner compliance.
8. Content localization and review.
9. Sanctions screening.
10. Incident and breach notification process.

---

## 9. Requirements

- `FR-LEG-001`: No degree, license, or accreditation claims.
- `FR-LEG-002`: Every credential shows scope and date.
- `FR-LEG-003`: EC terms state no cash value.
- `FR-LEG-004`: All fees are disclosed before purchase.
- `FR-LEG-005`: No dark patterns in cancellation or renewal.
- `FR-LEG-006`: Protected traits are never used in matching.
- `FR-LEG-007`: AI use is disclosed and documented.
- `FR-LEG-008`: Human oversight exists for high-stakes decisions.
- `FR-LEG-009`: Jurisdictional launch gates are completed.
- `FR-LEG-010`: Partners are screened for sanctions and licenses.
- `NFR-LEG-001`: Terms available in every supported language.
- `NFR-LEG-002`: Legal holds preserve data reliably.

---

## 10. Related

- [Credentials & employment](02-credentials-employment.md)
- [IP & content](03-ip-content.md)
- [Privacy & data](../09-trust-safety/03-privacy-data.md)
- [AI safety](../09-trust-safety/02-ai-safety.md)
