"""
Aurphyx_Memoree — FastAPI Routes
Local-only API on 127.0.0.1:7042
f0rg3d in l0v3 by Ross Edwards
"""

from fastapi import APIRouter, HTTPException
from typing import Dict, List, Optional
from pydantic import BaseModel

from core.schemas import (
    EpisodicMemory, SemanticMemory, ProceduralMemory, MetaMemory,
    ContextResponse, ThreadSummary
)
from core.memory_engine import MemoryEngine

router = APIRouter()
engine = MemoryEngine()


# ── Health ───────────────────────────────────────────────────────────────────

@router.get("/health")
def health():
    return {"status": "❤️‍🔥 Aurphyx_Memoree Daemon alive", "version": "0.1.0"}


# ── Episodic ─────────────────────────────────────────────────────────────────

@router.post("/memories/events", summary="Write episodic event")
def write_event(mem: EpisodicMemory) -> Dict:
    mem_id = engine.write_event(mem)
    return {"id": mem_id, "status": "stored", "type": "episodic"}


# ── Semantic ─────────────────────────────────────────────────────────────────

@router.post("/memories/semantic", summary="Embed semantic document")
def embed_document(mem: SemanticMemory) -> Dict:
    mem_id = engine.embed_document(mem)
    return {"id": mem_id, "status": "stored", "type": "semantic"}


# ── Procedural ───────────────────────────────────────────────────────────────

@router.post("/memories/procedural", summary="Store workflow")
def store_workflow(mem: ProceduralMemory) -> Dict:
    mem_id = engine.store_workflow(mem)
    return {"id": mem_id, "status": "stored", "type": "procedural"}


# ── Meta ─────────────────────────────────────────────────────────────────────

@router.post("/memories/meta", summary="Store invariant fact")
def store_fact(mem: MetaMemory) -> Dict:
    mem_id = engine.store_fact(mem)
    return {"id": mem_id, "status": "stored", "type": "meta"}


# ── Context ──────────────────────────────────────────────────────────────────

@router.get("/context/active", response_model=ContextResponse)
def read_context(project: str, llm: str = "perplexity", session_id: Optional[str] = None):
    return engine.read_context(project=project, llm=llm, session_id=session_id)


# ── Summarize ────────────────────────────────────────────────────────────────

class SummarizeRequest(BaseModel):
    session_id: str
    messages: Optional[List[Dict]] = None

@router.post("/threads/summarize", response_model=ThreadSummary)
def summarize_thread(req: SummarizeRequest):
    return engine.summarize_thread(session_id=req.session_id, messages=req.messages)


# ── LLM Sync ────────────────────────────────────────────────────────────────

class SyncRequest(BaseModel):
    llm: str
    session_id: str
    capabilities: Optional[Dict] = None
    last_seen_ts: Optional[str] = None

@router.post("/assistants/sync")
def sync_assistant_state(req: SyncRequest) -> Dict:
    import json, os
    from pathlib import Path
    from datetime import datetime
    sync_dir = Path(os.path.expanduser(f"~/.aurphyx/memoree/llm_sync/{req.llm}/sessions"))
    sync_dir.mkdir(parents=True, exist_ok=True)
    fp = sync_dir / f"{req.session_id}.json"
    with open(fp, "w") as f:
        json.dump({
            "llm": req.llm,
            "session_id": req.session_id,
            "capabilities": req.capabilities or {},
            "last_seen_ts": req.last_seen_ts or datetime.utcnow().isoformat(),
            "synced_at": datetime.utcnow().isoformat(),
        }, f, indent=2)
    return {"status": "synced", "llm": req.llm, "session_id": req.session_id}
