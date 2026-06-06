"""
Memoree — FastAPI Routes
═══════════════════════════════════════════════════════════════════════════════
Local-only REST + SSE streaming API on 127.0.0.1:7042.
MCP JSON-RPC 2.0 endpoint for LM Studio / Cursor / Claude Desktop.

  Path    : c:\\memoree\\routes.py
  Owner   : Ross Edwards / Aurphyx LLC
  GitHub  : rossaedwards | aurphyx
  ORCiD   : 0009-0008-0539-1289

Route Map
─────────────────────────────────────────────────────────────────────────────
  GET  /health                    → daemon liveness check
  GET  /diagnostics               → MemoreeDiagnostics snapshot
  GET  /projects                  → list all registered projects
  GET  /projects/{key}            → single ProjectMeta

  POST /memories/events           → write EpisodicMemory
  POST /memories/semantic         → embed SemanticMemory
  POST /memories/procedural       → store ProceduralMemory
  POST /memories/meta             → store MetaMemory
  POST /memories/quantum          → store QuantumMemory
  POST /memories/creative         → store CreativeMemory
  POST /memories/governance       → store GovernanceMemory
  POST /memories/upsert           → generic typed upsert
  POST /memories/bulk             → bulk upsert with dry-run support

  GET  /context/active            → full ContextResponse (JSON)
  GET  /stream/context            → SSE stream of ContextResponse chunks
  POST /query                     → structured MemoryQuery → ranked results

  POST /threads/summarize         → ThreadSummary (stub — engine method pending)
  POST /assistants/sync           → LLM session state sync to disk

  POST /mcp                       → MCP JSON-RPC 2.0 (LM Studio compatible)
═══════════════════════════════════════════════════════════════════════════════
f0rg3d in l0v3 by Ross Edwards
"""

from __future__ import annotations

import asyncio
import json
import logging
import os
from datetime import datetime, timezone
from pathlib import Path
from typing import Any, AsyncGenerator, Dict, List, Optional

from fastapi import APIRouter, HTTPException, Request
from fastapi.responses import JSONResponse, Response, StreamingResponse
from pydantic import BaseModel

from memory_engine import MemoryEngine
from schemas import (
    # Memory types
    EpisodicMemory,
    SemanticMemory,
    ProceduralMemory,
    MetaMemory,
    QuantumMemory,
    CreativeMemory,
    GovernanceMemory,
    # Request / response schemas
    BulkUpsertRequest,
    ContextResponse,
    MemoryQuery,
    ThreadSummary,
    UpsertMemoryRequest,
)

log    = logging.getLogger("memoree.routes")
router = APIRouter()
engine = MemoryEngine()


# ─────────────────────────────────────────────────────────────────────────────
# Helpers
# ─────────────────────────────────────────────────────────────────────────────

def _now_iso() -> str:
    return datetime.now(tz=timezone.utc).isoformat()


def _sse_event(data: Any, event: str = "message") -> str:
    """
    Format a Server-Sent Events frame.

    The MCP SSE spec uses `event:` + `data:` pairs separated by double
    newlines.  `data` is JSON-serialised so any Pydantic model or dict
    can be passed directly.
    """
    payload = data if isinstance(data, str) else json.dumps(data, default=str)
    return f"event: {event}\ndata: {payload}\n\n"


# ─────────────────────────────────────────────────────────────────────────────
# Health & Diagnostics
# ─────────────────────────────────────────────────────────────────────────────

@router.get("/health", tags=["System"])
def health() -> Dict:
    """Daemon liveness check — returns immediately with no backend I/O."""
    return {
        "status":    "alive",
        "service":   "memoree",
        "version":   "0.1.0",
        "timestamp": _now_iso(),
    }


@router.get("/diagnostics", tags=["System"])
def diagnostics():
    """
    Live MemoreeDiagnostics snapshot.

    Queries the vector backend for collection counts and reflects
    current uptime, active sessions, and LLM hook registry.
    """
    return engine.diagnostics()


# ─────────────────────────────────────────────────────────────────────────────
# Project Registry
# ─────────────────────────────────────────────────────────────────────────────

@router.get("/projects", tags=["Projects"])
def list_projects() -> List[Dict]:
    """List all projects registered in projects.json, sorted by key."""
    return [p.model_dump() for p in engine.list_projects()]


@router.get("/projects/{key}", tags=["Projects"])
def get_project(key: str) -> Dict:
    """Retrieve a single ProjectMeta by its canonical key."""
    meta = engine.get_project(key)
    if meta is None:
        raise HTTPException(status_code=404, detail=f"Project '{key}' not found in registry.")
    return meta.model_dump()


# ─────────────────────────────────────────────────────────────────────────────
# Memory Write Endpoints
# ─────────────────────────────────────────────────────────────────────────────

@router.post("/memories/events", tags=["Memory"])
def write_event(mem: EpisodicMemory) -> Dict:
    """Persist a single episodic conversation turn."""
    mem_id = engine.write_event(mem)
    return {"id": mem_id, "status": "stored", "type": "episodic"}


@router.post("/memories/semantic", tags=["Memory"])
def embed_document(mem: SemanticMemory) -> Dict:
    """Embed and persist a semantic knowledge document."""
    mem_id = engine.embed_document(mem)
    return {"id": mem_id, "status": "stored", "type": "semantic"}


@router.post("/memories/procedural", tags=["Memory"])
def store_workflow(mem: ProceduralMemory) -> Dict:
    """Store a procedural workflow / task recipe."""
    mem_id = engine.store_workflow(mem)
    return {"id": mem_id, "status": "stored", "type": "procedural"}


@router.post("/memories/meta", tags=["Memory"])
def store_fact(mem: MetaMemory) -> Dict:
    """Persist a confidence-tracked invariant fact."""
    mem_id = engine.store_fact(mem)
    return {"id": mem_id, "status": "stored", "type": "meta"}


@router.post("/memories/quantum", tags=["Memory"])
def store_quantum(mem: QuantumMemory) -> Dict:
    """Store a quantum / simulation state snapshot."""
    mem_id = engine.store_quantum(mem)
    return {"id": mem_id, "status": "stored", "type": "quantum"}


@router.post("/memories/creative", tags=["Memory"])
def store_creative(mem: CreativeMemory) -> Dict:
    """Persist a creative / narrative memory entry."""
    mem_id = engine.store_creative(mem)
    return {"id": mem_id, "status": "stored", "type": "creative"}


@router.post("/memories/governance", tags=["Memory"])
def store_governance(mem: GovernanceMemory) -> Dict:
    """Store a governance record (vote, policy, mandate, ledger entry)."""
    mem_id = engine.store_governance(mem)
    return {"id": mem_id, "status": "stored", "type": "governance"}


@router.post("/memories/upsert", tags=["Memory"])
def upsert_memory(request: UpsertMemoryRequest) -> Dict:
    """
    Generic typed upsert — routes to the correct write method based on
    `memory_type`.  Pass specialised fields in `extra`.
    """
    try:
        mem_id = engine.upsert(request)
    except ValueError as exc:
        raise HTTPException(status_code=422, detail=str(exc)) from exc
    return {"id": mem_id, "status": "stored", "type": request.memory_type}


@router.post("/memories/bulk", tags=["Memory"])
def bulk_upsert(request: BulkUpsertRequest) -> Dict:
    """
    Batch upsert of multiple memory records.

    Set `dry_run=true` to validate all records without writing to storage —
    useful for import previews and CI schema validation pipelines.
    Returns `written` (list of IDs) and `errors` (list of (index, message)).
    """
    return engine.bulk_upsert(request)


# ─────────────────────────────────────────────────────────────────────────────
# Context Read — JSON + SSE Stream
# ─────────────────────────────────────────────────────────────────────────────

@router.get(
    "/context/active",
    response_model=ContextResponse,
    tags=["Context"],
)
def read_context(
    project:    str,
    llm:        str           = "perplexity",
    session_id: Optional[str] = None,
    top_k:      int           = 5,
):
    """
    Assemble and return the full ContextResponse for a project as JSON.

    Includes all seven memory layers, active axioms, active dualities,
    DualityPair objects, related projects, and ProjectMeta — ready for
    direct injection into any LLM system-prompt hook.
    """
    return engine.read_context(
        project=project,
        llm=llm,
        session_id=session_id,
        top_k=top_k,
    )


async def _context_sse_generator(
    project:    str,
    llm:        str,
    session_id: Optional[str],
    top_k:      int,
) -> AsyncGenerator[str, None]:
    """
    Async generator that streams a ContextResponse as SSE events.

    Event sequence:
      1. `start`   → session metadata
      2. `layer`   → one event per memory layer (episodic, semantic, …)
      3. `meta`    → active axioms, dualities, invariants, project meta
      4. `done`    → total_memories count + generated_at timestamp

    This allows hook clients to start injecting context into their system
    prompt before the full payload has arrived — critical for large sessions
    with many memory layers.
    """
    try:
        ctx = engine.read_context(
            project=project,
            llm=llm,
            session_id=session_id,
            top_k=top_k,
        )
    except Exception as exc:
        yield _sse_event({"error": str(exc)}, event="error")
        return

    # ── 1. Start ──────────────────────────────────────────────────────────────
    yield _sse_event(
        {
            "project":    ctx.project,
            "session_id": ctx.session_id,
            "llm":        ctx.llm,
            "timestamp":  _now_iso(),
        },
        event="start",
    )
    await asyncio.sleep(0)

    # ── 2. Memory layers ──────────────────────────────────────────────────────
    layers = {
        "episodic":   ctx.episodic,
        "semantic":   ctx.semantic,
        "procedural": ctx.procedural,
        "meta":       ctx.meta,
        "quantum":    ctx.quantum,
        "creative":   ctx.creative,
        "governance": ctx.governance,
    }
    for layer_name, records in layers.items():
        yield _sse_event(
            {"layer": layer_name, "count": len(records), "records": records},
            event="layer",
        )
        await asyncio.sleep(0)

    # ── 3. Active invariants + project meta ───────────────────────────────────
    yield _sse_event(
        {
            "active_volumes":    ctx.active_volumes,
            "active_axioms":     ctx.active_axioms,
            "active_dualities":  ctx.active_dualities,
            "invariants":        ctx.invariants,
            "duality_pairs":     [dp.model_dump() for dp in ctx.duality_pairs],
            "related_projects":  ctx.related_projects,
            "project_meta":      ctx.project_meta.model_dump() if ctx.project_meta else None,
            "last_summary":      ctx.last_summary,
        },
        event="meta",
    )
    await asyncio.sleep(0)

    # ── 4. Done ───────────────────────────────────────────────────────────────
    yield _sse_event(
        {
            "total_memories": ctx.total_memories,
            "context_tokens": ctx.context_tokens,
            "generated_at":   ctx.generated_at.isoformat(),
        },
        event="done",
    )


@router.get(
    "/stream/context",
    tags=["Context"],
    summary="Stream context as Server-Sent Events",
)
async def stream_context(
    project:    str,
    llm:        str           = "perplexity",
    session_id: Optional[str] = None,
    top_k:      int           = 5,
):
    """
    Stream a ContextResponse as Server-Sent Events (SSE).

    The stream fires five event types in sequence:
      `start`  → session metadata
      `layer`  → one per memory type (7 total)
      `meta`   → active axioms, dualities, project meta
      `done`   → totals and timestamp

    Hook clients (perplexity_hook, supergrok_hook, etc.) should connect
    here instead of /context/active when context payloads are large.
    The streaming response allows the hook to begin system-prompt assembly
    before the full payload has arrived.

    Headers returned:
      Content-Type: text/event-stream
      Cache-Control: no-cache
      X-Accel-Buffering: no   ← disables Nginx proxy buffering for true SSE
    """
    headers = {
        "Cache-Control":    "no-cache",
        "X-Accel-Buffering": "no",
        "Connection":        "keep-alive",
    }
    return StreamingResponse(
        _context_sse_generator(
            project=project,
            llm=llm,
            session_id=session_id,
            top_k=top_k,
        ),
        media_type="text/event-stream",
        headers=headers,
    )


# ─────────────────────────────────────────────────────────────────────────────
# Structured Query
# ─────────────────────────────────────────────────────────────────────────────

@router.post("/query", tags=["Context"])
def query_memories(request: MemoryQuery) -> List[Dict]:
    """
    Execute a structured MemoryQuery across specified collections.

    Filters by project, memory type, similarity score floor, tags, and
    deprecation status. Returns results sorted by descending score.
    """
    results = engine.query(request)
    return [r.model_dump() for r in results]


# ─────────────────────────────────────────────────────────────────────────────
# Thread Summary
# ─────────────────────────────────────────────────────────────────────────────

class SummarizeRequest(BaseModel):
    session_id: str
    project:    str = "memoree"
    messages:   Optional[List[Dict]] = None


@router.post("/threads/summarize", tags=["Threads"])
def summarize_thread(req: SummarizeRequest) -> Dict:
    """
    Generate and store a compressed ThreadSummary for long-horizon context
    recovery.

    `summarize_thread` on the engine is a planned method — this route
    returns a 501 stub until the engine method is implemented.
    """
    if not hasattr(engine, "summarize_thread"):
        raise HTTPException(
            status_code=501,
            detail="summarize_thread is not yet implemented on MemoryEngine.",
        )
    summary: ThreadSummary = engine.summarize_thread(
        session_id=req.session_id,
        messages=req.messages,
    )
    return summary.model_dump()


# ─────────────────────────────────────────────────────────────────────────────
# LLM Session Sync
# ─────────────────────────────────────────────────────────────────────────────

class SyncRequest(BaseModel):
    llm:          str
    session_id:   str
    capabilities: Optional[Dict]  = None
    last_seen_ts: Optional[str]   = None


@router.post("/assistants/sync", tags=["Assistants"])
def sync_assistant_state(req: SyncRequest) -> Dict:
    """
    Persist LLM session state to disk under:
      ~/.memoree/llm_sync/{llm}/sessions/{session_id}.json

    Called by hook clients on session start/resume to register their
    capabilities and last-seen timestamp with the daemon.
    """
    sync_dir = (
        Path(os.path.expanduser("~"))
        / ".memoree"
        / "llm_sync"
        / req.llm
        / "sessions"
    )
    sync_dir.mkdir(parents=True, exist_ok=True)
    fp = sync_dir / f"{req.session_id}.json"

    now = _now_iso()
    payload = {
        "llm":          req.llm,
        "session_id":   req.session_id,
        "capabilities": req.capabilities or {},
        "last_seen_ts": req.last_seen_ts or now,
        "synced_at":    now,
    }
    fp.write_text(json.dumps(payload, indent=2), encoding="utf-8")
    log.info("[sync] llm=%s session=%s", req.llm, req.session_id)
    return {"status": "synced", "llm": req.llm, "session_id": req.session_id}


# ─────────────────────────────────────────────────────────────────────────────
# MCP — JSON-RPC 2.0 (LM Studio / Cursor / Claude Desktop)
# ─────────────────────────────────────────────────────────────────────────────

class _MCPContextArgs(BaseModel):
    project:    str
    llm:        Optional[str] = "perplexity"
    session_id: Optional[str] = None
    top_k:      Optional[int] = 5
    stream:     Optional[bool] = False


_MCP_TOOLS = [
    {
        "name":        "memoree_health",
        "description": "Check whether the Memoree daemon is alive and return version/uptime.",
        "inputSchema": {
            "type": "object",
            "properties": {},
            "additionalProperties": False,
        },
    },
    {
        "name":        "memoree_get_context",
        "description": (
            "Read the full active memory context for a project. Returns all "
            "seven memory layers, active axioms, active dualities, related "
            "projects, and ProjectMeta as a single JSON payload. "
            "Set stream=true to receive the response as an SSE stream URL instead."
        ),
        "inputSchema": {
            "type": "object",
            "properties": {
                "project":    {"type": "string",  "description": "Canonical project key, e.g. 'memoree', 'ftqc', 'rae'."},
                "llm":        {"type": "string",  "description": "LLM provider identifier.", "default": "perplexity"},
                "session_id": {"type": "string",  "description": "Optional session UUID for episodic scoping."},
                "top_k":      {"type": "integer", "description": "Max results per memory collection.", "default": 5},
                "stream":     {"type": "boolean", "description": "If true, returns SSE stream URL instead of inline JSON.", "default": False},
            },
            "required": ["project"],
            "additionalProperties": False,
        },
    },
    {
        "name":        "memoree_list_projects",
        "description": "List all projects registered in projects.json with their metadata.",
        "inputSchema": {
            "type": "object",
            "properties": {},
            "additionalProperties": False,
        },
    },
    {
        "name":        "memoree_diagnostics",
        "description": "Return a live MemoreeDiagnostics snapshot: uptime, memory counts, Qdrant status, active hooks.",
        "inputSchema": {
            "type": "object",
            "properties": {},
            "additionalProperties": False,
        },
    },
]


def _jsonrpc_ok(req_id: Any, result: Any) -> JSONResponse:
    return JSONResponse({"jsonrpc": "2.0", "id": req_id, "result": result})


def _jsonrpc_err(req_id: Any, code: int, message: str) -> JSONResponse:
    return JSONResponse(
        {"jsonrpc": "2.0", "id": req_id, "error": {"code": code, "message": message}},
        status_code=200,
    )


@router.post("/mcp", tags=["MCP"])
async def memoree_mcp(request: Request):
    """
    MCP-compatible JSON-RPC 2.0 endpoint for LM Studio, Cursor, and Claude Desktop.

    Supported methods:
      initialize               → server capabilities handshake
      notifications/initialized → ack (202 No Content)
      ping                     → keepalive
      tools/list               → enumerate available tools
      tools/call               → invoke a named tool

    Available tools:
      memoree_health           → daemon liveness + version
      memoree_get_context      → full context payload (JSON or SSE stream URL)
      memoree_list_projects    → project registry
      memoree_diagnostics      → live diagnostics snapshot

    Error codes follow JSON-RPC 2.0:
      -32700 Parse error
      -32601 Method not found
      -32000 Tool execution error
    """
    # ── Parse ─────────────────────────────────────────────────────────────────
    try:
        msg = await request.json()
    except Exception:
        return _jsonrpc_err(None, -32700, "Parse error — request body is not valid JSON.")

    method  = msg.get("method")
    req_id  = msg.get("id")
    params  = msg.get("params") or {}

    log.debug("[mcp] method=%s id=%s", method, req_id)

    # ── initialize ────────────────────────────────────────────────────────────
    if method == "initialize":
        proto = params.get("protocolVersion", "2025-03-26")
        return _jsonrpc_ok(
            req_id,
            {
                "protocolVersion": proto,
                "capabilities": {
                    "tools":     {"listChanged": False},
                    "streaming": {"sse": True},
                },
                "serverInfo": {"name": "memoree", "version": "0.1.0"},
                "instructions": (
                    "Memoree is the sovereign memory substrate for the Aurphyx LLC "
                    "ecosystem. Use memoree_get_context with a project key to retrieve "
                    "full memory context before answering. Use stream=true for large "
                    "sessions. Available projects: "
                    + ", ".join(engine.projects.keys())
                ),
            },
        )

    if method == "notifications/initialized":
        return Response(status_code=202)

    # ── ping ──────────────────────────────────────────────────────────────────
    if method == "ping":
        return _jsonrpc_ok(req_id, {})

    # ── tools/list ────────────────────────────────────────────────────────────
    if method == "tools/list":
        return _jsonrpc_ok(req_id, {"tools": _MCP_TOOLS})

    # ── tools/call ────────────────────────────────────────────────────────────
    if method == "tools/call":
        name      = params.get("name")
        arguments = params.get("arguments") or {}

        # memoree_health
        if name == "memoree_health":
            diag = engine.diagnostics()
            return _jsonrpc_ok(
                req_id,
                {
                    "content": [
                        {
                            "type": "text",
                            "text": (
                                f"Memoree alive — version 0.1.0 | "
                                f"uptime {diag.uptime_seconds:.1f}s | "
                                f"status {diag.status} | "
                                f"total_memories {diag.total_memories}"
                            ),
                        }
                    ]
                },
            )

        # memoree_get_context
        if name == "memoree_get_context":
            try:
                args = _MCPContextArgs(**arguments)
            except Exception as exc:
                return _jsonrpc_err(req_id, -32000, f"Invalid arguments: {exc}")

            # stream=True → return the SSE endpoint URL instead of inline payload
            if args.stream:
                sse_url = (
                    f"http://127.0.0.1:7042/stream/context"
                    f"?project={args.project}"
                    f"&llm={args.llm or 'perplexity'}"
                    + (f"&session_id={args.session_id}" if args.session_id else "")
                    + f"&top_k={args.top_k or 5}"
                )
                return _jsonrpc_ok(
                    req_id,
                    {
                        "content": [
                            {
                                "type": "text",
                                "text": f"SSE stream available at: {sse_url}",
                            },
                            {
                                "type": "resource",
                                "resource": {
                                    "uri":      sse_url,
                                    "mimeType": "text/event-stream",
                                    "text":     "Memoree context SSE stream",
                                },
                            },
                        ]
                    },
                )

            # stream=False → inline JSON payload
            try:
                ctx = engine.read_context(
                    project=args.project,
                    llm=args.llm or "perplexity",
                    session_id=args.session_id,
                    top_k=args.top_k or 5,
                )
                return _jsonrpc_ok(
                    req_id,
                    {
                        "content": [
                            {
                                "type": "text",
                                "text": ctx.model_dump_json(indent=2),
                            }
                        ]
                    },
                )
            except Exception as exc:
                log.error("[mcp] memoree_get_context failed: %s", exc)
                return _jsonrpc_err(req_id, -32000, f"Tool execution failed: {exc}")

        # memoree_list_projects
        if name == "memoree_list_projects":
            projects_list = [p.model_dump() for p in engine.list_projects()]
            return _jsonrpc_ok(
                req_id,
                {
                    "content": [
                        {
                            "type": "text",
                            "text": json.dumps(projects_list, indent=2, default=str),
                        }
                    ]
                },
            )

        # memoree_diagnostics
        if name == "memoree_diagnostics":
            diag = engine.diagnostics()
            return _jsonrpc_ok(
                req_id,
                {
                    "content": [
                        {
                            "type": "text",
                            "text": diag.model_dump_json(indent=2),
                        }
                    ]
                },
            )

        return _jsonrpc_err(req_id, -32601, f"Unknown tool: '{name}'")

    # ── fallback ──────────────────────────────────────────────────────────────
    return _jsonrpc_err(req_id, -32601, f"Method not found: '{method}'")