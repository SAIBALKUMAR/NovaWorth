# Credentials & Employment Claims

## 1. Purpose

A credential is a promise. If it overclaims, the whole economy loses credibility. This document defines what a credential asserts, how it is issued and verified, and what the product may and may not say to employers.

---

## 2. What a credential asserts

A credential asserts exactly this:

> "On this date, this learner demonstrated the level-`N` outcome for skill `X` under assessment version `V`, with integrity tier `T`, evidenced by artifacts hashed as `H`."

It does **not** assert:
- Licensure or legal authorization to practice.
- A degree or academic credit.
- Guaranteed employment or salary.
- General intelligence or character.
- Competence outside the stated level and scope.

---

## 3. Credential anatomy

| Field | Example |
|---|---|
| Credential ID | `cred_01H...` |
| Subject | Pseudonymous ID or consented name |
| Skill | `CMP-25 Backend development` |
| Level | `40` |
| Outcome statement | "Can design and operate a service handling 10k rps with observability" |
| Date | 2026-11-21 |
| Evidence hash | `sha256:...` |
| Rubric version | `rub-2026.1` |
| Assessment version | `asm-cmp25-l40-v3` |
| Integrity tier | `proctored` |
| Issuer | Education Economy |
| Revocation status | `active` |

Format: W3C Verifiable Credential (JSON-LD), signed; PDF and JSON export.

---

## 4. Verification

### 4.1 Public page
- Human-readable summary.
- Evidence hash and verification link.
- Revocation status.
- No raw responses, no PII beyond consent.

### 4.2 API
```
GET /v1/credentials/{id}
→ { valid, subject_pseudonym, skill, level, issued_at,
    integrity_tier, revocation_status, evidence_hash }
```
- Rate-limited.
- Requires an API key for bulk verification.
- Returns only consented fields.

### 4.3 QR code
Encodes the verification URL. Scannable by employers.

---

## 5. Revocation

| Trigger | Action |
|---|---|
| Confirmed fraud | Revoke + notify verifiers |
| Integrity failure | Revoke or downgrade tier |
| Assessment error | Re-issue corrected credential |
| Learner request | Mark private (not deleted if legal hold) |
| Account deletion | Credential retained as hash-only, or deleted per policy |

Revocation is public and immediate. Verifiers can subscribe to revocation events.

---

## 6. Employer claims

### 6.1 Allowed
- "Verified level 40 in Backend Development."
- "Assessed on 2026-11-21 under proctored conditions."
- "Evidence available on request with learner consent."

### 6.2 Not allowed
- "Certified engineer."
- "Licensed professional."
- "Equivalent to a bachelor's degree."
- "Top 1% globally" (unless a published, dated ranking exists).
- "Guaranteed job-ready."

### 6.3 Marketing rules
- Show distributions, not just successes.
- Disclose pass rates and time-to-level.
- No cherry-picked outcomes without context.
- Earnings claims include assumptions and ranges.

---

## 7. Employer integration

| Feature | Description |
|---|---|
| Candidate verification | Verify a credential ID or link |
| Cohort dashboard | Aggregate outcomes (consented) |
| Skill search | Find candidates by verified level |
| Bounty posting | Scoped work with clear payment |
| Interview kits | Level-aligned practical tasks |

Rules:
- No access to raw assessment responses.
- No access to life-experience data.
- No protected-trait filtering.
- Consent required for individual profiles.
- Fair-hiring audit annually.

---

## 8. Recognition partnerships

- Articulation agreements with employers.
- Credit recommendations via ACE/NARIC-style evaluation (where feasible).
- Alignment with national qualification frameworks.
- University partnerships for advanced standing.
- Professional body recognition for scoped competencies.

Each partnership is disclosed and scoped; none implies accreditation.

---

## 9. Disputes

- Employer disputes a credential → verification review.
- Learner disputes a revocation → appeal path.
- Third party alleges fraud → investigation with evidence.
- Outcomes documented and appealable.

---

## 10. Requirements

- `FR-LEG-101`: Every credential states scope, date, and integrity tier.
- `FR-LEG-102`: Credentials are machine-verifiable.
- `FR-LEG-103`: Revocation is public and immediate.
- `FR-LEG-104`: Employer views require consent.
- `FR-LEG-105`: Raw responses are never shared.
- `FR-LEG-106`: No licensure or degree equivalence claims.
- `FR-LEG-107`: Marketing discloses pass rates and assumptions.
- `FR-LEG-108`: Protected traits are excluded from matching.
- `FR-LEG-109`: Partnerships are disclosed and scoped.
- `FR-LEG-110`: Credential disputes have a documented path.
- `NFR-LEG-101`: Verification response <1 s.
- `NFR-LEG-102`: Revocation propagates <5 min.

---

## 11. Related

- [Profile & reputation](../07-experience/04-profile-reputation.md)
- [Regulatory](01-regulatory.md)
- [Assessment system](../05-assessment/01-assessment-system.md)
