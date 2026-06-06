"""
Memoree — Main Daemon Entry Point
═══════════════════════════════════════════════════════════════════════════════
Sovereign memory substrate for the Aurphyx LLC ecosystem.

  Path    : c:\\memoree\\memoree_service.py
  Owner   : Ross Edwards / Aurphyx LLC
  GitHub  : rossaedwards | aurphyx
  ORCiD   : 0009-0008-0539-1289
  Port    : 127.0.0.1:7042
  Protocol: HTTP/1.1/v1 + SSE streaming (StreamingResponse)
  MCP     : JSON-RPC 2.0 over HTTP (LM Studio compatible)

Lifespan
─────────
  startup  → MemoryEngine init, heartbeat stub, startup banner
  shutdown → graceful flush + log drain

Disabled
─────────
  HeartbeatLoop   → heartbeat.py  [awaiting integration]
═══════════════════════════════════════════════════════════════════════════════
f0rg3d in l0v3 by Ross Edwards
"""

from __future__ import annotations

import logging
import os
import sys
from contextlib import asynccontextmanager
from pathlib import Path

import uvicorn
from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware
from fastapi.middleware.gzip import GZipMiddleware

# Guarantee c:\memoree\ is always on the import path regardless of CWD
sys.path.insert(0, str(Path(__file__).parent))

from routes import router

# from heartbeat import HeartbeatLoop   # [DISABLED] awaiting integration

# ─────────────────────────────────────────────────────────────────────────────
# Logging
# ─────────────────────────────────────────────────────────────────────────────

LOG_PATH = Path(__file__).parent / "memoree_service.log"

logging.basicConfig(
    level=logging.INFO,
    format="%(asctime)s [%(levelname)s] %(name)s: %(message)s",
    handlers=[
        logging.FileHandler(str(LOG_PATH), encoding="utf-8"),
        logging.StreamHandler(sys.stdout),
    ],
)
log = logging.getLogger("memoree")

# ─────────────────────────────────────────────────────────────────────────────
# Lifespan
# ─────────────────────────────────────────────────────────────────────────────


@asynccontextmanager
async def lifespan(app: FastAPI):
    """
    Startup → yield → shutdown lifecycle manager.

    On startup:
      • Logs the daemon banner with port, MCP endpoint, and SSE stream URL.
      • HeartbeatLoop stub is preserved — uncomment to re-enable.

    On shutdown:
      • Logs graceful shutdown signal.
      • Add flush/cleanup hooks here as backends are integrated.
    """
    log.info("═" * 60)
    log.info("  Memoree Daemon — Sovereign Memory Substrate")
    log.info("  Owner   : Ross Edwards / Aurphyx LLC")
    log.info("  Binding : http://127.0.0.1:7042")
    log.info("  Health  : http://127.0.0.1:7042/health")
    log.info("  MCP     : http://127.0.0.1:7042/mcp")
    log.info("  Stream  : http://127.0.0.1:7042/stream/context")
    log.info("  Docs    : http://127.0.0.1:7042/docs")
    log.info("═" * 60)

    # heartbeat = HeartbeatLoop()          # [DISABLED]
    # asyncio.create_task(heartbeat.run()) # [DISABLED]

    yield

    log.info("Memoree shutting down — backends flushed.")


# ─────────────────────────────────────────────────────────────────────────────
# Application
# ─────────────────────────────────────────────────────────────────────────────

app = FastAPI(
    title="Memoree",
    description=(
        "Sovereign Memory Substrate for the Aurphyx LLC ecosystem. "
        "Provides episodic, semantic, procedural, meta, quantum, creative, "
        "and governance memory layers with SSE streaming and MCP/JSON-RPC "
        "for LM Studio compatibility."
    ),
    version="0.1.0",
    contact={
        "name": "Ross Edwards / Aurphyx LLC",
        "url": "https://github.com/rossaedwards",
        "email": "ross@aurphyx.com",
    },
    license_info={"name": "Proprietary — Aurphyx LLC"},
    lifespan=lifespan,
    docs_url="/docs",
    redoc_url="/redoc",
    openapi_url="/openapi.json",
)

# ── Middleware ────────────────────────────────────────────────────────────────

# Local-only — allow 127.0.0.1 loopback clients (LM Studio, hooks, etc.)
app.add_middleware(
    CORSMiddleware,
    allow_origins=[
        "http://127.0.0.1",
        "http://127.0.0.1:7042",
        "http://localhost",
        "http://localhost:7042",
    ],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

# Compress large context payloads — SSE streams and /context/active responses
# can be substantial; GZip at threshold 1 KB.
app.add_middleware(GZipMiddleware, minimum_size=1024)

# ── Routers ───────────────────────────────────────────────────────────────────

app.include_router(router)


# ─────────────────────────────────────────────────────────────────────────────
# Entry Point
# ─────────────────────────────────────────────────────────────────────────────

if __name__ == "__main__":
    uvicorn.run(
        "memoree_service:app",
        host="127.0.0.1",
        port=7042,
        reload=False,
        log_level="info",
        access_log=True,
        # HTTP/1.1 only — SSE requires persistent connections; HTTP/2 push
        # is not needed at localhost and adds TLS complexity for no gain.
        http="h11",
    )
