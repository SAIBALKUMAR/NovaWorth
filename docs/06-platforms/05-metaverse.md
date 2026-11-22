# Metaverse / WebXR (Optional)

## 1. Purpose

The metaverse layer is **optional**. It exists for shared spatial experiences that add real learning value: group simulations, spatial critique, remote mentoring, and public showcases. It is not a separate product and not required for any credential.

**Decision:** build only after Phase 3, gated by evidence that spatial improves outcomes and retention.

---

## 2. Use cases

| Use case | Value | Alternative |
|---|---|---|
| Group simulation | Shared incident command, kitchen, clinic | 2D multiplayer sim |
| Spatial critique | Walk around a 3D design together | Video review |
| Remote mentoring | Presence and pointing | SharePlay/2D call |
| Public showcase | Exhibition of learner work | Web gallery |
| Guild spaces | Social presence | Guild chat/forum |

---

## 3. Technology options

| Option | Pros | Cons |
|---|---|---|
| **WebXR** | Broad reach, no install | Performance limits, device support |
| **Unity WebGL** | Rich 3D, tooling | Heavy downloads |
| **Native visionOS** | Best performance | Apple-only |
| **Desktop VR (SteamVR)** | Immersive | Hardware cost |
| **2D multiplayer** | Universal | Less presence |

**Recommendation:** WebXR for shared spaces; native visionOS for premium spatial; 2D multiplayer as the default.

---

## 4. Architecture

```
WebXR client ──┐
Unity client ──┼──► Realtime gateway ──► Simulation service
visionOS ──────┘         │
                         ├── voice/text
                         ├── state sync
                         └── moderation
```

- Authoritative server for state.
- 20 Hz tick for shared sims.
- Voice with spatial audio.
- Text fallback.
- Recording with consent.

---

## 5. Safety and moderation

- Age gates.
- Guardian settings for minors.
- Proximity and personal space rules.
- Mute/block/report.
- Human moderation for public spaces.
- No private spaces for minors.
- Content ratings.
- Session time limits.

---

## 6. Interoperability

- Open avatar format (glTF/VRM).
- Portable identity (display name, avatar).
- No lock-in to one world.
- Export learner artifacts.
- Standard event schema.

---

## 7. Cost and performance

- Hosting: per-concurrent-user cost.
- Target: <$0.10/user/hour.
- 20 concurrent users per instance.
- Graceful degradation to 2D.
- Caching and instancing.

---

## 8. Go/no-go criteria

Proceed only if:
- Phase 3 data shows spatial improves outcomes by ≥15% for target skills.
- Cost per learner is within budget.
- Safety incidents are manageable.
- At least 3 partners commit to shared spaces.
- 2D parity is maintained.

---

## 9. Requirements

- `FR-PLT-401`: Metaverse is optional for all credentials.
- `FR-PLT-402`: A 2D equivalent exists.
- `FR-PLT-403`: Moderation and reporting are available.
- `FR-PLT-404`: Minors have restricted access.
- `FR-PLT-405`: Avatars are portable.
- `FR-PLT-406`: Voice/text fallback exists.
- `FR-PLT-407`: Sessions are recorded only with consent.
- `FR-PLT-408`: Cost per user is tracked.
- `FR-PLT-409`: No private spaces for minors.
- `FR-PLT-410`: Go/no-go is evidence-based.
- `NFR-PLT-401`: 20 Hz state sync.
- `NFR-PLT-402`: <150 ms voice latency.
- `NFR-PLT-403`: 99.5% uptime.
