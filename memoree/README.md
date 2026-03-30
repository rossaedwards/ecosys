# Aurphyx_Memoree Daemon v0.1

**Sovereign Memory, Identity & Continuity Substrate**
*f0rg3d in l0v3 by Ross Edwards | Aurphyx LLC*

---

## Architecture

| Layer | Technology | Role |
|---|---|---|
| Filesystem | AuraFS (fractal shards) | Canonical persistent storage |
| Vector DB | ChromaDB (local) | Semantic search & embeddings |
| Transitional | Memori + SQLite | Legacy bridge (Phase 0/I) |
| Embeddings | all-MiniLM-L6-v2 | Local, sovereign, no API key |
| API | FastAPI on 127.0.0.1:7042 | Unified read/write interface |

## Why ChromaDB over Memori for vectors?

- **AuraFS-aligned**: ChromaDB stores to a directory path — maps directly to AuraFS shard structure
- **Fully local & sovereign**: no API key, no telemetry (disabled), pure filesystem
- **Fractal-compatible**: collection-per-memory-type mirrors the 4-type schema
- **Blazing fast**: in-process, no network round-trips
- **Migratable**: when AuraFS gets a native vector index, swap `VectorBackend` only

## Quick Start

```bash
pip install -r requirements.txt
python daemon/memoree_service.py
```

## API Endpoints (localhost:7042)

| Method | Path | Description |
|---|---|---|
| GET | /health | Daemon status |
| POST | /memories/events | Write episodic event |
| POST | /memories/semantic | Embed semantic document |
| POST | /memories/procedural | Store workflow |
| POST | /memories/meta | Store invariant fact |
| GET | /context/active | Read context for project |
| POST | /threads/summarize | Summarize thread |
| POST | /assistants/sync | Sync LLM assistant state |

## Migration Phases

- **Phase 0**: Memori primary, Memoree API shim on top
- **Phase I**: Double-write AuraFS + ChromaDB, Memori mirror
- **Phase II**: Backfill Memori → AuraFS + ChromaDB, flip primary
- **Phase III**: Deprecate Memori (or keep as forensic read-only)
