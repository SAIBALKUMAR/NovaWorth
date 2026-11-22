# Mobile Platform

## 1. Purpose

Mobile is likely the primary device for the core audience. It must support learning in small pockets of time, offline, with camera/mic evidence and push nudges.

**Stack:** React Native + Expo (or native modules where needed), shared design system, offline-first sync.

---

## 2. Surfaces

| Surface | Purpose |
|---|---|
| Today | Next best action, review, momentum |
| Learn | Levels, practice, media |
| Assess | Gate attempts, artifacts |
| Wallet | XP, EC, awards, redemption |
| Profile | Skills, credentials, evidence |
| Guild | Social, leagues, mentoring |
| Capture | Camera/mic evidence upload |

---

## 3. Offline-first

### 3.1 Downloadable packs
- Level content (text, images, audio).
- Practice items.
- 1 assessment variant (low-stakes).
- Review queue.
- Glossary.

### 3.2 Sync
- Queue attempts, artifacts, reflections.
- Background sync when online.
- Conflict resolution: server wins for scores; learner edits preserved as drafts.
- Clear sync status.

### 3.3 Storage
- Cap packs at 200 MB per skill.
- Auto-evict completed packs.
- User controls downloads.
- Low-storage warnings.

---

## 4. Device capabilities

| Capability | Use |
|---|---|
| Camera | Artifact photos, practical evidence |
| Mic | Oral practice, role-play, pronunciation |
| Notifications | Daily loop, review due |
| Biometrics | Auth, high-stakes gate |
| GPS | Optional location for physical skills |
| Offline | Learning packs |
| Haptics | Feedback (subtle) |

Permissions are requested just-in-time with clear explanations.

---

## 5. Performance

| Metric | Target |
|---|---|
| Cold start | <2 s |
| Screen transition | <300 ms |
| Offline start | <1 s |
| Memory | <250 MB typical |
| Battery | <5%/hour learning |
| App size | <80 MB initial |

### 5.1 Low-end devices
- 2 GB RAM support.
- Reduce animations.
- Compress media.
- Lazy-load sims.
- Text-first mode.

---

## 6. Notifications

- Max 1 learning notification/day.
- Quiet hours.
- User-controlled categories.
- No streak shaming.
- Deep link to the exact next action.
- Easy unsubscribe.

---

## 7. Evidence capture

- Guided camera (framing, lighting).
- Photo/video compression.
- Offline queue.
- Metadata and timestamp.
- Consent for recording.
- Secure upload.

---

## 8. Accessibility

- Screen reader (VoiceOver/TalkBack).
- Dynamic type.
- High contrast.
- Reduced motion.
- Captions.
- Switch control.
- Voice control.
- Haptic alternatives.

---

## 9. Security

- Secure storage for tokens.
- Biometric lock optional.
- Certificate pinning.
- Jailbreak/root detection (high-stakes only).
- Remote wipe on logout.
- No sensitive data in logs.

---

## 10. App store strategy

- Free core learning.
- No pay-to-win.
- Clear privacy labels.
- Age rating by region.
- In-app purchases only for optional premium features.
- No external payment for digital goods where prohibited.

---

## 11. Requirements

- `FR-PLT-201`: Mobile supports offline levels 1–10.
- `FR-PLT-202`: Mobile supports camera/mic evidence.
- `FR-PLT-203`: Mobile syncs reliably after offline.
- `FR-PLT-204`: Mobile is accessible.
- `FR-PLT-205`: Notifications are capped.
- `FR-PLT-206`: Mobile works on 2 GB RAM devices.
- `FR-PLT-207`: Mobile supports biometric auth.
- `FR-PLT-208`: Mobile supports text-first mode.
- `FR-PLT-209`: Mobile respects quiet hours.
- `FR-PLT-210`: Mobile app size <80 MB.
- `NFR-PLT-201`: Cold start <2 s.
- `NFR-PLT-202`: Sync success ≥99%.
- `NFR-PLT-203`: Battery <5%/hour.
