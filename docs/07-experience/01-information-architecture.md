# Information Architecture

## 1. Purpose

The IA must answer three questions on every screen:
1. **Where am I?**
2. **What can I do next?**
3. **Why does it matter?**

It must work for a first-time learner, a level-80 master, a sponsor, and an author — without collapsing into a maze.

---

## 2. Primary navigation (learner)

| Tab | Purpose | Default screen |
|---|---|---|
| **Today** | Next best action, review, momentum | Session card |
| **Skills** | Catalog, my skills, levels | My skills |
| **Learn** | Current level, practice, media | Current level |
| **Wallet** | XP, EC, awards, redeem | Balance + activity |
| **Profile** | Credentials, evidence, reputation | Skill wallet |

Secondary:
- Guild
- Marketplace
- Settings
- Help

---

## 3. Screen map

```
Today
├── Session card (next action)
├── Review queue
├── Momentum (3 sessions/week)
└── Quick links

Skills
├── My skills
│   ├── Skill detail
│   │   ├── Level ladder (1–100)
│   │   ├── Current level
│   │   ├── Evidence
│   │   └── Prestige
├── Catalog (500)
│   ├── Domain
│   ├── Skill preview
│   └── Fit check
└── Recommendations

Learn
├── Level overview
├── Concept
├── Practice
├── Applied task
├── Gate
└── Debrief

Wallet
├── Balance (XP, EC)
├── Activity (awards, redemptions)
├── Goals (savings)
├── Redeem
└── Statements

Profile
├── Skill wallet
├── Credentials
├── Evidence
├── Reputation
├── Public profile
└── Settings

Guild
├── My guild
├── Leagues
├── Mentors
├── Peer review
└── Events

Marketplace
├── Categories
├── Essentials
├── Search
├── Item detail
├── Orders
└── Disputes
```

---

## 4. Hierarchy rules

- Max 3 taps to any core action.
- The next action is always one tap from Today.
- The wallet is always reachable.
- The catalog is never more than two taps away.
- Settings never block learning.
- No modal traps.

---

## 5. Search

- Global search: skills, levels, concepts, marketplace, help.
- Autocomplete with typo tolerance.
- Filters: domain, level, modality, time, accessibility.
- Recent and suggested.
- No search results = helpful alternatives, not a dead end.

---

## 6. Deep links

| Link | Destination |
|---|---|
| `/skill/CMP-25` | Skill detail |
| `/skill/CMP-25/level/30` | Level overview |
| `/learn/CMP-25/30` | Current session |
| `/wallet` | Wallet |
| `/credential/:id` | Public verification |
| `/guild/:id` | Guild |
| `/market/item/:id` | Item detail |

All deep links work unauthenticated where public; otherwise redirect to login with return path.

---

## 7. Responsive behavior

| Breakpoint | Layout |
|---|---|
| Mobile | Bottom tabs, single column |
| Tablet | Two columns, side nav |
| Desktop | Three columns, persistent nav |
| Vision Pro | Spatial windows + 2D fallback |

No feature is desktop-only.

---

## 8. States

Every screen defines:
- Loading.
- Empty.
- Error.
- Offline.
- Partial data.
- Permission denied.
- Success.

Empty states must offer an action, not just a message.

---

## 9. Accessibility

- Landmarks and headings.
- Skip links.
- Focus order.
- Screen-reader labels.
- Keyboard shortcuts.
- Reduced motion.
- High contrast.
- Text resize.

---

## 10. Requirements

- `FR-UX-001`: Every screen has a clear next action.
- `FR-UX-002`: Core actions are ≤3 taps.
- `FR-UX-003`: The catalog is always reachable.
- `FR-UX-004`: Search covers skills, levels, marketplace, help.
- `FR-UX-005`: Deep links work and preserve intent.
- `FR-UX-006`: Every screen has loading/empty/error/offline states.
- `FR-UX-007`: Navigation is keyboard and screen-reader accessible.
- `FR-UX-008`: No feature is desktop-only.
- `FR-UX-009`: The wallet is always one tap away.
- `FR-UX-010`: Settings never block learning.
- `NFR-UX-001`: Screen transition <300 ms.
- `NFR-UX-002`: Search results <200 ms.
