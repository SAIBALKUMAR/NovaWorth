# Web Platform

## 1. Purpose

The web app is the primary acquisition and deep-work surface. It must load fast on mid-range devices, work offline, and be fully accessible.

**Stack:** Next.js + React + TypeScript, Tailwind, PWA, WebGPU/WebGL for sims.

---

## 2. Surfaces

| Surface | Purpose |
|---|---|
| Marketing site | Explain, browse catalog, start onboarding |
| Learner app | Learn, practice, assess, wallet, marketplace |
| Skill pages | Public previews, level ladder, proof |
| Profile/wallet | Credentials, evidence, EC |
| Sponsor dashboard | Cohorts, budgets, outcomes |
| Author studio | Content authoring and review |
| Admin | Support, moderation, integrity |
| Verification | Public credential check |

---

## 3. Performance

| Metric | Target |
|---|---|
| LCP | <2.5 s on 4G mid-range |
| INP | <200 ms |
| CLS | <0.1 |
| First load JS | <200 KB gzipped |
| Time to interactive | <3 s |
| Offline start | <1 s |

### 3.1 Techniques
- Server components / streaming.
- Route-level code splitting.
- Image optimization (AVIF/WebP).
- Font subsetting.
- Prefetch likely next routes.
- Service worker caching.
- Edge caching for public pages.

---

## 4. PWA

- Installable.
- Offline shell.
- Level packs downloaded.
- Background sync for attempts.
- Push notifications (opt-in).
- App-like navigation.

### 4.1 Offline scope
- Levels 1–10 fully offline.
- Review queue offline.
- Attempts queued.
- No offline high-stakes gates.
- Conflict resolution: server wins for scores.

---

## 5. Browser support

| Browser | Support |
|---|---|
| Chrome/Edge (last 2) | Full |
| Safari (last 2) | Full |
| Firefox (last 2) | Full |
| Mobile Safari | Full |
| Chrome Android | Full |
| Older browsers | Graceful degradation |

No feature may require a browser-specific API without a fallback.

---

## 6. Accessibility

- WCAG 2.2 AA.
- Keyboard-only flows.
- Screen reader tested (NVDA, JAWS, VoiceOver).
- Captions/transcripts.
- Reduced motion.
- High contrast.
- Text resize to 200%.
- Focus visible.
- No color-only meaning.

---

## 7. Security

- HTTPS only.
- CSP, HSTS, X-Frame-Options.
- CSRF protection.
- Secure cookies.
- No secrets in client.
- Dependency scanning.
- Rate limiting.

---

## 8. SEO and sharing

- Public skill pages with structured data.
- Level ladders indexable.
- Credential verification pages.
- Open Graph cards.
- No indexing of private learner data.

---

## 9. Analytics

- Privacy-preserving by default.
- Consent-based marketing analytics.
- Product events via first-party pipeline.
- No third-party ad pixels on learning pages.

---

## 10. Requirements

- `FR-PLT-101`: Web works on mid-range Android and iOS.
- `FR-PLT-102`: Core learning works offline.
- `FR-PLT-103`: Web is WCAG 2.2 AA.
- `FR-PLT-104`: No third-party ad tracking on learning pages.
- `FR-PLT-105`: Public skill pages are indexable.
- `FR-PLT-106`: Credential verification is public.
- `FR-PLT-107`: Sponsor dashboards are consent-based.
- `FR-PLT-108`: Author studio supports review workflows.
- `FR-PLT-109`: Web supports keyboard-only completion.
- `FR-PLT-110`: Web supports 200% text zoom.
- `NFR-PLT-101`: LCP <2.5 s.
- `NFR-PLT-102`: INP <200 ms.
- `NFR-PLT-103`: Offline start <1 s.
