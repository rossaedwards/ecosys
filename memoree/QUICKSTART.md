# Memoree v3 — Tonight's Launch Checklist

> **Goal:** Qdrant running, daemon alive, first memory written, first recall verified.

---

## Step 1 — Start Qdrant (Docker)

```powershell
docker run -d --name memoree-qdrant `
  -p 6333:6333 `
  -v D:/aurphyx/memoree/qdrant:/qdrant/storage `
  qdrant/qdrant
```

Verify: http://localhost:6333/dashboard

---

## Step 2 — Install Python deps

```powershell
cd memoree
pip install -r requirements.txt
# CPU-only torch (saves ~2GB vs CUDA):
pip install torch --index-url https://download.pytorch.org/whl/cpu
```

---

## Step 3 — Configure your paths

Edit `config.yaml`:
- `memoree.data_root` → `D:/aurphyx/memoree` (or C: if no D:)
- `sqlite.path` → `D:/aurphyx/memoree/ledger.db`
- `qdrant.storage_path` → matches your Docker volume mount
- If using LM Studio for embeddings: set `embedder.backend: lmstudio`

---

## Step 4 — Start the daemon

```powershell
uvicorn memoree.core.api:app --host 127.0.0.1 --port 8765 --reload
```

Verify: http://127.0.0.1:8765/health

Expected response:
```json
{"status": "alive", "version": "3.1.0", "backend": "qdrant"}
```

---

## Step 5 — Write your first sovereign memory

```powershell
curl -X POST http://127.0.0.1:8765/write_event `
  -H "Content-Type: application/json" `
  -d '{
    "llm": "gemini",
    "model": "gemini-2.5-pro",
    "type": "semantic",
    "project": "ftqc",
    "subproject": "equilibrium-manifolds",
    "content": "FTQC equilibrium manifold lives on the Bloch sphere boundary at T=0. Error threshold ~1% for surface codes.",
    "tags": ["ftqc", "bloch-sphere", "surface-code"],
    "priority": 9,
    "confidence": 0.95,
    "mood": "focused"
  }'
```

---

## Step 6 — Recall it

```powershell
curl "http://127.0.0.1:8765/query_context?q=FTQC+equilibrium+manifold&project=ftqc"
```

---

## Step 7 — WebSocket test (optional)

```python
import asyncio, websockets, json

async def test():
    async with websockets.connect("ws://127.0.0.1:8765/ws/memory") as ws:
        # Ping
        await ws.send(json.dumps({"action": "ping", "payload": {}}))
        print(await ws.recv())
        # Write via WS
        await ws.send(json.dumps({
            "action": "write",
            "payload": {
                "llm": "nemotron",
                "model": "nemotron-3-nano-4b",
                "type": "episodic",
                "project": "god-mode-core",
                "content": "Nemotron discussed neglecton topology in the fractal lattice.",
                "tags": ["neglecton", "ftqc", "local"]
            }
        }))
        print(await ws.recv())

asyncio.run(test())
```

---

## Step 8 — Hook your LLMs

### Gemini
```python
import google.generativeai as genai
from memoree.hooks.gemini_hook import wrap_model

genai.configure(api_key="YOUR_KEY")
model = wrap_model(genai.GenerativeModel("gemini-2.5-pro"))
response = model.generate_content("Explain FTQC equilibrium manifolds")
# → auto-saved to Memoree with project='memoree-v3'
```

### Local (Nemotron / Qwen via LM Studio)
```python
from memoree.hooks.lmstudio_hook import nemotron, qwen

reply = nemotron("Describe neglecton topology", project="god-mode-core")
reply = qwen("What is the Bliss equation?", project="ftqc")
# → both auto-saved
```

### Perplexity
```python
import os
os.environ["PERPLEXITY_API_KEY"] = "pplx-..."
from memoree.hooks.perplexity_hook import chat
reply = chat("Latest MemOS v2 features?")
# → auto-saved
```

---

## MemOS (optional — enable when ready)

```powershell
docker run -d --name memoree-memos -p 9090:9090 memos/memos:latest
```

Then in `config.yaml`:
```yaml
memos:
  enabled: true
```

Every `write_event` will now also mirror into your MemOS `aurphyx_main` cube.

---

## Architecture at a Glance

```
LLM Response
    │
    ▼
hook (gemini/perplexity/lmstudio)
    │
    ▼
POST /write_event  (or WS write)
    │
    ├─► SQLite ledger (D:/aurphyx/memoree/ledger.db)  ← raw + FTS5 search
    ├─► Qdrant (localhost:6333)                        ← dense vector + filters
    ├─► MemOS cube (localhost:9090)  [optional]       ← AI-native memory OS
    └─► Sync queue → PocketBase / PowerSync           ← offline-first push
```
