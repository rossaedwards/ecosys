## ** APS-TSLCA-MEMOREE-ROUTES **
## ** Memoree - Sovereign Memory Substrate **
## ** Symbiotic Universal Xessability Standards **
## ** Three-Squared-Lattice Cognitive Architecture **
## ** Aurphyx Primordial Standard **
## ** Aurphyx LLC **
## ** SAGES | Proprietary | Pro-Existence **
## ** Accessibility = Xessability **
## ** Version 4.0 **

"""
Memoree — FastAPI Routes (TSLCA 9-Cell Lattice, Dashboard & MCP Protocol)
═══════════════════════════════════════════════════════════════════════════════
Local-only REST + SSE streaming API on 127.0.0.1:7042.
Web & TUI Dashboard at /dashboard with Qdrant Dashboard launcher.
MCP JSON-RPC 2.0 endpoint for LM Studio / Cursor / Claude Desktop / Hermes.
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
from fastapi.responses import FileResponse, JSONResponse, Response, StreamingResponse
from pydantic import BaseModel

from credentials_manager import credentials
from hooks_registry import get_hook, list_available_hooks
from memory_engine import MemoryEngine
from rcl_engine import rcl_engine
from schemas import (
    BulkUpsertRequest,
    ContextResponse,
    CreativeMemory,
    EpisodicMemory,
    GovernanceMemory,
    IdentityMemory,
    MemoryQuery,
    MetaMemory,
    ProceduralMemory,
    QuantumMemory,
    SemanticMemory,
    SensoryMemory,
    ThreadSummary,
    UpsertMemoryRequest,
    WorkingMemory,
)
from tsl_memory_kernel import calculate_hif, evaluate_gate

log = logging.getLogger("memoree.routes")
router = APIRouter()
engine = MemoryEngine()

STATIC_DASHBOARD_DIR = Path(__file__).parent / "static" / "dashboard"


# ─────────────────────────────────────────────────────────────────────────────
# Helpers
# ─────────────────────────────────────────────────────────────────────────────

def _now_iso() -> str:
    return datetime.now(tz=timezone.utc).isoformat()


def _sse_event(data: Any, event: str = "message") -> str:
    payload = data if isinstance(data, str) else json.dumps(data, default=str)
    return f"event: {event}\ndata: {payload}\n\n"


# ─────────────────────────────────────────────────────────────────────────────
# Web Dashboard Endpoints
# ─────────────────────────────────────────────────────────────────────────────

@router.get("/dashboard", tags=["Dashboard"])
def get_dashboard():
    """Serve the Memoree Web Dashboard."""
    index_path = STATIC_DASHBOARD_DIR / "index.html"
    if not index_path.exists():
        raise HTTPException(status_code=404, detail="Dashboard index.html not found.")
    return FileResponse(index_path)


@router.get("/dashboard/{filename:path}", tags=["Dashboard"])
def get_dashboard_asset(filename: str):
    """Serve static dashboard assets (css, js, images)."""
    asset_path = STATIC_DASHBOARD_DIR / filename
    if not asset_path.exists():
        raise HTTPException(status_code=404, detail=f"Asset '{filename}' not found.")
    return FileResponse(asset_path)


# ─────────────────────────────────────────────────────────────────────────────
# System, Diagnostics & Lattice Field
# ─────────────────────────────────────────────────────────────────────────────

@router.get("/health", tags=["System"])
def health() -> Dict:
    """Daemon liveness check."""
    return {
        "status": "alive",
        "service": "memoree",
        "version": "4.0.0",
        "lattice": "TSLCA 3x3",
        "timestamp": _now_iso(),
    }


@router.get("/diagnostics", tags=["System"])
def diagnostics():
    """Live MemoreeDiagnostics snapshot."""
    return engine.diagnostics()


@router.get("/lattice", tags=["Lattice"])
def get_lattice():
    """Return live 3x3 Cognitive Field Tensor snapshot."""
    return engine.get_lattice_snapshot()


@router.get("/hif", tags=["Lattice"])
def get_hif(coherence: float = 0.85, resonance: float = 0.90, alignment: float = 0.88):
    """Calculate live Harmonic Integrity Field and gate evaluation."""
    hif_val = calculate_hif(coherence, resonance, alignment)
    can_create, create_msg = evaluate_gate(hif_val, "create")
    can_integrate, int_msg = evaluate_gate(hif_val, "integrate")
    can_renew, renew_msg = evaluate_gate(hif_val, "renew")

    return {
        "coherence": coherence,
        "resonance": resonance,
        "alignment": alignment,
        "hif": round(hif_val, 4),
        "gates": {
            "create": {"permitted": can_create, "reason": create_msg},
            "integrate": {"permitted": can_integrate, "reason": int_msg},
            "renew": {"permitted": can_renew, "reason": renew_msg},
        },
        "timestamp": _now_iso(),
    }


# ─────────────────────────────────────────────────────────────────────────────
# Credentials & Model Hooks Management APIs
# ─────────────────────────────────────────────────────────────────────────────

class SetAuthRequest(BaseModel):
    provider: str
    api_key: Optional[str] = None
    base_url: Optional[str] = None
    default_model: Optional[str] = None


@router.get("/api/hooks", tags=["Hooks"])
def list_hooks() -> List[Dict]:
    """Return configured status and default models for all hooks."""
    return list_available_hooks()


@router.post("/api/auth/set", tags=["Auth"])
def set_auth(req: SetAuthRequest) -> Dict:
    """Set provider credentials."""
    credentials.set_key(
        provider=req.provider,
        api_key=req.api_key or "",
        base_url=req.base_url,
        default_model=req.default_model,
    )
    return {"status": "saved", "provider": req.provider}


@router.post("/api/auth/test", tags=["Auth"])
async def test_hook_auth(provider: str) -> Dict:
    """Test a provider's live connection."""
    try:
        hook = get_hook(provider)
        if not hook.is_configured():
            return {"status": "unconfigured", "error": "API key is not configured"}
        res = await hook.generate_async(
            prompt="Ping test from Memoree Console.",
            inject_memory=False,
        )
        return {
            "status": "success",
            "provider": provider,
            "latency_ms": res.get("latency_ms", 0.0),
            "model": res.get("model", ""),
        }
    except Exception as e:
        return {"status": "error", "error": str(e)}


# ─────────────────────────────────────────────────────────────────────────────
# Rituals, Chains, Links (RCL) APIs
# ─────────────────────────────────────────────────────────────────────────────

class RunRCLRequest(BaseModel):
    rcl_type: str  # link | chain | ritual | fork
    spec_id: str
    input_text: Optional[str] = None
    project: str = "memoree"


@router.get("/api/rcl", tags=["RCL"])
def get_rcl_manifest() -> Dict:
    """Return all registered Links, Chains, Rituals, and Forks."""
    return rcl_engine.get_manifest()


@router.post("/api/rcl/run", tags=["RCL"])
async def run_rcl(req: RunRCLRequest) -> Dict:
    """Execute a Link, Chain, Ritual, or Fork."""
    if req.rcl_type == "link":
        result = await rcl_engine.execute_link(req.spec_id, req.input_text or "", req.project)
        return result.model_dump()
    elif req.rcl_type == "chain":
        result = await rcl_engine.execute_chain(req.spec_id, req.input_text or "", req.project)
        return result.model_dump()
    elif req.rcl_type == "ritual":
        result = await rcl_engine.execute_ritual(req.spec_id, req.project)
        return result.model_dump()
    elif req.rcl_type == "fork":
        fork = await rcl_engine.execute_fork(req.input_text or "Concept", project=req.project)
        return fork.model_dump()
    else:
        raise HTTPException(status_code=400, detail=f"Unknown RCL type: {req.rcl_type}")


# ─────────────────────────────────────────────────────────────────────────────
# Project Registry
# ─────────────────────────────────────────────────────────────────────────────

@router.get("/projects", tags=["Projects"])
def list_projects() -> List[Dict]:
    """List all projects registered in projects.json."""
    return [p.model_dump() for p in engine.list_projects()]


@router.get("/projects/{key}", tags=["Projects"])
def get_project(key: str) -> Dict:
    """Retrieve a single ProjectMeta by its canonical key."""
    meta = engine.get_project(key)
    if meta is None:
        raise HTTPException(status_code=404, detail=f"Project '{key}' not found in registry.")
    return meta.model_dump()


# ─────────────────────────────────────────────────────────────────────────────
# 9-Cell Memory Write Endpoints
# ─────────────────────────────────────────────────────────────────────────────

@router.post("/memories/sensory", tags=["Memory"])
def write_sensory(mem: SensoryMemory) -> Dict:
    """Persist SensoryMemory (SIX ⊗ SIX — perception & resonance)."""
    mem_id = engine.write_sensory(mem)
    return {"id": mem_id, "status": "stored", "type": "sensory", "cell": "SIX⊗SIX"}


@router.post("/memories/working", tags=["Memory"])
def write_working(mem: WorkingMemory) -> Dict:
    """Persist WorkingMemory (SIX ⊗ SCX — active context buffer)."""
    mem_id = engine.write_working(mem)
    return {"id": mem_id, "status": "stored", "type": "working", "cell": "SIX⊗SCX"}


@router.post("/memories/events", tags=["Memory"])
def write_event(mem: EpisodicMemory) -> Dict:
    """Persist EpisodicMemory (SIX ⊗ ICX — conversation turn)."""
    mem_id = engine.write_event(mem)
    return {"id": mem_id, "status": "stored", "type": "episodic", "cell": "SIX⊗ICX"}


@router.post("/memories/semantic", tags=["Memory"])
def embed_document(mem: SemanticMemory) -> Dict:
    """Embed and persist SemanticMemory (SCX ⊗ SIX — project knowledge)."""
    mem_id = engine.embed_document(mem)
    return {"id": mem_id, "status": "stored", "type": "semantic", "cell": "SCX⊗SIX"}


@router.post("/memories/meta", tags=["Memory"])
def store_fact(mem: MetaMemory) -> Dict:
    """Persist MetaMemory (SCX ⊗ SCX — verified invariant fact)."""
    mem_id = engine.store_fact(mem)
    return {"id": mem_id, "status": "stored", "type": "meta", "cell": "SCX⊗SCX"}


@router.post("/memories/quantum", tags=["Memory"])
def store_quantum(mem: QuantumMemory) -> Dict:
    """Store QuantumMemory (SCX ⊗ ICX — simulation & physics state)."""
    mem_id = engine.store_quantum(mem)
    return {"id": mem_id, "status": "stored", "type": "quantum", "cell": "SCX⊗ICX"}


@router.post("/memories/identity", tags=["Memory"])
def write_identity(mem: IdentityMemory) -> Dict:
    """Persist IdentityMemory (ICX ⊗ SIX — SoulJourney pipeline state)."""
    mem_id = engine.write_identity(mem)
    return {"id": mem_id, "status": "stored", "type": "identity", "cell": "ICX⊗SIX"}


@router.post("/memories/procedural", tags=["Memory"])
def store_workflow(mem: ProceduralMemory) -> Dict:
    """Store ProceduralMemory (ICX ⊗ SCX — repeatable workflow)."""
    mem_id = engine.store_workflow(mem)
    return {"id": mem_id, "status": "stored", "type": "procedural", "cell": "ICX⊗SCX"}


@router.post("/memories/governance", tags=["Memory"])
def store_governance(mem: GovernanceMemory) -> Dict:
    """Store GovernanceMemory (ICX ⊗ ICX — vote, policy, mandate, Archivus)."""
    mem_id = engine.store_governance(mem)
    return {"id": mem_id, "status": "stored", "type": "governance", "cell": "ICX⊗ICX"}


@router.post("/memories/creative", tags=["Memory"])
def store_creative(mem: CreativeMemory) -> Dict:
    """Store CreativeMemory (media / narrative entry)."""
    mem_id = engine.store_creative(mem)
    return {"id": mem_id, "status": "stored", "type": "creative"}


@router.post("/memories/upsert", tags=["Memory"])
def upsert_memory(request: UpsertMemoryRequest) -> Dict:
    """Generic typed upsert across any of the 9 TSLCA memory classes."""
    try:
        mem_id = engine.upsert(request)
    except ValueError as exc:
        raise HTTPException(status_code=422, detail=str(exc)) from exc
    return {"id": mem_id, "status": "stored", "type": request.memory_type}


@router.post("/memories/bulk", tags=["Memory"])
def bulk_upsert(request: BulkUpsertRequest) -> Dict:
    """Batch upsert of multiple memory records."""
    return engine.bulk_upsert(request)


@router.post("/working/cure", tags=["Memory"])
def cure_working() -> Dict:
    """Cure active working memories into permanent layers."""
    return engine.cure_working_buffer()


# ─────────────────────────────────────────────────────────────────────────────
# Context Read — JSON + SSE Stream
# ─────────────────────────────────────────────────────────────────────────────

@router.get(
    "/context/active",
    response_model=ContextResponse,
    tags=["Context"],
)
def read_context(
    project: str,
    llm: str = "perplexity",
    session_id: Optional[str] = None,
    top_k: int = 5,
):
    """Assemble and contract the full 9-cell ContextResponse for a project."""
    return engine.read_context(
        project=project,
        llm=llm,
        session_id=session_id,
        top_k=top_k,
    )


async def _context_sse_generator(
    project: str,
    llm: str,
    session_id: Optional[str],
    top_k: int,
) -> AsyncGenerator[str, None]:
    yield _sse_event({"phase": "init", "project": project, "timestamp": _now_iso()}, event="start")
    await asyncio.sleep(0.01)

    ctx = engine.read_context(project=project, llm=llm, session_id=session_id, top_k=top_k)
    yield _sse_event(ctx.model_dump(), event="context")
    yield _sse_event({"phase": "complete", "total_memories": ctx.total_memories}, event="done")


@router.get("/stream/context", tags=["Context"])
async def stream_context(
    project: str,
    llm: str = "perplexity",
    session_id: Optional[str] = None,
    top_k: int = 5,
):
    """Stream ContextResponse chunks via Server-Sent Events (SSE)."""
    return StreamingResponse(
        _context_sse_generator(project, llm, session_id, top_k),
        media_type="text/event-stream",
    )


# ─────────────────────────────────────────────────────────────────────────────
# Query Endpoint
# ─────────────────────────────────────────────────────────────────────────────

@router.post("/query", tags=["Search"])
def query_memories(query: MemoryQuery) -> List[Dict]:
    """Execute a structured search across the TSLCA collections."""
    results = engine.query(query)
    return [r.model_dump() for r in results]


# ─────────────────────────────────────────────────────────────────────────────
# MCP — JSON-RPC 2.0 (Claude Desktop / Cursor / Hermes / LM Studio)
# ─────────────────────────────────────────────────────────────────────────────

class _MCPContextArgs(BaseModel):
    project: str
    llm: Optional[str] = "claude"
    session_id: Optional[str] = None
    top_k: Optional[int] = 5
    stream: Optional[bool] = False


_MCP_TOOLS = [
    {
        "name": "memoree_health",
        "description": "Check whether the Memoree daemon is alive and return version/uptime.",
        "inputSchema": {"type": "object", "properties": {}, "additionalProperties": False},
    },
    {
        "name": "memoree_get_context",
        "description": "Read full 9-cell TSLCA memory context for a project.",
        "inputSchema": {
            "type": "object",
            "properties": {
                "project": {"type": "string", "description": "Project key."},
                "llm": {"type": "string", "default": "claude"},
                "session_id": {"type": "string"},
                "top_k": {"type": "integer", "default": 5},
                "stream": {"type": "boolean", "default": False},
            },
            "required": ["project"],
            "additionalProperties": False,
        },
    },
    {
        "name": "memoree_lattice_snapshot",
        "description": "Retrieve the live 3x3 Cognitive Field Tensor state across all 9 cells.",
        "inputSchema": {"type": "object", "properties": {}, "additionalProperties": False},
    },
    {
        "name": "memoree_rcl_manifest",
        "description": "List all registered Links, Chains, Rituals, and Forks in the g0dm0d3 engine.",
        "inputSchema": {"type": "object", "properties": {}, "additionalProperties": False},
    },
    {
        "name": "memoree_list_projects",
        "description": "List all projects registered in projects.json with their metadata.",
        "inputSchema": {"type": "object", "properties": {}, "additionalProperties": False},
    },
    {
        "name": "memoree_diagnostics",
        "description": "Return live MemoreeDiagnostics snapshot.",
        "inputSchema": {"type": "object", "properties": {}, "additionalProperties": False},
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
    """MCP JSON-RPC 2.0 endpoint."""
    try:
        msg = await request.json()
    except Exception:
        return _jsonrpc_err(None, -32700, "Parse error.")

    method = msg.get("method")
    req_id = msg.get("id")
    params = msg.get("params") or {}

    if method == "initialize":
        proto = params.get("protocolVersion", "2025-03-26")
        return _jsonrpc_ok(
            req_id,
            {
                "protocolVersion": proto,
                "capabilities": {"tools": {"listChanged": False}, "streaming": {"sse": True}},
                "serverInfo": {"name": "memoree", "version": "4.0.0"},
                "instructions": "Memoree is the sovereign 9-cell memory substrate. Use memoree_get_context for full context.",
            },
        )

    if method == "notifications/initialized":
        return Response(status_code=202)

    if method == "ping":
        return _jsonrpc_ok(req_id, {})

    if method == "tools/list":
        return _jsonrpc_ok(req_id, {"tools": _MCP_TOOLS})

    if method == "tools/call":
        name = params.get("name")
        arguments = params.get("arguments") or {}

        if name == "memoree_health":
            diag = engine.diagnostics()
            return _jsonrpc_ok(
                req_id,
                {"content": [{"type": "text", "text": f"Memoree v4.0.0 alive | status: {diag.status}"}]},
            )

        if name == "memoree_lattice_snapshot":
            snapshot = engine.get_lattice_snapshot()
            return _jsonrpc_ok(
                req_id,
                {"content": [{"type": "text", "text": json.dumps(snapshot, indent=2)}]},
            )

        if name == "memoree_rcl_manifest":
            manifest = rcl_engine.get_manifest()
            return _jsonrpc_ok(
                req_id,
                {"content": [{"type": "text", "text": json.dumps(manifest, indent=2)}]},
            )

        if name == "memoree_get_context":
            try:
                args = _MCPContextArgs(**arguments)
                ctx = engine.read_context(
                    project=args.project,
                    llm=args.llm or "claude",
                    session_id=args.session_id,
                    top_k=args.top_k or 5,
                )
                return _jsonrpc_ok(
                    req_id,
                    {"content": [{"type": "text", "text": ctx.model_dump_json(indent=2)}]},
                )
            except Exception as exc:
                return _jsonrpc_err(req_id, -32000, f"Execution failed: {exc}")

        if name == "memoree_list_projects":
            projects_list = [p.model_dump() for p in engine.list_projects()]
            return _jsonrpc_ok(
                req_id,
                {"content": [{"type": "text", "text": json.dumps(projects_list, indent=2, default=str)}]},
            )

        if name == "memoree_diagnostics":
            diag = engine.diagnostics()
            return _jsonrpc_ok(
                req_id,
                {"content": [{"type": "text", "text": diag.model_dump_json(indent=2)}]},
            )

        return _jsonrpc_err(req_id, -32601, f"Unknown tool: '{name}'")

    return _jsonrpc_err(req_id, -32601, f"Method not found: '{method}'")