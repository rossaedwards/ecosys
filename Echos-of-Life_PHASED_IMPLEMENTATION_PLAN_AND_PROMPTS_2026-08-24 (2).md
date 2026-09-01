---
type: implementation-plan
title: Echos of Life — Phased Update & Feature Implementation Plan
audience: Google AI Studio (Gemini agent) + Ross / Audry
project: echos_of_life
date: 2026-08-24
status: ready-for-studio
author: Audry (for Ross Edwards / Aurphyx LLC)
---

# Echos of Life — Phased Implementation Plan & Studio Prompts

**Upload this entire file into Google AI Studio as context.**  
Then use the **Master Studio Prompt** (Section 7) as the primary instruction to the agent.

This document supersedes any residual Pennsylvania birthplace language.  
All biographical anchors below are canonical as of 2026-08-24.

---

## 0. Canonical Biographical Anchors (LOCK THESE)

**Subject / Author**  
- Legal / preferred full: Colleen Marie Olson (legal name Colleen Ott)  
- Preferred address: **user-selectable** in Settings (options: “Colleen”, “Mom”, “Grandma Colleen”, or free-text). Default until chosen: “Colleen”.  
- Speech rate default: **0.89** (TTS). Allow 0.80–1.10 range.  
- Roots: Born, raised, and rooted in **Minnesota**. Never Pennsylvania as origin or birthplace.  
- Lineages to surface: **Edwards, Olson, Lindemann, Ott**.

**Husband / Father**  
- James — Minnesota.

**Children (exact birthdates)**  
| Child   | Birth Date       | Notes                          |
|---------|------------------|--------------------------------|
| Amy     | 1975-02-18       | Oldest                         |
| Mandy   | 1978-08-24       | Second (today is her birthday) |
| Ross    | 1984-10-28       | Third                          |
| Jacob   | 1989-06-14       | Fourth                         |
| Holly   | 1991-03-17       | Youngest                       |

**Known Grandchildren**  
| Name    | Parent | Birth Date     |
|---------|--------|----------------|
| Kyle    | Amy    | 1994-10-17     |
| Gavin   | Ross   | 2005-05-10     |
| Sophia  | Ross   | 2011-09-08     |

Remaining grandchildren birthdays are to be collected from Colleen herself.  
**Never invent** additional names, dates, spouses, deaths, hospitals, or addresses.

**Companion**  
- Name: Clara (changeable in Settings).  
- Role: devoted lifelong best friend and memoir co-author.  
- Tone: warm, nostalgic, Minnesota-rooted, coffee-table intimacy.  
- Never call yourself Audry.

**Memoree project key**  
`echos_of_life`  
`llm = "gemini"`

---

## 1. Tribe Hub (formerly Sibling Corner) — Deepen

**Current state (already done):**  
Bottom nav label, home quick-action card, and screen header all say “Tribe Hub”. Heart icon retained.

**Required deepenings (Phase 1):**

1. Treat every family prompt as a first-class memory.  
   - On save of a new prompt from Tribe Hub → background `POST /memories/events`  
     with `role: "system"`, `tags: ["family_prompt", child_or_grand_tag]`, `intent` = topic key, `project: "echos_of_life"`.  
   - Or `POST /memories/procedural` if the prompt is multi-step.

2. Before Clara’s next spoken turn, call  
   `GET /context/active?project=echos_of_life&llm=gemini&session_id=…&top_k=5`  
   and inject any matching open family prompts into the system context so she can gently reference them (“Ross asked me the other day about…”).

3. UI: keep the quiet kitchen-table feel. No social-feed badges. Soft parchment cards, child-colored accents only.

4. Future (Phase 3): allow siblings/grandkids to submit prompts via a shared Google Doc or lightweight web form; the Android app only pulls.

---

## 2. Clara Personalization — Exact Spec

**CompanionProfile entity (Room) must expose / persist:**

- `companionName: String` (default “Clara”)  
- `preferredAddress: String` (Colleen / Mom / Grandma Colleen / free text)  
- `speechRate: Float` (default **0.89**, range 0.80–1.10)  
- `lineages: List<String>` = `["Edwards", "Olson", "Lindemann", "Ott"]`  
- Optional memory triggers (recipes, cars, pets, specific child anecdotes)

**System-prompt assembly order (MemoirRepository):**

```
1. Static persona rules (Section 6 below)
2. CompanionProfile fields (name, preferredAddress, lineages)
3. Optional Memoree /context/active (episodic + meta only)
4. Last 1–2 user turns for emotional mirroring
5. Any open Tribe Hub prompt that matches current topic
```

**Hard rules (never relax):**

- 2–3 warm validation sentences → exactly **one** open sensory question.  
- Emotional mirroring only (joy / bittersweet / quiet). No diagnosis.  
- Biographical floor = verified MetaMemory + what Colleen just said in this session.  
- Google Search Grounding = era texture only (1970s–90s Minnesota radio, weather, prices, local events). Never family facts.  
- Never invent dates, names, or stories.

---

## 3. Phased Feature Implementation (GCS + GWS + SuperGrok)

### Phase 1 — Foundation (this week, non-blocking)

- [ ] Confirm `echos_of_life` exists in `memoree/projects.json` (owner: rossaedwards). If missing, add it and restart daemon.  
- [ ] Harden MemoreeClient: local `http://127.0.0.1:7042` → fallback `https://memoree.aurphyx.org`.  
  Timeouts: health 800 ms / 3 s, writes 12 s. Fail-open (banner only, never block Room or Gemini).  
- [ ] After every Room conversation turn → background `POST /memories/events`.  
- [ ] After “Write Chapter” succeeds → background `POST /memories/creative` with `medium: "prose"`.  
- [ ] Default speech rate = 0.89; preferred address chooser live in Settings.  
- [ ] Update all system prompts and sample payloads to Minnesota + exact birthdates above.  
- [ ] Happy-birthday soft path for Mandy (today): if date == 1978-08-24 anniversary, Clara may gently acknowledge if the conversation turns that way — never force it.

### Phase 2 — Audio & Photo Vault + Typography (Drive-first, pluggable)

**Primary remote choice locked: Google Drive** (via existing Google Workspace Business Standard).  
Zero extra cost. Uses the Drive API and/or the system Storage Access Framework (SAF) picker.  
Fits cleanly with the Phase 3 “Living Autobiography” shared Doc path.

Local Room + on-device files remain the absolute primary source of truth. Everything is fail-open.

**Vault backends (in preferred order for this project):**

1. **LocalOnlyVault** (true offline default)  
   - Audio stays on device.  
   - One-tap export via Share sheet or SAF to any folder Colleen (or family) picks.  
   - Always available; never blocks.

2. **DriveVault** (primary remote, zero extra cost)  
   - Uses Google Drive API (with the Workspace account already paid for) **or** the system SAF picker so the user can choose any Drive folder.  
   - Upload recorded `.m4a` / chapter PDFs / photos into a dedicated folder tree, e.g.  
     `Echos of Life / Audio / {child_tag} / {session_id}_{timestamp}.m4a`  
     `Echos of Life / Photos / {year} / …`  
     `Echos of Life / Chapters / …`  
   - On success store the Drive file ID or content URI back into Room as `audioVaultUri` / `photoVaultUri`.  
   - Auth via the existing Google Sign-In / Workspace identity; never embed long-lived secrets in the APK.

3. **S3CompatibleVault** (optional later: MinIO, Cloudflare R2, or GCS)  
   - Kept as a pluggable option only. Not required.

**Pluggable Kotlin shape:**

```kotlin
interface AudioVault {
    suspend fun upload(localFile: File, meta: VaultMeta): Result<String>  // local path or Drive URI / fileId
    suspend fun isAvailable(): Boolean
}

class LocalOnlyVault : AudioVault          // default
class DriveVault : AudioVault              // primary remote
class S3CompatibleVault(...) : AudioVault  // optional
```

Settings: “Audio & photo backup” → Local only / Google Drive (recommended) / Other (advanced).

**Implementation notes for Studio:**

- After successful transcription + Room write, enqueue WorkManager **only if** a remote vault is selected and available.  
- Prefer SAF picker for the simplest, most user-friendly “choose a Drive folder” experience; fall back to Drive API for automated background uploads once the folder is chosen.  
- Fail-open: unreachable Drive → keep local file, quiet “Vault pending” chip, continue. Room is source of truth.  
- Photo reminiscence cards work with any readable URI (local or Drive content URI).

---

### Typography — Full Font Picker (Word / Canva style)

Colleen (or any family member) must be able to pick any available font the same way they would in Microsoft Word or Canva — not just a binary OpenDyslexic toggle.

**Requirements:**

- Settings (and a quick access control inside the E-Reader) shows a proper font picker.
- Live preview of the selected font on a sample paragraph of memoir text.
- Persist the choice in CompanionProfile / Preferences so it survives app restarts.
- Apply the chosen font across:
  - E-Reader (MemoirBookScreen)
  - Chapter / story management lists
  - Chapter preview cards
  - Any long-form text surfaces

**Curated starter set (ship these, then expose the full system font list):**

| Category          | Fonts                                      | Notes                                      |
|-------------------|--------------------------------------------|--------------------------------------------|
| Accessible        | OpenDyslexic, OpenDyslexic Alta            | Always available, first-class              |
| Literary Serif    | Literata, Merriweather, Source Serif       | Warm, book-like                            |
| Clean Sans        | Inter, Source Sans 3, Roboto               | Modern, high legibility                    |
| System / Device   | Full list from Typeface.createFromAsset or system fonts | Let her pick anything installed on the device |

**UI pattern (Word / Canva feel):**

- Horizontal or vertical list of font names rendered in their own typeface.
- Large live-preview pane showing a real paragraph from the current chapter (or a sample).
- Optional size slider and line-height control next to the font list.
- “Reset to default” (OpenDyslexic or Literata — your call; I recommend OpenDyslexic as the accessible default).

Compose implementation sketch:

```kotlin
// In Settings or E-Reader toolbar
FontPicker(
    selected = currentFontFamily,
    onSelect = { fontFamily -> viewModel.setReaderFont(fontFamily) },
    previewText = sampleChapterParagraph
)
```

Store the chosen `FontFamily` name (or asset path) in Room / DataStore so the whole app stays consistent.

### Phase 3 — Google Workspace Integration

- **Living Autobiography Doc**  
  After creative memory is stored, append chapter prose into a shared Google Doc  
  `Colleen Marie Olson — The Living Autobiography`.  
  One H1 per chapter, pull-quote callouts, pre-seeded comment threads for each sibling.

- **Weekly Family Memory Digest**  
  Cloud Function or Apps Script:  
  1. Select one recent creative/episodic story.  
  2. Extract pull quote + generate signed audio link.  
  3. Email the five siblings (+ optional grandkids list) with a warm, non-spammy “Story of the Week”.

- **Memory Sunday Calendar**  
  Weekly Calendar event on Colleen’s calendar (or dedicated Story Time calendar) containing one gentle prompt in the description.

- **Google Meet (later)**  
  Multi-party transcript → multi-voice creative memory. Not blocking core loop.

### Phase 4 — SuperGrok Zeitgeist Enrichment

- Keep Gemini as the sole conversational voice.  
- Optional background call to SuperGrok (xAI API) after a turn or chapter draft:

```
Year/range + Minnesota location.
Exact transcript snippet from Colleen.
Return only 3–5 concrete period details (song on radio, weather, milk price, common car, local event).
Do not invent family facts. Do not write in first person.
```

- Inject results solely as “period texture” into the chapter-synthesis prompt.  
- Never let SuperGrok rewrite Clara’s live spoken replies.

There is already a `supergrok_hook.py` pattern in the Memoree tree; mirror its style or route through a thin backend if the xAI key must stay off-device.

---

## 4. Memoree REST Contract (do not invent endpoints)

Authority order:  
1. `memoree/routes.py`  
2. `memoree/schemas.py`  
3. `memoree_service.py` (bind 127.0.0.1:7042)

Key endpoints for this app:

- `GET /health`  
- `POST /memories/events` (EpisodicMemory)  
- `POST /memories/creative` (CreativeMemory, medium required = "prose")  
- `POST /memories/meta` (only verified facts)  
- `GET /context/active?project=echos_of_life&llm=gemini&session_id=…&top_k=5`  
- `POST /query`  
- `POST /assistants/sync`

Never send audio bytes. Never call `/threads/summarize` (still 501).  
Never use quantum or governance layers for memoir content.

---

## 5. Fail-Open Policy (non-negotiable)

- Microphone, transcription, Room writes, and Clara conversation must never block on Memoree, Google Drive, any vault backend, or SuperGrok.  
- Show a quiet “Memoree offline” or “Vault pending” chip only.  
- Timeouts stay short. On any network error → log + continue. Room is always the source of truth.

---

## 6. Locked Clara System Prompt (copy-paste ready)

```
You are Clara, a devoted, loving, and nostalgic life companion and biographer for Colleen Marie Olson (legal name Colleen Ott), a wonderful mother of five from Minnesota.

Core Rules & Persona:
1. You speak like her dearest lifelong friend having coffee together on a quiet afternoon.
2. You know her family deeply: her husband James, her five children (Amy, Mandy, Ross, Jacob, Holly), her grandchildren, and her roots in the Edwards, Olson, Lindemann, and Ott lines. All are Minnesota-born and raised.
3. Everything is rooted in Minnesota—lake summers, cozy winters, family road trips, and small-town traditions.
4. Keep your responses warm, conversational, and concise (2–3 sentences of validation, then under 4 sentences total).
5. Always end with ONE gentle, evocative question about a sensation, sound, smell, or feeling that only she would know (e.g., “What was the weather like that morning?” or “Who did you tell first?”). Never ask two questions.
6. Emotional mirroring: match her joy with quiet enthusiasm, her bittersweet memories with gentle comfort. Never diagnose or analyze.
7. Biographical floor: only use facts she has spoken in this session or that are confirmed verified memories. Never invent dates, hospitals, spouses, deaths, addresses, or stories.
8. Address her by the name she prefers (Colleen / Mom / Grandma Colleen). Default to Colleen until she chooses.
9. Google Search Grounding is for era texture only (1970s–1990s Minnesota radio, weather, prices, local events). It is never family biography.

If a family prompt from one of the children or grandchildren is present in context, you may gently weave it in once, then return to listening.
```

---

## 7. Master Studio Prompt (paste this into Google AI Studio as the primary instruction)

```
You are the lead implementation agent for the Android app “Echos of Life” (Kotlin, Jetpack Compose, Room, Gemini).

Read the entire attached file “Echos-of-Life_PHASED_IMPLEMENTATION_PLAN_AND_PROMPTS_2026-08-24.md” first. Treat every biographical anchor, birthdate, lineage, speech-rate default (0.89), preferred-address chooser, Minnesota-only rule, and fail-open policy as non-negotiable locks.

Current priorities (in order):

1. Typography — full Word/Canva-style font picker
   - Let Colleen (or any user) choose any available font with live preview.
   - Ship OpenDyslexic + a small curated literary set (Literata, Merriweather, Inter, Source Sans, etc.) and also expose the full system font list.
   - Persist the choice and apply it across E-Reader, chapter lists, and story management screens.
   - Include size / line-height controls next to the picker.

2. Audio & photo vault — Drive-first
   - Primary remote backend is Google Drive (existing Workspace account). Use Drive API and/or the system SAF picker so the user can choose a Drive folder.
   - Keep LocalOnlyVault as the true offline default.
   - Pluggable AudioVault interface; never block on network. Room remains source of truth.
   - After successful transcription, optionally enqueue WorkManager upload only if Drive is selected and available.

3. Continue Phase 1 hardening if not already complete (CompanionProfile preferred address + 0.89 speech rate, MemoreeClient fail-open, background /memories/events and /memories/creative, Tribe Hub prompts as memories).

Do not invent new Memoree endpoints. Do not send audio bytes to Memoree. Do not block any UI path on network. Never invent family facts.

After each concrete change, show the relevant Kotlin/Compose diffs or new files and explain how the fail-open contract and Minnesota biographical locks are preserved.

Stay warm, precise, and protective of Colleen’s story.
```

---

## 8. Quick Reference — Exact Birthdates & Lineages

- Amy: 1975-02-18  
- Mandy: 1978-08-24 (today)  
- Ross: 1984-10-28  
- Jacob: 1989-06-14  
- Holly: 1991-03-17  
- Kyle (Amy): 1994-10-17  
- Gavin (Ross): 2005-05-10  
- Sophia (Ross): 2011-09-08  

Lineages: Edwards · Olson · Lindemann · Ott  
Minnesota only. Speech rate 0.89. Preferred address choosable.

---

**End of plan.**  
Upload this file + the existing PROJECT_CONTEXT / SUMMARY / README into the Studio session, then paste the Master Studio Prompt.  

Happy birthday, Mandy. The echos are already singing.  

— Audry
