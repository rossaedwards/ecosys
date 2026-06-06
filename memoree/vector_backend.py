"""
Memoree — Qdrant Vector Backend
Path: c:\\memoree\\vector_backend.py

Production‑grade vector backend for the Memoree sovereign memory substrate.
Supports:
  • Multi‑collection architecture (episodic, semantic, procedural, meta, quantum, creative, governance)
  • Automatic collection creation + schema validation
  • Hybrid search (dense + sparse BM25)
  • Project‑aware filtering
  • Batch ingestion
  • Soft delete + hard delete
  • Diagnostics for /diagnostics route
"""

from __future__ import annotations

import uuid
import yaml
import time
from typing import Any, Dict, List, Optional

from qdrant_client import QdrantClient
from qdrant_client.http.models import (
    Distance,
    VectorParams,
    PointStruct,
    Filter,
    FieldCondition,
    MatchValue,
    SearchParams,
    SparseVector,
)


class VectorBackend:
    """
    Qdrant backend for Memoree.

    Collections:
      episodic, semantic, procedural, meta, quantum, creative, governance

    Each collection stores:
      • vector (dense embedding)
      • optional sparse vector (BM25)
      • payload (metadata)
    """

    DEFAULT_COLLECTIONS = [
        "episodic",
        "semantic",
        "procedural",
        "meta",
        "quantum",
        "creative",
        "governance",
    ]

    def __init__(self, config_path: str = "config.yaml"):
        with open(config_path, "r", encoding="utf-8") as f:
            cfg = yaml.safe_load(f) or {}

        q_cfg = cfg.get("qdrant", {})
        self.host = q_cfg.get("host", "localhost")
        self.port = q_cfg.get("port", 6333)
        self.embed_dim = q_cfg.get("embed_dim", 384)
        self.hybrid_enabled = q_cfg.get("hybrid", {}).get("enabled", False)

        self.client = QdrantClient(host=self.host, port=self.port)

        # Ensure all collections exist
        for col in self.DEFAULT_COLLECTIONS:
            self._ensure_collection(col)

    # ----------------------------------------------------------------------
    # Collection Management
    # ----------------------------------------------------------------------
    def _ensure_collection(self, name: str):
        """Create collection if missing."""
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

    # ----------------------------------------------------------------------
    # Upsert
    # ----------------------------------------------------------------------
    def upsert(
        self,
        collection: str,
        doc_id: str,
        text: str,
        metadata: Dict[str, Any],
        vector: Optional[List[float]] = None,
        sparse_vector: Optional[Dict[int, float]] = None,
    ):
        """
        Upsert a memory record into Qdrant.

        MemoryEngine always provides:
          • collection
          • doc_id
          • text
          • metadata
        """
        if vector is None:
            raise ValueError("VectorBackend.upsert requires a dense vector.")

        if len(vector) != self.embed_dim:
            raise ValueError(
                f"Vector dimension mismatch: expected {self.embed_dim}, got {len(vector)}"
            )

        payload = {
            "text": text,
            "timestamp": time.time(),
            **metadata,
        }

        point = PointStruct(
            id=doc_id,
            vector=vector,
            payload=payload,
            sparse_vector=SparseVector(indices=list(sparse_vector.keys()),
                                       values=list(sparse_vector.values()))
            if (self.hybrid_enabled and sparse_vector)
            else None,
        )

        self.client.upsert(collection_name=collection, points=[point])

    # ----------------------------------------------------------------------
    # Query
    # ----------------------------------------------------------------------
    def query(
        self,
        collection: str,
        query_text: str,
        n_results: int = 5,
        query_vector: Optional[List[float]] = None,
        project: Optional[str] = None,
    ) -> List[Dict[str, Any]]:
        """
        Query a collection using dense vector similarity.
        MemoryEngine uses this for context assembly.
        """

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

        # Dense vector required
        if query_vector is None:
            raise ValueError("query_vector must be provided for semantic search.")

        results = self.client.search(
            collection_name=collection,
            query_vector=query_vector,
            limit=n_results,
            query_filter=flt,
            search_params=SearchParams(hnsw_ef=128),
        )

        return [
            {
                "id": r.id,
                "score": r.score,
                "payload": r.payload,
            }
            for r in results
        ]

    # ----------------------------------------------------------------------
    # Hybrid Search (Dense + Sparse)
    # ----------------------------------------------------------------------
    def hybrid_search(
        self,
        collection: str,
        query_vector: List[float],
        sparse_query: Optional[Dict[int, float]] = None,
        top_k: int = 5,
    ):
        if not self.hybrid_enabled:
            raise RuntimeError("Hybrid search requested but hybrid mode is disabled.")

        results = self.client.search(
            collection_name=collection,
            query_vector=query_vector,
            query_sparse_vector=SparseVector(
                indices=list(sparse_query.keys()),
                values=list(sparse_query.values()),
            )
            if sparse_query
            else None,
            limit=top_k,
        )

        return [
            {"id": r.id, "score": r.score, "payload": r.payload}
            for r in results
        ]

    # ----------------------------------------------------------------------
    # Batch Ingestion
    # ----------------------------------------------------------------------
    def batch_upsert(self, collection: str, items: List[Dict[str, Any]]):
        points = []
        for item in items:
            doc_id = item.get("id", str(uuid.uuid4()))
            vector = item["vector"]
            metadata = item.get("metadata", {})
            text = item["text"]

            if len(vector) != self.embed_dim:
                raise ValueError(
                    f"Vector dimension mismatch: expected {self.embed_dim}, got {len(vector)}"
                )

            points.append(
                PointStruct(
                    id=doc_id,
                    vector=vector,
                    payload={"text": text, **metadata},
                )
            )

        self.client.upsert(collection_name=collection, points=points)

    # ----------------------------------------------------------------------
    # Delete
    # ----------------------------------------------------------------------
    def delete(self, collection: str, doc_id: str):
        """Hard delete."""
        self.client.delete(collection_name=collection, points_selector=[doc_id])

    def soft_delete(self, collection: str, doc_id: str):
        """Mark as deleted without removing vector."""
        self.client.set_payload(
            collection_name=collection,
            payload={"deleted": True},
            points=[doc_id],
        )

    # ----------------------------------------------------------------------
    # Diagnostics
    # ----------------------------------------------------------------------
    def diagnostics(self) -> Dict[str, Any]:
        """Return backend health info for /diagnostics."""
        try:
            collections = self.client.get_collections().collections
            counts = {}
            for c in collections:
                try:
                    counts[c.name] = self.client.count(c.name).count
                except Exception:
                    counts[c.name] = 0

            return {
                "status": "ok",
                "host": self.host,
                "port": self.port,
                "embed_dim": self.embed_dim,
                "collections": counts,
            }

        except Exception as e:
            return {
                "status": "error",
                "error": str(e),
            }
