# Text & Video Learning

## 1. Purpose

Text and video are the workhorses of the product. They must be clear, accessible, cheap to serve, and honest about AI involvement.

**Rule:** no level may require video. Text is always available.

---

## 2. Text learning

### 2.1 Structure
Every concept page has:
1. **Outcome** — what you will be able to do.
2. **Why it matters** — a real consequence.
3. **Explanation** — 150–400 words, plain language.
4. **Example** — concrete, worked.
5. **Counterexample** — a common mistake.
6. **Check** — 1–2 retrieval questions.
7. **Next** — practice or related concept.

### 2.2 Reading levels
| Band | Target reading level | Notes |
|---|---|---|
| 1–10 | Grade 6–8 | Short sentences, defined terms |
| 11–30 | Grade 8–10 | Technical terms introduced |
| 31–60 | Grade 10–12 | Dense but structured |
| 61–100 | Expert | Assumes domain fluency |

### 2.3 Formatting
- Short paragraphs (≤4 sentences).
- Descriptive headings.
- Bulleted steps for procedures.
- Callouts for safety, misconceptions, and pro tips.
- Glossary links.
- No walls of text.

### 2.4 Accessibility
- Semantic HTML.
- Alt text for images.
- Captions/transcripts for video.
- Adjustable text size and contrast.
- Screen-reader-tested.
- Plain-language toggle.
- Translation.

---

## 3. Interactive video

### 3.1 Structure
- 3–8 minute chapters.
- A checkpoint every 2–4 minutes.
- Transcript synced to playback.
- Chapters and search.
- Speed control (0.5×–2×).
- Captions on by default.

### 3.2 Checkpoints
- 1 retrieval question per chapter.
- Immediate feedback.
- Cannot skip without answering (or "I don't know").
- Checkpoints count toward practice, not the gate.

### 3.3 Video types
| Type | Use | Review |
|---|---|---|
| Expert talking head | Context, nuance | SME |
| Demonstration | Physical/procedural | SME + safety |
| Screen recording | Software | SME |
| Animation | Abstract concepts | SME + ID |
| AI-generated | Scale, variation | Human review + label |

### 3.4 AI-generated video policy
- Labeled "AI-generated" in the player.
- Human expert reviews script and final cut.
- No synthetic person presented as a real expert.
- No deepfakes of real people without consent.
- Provenance metadata attached.

---

## 4. Media pipeline

```
Script → Storyboard → Record/Generate → Edit → Captions
→ Transcript → Accessibility check → SME review → Publish
```

### 4.1 Technical specs
| Spec | Target |
|---|---|
| Resolution | 1080p primary, 720p fallback |
| Audio | Mono/stereo, normalized |
| Captions | WebVTT, 99% accuracy |
| Transcript | Searchable, timestamped |
| Duration | 3–8 min/chapter |
| File size | <100 MB/chapter |
| Streaming | HLS/DASH, adaptive |

### 4.2 Low bandwidth
- 480p and audio-only options.
- Downloadable packs.
- Transcript-first mode.
- No autoplay.

---

## 5. Provenance

Every asset has:
- Author, reviewer, date.
- AI involvement (none, assisted, generated).
- Model/version if AI.
- Source pack.
- Version.
- Review status.

Learners can tap "How was this made?" to see provenance.

---

## 6. Quality standards

| Criterion | Standard |
|---|---|
| Accuracy | SME sign-off |
| Clarity | Learner clarity ≥4/5 |
| Pace | No more than 150 wpm narration |
| Examples | At least 1 concrete example |
| Counterexamples | At least 1 |
| Accessibility | WCAG 2.2 AA |
| Captions | 99% accuracy |
| Length | Within band target |
| Freshness | Reviewed within 18 months |

---

## 7. Anti-passivity

Watching is not learning. Every video must have:
- A checkpoint.
- A practice task.
- A reflection prompt.
- A transcript for retrieval.

No XP for watching alone. XP requires practice or assessment.

---

## 8. Personalization

- Reading level adjustment.
- Example substitution (with consent).
- Language.
- Playback speed.
- Captions/transcript default.
- Audio description.

---

## 9. Requirements

- `FR-LRN-201`: Every level has a text path.
- `FR-LRN-202`: Every video has captions and a transcript.
- `FR-LRN-203`: Every video has checkpoints.
- `FR-LRN-204`: AI-generated media is labeled.
- `FR-LRN-205`: Every asset has provenance.
- `FR-LRN-206`: Text meets reading-level targets.
- `FR-LRN-207`: Media works at 480p and audio-only.
- `FR-LRN-208`: No autoplay.
- `FR-LRN-209`: Learners can download level packs.
- `FR-LRN-210`: No XP for passive watching.
- `NFR-LRN-201`: Video start <2 s on 4G.
- `NFR-LRN-202`: Transcript search <200 ms.
- `NFR-LRN-203`: Caption accuracy ≥99%.
