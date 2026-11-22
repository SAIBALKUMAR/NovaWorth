# Accessibility & Offline / Low Bandwidth

## 1. Purpose

Access is a product requirement, not a compliance checkbox. Many learners will use low-end devices, slow networks, screen readers, or alternative input. If the product excludes them, the economy excludes them.

**Targets:** WCAG 2.2 AA, full keyboard/screen-reader flows, captions, offline levels 1–10, and a text-first mode.

---

## 2. Accessibility standards

| Standard | Requirement |
|---|---|
| WCAG 2.2 | AA |
| ADA | US compliance |
| European Accessibility Act | EU compliance |
| EN 301 549 | Public sector |
| ARIA Authoring Practices | Widgets |
| Platform guidelines | iOS/Android/visionOS |

---

## 3. Disability coverage

| Need | Support |
|---|---|
| Blind/low vision | Screen reader, braille, audio, high contrast, text resize |
| Deaf/hard of hearing | Captions, transcripts, visual alerts, sign language option |
| Motor | Keyboard, switch, voice, alternative input, extended time |
| Cognitive | Plain language, chunking, reminders, reduced distraction |
| Dyslexia | Font options, spacing, text-to-speech |
| Photosensitive | Reduced motion, no flashing |
| Color vision | No color-only meaning |
| Speech | Text input, AAC support |
| Temporary injury | Alternative input, extensions |

---

## 4. Content accessibility

- Alt text for every meaningful image.
- Captions 99% accuracy.
- Transcripts searchable.
- Audio descriptions for visual content.
- Plain-language summaries.
- Consistent heading structure.
- Tables with headers.
- No image-only assessments.
- Accessible math (MathML).
- Accessible code blocks.

---

## 5. Interaction accessibility

- Keyboard-only completion of every flow.
- Visible focus.
- No drag-only.
- No hover-only.
- No time-only.
- No gesture-only.
- Screen-reader labels for all controls.
- Error messages linked to fields.
- Skip links.
- Reduced motion setting.

---

## 6. Assessment accommodations

- Extended time (1.5×/2×).
- Separate room.
- Reader/scribe.
- Oral instead of written.
- Screen reader/braille.
- Captions.
- Alternative input.
- Breaks.
- Custom color/contrast.

Accommodations are recorded and do not lower integrity requirements or scoring standards.

---

## 7. Offline / low bandwidth

### 7.1 Modes
| Mode | Data | Features |
|---|---|---|
| Full | Wi-Fi/4G | All |
| Lite | 3G | Text, compressed images, no autoplay |
| Offline | None | Downloaded levels 1–10, review, queued attempts |
| Audio-only | Low | Narrated concepts, oral practice |

### 7.2 Offline packs
- Text, images, audio.
- Practice items.
- 1 low-stakes assessment variant.
- Review queue.
- Glossary.
- Cap: 200 MB/skill.

### 7.3 Sync
- Queue attempts/artifacts.
- Background sync.
- Conflict resolution.
- Clear status.
- No data loss.

---

## 8. Device matrix

| Device | Target |
|---|---|
| Low-end Android (2 GB RAM) | Core learning |
| Mid-range Android | Full |
| iPhone SE | Full |
| iPhone 15+ | Full |
| iPad | Full |
| Desktop Chrome/Firefox/Safari | Full |
| Vision Pro | Spatial + 2D |
| Screen readers | NVDA, JAWS, VoiceOver, TalkBack |
| Switch control | Full |
| Braille display | Full |

---

## 9. Testing

- Automated axe/Lighthouse checks.
- Manual screen-reader passes.
- Keyboard-only passes.
- User testing with disabled learners.
- Low-bandwidth network tests.
- Device lab.
- Regression suite per release.

---

## 10. Requirements

- `FR-PLT-501`: WCAG 2.2 AA compliance.
- `FR-PLT-502`: Every flow is keyboard-completable.
- `FR-PLT-503`: Every video has captions/transcript.
- `FR-PLT-504`: Every image has alt text.
- `FR-PLT-505`: Text resize to 200% without loss.
- `FR-PLT-506`: Reduced motion is respected.
- `FR-PLT-507`: Levels 1–10 work offline.
- `FR-PLT-508`: Lite mode works on 3G.
- `FR-PLT-509`: Accommodations are available for all assessments.
- `FR-PLT-510`: Disabled learners are included in testing.
- `NFR-PLT-501`: No blocking accessibility defects at release.
- `NFR-PLT-502`: Offline sync success ≥99%.
- `NFR-PLT-503`: Lite mode page weight <300 KB.
