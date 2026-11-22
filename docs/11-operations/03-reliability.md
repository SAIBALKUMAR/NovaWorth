# Reliability & Cost Controls

## 1. Purpose

This document defines SLOs, capacity planning, cost controls, and the operational practices that keep the product fast, available, and affordable at scale.

**Two budgets matter equally:** the error budget and the money budget.

---

## 2. Service level objectives

| Service | Availability | Latency | Error budget (30d) |
|---|---|---|---|
| API gateway | 99.95% | p95 <300 ms | 21.6 min |
| Identity | 99.95% | p95 <250 ms | 21.6 min |
| Learning | 99.9% | p95 <400 ms | 43.2 min |
| Assessment | 99.9% | p95 <500 ms | 43.2 min |
| Ledger | 99.99% | p99 <150 ms | 4.3 min |
| Marketplace | 99.9% | p95 <500 ms | 43.2 min |
| AI tutor | 99.5% | first token <2 s | 3.6 h |
| Media CDN | 99.9% | start <2 s | 43.2 min |
| Notifications | 99.5% | <5 s | 3.6 h |

### 2.1 Error budget policy
- Budget >50%: ship normally.
- Budget 20–50%: review risk, freeze risky changes.
- Budget <20%: feature freeze; reliability work only.
- Budget exhausted: postmortem required; release requires exec approval.

---

## 3. Capacity planning

### 3.1 Inputs
- Weekly active learners.
- Peak concurrency (typically 2–3× average).
- Assessment spikes (end of month, cohort deadlines).
- Media egress.
- AI token demand.

### 3.2 Model
```
peak_rps = WAL × sessions_per_learner_per_day × requests_per_session / 86,400 × peak_factor
```
Plan for 2× forecast with 30% headroom.

### 3.3 Load testing
- Monthly load test at 2× peak.
- Quarterly soak test (24h).
- Chaos drills on critical paths.
- Game-day for assessment spikes.

---

## 4. Cost model

### 4.1 Unit economics

| Cost | Driver | Target |
|---|---|---|
| AI inference | Tokens per active learner | <$1.50/learner/month |
| Media egress | Video minutes | <$0.35/learner/month |
| Compute | Requests, sims | <$0.60/learner/month |
| Storage | Artifacts, media | <$0.15/learner/month |
| Support | Tickets | <$0.40/learner/month |
| Payments | Payouts | 1.5–3% of GMV |
| **Total** | | **<$3.50/learner/month** |

### 4.2 Levers
- Model routing and caching.
- Transcript-first media.
- Sim instancing and replay caching.
- Artifact lifecycle (cold storage).
- Regional compute placement.
- Batch grading.

### 4.3 Guardrails
- Per-feature budget alerts at 80%.
- Hard caps with graceful degradation.
- Weekly cost review.
- Anomaly detection on spend.

---

## 5. Observability

| Signal | Tool | Retention |
|---|---|---|
| Metrics | Prometheus/Grafana | 13 months |
| Traces | OpenTelemetry | 30 days |
| Logs | Centralized | 90 days |
| Errors | Sentry | 90 days |
| Synthetic checks | Uptime monitor | 13 months |
| Business metrics | Warehouse | 25 months |

### 5.1 Dashboards
- Golden signals per service.
- Learner journey funnel.
- Assessment health.
- Ledger reconciliation.
- AI cost and quality.
- Marketplace fulfillment.

### 5.2 Alerts
- SLO burn rate (fast and slow).
- Error spikes.
- Latency regression.
- Ledger imbalance.
- Cost anomaly.
- Safety classifier failure.

Every alert has a runbook and an owner.

---

## 6. Data protection and backup

| Data | Backup | RPO | RTO |
|---|---|---|---|
| Ledger | Continuous WAL + daily snapshot | <5 min | <1 h |
| Learner data | Daily snapshot + PITR | <15 min | <4 h |
| Content | Versioned object store | 0 | <1 h |
| Artifacts | Cross-region replication | <1 h | <4 h |
| Analytics | Rebuildable from events | <24 h | <48 h |

Restore drills quarterly. A backup that has never been restored is a hypothesis.

---

## 7. Change management

- All changes via CI/CD.
- Canary 1% → 10% → 50% → 100%.
- Automatic rollback on SLO burn.
- Database migrations backward-compatible.
- Feature flags for risky changes.
- Change freeze during major assessment windows unless approved.

---

## 8. On-call

- Follow-the-sun where possible.
- Primary/secondary rotation, 1 week.
- Page only for actionable alerts.
- 15-minute acknowledgment.
- Escalation to service owner.
- Compensation and fatigue limits.

---

## 9. Metrics

| Metric | Target |
|---|---|
| Availability vs SLO | ≥99.9% |
| p95 API latency | <300 ms |
| Error budget remaining | >30% |
| Cost per learner | <$3.50/month |
| Cost anomaly detection | <1h |
| Backup restore success | 100% |
| Change failure rate | <10% |
| MTTR | <45 min |
| Toil ratio | <30% |

---

## 10. Requirements

- `FR-OPS-301`: Every service has an SLO and owner.
- `FR-OPS-302`: Error budget policy is enforced.
- `FR-OPS-303`: Capacity is planned for 2× forecast.
- `FR-OPS-304`: Cost is tracked per feature and learner.
- `FR-OPS-305`: Alerts have runbooks.
- `FR-OPS-306`: Backups are tested quarterly.
- `FR-OPS-307`: Deploys are canaried and reversible.
- `FR-OPS-308`: Ledger reconciliation runs daily.
- `FR-OPS-309`: On-call has 15-minute acknowledgment.
- `FR-OPS-310`: Cost caps degrade gracefully, never break learning.
- `NFR-OPS-301`: p95 API <300 ms.
- `NFR-OPS-302`: RPO <5 min for ledger.
- `NFR-OPS-303`: RTO <1 h for ledger.

---

## 11. Related

- [Architecture](../06-platforms/01-architecture.md)
- [Incident response](04-incident-response.md)
- [Analytics](../10-data-ai/05-analytics.md)
