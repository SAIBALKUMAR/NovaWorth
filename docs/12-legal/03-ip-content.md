# IP, Content Licensing & AI Provenance

## 1. Purpose

The content factory uses AI, human authors, licensed material, learner artifacts, and partner brands. This document defines ownership, licensing, provenance, and infringement handling.

**Principle:** every asset has a clear chain of rights.

---

## 2. Ownership model

| Asset | Owner | License to platform |
|---|---|---|
| Platform code | Platform | Proprietary |
| Curriculum (levels, items) | Platform or contracted author | Assignment or exclusive license |
| Human-authored media | Author/contractor | Work-for-hire or exclusive license |
| AI-generated drafts | Platform (subject to provider terms) | Per provider terms |
| Learner artifacts | Learner | Limited license for assessment/verification |
| Partner brands | Partner | Scoped marketing license |
| Third-party references | Original owner | Fair use or licensed excerpt |

---

## 3. Author agreements

- Assignment of work product or exclusive license.
- Moral rights handled per jurisdiction.
- Attribution policy (opt-in for public credit).
- Royalty or flat fee, disclosed.
- Warranty of originality.
- Indemnity for infringement.
- Confidentiality.
- AI-use disclosure by the author.

---

## 4. AI provenance

### 4.1 Required metadata
| Field | Example |
|---|---|
| AI involvement | `none` / `assisted` / `generated` |
| Model | `provider/model@version` |
| Prompt/template version | `lvl-draft-v4.2` |
| Source pack | `src-cmp25-v3` |
| Human editor | `usr_...` |
| Reviewers | SME, ID, safety, accessibility |
| Date | ISO 8601 |

### 4.2 Rules
- No AI-generated content publishes without human review.
- AI must not reproduce copyrighted text verbatim beyond fair use.
- Citations are verified; fabricated citations are a publish blocker.
- Model provider terms must permit commercial use and output ownership.
- Learners can see a "How was this made?" summary.

### 4.3 Disclosure to learners
- Label AI-generated media in the player.
- Label AI-assisted grading where relevant.
- Explain limitations.
- Never present a synthetic person as a real expert.

---

## 5. Third-party content

### 5.1 Allowed with license
- Textbook excerpts (licensed).
- Standards documents (licensed).
- Images/diagrams (licensed or CC).
- Datasets (licensed).
- Code snippets (license-compatible).

### 5.2 Allowed as reference
- Links to public sources.
- Short quotations with attribution.
- Facts and data (not protected as such, but verified).

### 5.3 Not allowed
- Pirated textbooks or paywalled content.
- Unlicensed images or music.
- Scraped proprietary course content.
- Reproducing a competitor's item bank.
- Trademark use implying endorsement.

---

## 6. Learner content

- Learner owns their artifacts.
- Platform gets a limited license to store, score, and verify.
- Public sharing is opt-in.
- Learners can export and delete (subject to credential retention).
- Peer reviewers see only what is shared.
- No use of learner artifacts to train models without explicit consent.

---

## 7. Open-source and licenses

- Maintain an SBOM and license inventory.
- Copyleft dependencies isolated from proprietary code.
- Attribution file for OSS.
- License compatibility checks in CI.
- No GPL code in closed client bundles without review.

---

## 8. Brand and trademark

- Platform marks are protected.
- Partner marks used only under agreement.
- No implied endorsement.
- Credential badges follow brand guidelines.
- Domain names and social handles monitored.

---

## 9. Infringement handling

```
Notice → Acknowledge → Assess → Remove/Disable (if valid)
→ Notify uploader → Counter-notice path → Restore or keep down
→ Record → Repeat-infringer policy
```

- Designated agent for DMCA (US) and equivalents.
- Response within legal timelines.
- Repeat infringers lose author/publisher access.
- Learner artifacts: learner notified before removal where lawful.

---

## 10. Data and database rights

- Curriculum database may attract sui generis rights in some jurisdictions.
- Scraping is prohibited by terms and technical controls.
- Item banks are watermarked and monitored.
- API access is licensed, not sold.

---

## 11. Requirements

- `FR-LEG-201`: Every asset has a rights record.
- `FR-LEG-202`: Every AI-involved asset has provenance.
- `FR-LEG-203`: Human review precedes AI content publication.
- `FR-LEG-204`: Citations are verified before publish.
- `FR-LEG-205`: Learners own their artifacts with a limited license.
- `FR-LEG-206`: No model training on learner data without consent.
- `FR-LEG-207`: OSS licenses are inventoried and compatible.
- `FR-LEG-208`: Infringement notices are handled within legal timelines.
- `FR-LEG-209`: Repeat infringers lose publishing access.
- `FR-LEG-210`: Partner marks are used only under agreement.
- `NFR-LEG-201`: Rights record lookup <200 ms.
- `NFR-LEG-202`: Provenance available for 100% of assets.

---

## 12. Related

- [Content generation](../10-data-ai/03-content-generation.md)
- [Regulatory](01-regulatory.md)
- [Privacy & data](../09-trust-safety/03-privacy-data.md)
