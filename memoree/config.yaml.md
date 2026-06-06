config.yaml.md
# Memoree — Sovereign Memory Substrate
# Cleaned for C:\memoree flat structure + LM Studio MCP
# Date: 2026-04-03

memoree:
  version: "3.1.0"
  data_root: "C:/memoree"
  project: memoree

# ─── Vector Store ───────────────────────────────────────────────
vector_backend: qdrant
qdrant:
  host: localhost
  port: 6333
  collection: memoree_events
  storage_path: "C:/memoree/qdrant"
  embed_dim: 384
  hybrid:
    enabled: true
    sparse_model: bm25

# ChromaDB disabled for now (we can re-enable later)
# chroma:
#   persist_dir: "C:/memoree/embeddings"
#   embedding_model: "all-MiniLM-L6-v2"

# LanceDB disabled for now
# lancedb:
#   path: "C:/memoree/embeddings"
#   table_name: memoree_events

# ─── SQLite Ledger ──────────────────────────────────────────────
sqlite:
  path: "C:/memoree/ledger.db"
  wal_mode: true

# ─── Embedder ───────────────────────────────────────────────────
embedder:
  backend: sentence_transformers
  model: all-MiniLM-L6-v2
  lmstudio_url: "http://localhost:1234/v1/embeddings"

# ─── LLM Hooks ──────────────────────────────────────────────────
hooks:
  gemini:
    enabled: true
    auto_save: true
    default_project: memoree
  perplexity:
    enabled: true
    auto_save: true
    default_project: memoree
  supergrok:
    enabled: true
    auto_save: true
    default_project: memoree
  lmstudio:
    enabled: true
    auto_save: true
    base_url: "http://localhost:1234/v1"
    models:
      liquid/lfm2.5-1.2b:
        alias: liquid
        default_project: memoree
      nemotron-3-nano-4b:
        alias: nemotron
        default_project: memoree
      qwen-3.5-4b:
        alias: qwen
        default_project: memoree

# ─── MemOS Overlay (optional) ───────────────────────────────────
memos:
  enabled: false
  api_url: "http://localhost:9090"
  cube_id: rossaedwards_main

# ─── Rich Metadata Defaults ─────────────────────────────────────
metadata_defaults:
  priority: 5
  mood: neutral
  confidence: 0.84
  source: api

# ─── Sync Layer ─────────────────────────────────────────────────
sync:
  backend: pocketbase
  pocketbase:
    enabled: true
    url: "http://localhost:8090"
    collection: memoree_events
  powersync:
    supabase_url: ""
    supabase_anon_key: ""
    queue_path: "C:/memoree/sync_queue.db"

# ─── FastAPI + WebSocket Daemon ──────────────────────────────────
server:
  host: 127.0.0.1
  port: 7042
  reload: true
  ws_path: /ws/memory
