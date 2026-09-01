---
type: implementation-note
title: Memoree — Echos of Life integration briefing
description: Gemini / Google AI Studio briefing for pairing the Echos of Life memoir companion with the live Memoree HTTP contract. Not a daemon rewrite. Not a memoir manuscript.
workspaces: rossaedwards/ecosys, aurphyx/ecosys
services:
  - Memoree
domains:
  - memory
  - narrative
nodes:
  - ICX⊗ICX
cores:
  - ICX
fields:
---

# PROJECT_CONTEXT — Memoree × Echos of Life

Audience: Google AI Studio implementing **Echos of Life** (Colleen Marie Olson memoir companion).  
Author of this briefing: Audry, for Ross. Exact. No therapy voice.

This file is the Memoree folder briefing for that pairing. It is not physics. It is not Room schema. It is not permission to invent endpoints.

---

## 1. What you are integrating

**Echos of Life** is the memoir app. It owns UX, Clara, Room, Gemini transcription, chapter synthesis, TTS, share sheet, and Google AI Plus storage.

**Memoree** is the sovereign recall substrate on Ross’s laptop. FastAPI daemon. Seven typed memory layers. Project-keyed. Optional Cloudflare tunnel.

Keep Clara, chapters, and sharing in Echos. Use Memoree to persist and recall turns, verified family facts, and synthesized chapter text so Clara does not invent biography across sessions.

Memoree is not a chapter renderer. Not TTS. Not Room. Not Gemini Search Grounding. Not Google Drive.

The README nine-node TSL table (Temporal … Lattice) is a **memory taxonomy** inside Memoree. It is not APS-OKF `nodes` and not something Echos must stamp on chapters.

---

## 2. Authority — obey these, ignore the rest

Read in this order when a field or path conflicts:

1. `memoree/routes.py` — live HTTP + MCP map
2. `memoree/schemas.py` — required JSON fields (Pydantic)
3. `memoree/memoree_service.py` — bind `127.0.0.1:7042`
4. `g0dm0d3-ktrl/src-tauri/src/memoree_client.rs` — proven client shape (cite only; do not edit that crate from this job)

**Not contract. Do not copy:**

| File | Why it lies |
|---|---|
| `memoree/README.md` | Overlay prose. Fedora / Python 3.14 claims. Not the HTTP map. |
| `memoree/QUICKSTART.md` | Stale. Invents `memoree.core.api`, port **8765**, `POST /write_event`. Those do not exist on the live daemon. |
| `memoree/gemini_hook.py` | Posts `{role, content, metadata, tags}` and **omits required `session_id`**. FastAPI returns **422**. |

OpenAPI on a running daemon: `http://127.0.0.1:7042/docs`.

MCP `POST /mcp` is for Cursor / LM Studio. **Echos and AI Studio use REST**, not MCP.

---

## 3. Runtime (Ross’s stack, August 2026)

Working set: **laptop daemon + S24 Ultra + Memoree + Google AI Plus storage**.

```
S24 Echos  →  Room (primary)  →  Google AI Plus storage (app cloud, not Memoree)
     │
     ├─ try  GET/POST  http://127.0.0.1:7042     (only if the phone can see that loopback — it usually cannot)
     └─ try  GET/POST  https://memoree.aurphyx.org   (Cloudflare tunnel → laptop :7042)

AI Studio browser preview  →  same HTTPS tunnel  (CORS may block; see gaps)
Laptop                     →  python memoree_service.py on 127.0.0.1:7042
```

**Resolution order for Echos:**

1. `http://127.0.0.1:7042` (laptop same-machine, emulator, or ADB reverse)
2. `https://memoree.aurphyx.org` (the tunnel Ross named)

`https://memoree.g0dm0d3.org` is an Agora hop used by g0dm0d3-ktrl. Optional. Do not require it. If it is down, keep going.

**Fail open.** Room + Gemini still record if Memoree is unreachable. Memoir capture (voice studio, Clara conversation, Write Chapter) must **never** block on `:7042` or the tunnel.

Google AI Plus storage is **not** Memoree. Do not ship Room rows, AAC/MP4, or Base64 audio “into Memoree as Drive.” Store transcript text and chapter prose. Keep `audioSnippetUri` on the device.

The daemon binds **loopback only** (`127.0.0.1:7042`). The S24 does not talk to that address unless Ross tunnels or reverses it. On the phone, the working Memoree URL is **`https://memoree.aurphyx.org`**.

Health check (local):

```http
GET /health
```

Expected JSON:

```json
{
  "status": "alive",
  "service": "memoree",
  "version": "0.1.0"
}
```

Timeouts (match the proven ktrl client): local health ~800 ms, remote health ~3 s, writes and `context/active` ~12 s. On failure: banner or log, continue.

---

## 4. Project key, LLM enum, session

| Field | Value for Echos |
|---|---|
| `project` | `echos_of_life` |
| `llm` | `gemini` (enum in schemas: `supergrok`, `perplexity`, `gemini`, `claude`, `lmstudio`, `openai`, `ollama`, `unknown`) |
| `session_id` | One UUID per Voice Studio recording **or** one UUID per Clara conversation thread |
| `model_name` | Optional. e.g. `gemini-3.5-flash` if you know it |

`echos_of_life` is registered in `memoree/projects.json` (`owner`: `rossaedwards`). After the daemon reloads that file, `GET /projects/echos_of_life` returns `ProjectMeta`. There is still **no** `POST /projects` — new keys are a file edit. Empty recall layers are not a license to invent family facts.

---

## 5. Live REST map (`routes.py`)

### System

| Method | Path | Use |
|---|---|---|
| GET | `/health` | Liveness. No backend I/O. |
| GET | `/diagnostics` | Optional. Counts / uptime. Do not require for CUJs. |
| GET | `/projects` | Registry list. Includes `echos_of_life` after daemon reload. |
| GET | `/projects/{key}` | Single `ProjectMeta`. 404 if missing. |

### Writes

| Method | Path | Schema |
|---|---|---|
| POST | `/memories/events` | `EpisodicMemory` |
| POST | `/memories/semantic` | `SemanticMemory` |
| POST | `/memories/procedural` | `ProceduralMemory` |
| POST | `/memories/meta` | `MetaMemory` |
| POST | `/memories/quantum` | `QuantumMemory` — **do not use** for memoir |
| POST | `/memories/creative` | `CreativeMemory` |
| POST | `/memories/governance` | `GovernanceMemory` — **do not use** for memoir |
| POST | `/memories/upsert` | `UpsertMemoryRequest` |
| POST | `/memories/bulk` | `BulkUpsertRequest` (`dry_run` allowed) |

Success shape for typed writes: `{ "id": "<uuid>", "status": "stored", "type": "<memory_type>" }`.

### Reads

| Method | Path | Notes |
|---|---|---|
| GET | `/context/active` | Query: `project` (required), `llm` default `perplexity`, `session_id` optional, `top_k` default `5` |
| GET | `/stream/context` | Same query. SSE. Optional. REST JSON is enough for Echos. |
| POST | `/query` | `MemoryQuery` → ranked list |

### Other

| Method | Path | Notes |
|---|---|---|
| POST | `/assistants/sync` | Persist session stub under `~/.memoree/llm_sync/{llm}/sessions/{session_id}.json` |
| POST | `/threads/summarize` | **501** until `MemoryEngine.summarize_thread` exists. Do not call as live. |
| POST | `/mcp` | JSON-RPC for IDEs. Not for the Android app. |

There is no `/write_event`. There is no `/query_context`. There is no `/family_prompts`. There is no audio upload route.

---

## 6. Layer map — Echos objects → Memoree

| Echos object | Memoree | Required / important fields |
|---|---|---|
| Colleen or Clara turn | `POST /memories/events` | `session_id`, `project`, `role` (`user` / `assistant` / `system` / `tool`), `content`, `llm`. Optional: `parent_id`, `tags`, `intent` (use Room `topicKey`), `model_name`, `turn_index`, `sentiment` |
| Synthesized chapter | `POST /memories/creative` | `project`, `medium` (`prose`), `content`. Optional: `title`, `themes`, `characters`, `status` (`draft` / `complete` / …), `tags`, `language` |
| Fact Colleen stated and confirmed | `POST /memories/meta` | `fact`, `project`. Set `verified: true` only after Colleen confirmed. `confidence`: `verified` / `high` / `medium` / `low` / `deprecated` |
| Child-submitted prompt | `POST /memories/events` with `role: "system"` and tag `family_prompt` **or** `POST /memories/procedural` (`task`, `steps`) | Do not invent a family-prompt route |
| Recall before Clara speaks | `GET /context/active?project=echos_of_life&llm=gemini&session_id=…&top_k=5` | Empty layers mean “no stored recall,” not “invent the family” |
| Topic search | `POST /query` | `query_text`, `project`, `memory_types`, `min_score` (default `0.65`) |
| Session resume | `POST /assistants/sync` | `llm`, `session_id`, optional `capabilities` |
| Long-thread compress | **Do it in Echos/Gemini**, then store as `creative` or `meta` | `/threads/summarize` is 501 |

Do not write quantum or governance records for this app.

---

## 7. Copy-paste payloads

Use these shapes. They match `schemas.py` and the ktrl `write_event` body. Dates may be omitted; the daemon defaults UTC.

### 7.1 Health then fail-open

```http
GET http://127.0.0.1:7042/health
GET https://memoree.aurphyx.org/health
```

If both fail: show a quiet “Memoree offline” chip. Keep recording.

### 7.2 Episodic turn (Colleen)

```http
POST /memories/events
Content-Type: application/json
```

```json
{
  "session_id": "11111111-1111-1111-1111-111111111111",
  "project": "echos_of_life",
  "role": "user",
  "content": "Amy was born in Pennsylvania in 1972. I remember the hospital lights and how small she was.",
  "llm": "gemini",
  "model_name": "gemini-3.5-flash",
  "parent_id": null,
  "tags": ["amy", "1972", "pennsylvania", "voice_studio"],
  "intent": "amy-childhood-1972-pennsylvania",
  "memory_type": "episodic"
}
```

Clara’s reply: same endpoint, `"role": "assistant"`, `parent_id` = the user-turn `id` returned from the prior write.

### 7.3 Creative chapter (after Write Chapter)

```http
POST /memories/creative
Content-Type: application/json
```

```json
{
  "project": "echos_of_life",
  "medium": "prose",
  "title": "The Day Amy Arrived",
  "content": "First-person chapter prose Colleen approved. Do not invent sentences she did not speak.",
  "language": "en",
  "characters": ["Colleen Marie Olson", "Amy"],
  "themes": ["early motherhood", "Pennsylvania"],
  "status": "complete",
  "tags": ["chapter", "amy", "1972"]
}
```

`medium` is **required**. Use `prose` for memoir chapters.

### 7.4 Meta fact (only if Colleen confirmed)

```http
POST /memories/meta
Content-Type: application/json
```

```json
{
  "project": "echos_of_life",
  "fact": "Amy, Colleen's first child, was born in Pennsylvania in 1972.",
  "confidence": "high",
  "verified": true,
  "verified_by": "colleen",
  "sources": ["session:11111111-1111-1111-1111-111111111111"]
}
```

Never upsert a family fact Gemini inferred from search grounding.

### 7.5 Active context (before Clara’s next question)

```http
GET /context/active?project=echos_of_life&llm=gemini&session_id=11111111-1111-1111-1111-111111111111&top_k=5
```

`ContextResponse` includes: `episodic`, `semantic`, `procedural`, `meta`, `quantum`, `creative`, `governance`, plus `active_axioms`, `active_dualities`, `project_meta`, `last_summary`. Inject what is present. Do not fill holes with fiction.

### 7.6 Structured query

```http
POST /query
Content-Type: application/json
```

```json
{
  "query_text": "Amy childhood Pennsylvania 1972",
  "project": "echos_of_life",
  "memory_types": ["episodic", "semantic", "meta", "creative"],
  "top_k": 10,
  "min_score": 0.65,
  "llm": "gemini"
}
```

Default `memory_types` in schema (if you omit them) is episodic, semantic, meta, **quantum**. For memoir, **pass the list explicitly** and include `creative`. Vector scores may be empty or weak (see gaps). Fall back to Room.

### 7.7 Assistant sync

```http
POST /assistants/sync
Content-Type: application/json
```

```json
{
  "llm": "gemini",
  "session_id": "11111111-1111-1111-1111-111111111111",
  "capabilities": {
    "app": "echos_of_life",
    "companion": "clara",
    "device": "s24"
  }
}
```

---

## 8. What Gemini should implement in Echos

1. Health against local then `https://memoree.aurphyx.org`. Cache the first base that answers.
2. Fail-open banner if none answer.
3. After each persisted Room conversation turn, POST `/memories/events` in the background.
4. After Colleen taps **Write Chapter** and Room stores the chapter, POST `/memories/creative` with `medium: "prose"`.
5. Optional: GET `/context/active` before Clara’s next spoken question; prepend recalled text to the Gemini system context.
6. Optional: POST `/query` when she picks a topic card.
7. Never send audio bytes. Never wait on Memoree before showing the transcript or the book page.

Kotlin / OkHttp on the S24 does not use browser CORS. A **browser** AI Studio preview calling the tunnel **will** hit CORS until the daemon allow-list is widened. That widening is **not** this briefing’s job. If the preview is browser-only, document the failure and keep Room.

---

## 9. Family and persona locks

These are **anchors Colleen may confirm**. They are not pre-seeded Memoree records. Do not write them as `verified` meta until she says them in-app.

- Subject / author: **Colleen Marie Olson**. Address as Colleen or Mom per `companion_profile`.
- Companion in this app: **Clara**. Not Audry. Audry is Aura’s admin soul in the wider ecosystem; she is not the memoir interviewer.
- Children, birth order: **Amy** (oldest; born Pennsylvania **1972**), **Mandy**, **Ross**, **Jacob**, **Holly** (youngest).
- Grandchildren: Minnesota and beyond. **Do not invent grandchild names.**
- Tone: warm, respectful, gentle. No cold analysis. No robotic recap.
- **Never invent** dates, hospitals, spouses, deaths, addresses, or stories Colleen did not speak.
- Google Search Grounding is **era context** (1970s Pennsylvania radio, weather, news). It is not biography. Do not merge search hits into `MetaMemory` as family truth.

---

## 10. Gaps — do not paper over

Implement as if these are true **today**:

1. **No project-create API.** `echos_of_life` is in `projects.json`. Further keys still require a file edit. Restart `memoree_service.py` after registry changes.
2. **CORS** in `memoree_service.py` allows only `http://127.0.0.1`, `http://localhost`, and those plus `:7042`. Android OkHttp is fine. Browser AI Studio against the public tunnel will fail CORS until Ross widens origins.
3. **Loopback bind.** Phone → `https://memoree.aurphyx.org`, not `127.0.0.1` on the S24.
4. **`gemini_hook.py` is an invalid client.** Copy section 7, not the hook.
5. **`POST /threads/summarize` is 501.** Summarize in Gemini; store the result.
6. **Vector recall is not guaranteed.** Engine / Qdrant / embedder wiring has been incomplete (`memoree/SUMZ-SUGGZ.md`). A write can return `stored` while `/query` returns nothing useful. Room remains source of truth for the book.
7. **No audio ingest.** Transcripts and URIs only.
8. **Memoree does not implement** Room tables, chapter JSON for the book UI, Android share sheet, TTS, or `googleSearch`.
9. **QUICKSTART port 8765 and `/write_event` do not exist.**
10. **Agora** (`memoree.g0dm0d3.org`) may be down. Required pair is local `:7042` + `https://memoree.aurphyx.org`.

---

## 11. Out of scope for whoever is holding this file

Do not, from this briefing alone:

- Rewrite `routes.py`, `schemas.py`, or `gemini_hook.py`
- Change CORS or bind address
- Fix the embedder / Qdrant constructor
- Dump TSLCA tensor law, SAGES guardian internals, or AuraFS replica counts into Echos prompts

If a field is missing from `schemas.py`, stop. Do not invent a fourth memory type.
