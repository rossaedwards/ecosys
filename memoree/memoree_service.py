## ** APS-TSLCA-MEMOREE-SERVICE **
## ** Memoree - Sovereign Memory Substrate **
## ** Symbiotic Universal Xessability Standards **
## ** Three-Squared-Lattice Cognitive Architecture **
## ** Aurphyx Primordial Standard **
## ** Aurphyx LLC **
## ** SAGES | Proprietary | Pro-Existence **
## ** Accessibility = Xessability **
## ** Version 4.0 **

"""
Memoree — Main Daemon Entry Point (TSLCA 9-Cell Lattice & Cloudflare Edge)
═══════════════════════════════════════════════════════════════════════════════
Sovereign memory substrate for the Aurphyx LLC ecosystem.

  Path    : c:\\memoree\\memoree_service.py
  Owner   : Ross Edwards / Aurphyx LLC
  GitHub  : rossaedwards | aurphyx
  ORCiD   : 0009-0008-0539-1289
  Port    : 127.0.0.1:7042
  Edge    : https://memoree.aurphyx.com
  Tunnel  : 5b13dbbe-9a8d-4d0e-b4d3-08ba18fda966
  Protocol: HTTP/1.1/v1 + SSE streaming (StreamingResponse)
  MCP     : JSON-RPC 2.0 over HTTP (Claude / Cursor / Hermes / LM Studio)
═══════════════════════════════════════════════════════════════════════════════
f0rg3d in l0v3 by Ross Edwards
"""

from __future__ import annotations

import asyncio
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

from heartbeat import HeartbeatLoop
from routes import router

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
    """
    log.info("=" * 65)
    log.info("  Memoree v4.0.0 Daemon - Sovereign Memory Substrate")
    log.info("  Architecture : Three-Squared-Lattice Cognitive Architecture (TSLCA)")
    log.info("  Owner        : Ross Edwards / Aurphyx LLC")
    log.info("  Local Binding: http://127.0.0.1:7042")
    log.info("  Cloudflare   : https://memoree.aurphyx.com")
    log.info("  Health       : http://127.0.0.1:7042/health")
    log.info("  Lattice API  : http://127.0.0.1:7042/lattice")
    log.info("  MCP Endpoint : http://127.0.0.1:7042/mcp")
    log.info("  Stream       : http://127.0.0.1:7042/stream/context")
    log.info("  OpenAPI Docs : http://127.0.0.1:7042/docs")
    log.info("=" * 65)

    heartbeat = HeartbeatLoop()
    heartbeat_task = asyncio.create_task(heartbeat.run())

    yield

    heartbeat.stop()
    heartbeat_task.cancel()
    log.info("Memoree shutting down - backends flushed & heartbeat stopped.")



# ─────────────────────────────────────────────────────────────────────────────
# Application
# ─────────────────────────────────────────────────────────────────────────────

app = FastAPI(
    title="Memoree",
    description=(
        "Sovereign Memory Substrate for the Aurphyx LLC ecosystem. "
        "Provides non-commutative 9-cell TSLCA memory layers (Sensory, Working, "
        "Episodic, Semantic, Meta, Quantum, Identity, Procedural, Governance) "
        "gated by the Harmonic Integrity Field (HIF) with SSE streaming and "
        "MCP JSON-RPC 2.0 interface for Claude, Cursor, Hermes, and Gemini."
    ),
    version="4.0.0",
    contact={
        "name": "Ross Edwards / Aurphyx LLC",
        "url": "https://github.com/rossaedwards",
        "email": "ross@aurphyx.com",
    },
    license_info={"name": "Proprietary — Aurphyx LLC / SAGES Pro-Existence"},
    lifespan=lifespan,
    docs_url="/docs",
    redoc_url="/redoc",
    openapi_url="/openapi.json",
)

# ── Middleware ────────────────────────────────────────────────────────────────

app.add_middleware(
    CORSMiddleware,
    allow_origins=[
        "http://127.0.0.1",
        "http://127.0.0.1:7042",
        "http://localhost",
        "http://localhost:7042",
        "https://memoree.aurphyx.com",
        "http://memoree.aurphyx.com",
    ],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

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
        log_level="info",
        reload=False,
    )
