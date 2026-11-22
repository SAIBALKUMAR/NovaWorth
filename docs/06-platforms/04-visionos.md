# Apple Vision Pro (visionOS)

## 1. Purpose

Vision Pro is for skills where **space, presence, and embodiment** matter: physical craft, spatial design, interpersonal role-play, simulation, and immersive study. It is an enhancement, never a requirement.

**Stack:** SwiftUI + RealityKit + ARKit, native visionOS; shared content API; 2D fallback.

---

## 2. Principles

1. **No skill is gated by hardware.** Every Vision Pro experience has a 2D equivalent.
2. **Comfort first.** Short sessions, stable anchors, no forced motion.
3. **Presence, not novelty.** Use space only when it improves learning.
4. **Accessibility from day one.** Voice, gaze, hand, and switch alternatives.
5. **Share the moment.** SharePlay for role-play and teaching.

---

## 3. Use cases

| Use case | Example | Value |
|---|---|---|
| Spatial craft | Knife skills, plumbing, wiring, woodworking | 3D placement and tool tracking |
| Design review | 3D models, architecture, UX | Walk around the artifact |
| Role-play | Difficult conversations, sales, care | Presence and gaze |
| Simulation | Incident command, kitchen, clinic | Spatial layout and pressure |
| Study space | Focus room, memory palace | Environment control |
| Teaching | Mentor observes and annotates | Shared spatial context |

---

## 4. Experience patterns

### 4.1 Immersive lesson
- A 3D scene with a concept model.
- Learner manipulates objects.
- Tutor narrates and questions.
- Checkpoints in-world.
- Exit to 2D notes.

### 4.2 Spatial simulation
- A room-scale scenario.
- Characters, tools, and hazards.
- Hand/eye tracking for actions.
- Real-time state and consequences.
- After-action review in 2D.

### 4.3 Mentor SharePlay
- Learner and mentor share a space.
- Mentor points, annotates, demonstrates.
- Session recorded (with consent).
- Feedback attached to artifacts.

---

## 5. Interaction

| Input | Use |
|---|---|
| Eyes | Target selection |
| Hands | Manipulation, gestures |
| Voice | Commands, dialogue |
| Keyboard | Text entry |
| Gamepad | Optional sims |

### 5.1 Comfort
- Keep content within a comfortable field.
- Avoid rapid camera motion.
- Use stable anchors.
- Allow seated mode.
- Respect system accessibility settings.

---

## 6. Content pipeline

- 3D assets authored in Blender/USDZ.
- Optimization for visionOS (poly count, textures).
- Spatial audio.
- Captions in-world and 2D.
- Interaction scripts.
- Performance budget: 90 fps target, 72 fps minimum.

---

## 7. 2D fallback

Every spatial experience has:
- A video walkthrough.
- A 2D interactive equivalent.
- The same assessment rubric.
- No advantage for Vision Pro users in scoring.

---

## 8. Accessibility

- VoiceOver support.
- Switch control.
- Head/gaze alternatives.
- Subtitles and transcripts.
- Reduced motion.
- Seated mode.
- High contrast.
- No hand-only requirement.

---

## 9. Privacy and safety

- Camera/mic only with consent.
- No recording without explicit action.
- Spatial data minimized.
- Safe play space warnings.
- Session time limits and break nudges.
- No immersive content for minors without guardian settings.

---

## 10. Technical

| Component | Approach |
|---|---|
| UI | SwiftUI windows + volumes |
| 3D | RealityKit |
| Tracking | ARKit hand/eye |
| Audio | Spatial audio |
| Networking | Shared API + WebSocket |
| Auth | Sign in with Apple + passkey |
| Offline | Cached 2D content; limited 3D |
| Analytics | Privacy-preserving events |

---

## 11. Requirements

- `FR-PLT-301`: Every Vision Pro experience has a 2D fallback.
- `FR-PLT-302`: No scoring advantage for spatial users.
- `FR-PLT-303`: Sessions support seated mode.
- `FR-PLT-304`: Captions/transcripts are available.
- `FR-PLT-305`: SharePlay supports mentor sessions.
- `FR-PLT-306`: Recording requires explicit consent.
- `FR-PLT-307`: Minors have guardian settings.
- `FR-PLT-308`: Content meets performance budgets.
- `FR-PLT-309`: Accessibility settings are respected.
- `FR-PLT-310`: Spatial data is minimized.
- `NFR-PLT-301`: 90 fps target; 72 fps minimum.
- `NFR-PLT-302`: Launch <3 s.
- `NFR-PLT-303`: No motion sickness in 95% of testers.
