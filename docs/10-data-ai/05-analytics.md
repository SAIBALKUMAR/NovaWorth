# Analytics

## 1. Purpose

Analytics answers: are learners learning, is the economy healthy, is the content good, and where does the product break? It must be privacy-preserving and auditable.

---

## 2. Event schema

```json
{
  "event_id": "evt_...",
  "type": "level.passed",
  "version": 1,
  "occurred_at": "2026-11-21T10:00:00Z",
  "user_pseudo_id": "u_...",
  "session_id": "s_...",
  "platform": "web|ios|android|visionos",
  "app_version": "1.2.3",
  "skill_id": "CMP-25",
  "level": 30,
  "properties": {},
  "trace_id": "..."
}
```

### 2.1 Rules
- No PII.
- Pseudonymous IDs.
- Schema registry.
- Client + server events.
- Idempotent ingestion.

---

## 3. Core events

| Area | Events |
|---|---|
| Acquisition | `page_viewed`, `signup_started`, `signup_completed` |
| Onboarding | `step_viewed`, `step_completed`, `recommendation_shown`, `recommendation_accepted` |
| Learning | `session_started`, `content_viewed`, `practice_attempted`, `review_completed` |
| Assessment | `gate_started`, `gate_submitted`, `gate_passed`, `gate_failed` |
| Economy | `award_posted`, `wallet_viewed`, `order_placed`, `order_fulfilled` |
| Social | `guild_joined`, `mentor_session`, `peer_review` |
| Trust | `integrity_flagged`, `appeal_filed`, `appeal_decided` |

---

## 4. Warehouse

- Raw events → staging → marts.
- dbt models.
- Daily/hourly refresh.
- Data quality tests.
- Lineage.
- Access controls.

### 4.1 Marts
- `dim_user`, `dim_skill`, `dim_level`, `dim_content`.
- `fct_sessions`, `fct_attempts`, `fct_awards`, `fct_orders`.
- `mart_learning_funnel`, `mart_economy`, `mart_content_health`.

---

## 5. Dashboards

| Dashboard | Audience | Cadence |
|---|---|---|
| North star | All | Weekly |
| Learning funnel | Product | Daily |
| Content health | Learning | Weekly |
| Economy | Finance/Leadership | Monthly |
| Marketplace | Partnerships | Weekly |
| Trust | Trust & Safety | Weekly |
| Accessibility | Product/QA | Monthly |
| AI cost/quality | Engineering | Weekly |

---

## 6. Experimentation

- Assignment at user level.
- Guardrail metrics.
- Sequential testing.
- No peeking without correction.
- Economy experiments require holdout.
- Results archived.

---

## 7. Privacy

- Aggregation thresholds (k≥10).
- No re-identification.
- No PII in events.
- Consent for research.
- Retention per policy.
- Access logged.

---

## 8. Data quality

| Check | Frequency |
|---|---|
| Event schema validation | Real-time |
| Null/duplicate checks | Daily |
| Reconciliation (ledger) | Daily |
| Funnel anomaly | Daily |
| Metric drift | Weekly |

---

## 9. Requirements

- `FR-ANL-001`: Every event has a schema and version.
- `FR-ANL-002`: No PII in analytics.
- `FR-ANL-003`: Events are idempotent.
- `FR-ANL-004`: Dashboards have owners.
- `FR-ANL-005`: Experiments have guardrails.
- `FR-ANL-006`: Economy data reconciles with ledger.
- `FR-ANL-007`: Access is role-based.
- `FR-ANL-008`: Aggregation thresholds are enforced.
- `FR-ANL-009`: Data quality tests run daily.
- `FR-ANL-010`: Retention is enforced.
- `NFR-ANL-001`: Event ingestion 10k/s.
- `NFR-ANL-002`: Dashboard freshness <1h.
