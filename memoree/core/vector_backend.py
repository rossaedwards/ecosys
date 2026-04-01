"""Memoree v3 — Swappable vector backend.
Default: LanceDB (embedded, columnar, CPU-fast).
Fallback: Qdrant (local binary, metadata-filter champion).

Switch in config.yaml: vector_backend: lancedb | qdrant
"""
from __future__ import annotations
import os
from typing import Optional
import yaml

_CONFIG_PATH = os.path.join(os.path.dirname(__file__), "..", "config.yaml")


def _cfg() -> dict:
    with open(_CONFIG_PATH) as f:
        return yaml.safe_load(f)


# ---------------------------------------------------------------------------
# LanceDB backend
# ---------------------------------------------------------------------------

_lance_table = None


def _get_lance_table():
    global _lance_table
    if _lance_table is not None:
        return _lance_table
    import lancedb  # type: ignore
    cfg = _cfg()
    ldb_path = cfg["lancedb"]["path"]
    os.makedirs(ldb_path, exist_ok=True)
    db = lancedb.connect(ldb_path)
    table_name = cfg["lancedb"]["table_name"]
    dim = cfg["lancedb"]["embed_dim"]
    if table_name not in db.table_names():
        import pyarrow as pa  # type: ignore
        schema = pa.schema([
            pa.field("id", pa.string()),
            pa.field("llm", pa.string()),
            pa.field("type", pa.string()),
            pa.field("content", pa.string()),
            pa.field("tags", pa.string()),
            pa.field("timestamp", pa.string()),
            pa.field("vector", pa.list_(pa.float32(), dim)),
        ])
        _lance_table = db.create_table(table_name, schema=schema)
    else:
        _lance_table = db.open_table(table_name)
    return _lance_table


async def upsert(event_id: str, vector: list[float], payload: dict):
    backend = _cfg().get("vector_backend", "lancedb")
    if backend == "qdrant":
        await _qdrant_upsert(event_id, vector, payload)
    else:
        _lance_upsert(event_id, vector, payload)


def _lance_upsert(event_id: str, vector: list[float], payload: dict):
    table = _get_lance_table()
    row = {
        "id": event_id,
        "llm": payload.get("llm", ""),
        "type": payload.get("type", "episodic"),
        "content": payload.get("content", ""),
        "tags": str(payload.get("tags", [])),
        "timestamp": payload.get("timestamp", ""),
        "vector": vector,
    }
    table.add([row])


async def vector_search(
    query_vec: list[float],
    llm: Optional[str] = None,
    type_: Optional[str] = None,
    top_k: int = 5,
) -> list[dict]:
    backend = _cfg().get("vector_backend", "lancedb")
    if backend == "qdrant":
        return await _qdrant_search(query_vec, llm, type_, top_k)
    return _lance_search(query_vec, llm, type_, top_k)


def _lance_search(
    query_vec: list[float],
    llm: Optional[str],
    type_: Optional[str],
    top_k: int,
) -> list[dict]:
    table = _get_lance_table()
    q = table.search(query_vec).limit(top_k * 3)  # over-fetch then filter
    results = q.to_list()
    filtered = []
    for r in results:
        if llm and r.get("llm") != llm:
            continue
        if type_ and r.get("type") != type_:
            continue
        filtered.append({"id": r["id"], "content": r["content"], "score": float(r.get("_distance", 0))})
        if len(filtered) >= top_k:
            break
    return filtered


# ---------------------------------------------------------------------------
# Qdrant fallback backend
# ---------------------------------------------------------------------------

_qdrant_client = None


def _get_qdrant():
    global _qdrant_client
    if _qdrant_client is not None:
        return _qdrant_client
    from qdrant_client import QdrantClient  # type: ignore
    from qdrant_client.models import Distance, VectorParams  # type: ignore
    cfg = _cfg()["qdrant"]
    client = QdrantClient(host=cfg["host"], port=cfg["port"])
    dim = _cfg()["lancedb"]["embed_dim"]  # shared dim config
    collections = [c.name for c in client.get_collections().collections]
    if cfg["collection"] not in collections:
        client.create_collection(
            collection_name=cfg["collection"],
            vectors_config=VectorParams(size=dim, distance=Distance.COSINE),
        )
    _qdrant_client = client
    return client


async def _qdrant_upsert(event_id: str, vector: list[float], payload: dict):
    from qdrant_client.models import PointStruct  # type: ignore
    cfg = _cfg()["qdrant"]
    client = _get_qdrant()
    client.upsert(
        collection_name=cfg["collection"],
        points=[PointStruct(id=event_id, vector=vector, payload=payload)],
    )


async def _qdrant_search(
    query_vec: list[float],
    llm: Optional[str],
    type_: Optional[str],
    top_k: int,
) -> list[dict]:
    from qdrant_client.models import Filter, FieldCondition, MatchValue  # type: ignore
    cfg = _cfg()["qdrant"]
    client = _get_qdrant()
    must = []
    if llm:
        must.append(FieldCondition(key="llm", match=MatchValue(value=llm)))
    if type_:
        must.append(FieldCondition(key="type", match=MatchValue(value=type_)))
    q_filter = Filter(must=must) if must else None
    hits = client.search(
        collection_name=cfg["collection"],
        query_vector=query_vec,
        query_filter=q_filter,
        limit=top_k,
    )
    return [{"id": str(h.id), "content": h.payload.get("content", ""), "score": h.score} for h in hits]
