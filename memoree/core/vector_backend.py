"""Memoree v3 — Swappable vector backend.

Primary  : Qdrant (local Docker, Rust, hybrid sparse+dense, metadata filtering)
Fallback : LanceDB (embedded columnar, zero-server)

Swap in config.yaml:  vector_backend: qdrant | lancedb
"""
from __future__ import annotations
import os
from typing import Optional
import yaml

_CONFIG_PATH = os.path.join(os.path.dirname(__file__), "..", "config.yaml")
_ACTIVE_BACKEND: Optional[str] = None


def _cfg() -> dict:
    with open(_CONFIG_PATH) as f:
        return yaml.safe_load(f)


def active_backend() -> str:
    global _ACTIVE_BACKEND
    if _ACTIVE_BACKEND is None:
        _ACTIVE_BACKEND = _cfg().get("vector_backend", "qdrant")
    return _ACTIVE_BACKEND


# ──────────────────────────────────────────────────────────────────
# Qdrant Backend
# ──────────────────────────────────────────────────────────────────

_qdrant_client = None


def _qdrant_client_get():
    global _qdrant_client
    if _qdrant_client is not None:
        return _qdrant_client
    from qdrant_client import QdrantClient
    from qdrant_client.models import Distance, VectorParams, SparseVectorParams, SparseIndexParams
    cfg = _cfg()["qdrant"]
    dim = cfg.get("embed_dim", 384)
    client = QdrantClient(host=cfg["host"], port=cfg["port"])
    coll = cfg["collection"]
    existing = [c.name for c in client.get_collections().collections]
    if coll not in existing:
        # Create with both dense + sparse vectors for hybrid search
        client.create_collection(
            collection_name=coll,
            vectors_config={"dense": VectorParams(size=dim, distance=Distance.COSINE)},
            sparse_vectors_config={
                "sparse": SparseVectorParams(index=SparseIndexParams(on_disk=False))
            },
        )
    _qdrant_client = client
    return client


async def _qdrant_upsert(event_id: str, vector: list[float], payload: dict):
    from qdrant_client.models import PointStruct, NamedVector
    client = _qdrant_client_get()
    cfg = _cfg()["qdrant"]
    client.upsert(
        collection_name=cfg["collection"],
        points=[
            PointStruct(
                id=event_id,
                vector={"dense": vector},
                payload=payload,
            )
        ],
    )


async def _qdrant_search(
    query_vec: list[float],
    llm: Optional[str],
    type_: Optional[str],
    project: Optional[str],
    tags: list[str],
    min_confidence: float,
    min_priority: int,
    top_k: int,
) -> list[dict]:
    from qdrant_client.models import Filter, FieldCondition, MatchValue, Range, MatchAny
    cfg = _cfg()["qdrant"]
    must = []
    if llm:
        must.append(FieldCondition(key="llm",     match=MatchValue(value=llm)))
    if type_:
        must.append(FieldCondition(key="type",    match=MatchValue(value=type_)))
    if project:
        must.append(FieldCondition(key="project", match=MatchValue(value=project)))
    if tags:
        must.append(FieldCondition(key="tags",    match=MatchAny(any=tags)))
    if min_priority > 1:
        must.append(FieldCondition(key="priority", range=Range(gte=min_priority)))
    if min_confidence > 0.0:
        must.append(FieldCondition(key="confidence", range=Range(gte=min_confidence)))
    q_filter = Filter(must=must) if must else None
    client = _qdrant_client_get()
    hits = client.search(
        collection_name=cfg["collection"],
        query_vector=("dense", query_vec),
        query_filter=q_filter,
        limit=top_k,
        with_payload=True,
    )
    return [
        {
            "id":      str(h.id),
            "content": h.payload.get("content", ""),
            "score":   h.score,
            "payload": h.payload,
        }
        for h in hits
    ]


# ──────────────────────────────────────────────────────────────────
# LanceDB Fallback Backend
# ──────────────────────────────────────────────────────────────────

_lance_table = None


def _lance_table_get():
    global _lance_table
    if _lance_table is not None:
        return _lance_table
    import lancedb
    import pyarrow as pa
    cfg = _cfg()["lancedb"]
    path = cfg["path"]
    os.makedirs(path, exist_ok=True)
    db  = lancedb.connect(path)
    tbl = cfg["table_name"]
    dim = cfg.get("embed_dim", 384)
    if tbl not in db.table_names():
        schema = pa.schema([
            pa.field("id",         pa.string()),
            pa.field("llm",        pa.string()),
            pa.field("type",       pa.string()),
            pa.field("project",    pa.string()),
            pa.field("subproject", pa.string()),
            pa.field("content",    pa.string()),
            pa.field("tags",       pa.string()),
            pa.field("priority",   pa.int32()),
            pa.field("confidence", pa.float32()),
            pa.field("mood",       pa.string()),
            pa.field("timestamp",  pa.string()),
            pa.field("vector",     pa.list_(pa.float32(), dim)),
        ])
        _lance_table = db.create_table(tbl, schema=schema)
    else:
        _lance_table = db.open_table(tbl)
    return _lance_table


async def _lance_upsert(event_id: str, vector: list[float], payload: dict):
    table = _lance_table_get()
    table.add([{
        "id":         event_id,
        "llm":        payload.get("llm", ""),
        "type":       payload.get("type", "episodic"),
        "project":    payload.get("project", ""),
        "subproject": payload.get("subproject", ""),
        "content":    payload.get("content", ""),
        "tags":       str(payload.get("tags", [])),
        "priority":   int(payload.get("priority", 5)),
        "confidence": float(payload.get("confidence", 0.8)),
        "mood":       payload.get("mood", "neutral"),
        "timestamp":  payload.get("timestamp", ""),
        "vector":     vector,
    }])


async def _lance_search(
    query_vec: list[float],
    llm: Optional[str],
    type_: Optional[str],
    project: Optional[str],
    top_k: int,
) -> list[dict]:
    table = _lance_table_get()
    results = table.search(query_vec).limit(top_k * 3).to_list()
    filtered = []
    for r in results:
        if llm     and r.get("llm")     != llm:     continue
        if type_   and r.get("type")    != type_:   continue
        if project and r.get("project") != project: continue
        filtered.append({"id": r["id"], "content": r["content"], "score": float(r.get("_distance", 0)), "payload": r})
        if len(filtered) >= top_k:
            break
    return filtered


# ──────────────────────────────────────────────────────────────────
# Public interface (called by api.py — backend-agnostic)
# ──────────────────────────────────────────────────────────────────

async def upsert(event_id: str, vector: list[float], payload: dict):
    if active_backend() == "qdrant":
        await _qdrant_upsert(event_id, vector, payload)
    else:
        await _lance_upsert(event_id, vector, payload)


async def vector_search(
    query_vec:      list[float],
    llm:            Optional[str]  = None,
    type_:          Optional[str]  = None,
    project:        Optional[str]  = None,
    tags:           list[str]      = [],
    min_confidence: float          = 0.0,
    min_priority:   int            = 1,
    top_k:          int            = 8,
) -> list[dict]:
    if active_backend() == "qdrant":
        return await _qdrant_search(query_vec, llm, type_, project, tags, min_confidence, min_priority, top_k)
    return await _lance_search(query_vec, llm, type_, project, top_k)
