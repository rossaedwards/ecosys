"""Memoree v3 — FastAPI daemon with WebSocket /ws/memory.

REST endpoints:
  POST /write_event        — persist a memory
  POST /read_context       — hybrid semantic recall
  GET  /query_context      — same via query params
  GET  /list_events        — raw ledger dump
  GET  /health             — liveness + backend info

WebSocket:
  WS  /ws/memory           — real-time read/write for agents
    actions: write | read | ping
"""
from __future__ import annotations
import json
from contextlib import asynccontextmanager
from typing import Optional

from fastapi import FastAPI, HTTPException, Query, WebSocket, WebSocketDisconnect
from fastapi.middleware.cors import CORSMiddleware

from .schemas import (
    MemoryEvent, ContextQuery, ContextResult,
    WriteResponse, ReadResponse, WSIncoming, WSOutgoing,
)
from .ledger        import init_db, insert_event, list_events, keyword_search
from .vector_backend import upsert, vector_search, active_backend
from .embedder       import embed

# Optional integrations — imported lazily
try:
    from ..memos.memos_overlay import mirror_event, recall_from_memos
    _MEMOS_AVAILABLE = True
except ImportError:
    _MEMOS_AVAILABLE = False

try:
    from ..sync.powersync_client import enqueue as ps_enqueue, start_sync_daemon
    _SYNC_AVAILABLE = True
except ImportError:
    _SYNC_AVAILABLE = False


@asynccontextmanager
async def lifespan(app: FastAPI):
    await init_db()
    if _SYNC_AVAILABLE:
        start_sync_daemon()
    yield


app = FastAPI(title="Memoree v3", version="3.1.0", lifespan=lifespan)
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_methods=["*"],
    allow_headers=["*"],
)


# ─────────────────────────────────────────────
# Helpers
# ─────────────────────────────────────────────

async def _do_write(event: MemoryEvent) -> WriteResponse:
    event_dict = event.model_dump()
    event_dict["timestamp"] = event.timestamp.isoformat()
    event_dict["tags"]      = event.tags
    # Embed
    vecs               = embed([event.content])
    event_dict["embedding"] = vecs[0]
    # SQLite ledger
    await insert_event(event_dict)
    # Vector store
    await upsert(event.id, vecs[0], event_dict)
    # Optional mirrors
    if _MEMOS_AVAILABLE:
        mirror_event(event_dict)
    if _SYNC_AVAILABLE:
        ps_enqueue(event_dict)
    return WriteResponse(id=event.id, status="ok")


async def _do_read(query: ContextQuery) -> ReadResponse:
    qvec = embed([query.query])[0]
    vec_results = await vector_search(
        qvec,
        llm=query.llm,
        type_=query.type,
        project=query.project,
        tags=query.tags,
        min_confidence=query.min_confidence,
        min_priority=query.min_priority,
        top_k=query.top_k,
    )
    # Keyword fallback
    kw_results = []
    if len(vec_results) < query.top_k:
        kw_results = await keyword_search(query.query, llm=query.llm, limit=query.top_k)
    seen = {r["id"] for r in vec_results}
    merged = vec_results[:]
    for kw in kw_results:
        if kw["id"] not in seen:
            merged.append({"id": kw["id"], "content": kw["content"], "score": 0.5, "payload": kw})
    # Also pull MemOS enrichment if enabled
    if _MEMOS_AVAILABLE:
        memos_hits = recall_from_memos(query.query, top_k=3)
        for mh in memos_hits:
            if mh.get("id") not in seen:
                merged.append({"id": mh.get("id", ""), "content": mh.get("content", ""), "score": mh.get("score", 0.4), "payload": mh})
    # Hydrate from SQLite
    raw_events = await list_events(limit=500)
    event_map  = {e["id"]: e for e in raw_events}
    results    = []
    for r in merged[: query.top_k]:
        ev_raw = event_map.get(r["id"]) or r.get("payload", {})
        if not ev_raw:
            continue
        try:
            if isinstance(ev_raw.get("tags"), str):
                ev_raw["tags"] = json.loads(ev_raw["tags"])
            if isinstance(ev_raw.get("meta"), str):
                ev_raw["meta"] = json.loads(ev_raw["meta"])
            results.append(ContextResult(
                event=MemoryEvent(**ev_raw),
                score=r["score"],
                source="hybrid" if r["score"] < 1.0 else "vector",
            ))
        except Exception:
            continue
    return ReadResponse(results=results, total=len(results), query=query.query, backend=active_backend())


# ─────────────────────────────────────────────
# REST routes
# ─────────────────────────────────────────────

@app.get("/health")
async def health():
    return {"status": "alive", "version": "3.1.0", "backend": active_backend()}


@app.post("/write_event", response_model=WriteResponse)
async def write_event(event: MemoryEvent):
    try:
        return await _do_write(event)
    except Exception as exc:
        raise HTTPException(status_code=500, detail=str(exc)) from exc


@app.post("/read_context", response_model=ReadResponse)
async def read_context(query: ContextQuery):
    try:
        return await _do_read(query)
    except Exception as exc:
        raise HTTPException(status_code=500, detail=str(exc)) from exc


@app.get("/query_context", response_model=ReadResponse)
async def query_context(
    q:       str           = Query(...),
    llm:     Optional[str] = None,
    project: Optional[str] = None,
    top_k:   int           = 8,
):
    return await _do_read(ContextQuery(query=q, llm=llm, project=project, top_k=top_k))


@app.get("/list_events")
async def list_events_route(
    llm:    Optional[str] = None,
    type_:  Optional[str] = None,
    project: Optional[str] = None,
    limit:  int = 50,
):
    rows = await list_events(llm=llm, type_=type_, project=project, limit=limit)
    return {"events": rows, "total": len(rows), "backend": active_backend()}


# ─────────────────────────────────────────────
# WebSocket /ws/memory
# ─────────────────────────────────────────────

@app.websocket("/ws/memory")
async def ws_memory(websocket: WebSocket):
    """Real-time memory socket for agents.

    Send JSON: {"action": "write", "payload": {<MemoryEvent fields>}}
    Send JSON: {"action": "read",  "payload": {<ContextQuery fields>}}
    Send JSON: {"action": "ping"}
    """
    await websocket.accept()
    try:
        while True:
            raw  = await websocket.receive_text()
            msg  = WSIncoming.model_validate_json(raw)
            out: WSOutgoing
            if msg.action == "ping":
                out = WSOutgoing(action="pong", data={"status": "alive"})
            elif msg.action == "write":
                event = MemoryEvent(**msg.payload)
                resp  = await _do_write(event)
                out   = WSOutgoing(action="write_ack", data=resp.model_dump())
            elif msg.action == "read":
                query  = ContextQuery(**msg.payload)
                result = await _do_read(query)
                out    = WSOutgoing(action="read_result", data=result.model_dump())
            else:
                out = WSOutgoing(action="error", data={}, ok=False, error=f"Unknown action: {msg.action}")
            await websocket.send_text(out.model_dump_json())
    except WebSocketDisconnect:
        pass
    except Exception as exc:
        err = WSOutgoing(action="error", data={}, ok=False, error=str(exc))
        try:
            await websocket.send_text(err.model_dump_json())
        except Exception:
            pass
