# Data Model

## 1. Purpose

This document defines the core entities, relationships, identifiers, and data lifecycle. It is the shared contract for backend, data, AI, and analytics.

---

## 2. Entity map

```
User ──< Consent
  │
  ├──< ExperienceNode ──< Edge
  ├──< Profile
  ├──< LearnerSkill ──< LevelProgress ──< Attempt
  │                         │                │
  │                         │                ├──< Response
  │                         │                ├──< Artifact
  │                         │                └──< RubricScore
  │                         │
  │                         └──< Award ──< LedgerEntry
  │
  ├──< Wallet ──< LedgerEntry
  ├──< Order ──< OrderItem
  ├──< GuildMember
  └──< ReputationEvent

Skill ──< Level ──< Concept ──< Item
  │         │          │
  │         └──< Content
  └──< PrerequisiteEdge
```

---

## 3. Core entities

### 3.1 User
| Field | Type | Notes |
|---|---|---|
| `user_id` | UUID | Pseudonymous |
| `email_hash` | string | Unique |
| `display_name` | string | Optional |
| `locale` | string | BCP-47 |
| `region` | string | ISO |
| `dob_year` | int | Age gating only |
| `guardian_id` | UUID | Nullable |
| `status` | enum | active, suspended, deleted |
| `created_at` | timestamp | |

### 3.2 Consent
| Field | Type |
|---|---|
| `consent_id` | UUID |
| `user_id` | UUID |
| `purpose` | enum |
| `granted` | bool |
| `version` | string |
| `granted_at` | timestamp |
| `revoked_at` | timestamp |

### 3.3 ExperienceNode
| Field | Type |
|---|---|
| `node_id` | UUID |
| `user_id` | UUID |
| `type` | enum (role, project, experience, interest, constraint, evidence) |
| `label` | string |
| `detail` | jsonb |
| `confidence` | enum |
| `visibility` | enum |
| `created_at` | timestamp |

### 3.4 Skill
| Field | Type |
|---|---|
| `skill_id` | string (`CMP-25`) |
| `domain_code` | string |
| `name` | string |
| `archetype` | enum |
| `level_100_outcome` | text |
| `domain_multiplier` | decimal |
| `safety_class` | enum |
| `version` | semver |

### 3.5 Level
| Field | Type |
|---|---|
| `level_id` | string (`CMP-25-L42`) |
| `skill_id` | string |
| `level` | int |
| `title` | string |
| `outcome` | text |
| `prerequisites` | jsonb |
| `expected_hours` | decimal |
| `difficulty` | decimal |
| `assessment_blueprint` | jsonb |
| `version` | semver |
| `status` | enum |

### 3.6 Concept
| Field | Type |
|---|---|
| `concept_id` | UUID |
| `level_id` | string |
| `name` | string |
| `description` | text |
| `prerequisites` | jsonb |

### 3.7 Content
| Field | Type |
|---|---|
| `content_id` | UUID |
| `concept_id` | UUID |
| `type` | enum (text, video, audio, example, practice) |
| `body` | text/uri |
| `provenance` | jsonb |
| `version` | semver |
| `locale` | string |

### 3.8 Item
| Field | Type |
|---|---|
| `item_id` | UUID |
| `level_id` | string |
| `type` | enum |
| `stem` | text |
| `options` | jsonb |
| `answer` | encrypted |
| `difficulty` | decimal |
| `discrimination` | decimal |
| `exposure` | int |
| `version` | semver |

### 3.9 Attempt
| Field | Type |
|---|---|
| `attempt_id` | UUID |
| `user_id` | UUID |
| `level_id` | string |
| `kind` | enum (practice, gate, diagnostic) |
| `started_at` | timestamp |
| `submitted_at` | timestamp |
| `score` | decimal |
| `integrity_score` | decimal |
| `status` | enum |
| `rule_version` | string |

### 3.10 Award
| Field | Type |
|---|---|
| `award_id` | UUID |
| `user_id` | UUID |
| `skill_id` | string |
| `level` | int |
| `attempt_id` | UUID |
| `xp` | int |
| `ec` | int |
| `rule_version` | string |
| `funding_pool_id` | UUID |
| `status` | enum |

### 3.11 LedgerEntry
| Field | Type |
|---|---|
| `entry_id` | UUID |
| `transaction_id` | UUID |
| `account_id` | UUID |
| `direction` | enum |
| `amount` | bigint |
| `currency` | string |
| `reference_type` | enum |
| `reference_id` | UUID |
| `created_at` | timestamp |

### 3.12 Order
| Field | Type |
|---|---|
| `order_id` | UUID |
| `user_id` | UUID |
| `item_id` | UUID |
| `quantity` | int |
| `ec_total` | int |
| `status` | enum |
| `escrow_tx_id` | UUID |
| `fulfillment` | jsonb |
| `created_at` | timestamp |

### 3.13 ReputationEvent
| Field | Type |
|---|---|
| `event_id` | UUID |
| `user_id` | UUID |
| `type` | enum |
| `weight` | decimal |
| `source_id` | UUID |
| `created_at` | timestamp |

---

## 4. Identifiers

| Entity | Format |
|---|---|
| User | `usr_` + UUID |
| Skill | `DOMAIN-NN` |
| Level | `SKILL-L##` |
| Attempt | `att_` + UUID |
| Award | `awd_` + UUID |
| Ledger | `led_` + UUID |
| Order | `ord_` + UUID |
| Credential | `cred_` + UUID |

---

## 5. Versioning

- Skills, levels, content, items, rubrics: semver.
- Rule versions: `score-YYYY.N`.
- Events: schema version.
- Historical attempts retain their version.
- Migrations are backward-compatible or dual-written.

---

## 6. Data lifecycle

| Stage | Policy |
|---|---|
| Create | Validate, encrypt sensitive fields |
| Update | Audit log, version |
| Delete | Soft delete → hard delete per retention |
| Export | JSON/PDF, 30 days |
| Archive | Cold storage after 2 years |
| Purge | Per retention policy |

---

## 7. Privacy

- Pseudonymous IDs.
- PII separated from behavioral data.
- Field-level encryption for sensitive data.
- Access controls and audit.
- No PII in analytics events.

---

## 8. Requirements

- `FR-DAT-001`: Every entity has a stable ID.
- `FR-DAT-002`: Every content/assessment artifact is versioned.
- `FR-DAT-003`: Ledger entries are immutable.
- `FR-DAT-004`: PII is separated from events.
- `FR-DAT-005`: Sensitive fields are encrypted.
- `FR-DAT-006`: Deletion propagates within 30 days.
- `FR-DAT-007`: Exports include all learner data.
- `FR-DAT-008`: Schema changes are versioned.
- `FR-DAT-009`: Access is audited.
- `FR-DAT-010`: Retention is enforced automatically.
- `NFR-DAT-001`: Point lookup <50 ms.
- `NFR-DAT-002`: Event ingestion 10k/s.
