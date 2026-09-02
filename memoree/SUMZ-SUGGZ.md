---
type: implementation-note
title: Memoree — Summary and Suggestions
description: Status of the sovereign TSL memory daemon, multi-model hooks suite, CLI tool, Web Dashboard with Qdrant launcher, and RCL orchestration engine.
workspaces: rossaedwards/ecosys, aurphyx/ecosys
services:
  - Audry
  - AuraFS
  - Memoree
  - g0dm0d3-ktrl
domains:
  - cognition
  - systems
  - quantum
  - orchestration
nodes:
  - SIX⊗SIX
  - SIX⊗SCX
  - SIX⊗ICX
  - SCX⊗SIX
  - SCX⊗SCX
  - SCX⊗ICX
  - ICX⊗SIX
  - ICX⊗SCX
  - ICX⊗ICX
cores:
  - SIX
  - SCX
  - ICX
fields:
  - cognitive-field-tensor
  - rituals-chains-links
---

# Memoree — SUMZ / SUGGZ (v4.0 Complete & Ecosystem Aligned)

**Folder:** `memoree/`  
**Role:** Three-Squared-Lattice Cognitive Architecture (TSLCA) 9-cell memory substrate & Multi-Model Console. FastAPI daemon on `127.0.0.1:7042`. Cloudflare Zero Trust Tunnel at `https://memoree.aurphyx.com` (`5b13dbbe-9a8d-4d0e-b4d3-08ba18fda966`). Qdrant Docker container on port `6333:6333` (`memoree-qdrant`). MCP JSON-RPC for Claude Desktop / Cursor / Hermes / LM Studio.

---

## What Exists & Status (v4.0 Released)

| File / Component | Role / Capabilities | Status |
|---|---|---|
| [`schemas.py`](file:///c:/rossaedwards/ecosys/memoree/schemas.py) | Full 9 TSLCA memory models, `QuantumBand` (`Q1`–`Q13`), RCL specifications (`LinkSpec`, `ChainSpec`, `RitualSpec`, `ForkSpec`), `SoulJourneyStage`. | 🟢 v4.0 Verified |
| [`credentials_manager.py`](file:///c:/rossaedwards/ecosys/memoree/credentials_manager.py) | Secure local storage (`~/.memoree/credentials.json`) with safe masking, env var fallbacks, and connection probing. | 🟢 v4.0 Verified |
| [`cli.py`](file:///c:/rossaedwards/ecosys/memoree/cli.py) | CLI suite: `auth status/set/test`, `status`, `lattice`, `recall`, `cure`, `rcl`, `dashboard`. | 🟢 v4.0 Verified |
| [`static/dashboard/`](file:///c:/rossaedwards/ecosys/memoree/static/dashboard/index.html) | Glassmorphism Web Console at `/dashboard` with **Qdrant Dashboard Launcher (`localhost:6333/dashboard`)**, 3×3 Lattice visualizer, HIF gate inspector, and RCL studio. | 🟢 v4.0 Verified |
| [`hooks_registry.py`](file:///c:/rossaedwards/ecosys/memoree/hooks_registry.py) | Unified multi-model dispatcher (`gemini`, `claude`, `supergrok`, `perplexity`, `hermes`, `lmstudio`, `ollama`). | 🟢 v4.0 Verified |
| [`rcl_engine.py`](file:///c:/rossaedwards/ecosys/memoree/rcl_engine.py) | Implements Level 1 Links, Level 2 Chains (`Idea 2 Sold`, `Debug Sandbox`), Level 3 Rituals (`24h Alignment`, `Academic Harvest`), Level 4 Forkz (multiverse reality branching). | 🟢 v4.0 Verified |
| [`gemini_hook.py`](file:///c:/rossaedwards/ecosys/memoree/gemini_hook.py) | Real Google Gemini 2.0 / 1.5 API client with 9-cell context pre-fetching and turn archiving. | 🟢 v4.0 Verified |
| [`claude_hook.py`](file:///c:/rossaedwards/ecosys/memoree/claude_hook.py) | Real Anthropic Claude 3.7 / 3.5 Sonnet client with thinking support and context injection. | 🟢 v4.0 Verified |
| [`supergrok_hook.py`](file:///c:/rossaedwards/ecosys/memoree/supergrok_hook.py) | Real xAI Grok-3 / Grok-2 client with reasoning tokens and context injection. | 🟢 v4.0 Verified |
| [`perplexity_hook.py`](file:///c:/rossaedwards/ecosys/memoree/perplexity_hook.py) | Real Perplexity Sonar Pro client with web citation and domain capture. | 🟢 v4.0 Verified |
| [`hermes_hook.py`](file:///c:/rossaedwards/ecosys/memoree/hermes_hook.py) | Real Nous Hermes Plus ACP adapter and memory provider. | 🟢 v4.0 Verified |
| [`lmstudio_hook.py`](file:///c:/rossaedwards/ecosys/memoree/lmstudio_hook.py) | Local LM Studio model client with model switching (`liquid`, `nemotron`, `qwen`). | 🟢 v4.0 Verified |
| [`ollama_hook.py`](file:///c:/rossaedwards/ecosys/memoree/ollama_hook.py) | Local Ollama offline model client. | 🟢 v4.0 Verified |
| [`tsl_memory_kernel.py`](file:///c:/rossaedwards/ecosys/memoree/tsl_memory_kernel.py) | HIF formula ($\text{HIF} = \sqrt[3]{C\cdot R\cdot A} \cdot \Phi$), TTG gates ($H_{\text{create}}=0.65, H_{\text{integrate}}=0.55, H_{\text{renew}}=0.35$), SUXS-IFO prompt contraction. | 🟢 v4.0 Verified |
| [`vector_backend.py`](file:///c:/rossaedwards/ecosys/memoree/vector_backend.py) | 9 discrete Qdrant collections with native `query_points` and local embedding fallback. | 🟢 v4.0 Verified |
| [`memory_engine.py`](file:///c:/rossaedwards/ecosys/memoree/memory_engine.py) | 9-cell write handlers, SUXS-IFO context contraction, buffer curing, live 3×3 field tensor snapshot. | 🟢 v4.0 Verified |
| [`routes.py`](file:///c:/rossaedwards/ecosys/memoree/routes.py) | Endpoints: `/dashboard`, `/api/hooks`, `/api/auth`, `/api/rcl`, `/lattice`, `/hif`, `/query`, `/context/active`, and MCP tools. | 🟢 v4.0 Verified |
| [`heartbeat.py`](file:///c:/rossaedwards/ecosys/memoree/heartbeat.py) | 30s background loop curing working memories into permanent layers and pulsing vector diagnostics. | 🟢 v4.0 Verified |
| [`memoree_service.py`](file:///c:/rossaedwards/ecosys/memoree/memoree_service.py) | Uvicorn + FastAPI daemon with active Heartbeat lifespan, CORS for `memoree.aurphyx.com`. | 🟢 v4.0 Verified |

---

## Infrastructure Matrix

- **Web Dashboard:**
  - Local URL: `http://127.0.0.1:7042/dashboard`
  - Public Route: `https://memoree.aurphyx.com/dashboard`
- **Qdrant Vector Database:**
  - Name: `memoree-qdrant` (Container ID: `84bea29f5a8c`)
  - Web UI: `http://localhost:6333/dashboard`
  - Ports: `6333:6333`
- **Cloudflare Zero Trust Tunnel:**
  - Name: `Memoree` (Tunnel ID: `5b13dbbe-9a8d-4d0e-b4d3-08ba18fda966`)
  - Route: `https://memoree.aurphyx.com` → `http://127.0.0.1:7042`
- **Windows NSSM Service:**
  - Service: `MemoreeDaemon`
  - Restart command: `c:\toolz\nssm.exe restart MemoreeDaemon` (run as Administrator)

---

## Quick CLI Reference

```powershell
# Check configured model hooks & keys
python cli.py auth status

# Configure an API key
python cli.py auth gemini <YOUR_KEY>
python cli.py auth claude <YOUR_KEY>
python cli.py auth grok <YOUR_KEY>
python cli.py auth perplexity <YOUR_KEY>

# Test live connection probe across all hooks
python cli.py auth test

# Inspect system status & 3x3 Cognitive Field Tensor
python cli.py status
python cli.py lattice

# Query memories across 9 collections
python cli.py recall "topological quantum computing" --top-k 5

# List Rituals, Chains, Links, & Forkz
python cli.py rcl list

# Open the Web Dashboard
python cli.py dashboard
```
