# Memoree v3

> **Blank-slate rewrite — 2026 open-source only.**  
> Keep the name. Keep the API (`write_event`, `read_context`, etc.). Burn everything else.

## Philosophy
Memoree v3 is a **persistent, queryable memory ledger** for all LLMs you run.  
Episodic chats, semantic notes, procedural workflows, meta-facts — all stored locally first,  
synced to cloud when WiFi permits, recalled by any agent on demand.

```
local-first → multi-modal → agent-aware → offline-sync
```

## Stack (2026 bleeding-edge)
| Layer | Technology | Why |
|---|---|---|
| Vector store | **LanceDB** (embedded) | Zero-server, columnar, hybrid keyword+vector, CPU-fast |
| Fallback vector | **Qdrant** (local binary) | Superior metadata filters when needed |
| Raw ledger | **SQLite** via `aiosqlite` + FTS5 | Chats as JSON rows, zero infra |
| Memory OS overlay | **MemOS** (optional) | AI-native multi-cube isolation, feedback loops |
| Sync | **PowerSync + Supabase** free tier | Offline SQLite queue → auto-push on WiFi |
| Embeddings | `sentence-transformers` or LM Studio local | CPU-only, no API key needed |
| API server | **FastAPI** + uvicorn | REST + future WS |

## Directory
```
memoree/
├── core/
│   ├── api.py              # write_event, read_context, query_context, list_events
│   ├── ledger.py           # SQLite aiosqlite layer (episodic raw store)
│   ├── vector_backend.py   # LanceDB adapter (swappable to Qdrant)
│   ├── schemas.py          # Pydantic v2 models (MemoryEvent, ContextQuery, etc.)
│   └── embedder.py         # sentence-transformers / local LM Studio embed endpoint
├── sync/
│   ├── powersync_client.py # PowerSync + Supabase offline-first push
│   └── pocketbase_mirror.py# PocketBase fallback mirror (optional)
├── hooks/
│   ├── gemini_hook.py      # Post-response auto-save for Gemini SDK
│   ├── perplexity_hook.py  # Perplexity Pro API hook
│   ├── supergrok_hook.py   # SuperGrok manual/WS bridge (stub)
│   └── lmstudio_hook.py    # LM Studio headless callback (Nemotron/Qwen)
├── memos/
│   └── memos_overlay.py    # MemOS FastAPI bridge (optional Docker)
├── config.yaml             # All paths, backends, LLM names — one file to rule them
├── requirements.txt        # pip install -r requirements.txt
└── README.md
```

## Quickstart (Windows, D: drive)
```powershell
# 1. Install
pip install -r memoree/requirements.txt

# 2. Configure (edit config.yaml — set lancedb_path to D:/aurphyx/embeddings)
copy memoree\config.yaml memoree\config.local.yaml

# 3. Run daemon
uvicorn memoree.core.api:app --host 127.0.0.1 --port 8765 --reload

# 4. Test
curl -X POST http://127.0.0.1:8765/write_event \
  -H 'Content-Type: application/json' \
  -d '{"llm":"gemini","type":"episodic","content":"We discussed FTQC equilibrium manifolds."}'
```

## LLM hooks
- **Gemini / Perplexity**: SDK wrapper → POST to `/write_event` after each response
- **SuperGrok**: Browser extension stub or manual POST
- **Locals (Nemotron 3 Nano 4B / Qwen 3.5 4B)**: LM Studio headless callback → `lmstudio_hook.py`

## Swapping backends
Change `vector_backend` in `config.yaml` from `lancedb` to `qdrant` — no other edits needed.
