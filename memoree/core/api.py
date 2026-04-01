"""Memoree v3 — FastAPI daemon.

Endpoints (high-level API — stable across v3 swaps):
  POST /write_event       — write a memory event
  POST /read_context      — semantic search (vector + keyword hybrid)
  GET  /query_context     — alias with GET params
  GET  /list_events       — list raw events with filters
  GET  /health            — heartbeat
"""
from __future__ import annotations
from contextlib import asynccontextmanager
from typing import Optional

from fastapi import FastAPI, HTTPException, Query
from fastapi.middleware.cors import CORSMiddleware

from .schemas import MemoryEvent, ContextQuery, ContextResult, WriteResponse, ReadResponse
from .ledger import init_db, insert_event, list_events, keyword_search
from .vector_backend import upsert, vector_search
from .embedder import embed


@asynccontextmanager
async def lifespan(app: FastAPI):
    await init_db()
    yield


app = FastAPI(title="Memoree v3", version="3.0.0", lifespan=lifespan)
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_methods=["*"],
    allow_headers=["*"],
)


@app.get("/health")
async def health():
    return {"status": "alive", "version": "3.0.0"}


@app.post("/write_event", response_model=WriteResponse)
async def write_event(event: MemoryEvent):
    """Persist a memory event to SQLite ledger + vector store."""
    try:
        event_dict = event.model_dump()
        event_dict["timestamp"] = event.timestamp.isoformat()
        # embed
        vecs = embed([event.content])
        event_dict["embedding"] = vecs[0]
        # write to SQLite
        await insert_event(event_dict)
        # write to vector backend
        await upsert(event.id, vecs[0], event_dict)
        return WriteResponse(id=event.id, status="ok")
    except Exception as exc:
        raise HTTPException(status_code=500, detail=str(exc)) from exc


@app.post("/read_context", response_model=ReadResponse)
async def read_context(query: ContextQuery):
    """Semantic recall: hybrid vector + keyword search."""
    try:
        qvec = embed([query.query])[0]
        vec_results = await vector_search(
            qvec, llm=query.llm, type_=query.type, top_k=query.top_k
        )
        # enrich with keyword fallback if sparse
        kw_results = []
        if len(vec_results) < query.top_k:
            kw_results = await keyword_search(query.query, llm=query.llm, limit=query.top_k)
        seen_ids = {r["id"] for r in vec_results}
        merged = vec_results[:]
        for kw in kw_results:
            if kw["id"] not in seen_ids:
                merged.append({"id": kw["id"], "content": kw["content"], "score": 0.5})
        raw_events = await list_events(llm=query.llm, type_=query.type, limit=100)
        event_map = {e["id"]: e for e in raw_events}
        results = []
        for r in merged[: query.top_k]:
            ev_raw = event_map.get(r["id"])
            if ev_raw:
                import json
                ev_raw["tags"] = json.loads(ev_raw.get("tags", "[]"))
                ev_raw["meta"] = json.loads(ev_raw.get("meta", "{}"))
                results.append(ContextResult(event=MemoryEvent(**ev_raw), score=r["score"]))
        return ReadResponse(results=results, total=len(results), query=query.query)
    except Exception as exc:
        raise HTTPException(status_code=500, detail=str(exc)) from exc


@app.get("/query_context", response_model=ReadResponse)
async def query_context(
    q: str = Query(..., description="Natural-language query"),
    llm: Optional[str] = None,
    top_k: int = 5,
):
    return await read_context(ContextQuery(query=q, llm=llm, top_k=top_k))


@app.get("/list_events")
async def list_events_route(
    llm: Optional[str] = None,
    type_: Optional[str] = None,
    limit: int = 50,
):
    rows = await list_events(llm=llm, type_=type_, limit=limit)
    return {"events": rows, "total": len(rows)}
