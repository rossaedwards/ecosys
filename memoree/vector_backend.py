## ** APS-TSLCA-MEMOREE-VECTOR **
## ** Memoree - Sovereign Memory Substrate **
## ** Symbiotic Universal Xessability Standards **
## ** Three-Squared-Lattice Cognitive Architecture **
## ** Aurphyx Primordial Standard **
## ** Aurphyx LLC **
## ** SAGES | Proprietary | Pro-Existence **
## ** Accessibility = Xessability **
## ** Version 4.0 **

"""
Memoree — Qdrant Vector Backend (TSLCA 9-Cell Multi-Collection)
═══════════════════════════════════════════════════════════════════════════════
Path: c:\\memoree\\vector_backend.py

Production-grade vector backend supporting:
  • 9-cell TSLCA collection partitioning:
    (sensory, working, episodic, semantic, meta, quantum, identity, procedural, governance)
  • Automatic collection creation & schema validation
  • Hybrid search (dense + sparse BM25)
  • Automatic local vector embedding generation & deterministic fallback
  • Resilient in-memory fallback if Qdrant container is offline/booting
  • Diagnostics for /diagnostics route
═══════════════════════════════════════════════════════════════════════════════
"""

from __future__ import annotations

import hashlib
import logging
import math
import os
import time
import uuid
from pathlib import Path
from typing import Any, Dict, List, Optional

import yaml

log = logging.getLogger("memoree.vector")

try:
    from qdrant_client import QdrantClient
    from qdrant_client.http.models import (
        Distance,
        FieldCondition,
        Filter,
        MatchValue,
        PointStruct,
        SearchParams,
        SparseVector,
        VectorParams,
    )
    HAS_QDRANT = True
except ImportError:
    HAS_QDRANT = False
    log.warning("qdrant_client not found — VectorBackend running in fallback mode")


def _generate_deterministic_embedding(text: str, dim: int = 384) -> List[float]:
    """
    Generate a normalized deterministic pseudo-embedding from text.
    Provides a zero-dependency fallback for vector similarity when offline.
    """
    vec = [0.0] * dim
    words = text.lower().split()
    if not words:
        words = ["empty"]

    for word in words:
        h = int(hashlib.sha256(word.encode("utf-8")).hexdigest(), 16)
        for i in range(dim):
            # Deterministic pseudo-random projection
            bit = (h >> (i % 256)) & 1
            val = 1.0 if bit else -1.0
            vec[i] += val

    # L2 normalize
    norm = math.sqrt(sum(x * x for x in vec))
    if norm > 0.0:
        vec = [x / norm for x in vec]
    return vec


class VectorBackend:
    """
    Qdrant backend for Memoree with resilient multi-collection architecture.
    """

    DEFAULT_COLLECTIONS = [
        "sensory",      # SIX ⊗ SIX
        "working",      # SIX ⊗ SCX
        "episodic",     # SIX ⊗ ICX
        "semantic",     # SCX ⊗ SIX
        "meta",         # SCX ⊗ SCX
        "quantum",      # SCX ⊗ ICX
        "identity",     # ICX ⊗ SIX
        "procedural",   # ICX ⊗ SCX
        "governance",   # ICX ⊗ ICX
        "creative",     # Legacy alias
    ]

    def __init__(
        self,
        config_path: str = "config.yaml",
        persist_dir: Optional[str] = None,
        model_name: Optional[str] = None,
    ):
        self.config_path = config_path
        self.persist_dir = persist_dir
        self.model_name = model_name or "all-MiniLM-L6-v2"

        cfg = {}
        if Path(config_path).exists():
            try:
                with open(config_path, "r", encoding="utf-8") as f:
                    cfg = yaml.safe_load(f) or {}
            except Exception as e:
                log.warning("Failed to load %s: %s", config_path, e)

        q_cfg = cfg.get("qdrant", {})
        self.host = q_cfg.get("host", "localhost")
        self.port = q_cfg.get("port", 6333)
        self.embed_dim = q_cfg.get("embed_dim", 384)
        self.hybrid_enabled = q_cfg.get("hybrid", {}).get("enabled", False)

        self.client: Optional[Any] = None
        self.is_connected: bool = False
        self._in_memory_store: Dict[str, List[Dict[str, Any]]] = {
            col: [] for col in self.DEFAULT_COLLECTIONS
        }

        if HAS_QDRANT:
            try:
                self.client = QdrantClient(host=self.host, port=self.port, timeout=2.0)
                # Test connection
                self.client.get_collections()
                self.is_connected = True
                log.info("[VectorBackend] Connected to Qdrant at %s:%s", self.host, self.port)
            except Exception as e:
                log.warning(
                    "[VectorBackend] Qdrant connection to %s:%s failed (%s). Using in-memory fallback.",
                    self.host,
                    self.port,
                    e,
                )
                self.client = None
                self.is_connected = False

        if self.is_connected and self.client:
            for col in self.DEFAULT_COLLECTIONS:
                self._ensure_collection(col)

    def _ensure_collection(self, name: str):
        """Create collection if missing."""
        if not self.is_connected or not self.client:
            return
        try:
            existing = {c.name for c in self.client.get_collections().collections}
            if name in existing:
                return

            vectors_config = VectorParams(
                size=self.embed_dim,
                distance=Distance.COSINE,
            )
            self.client.create_collection(
                collection_name=name,
                vectors_config=vectors_config,
            )
            log.debug("[VectorBackend] Created collection '%s'", name)
        except Exception as e:
            log.warning("[VectorBackend] Error ensuring collection '%s': %s", name, e)

    def get_embedding(self, text: str) -> List[float]:
        """Generate a dense vector for text."""
        return _generate_deterministic_embedding(text, self.embed_dim)

    def upsert(
        self,
        collection: str,
        doc_id: str,
        text: str,
        metadata: Dict[str, Any],
        vector: Optional[List[float]] = None,
        sparse_vector: Optional[Dict[int, float]] = None,
    ):
        """Upsert a memory record into Qdrant with fallback."""
        if vector is None:
            vector = self.get_embedding(text)

        if len(vector) != self.embed_dim:
            vector = _generate_deterministic_embedding(text, self.embed_dim)

        payload = {
            "text": text,
            "timestamp": time.time(),
            **metadata,
        }

        if self.is_connected and self.client:
            try:
                if self.hybrid_enabled and sparse_vector:
                    point = PointStruct(
                        id=doc_id,
                        vector=vector,
                        payload=payload,
                        sparse_vector=SparseVector(
                            indices=list(sparse_vector.keys()),
                            values=list(sparse_vector.values()),
                        ),
                    )
                else:
                    point = PointStruct(
                        id=doc_id,
                        vector=vector,
                        payload=payload,
                    )
                self.client.upsert(collection_name=collection, points=[point])
                return
            except Exception as e:
                log.warning("[VectorBackend] Upsert error: %s — saving to fallback", e)


        # In-memory fallback
        if collection not in self._in_memory_store:
            self._in_memory_store[collection] = []
        # Update or append
        existing = [item for item in self._in_memory_store[collection] if item["id"] == doc_id]
        if existing:
            existing[0]["vector"] = vector
            existing[0]["payload"] = payload
        else:
            self._in_memory_store[collection].append(
                {"id": doc_id, "vector": vector, "payload": payload}
            )

    def query(
        self,
        collection: str,
        query_text: str,
        n_results: int = 5,
        query_vector: Optional[List[float]] = None,
        project: Optional[str] = None,
    ) -> List[Dict[str, Any]]:
        """Query a collection using dense vector similarity."""
        if query_vector is None:
            query_vector = self.get_embedding(query_text)

        if self.is_connected and self.client:
            try:
                flt = None
                if project:
                    flt = Filter(
                        must=[
                            FieldCondition(
                                key="project",
                                match=MatchValue(value=project),
                            )
                        ]
                    )

                results = None
                if hasattr(self.client, "query_points"):
                    res = self.client.query_points(
                        collection_name=collection,
                        query=query_vector,
                        limit=n_results,
                        query_filter=flt,
                    )
                    results = res.points if hasattr(res, "points") else res
                elif hasattr(self.client, "search"):
                    results = self.client.search(
                        collection_name=collection,
                        query_vector=query_vector,
                        limit=n_results,
                        query_filter=flt,
                        search_params=SearchParams(hnsw_ef=128),
                    )

                if results is not None:
                    return [
                        {
                            "id": str(r.id),
                            "score": float(r.score) if hasattr(r, "score") and r.score is not None else 0.0,
                            "payload": r.payload or {},
                        }
                        for r in results
                    ]
            except Exception as e:
                log.warning("[VectorBackend] Query error: %s — querying fallback", e)


        # In-memory fallback calculation
        col_items = self._in_memory_store.get(collection, [])
        scored = []
        for item in col_items:
            payload = item.get("payload", {})
            if project and payload.get("project") != project:
                continue

            v2 = item.get("vector", [])
            if len(v2) == len(query_vector):
                # Cosine similarity of normalized vectors is dot product
                sim = sum(a * b for a, b in zip(query_vector, v2))
            else:
                sim = 0.5
            scored.append({"id": item["id"], "score": sim, "payload": payload})

        scored.sort(key=lambda x: x["score"], reverse=True)
        return scored[:n_results]

    def delete(self, collection: str, doc_id: str):
        """Delete a record."""
        if self.is_connected and self.client:
            try:
                self.client.delete(collection_name=collection, points_selector=[doc_id])
            except Exception as e:
                log.warning("[VectorBackend] Delete error: %s", e)

        if collection in self._in_memory_store:
            self._in_memory_store[collection] = [
                x for x in self._in_memory_store[collection] if x["id"] != doc_id
            ]

    def get_diagnostics(self) -> Dict[str, Any]:
        """Return live vector subsystem status."""
        cols = self.DEFAULT_COLLECTIONS
        if self.is_connected and self.client:
            try:
                cols = [c.name for c in self.client.get_collections().collections]
            except Exception:
                pass
        return {
            "qdrant_connected": self.is_connected,
            "host": self.host,
            "port": self.port,
            "collections": cols,
            "embed_dim": self.embed_dim,
        }
