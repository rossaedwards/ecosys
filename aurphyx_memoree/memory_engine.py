"""
Aurphyx_Memoree — Memory Engine
Orchestrates AuraFS + ChromaDB + Memori bridge.
All write paths: AuraFS shard → ChromaDB embed → Memori mirror (if enabled).
f0rg3d in l0v3 by Ross Edwards
"""

import yaml
import os
from datetime import datetime
from pathlib import Path
from typing import Any, Dict, List, Optional

from core.schemas import (
    EpisodicMemory, SemanticMemory, ProceduralMemory, MetaMemory,
    ContextResponse, ThreadSummary, MemoryType
)
from core.aurafs_backend import AuraFSBackend
from core.vector_backend import VectorBackend
from core.memori_bridge import MemoriBridge


def _load_config(config_path: str = "daemon/config.yaml") -> Dict:
    try:
        with open(config_path, "r") as f:
            return yaml.safe_load(f)
    except FileNotFoundError:
        return {}


class MemoryEngine:
    """
    Central memory orchestrator for Aurphyx_Memoree Daemon.
    """

    def __init__(self, config_path: str = "daemon/config.yaml"):
        cfg = _load_config(config_path)
        aurafs_root = cfg.get("aurafs", {}).get("root", "~/.aurphyx/memoree")
        chroma_dir  = cfg.get("chroma", {}).get("persist_dir", "~/.aurphyx/memoree/embeddings")
        embed_model = cfg.get("chroma", {}).get("embedding_model", "all-MiniLM-L6-v2")
        self.mirror = cfg.get("backend", {}).get("mirror_to_primary", True)

        self.aurafs = AuraFSBackend(root_path=aurafs_root)
        self.vector = VectorBackend(persist_dir=chroma_dir, model_name=embed_model)
        self.memori = MemoriBridge()
        print("🔥 [MemoryEngine] All backends initialized")

    # ── Write: Episodic ──────────────────────────────────────────────────────

    def write_event(self, mem: EpisodicMemory) -> str:
        self.aurafs.write_episodic(mem)
        self.vector.upsert(
            collection="episodic",
            doc_id=mem.id,
            text=mem.content,
            metadata={"session_id": mem.session_id, "role": mem.role,
                       "llm": mem.llm, "timestamp": mem.timestamp.isoformat()}
        )
        if self.mirror:
            self.memori.mirror_episodic(role=mem.role, content=mem.content, llm=mem.llm)
        return mem.id

    # ── Write: Semantic ──────────────────────────────────────────────────────

    def embed_document(self, mem: SemanticMemory) -> str:
        self.aurafs.write_semantic(mem)
        full_text = f"Project: {mem.project}\nCategory: {mem.category}\nTags: {', '.join(mem.tags)}\n\n{mem.content}"
        self.vector.upsert(
            collection="semantic",
            doc_id=mem.id,
            text=full_text,
            metadata={"project": mem.project, "category": mem.category,
                       "tags": str(mem.tags), "timestamp": mem.timestamp.isoformat()}
        )
        if self.mirror:
            self.memori.mirror_semantic(
                project=mem.project, category=mem.category,
                content=mem.content, tags=mem.tags, relationships=mem.relationships
            )
        return mem.id

    # ── Write: Procedural ────────────────────────────────────────────────────

    def store_workflow(self, mem: ProceduralMemory) -> str:
        self.aurafs.write_procedural(mem)
        steps_text = "\n".join(f"{i+1}. {s}" for i, s in enumerate(mem.steps))
        self.vector.upsert(
            collection="procedural",
            doc_id=mem.id,
            text=f"Task: {mem.task}\n\nSteps:\n{steps_text}",
            metadata={"task": mem.task, "frequency": str(mem.frequency),
                       "success_rate": str(mem.success_rate)}
        )
        if self.mirror:
            self.memori.mirror_procedural(task=mem.task, steps=mem.steps,
                                          frequency=mem.frequency, success_rate=mem.success_rate)
        return mem.id

    # ── Write: Meta ──────────────────────────────────────────────────────────

    def store_fact(self, mem: MetaMemory) -> str:
        self.aurafs.write_meta(mem)
        self.vector.upsert(
            collection="meta",
            doc_id=mem.id,
            text=mem.fact,
            metadata={"verified": str(mem.verified), "confidence": str(mem.confidence),
                       "deprecated": str(mem.deprecated)}
        )
        if self.mirror:
            self.memori.mirror_meta(fact=mem.fact, confidence=mem.confidence,
                                    sources=mem.sources, verified=mem.verified)
        return mem.id

    # ── Read: Context ────────────────────────────────────────────────────────

    def read_context(self, project: str, llm: str = "perplexity",
                     session_id: Optional[str] = None) -> ContextResponse:
        episodic_raw = self.vector.query("episodic", query_text=project, n_results=5,
                                          where={"llm": llm} if llm else None)
        semantic_raw = self.vector.query("semantic", query_text=project, n_results=5,
                                          where={"project": project})
        state = self.aurafs.read_state("projects")
        proj_state = state.get(project, {})
        summary = self.aurafs.read_summary(session_id) if session_id else None

        return ContextResponse(
            project=project,
            llm=llm,
            session_id=session_id,
            last_summary=summary.get("short_summary") if summary else None,
            episodic=episodic_raw,
            semantic=semantic_raw,
            active_volumes=proj_state.get("active_volumes", []),
            active_axioms=proj_state.get("active_axioms", []),
            active_dualities=proj_state.get("active_dualities", []),
            invariants=self._load_invariants(),
        )

    def _load_invariants(self) -> List[str]:
        metas = self.aurafs.read_meta(limit=100)
        return [m["fact"] for m in metas
                if m.get("verified") and not m.get("deprecated", False)]

    # ── Summarize Thread ─────────────────────────────────────────────────────

    def summarize_thread(self, session_id: str, messages: Optional[List[Dict]] = None) -> ThreadSummary:
        if not messages:
            messages = self.aurafs.read_episodic(session_id, limit=50)
        combined = " | ".join(m.get("content", "") for m in messages[:20])
        short = combined[:200] + "..." if len(combined) > 200 else combined
        summary = ThreadSummary(
            session_id=session_id,
            short_summary=short,
            long_summary=combined,
            key_topics=[],
            aps_refs=[],
            embedding_cluster_ids=[]
        )
        self.aurafs.write_summary(session_id, summary.model_dump())
        return summary
