# Profile, Skill Wallet & Reputation

## 1. Purpose

The profile is the learner's portable proof: what they can do, how it was verified, and how trustworthy they are. It must be useful to the learner, credible to employers, and resistant to inflation.

---

## 2. Skill wallet

### 2.1 Contents
- Skills and levels.
- XP and EC (private by default).
- Credentials (verifiable).
- Evidence artifacts.
- Prestige.
- Reputation.
- Current vs expired badges.

### 2.2 Display
| Element | Public | Private |
|---|---|---|
| Skill + level | Yes (opt-in) | Default private |
| Credential | Yes | — |
| Evidence | Opt-in | Default private |
| EC balance | No | Yes |
| Reputation | Aggregate | Detail |
| Real name | Opt-in | Default display name |

### 2.3 Portability
- Export as JSON + PDF.
- W3C Verifiable Credentials.
- Public verification URL.
- No lock-in.

---

## 3. Credentials

### 3.1 Contents
- Learner (pseudonymous ID or name).
- Skill, level, date.
- Evidence hash.
- Rubric version.
- Integrity tier.
- Issuer signature.
- Revocation status.

### 3.2 Verification
- Public page.
- QR code.
- API for employers.
- Revocation list.

### 3.3 Claims
- "Verified level 40 in Backend Development."
- Not "licensed engineer."
- Not "guaranteed job."
- Scope and date always shown.

---

## 4. Reputation

### 4.1 Inputs
| Input | Weight |
|---|---|
| Verified evidence | 30% |
| Peer review quality | 20% |
| Mentoring outcomes | 20% |
| Integrity history | 20% |
| Community conduct | 10% |

### 4.2 Properties
- 0–100.
- Decays slowly with inactivity.
- Recoverable.
- Not purchasable.
- Not transferable.

### 4.3 Uses
- Examiner eligibility.
- Guild leadership.
- Marketplace trust.
- Higher redemption limits.
- Bounty access.

---

## 5. Public profile

### 5.1 Sections
- Headline and bio.
- Skill wallet.
- Featured evidence.
- Prestige.
- Guilds.
- Availability for mentoring/bounties.

### 5.2 Controls
- Per-section visibility.
- Incognito mode.
- Block search indexing.
- Delete account.

---

## 6. Employer view

- Consent-based.
- Verified credentials only.
- No raw assessment responses.
- No life experience data.
- Aggregate cohort views.
- API integration.

---

## 7. Anti-gaming

| Risk | Control |
|---|---|
| Fake credentials | Signed VCs, verification |
| Inflated claims | Evidence hash, scope |
| Bought reputation | Not purchasable |
| Collusive reviews | Graph analysis |
| Plagiarized portfolio | Provenance, oral defense |
| Account sale | Device/behavior checks |

---

## 8. Requirements

- `FR-SOC-101`: Every credential is verifiable.
- `FR-SOC-102`: Profiles have granular privacy.
- `FR-SOC-103`: EC balance is private by default.
- `FR-SOC-104`: Credentials are exportable.
- `FR-SOC-105`: Reputation is not purchasable.
- `FR-SOC-106`: Employer views require consent.
- `FR-SOC-107`: Raw responses are never shared.
- `FR-SOC-108`: Revocation is visible.
- `FR-SOC-109`: Minors are private by default.
- `FR-SOC-110`: Public profiles can be deleted.
- `NFR-SOC-101`: Verification page <1 s.
- `NFR-SOC-102`: Export <24 h.
