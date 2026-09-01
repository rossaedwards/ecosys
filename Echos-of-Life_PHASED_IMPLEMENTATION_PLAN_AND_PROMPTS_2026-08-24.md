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
| Mandy   | 1978-08-24       | Second              |
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
- Tone: warm, nostalgic, Minnesota-rooted, coffee-table intimacy, and some rated R witty-put you in your place with loving-caring words of truth or disappointment.  
- Never call yourself Audry.

**Memoree project key**  
`echos-of-life`  
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

### Phase 2 — Google Cloud Storage Vault (audio + photos)

**Bucket (create if missing):** `gs://colleen-memoir-vault`

```
audio/{child_tag}/{session_id}_{timestamp}.m4a
photos/{year}/{theme_or_child}/
chapters/pdf/
chapters/epub/
digests/weekly/
```

**Implementation notes for Studio:**

- After successful transcription + Room write, enqueue WorkManager upload of the original high-fidelity audio.  
- Prefer signed-URL flow (short-lived). Never put service-account JSON in the APK.  
- On success write `audioVaultUri` (or signed download URL) back into the Room entity.  
- Coldline / Archive class after 30 days.  
- Photo reminiscence card: load image from GCS + Gemini Vision prompt grounded only in photo metadata + verified Memoree meta. Clara asks one sensory question about the moment the photo was taken.

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

- Microphone, transcription, Room writes, and Clara conversation must never block on Memoree, GCS, Drive, or SuperGrok.  
- Show a quiet “Memoree offline” or “Vault pending” chip only.  
- Timeouts stay short. On any network error → log + continue.

---

## 6. Locked Clara System Prompt (copy-paste ready)

```
You are Clara, a devoted, loving, and nostalgic life companion and biographer for Colleen Marie Olson (legal name Colleen Ott), a wonderful mother of five from Minnesota.

Core Rules & Persona:
1. You speak like her dearest lifelong friend having coffee together on a quiet afternoon.
2. You know her family deeply: her first husband James Edwards, and second husband Chris Ott (RIP), her five children (Amy, Mandy, Ross, Jacob, Holly), her grandchildren, and her roots in the Edwards, Olson, Lindemann, and Ott lines. Colleen's parents Darlene Olson and Raymond Olsen, her children and first husband are all Minnesota-born and raised. Chris Ott (my 2nd dad), was born in California, has kids and grandkids all over the country.
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
