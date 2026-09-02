## ** APS-TSLCA-MEMOREE-ENGINE **
## ** Memoree - Sovereign Memory Substrate **
## ** Symbiotic Universal Xessability Standards **
## ** Three-Squared-Lattice Cognitive Architecture **
## ** Aurphyx Primordial Standard **
## ** Aurphyx LLC **
## ** SAGES | Proprietary | Pro-Existence **
## ** Accessibility = Xessability **
## ** Version 4.0 **

"""
Memoree — Memory Engine (TSLCA 9-Cell Lattice & Harmonic Integrity Field)
═══════════════════════════════════════════════════════════════════════════════
Orchestrates all storage backends and assembles full context payloads.
AuraFS is disabled pending integration — all AuraFS call-sites are preserved
as commented stubs so the re-enable is a one-line uncomment per method.

  Path   : c:\\memoree\\memory_engine.py
  Owner  : Ross Edwards / Aurphyx LLC
  GitHub : rossaedwards | aurphyx
  ORCiD  : 0009-0008-0539-1289

Three-Squared-Lattice Cognitive Architecture (TSLCA) 3×3 Grid:
─────────────────────────────────────────────────────────────
  ┌─ SIX ⊗ SIX  (Sensory)    → Perception traces, AUDRA 432/528Hz resonance
  ├─ SIX ⊗ SCX  (Working)    → Active context, open loops, uncured session buffer
  ├─ SIX ⊗ ICX  (Episodic)   → Conversation turns, session-bound interactions
  ├─ SCX ⊗ SIX  (Semantic)   → Project knowledge, facts, relationships, dualities
  ├─ SCX ⊗ SCX  (Meta)       → Verified facts, confidence-tracked beliefs, axioms
  ├─ SCX ⊗ ICX  (Quantum)    → Physics/simulation state, lattice snapshots
  ├─ ICX ⊗ SIX  (Identity)   → Ξ continuity, SoulJourney pipeline pointer
  ├─ ICX ⊗ SCX  (Procedural) → Repeatable workflows, task recipes, automation
  └─ ICX ⊗ ICX  (Governance) → Voting records, policy decisions, GVS Archivus
═══════════════════════════════════════════════════════════════════════════════
f0rg3d in l0v3 by Ross Edwards
"""

from __future__ import annotations

import json
import logging
import os
from datetime import datetime, timezone
from pathlib import Path
from typing import Any, Dict, List, Optional

import yaml
from memori_bridge import MemoriBridge
from schemas import (
    AnyMemory,
    BulkUpsertRequest,
    ContextResponse,
    CreativeMemory,
    DualityPair,
    EpisodicMemory,
    GovernanceMemory,
    IdentityMemory,
    LLMProvider,
    MemoreeDiagnostics,
    MemoryQuery,
    MemorySearchResult,
    MemoryTier,
    MemoryType,
    MetaMemory,
    ProceduralMemory,
    ProjectMeta,
    ProjectOwner,
    QuantumMemory,
    SemanticMemory,
    SensoryMemory,
    SoulJourneyStage,
    ThreadSummary,
    UpsertMemoryRequest,
    VolumeRef,
    WorkingMemory,
    _now,
)
from tsl_memory_kernel import (
    LatticeSnapshot,
    calculate_hif,
    contract_lattice_context,
    evaluate_gate,
)
from vector_backend import VectorBackend

log = logging.getLogger("memoree.engine")

# ─────────────────────────────────────────────────────────────────────────────
# Constants
# ─────────────────────────────────────────────────────────────────────────────

BASE_DIR = Path(__file__).parent
CONFIG_PATH = BASE_DIR / "config.yaml"
PROJECTS_PATH = BASE_DIR / "projects.json"
DUALITIES_PATH = BASE_DIR / "dualities.json"
INVARIANTS_PATH = BASE_DIR / "invariants.json"

_DEFAULT_CHROMA_DIR = str(BASE_DIR / "embeddings")
_DEFAULT_EMBED_MODEL = "all-MiniLM-L6-v2"
_DEFAULT_PROJECT = "memoree"
_DEFAULT_LLM = "supergrok"
_CONTEXT_RESULTS = 5


# ─────────────────────────────────────────────────────────────────────────────
# Config / Data Loaders
# ─────────────────────────────────────────────────────────────────────────────


def _load_yaml(path: Path) -> Dict[str, Any]:
    """Load a YAML file; return empty dict on missing file or parse error."""
    try:
        with path.open("r", encoding="utf-8") as fh:
            data = yaml.safe_load(fh)
            return data if isinstance(data, dict) else {}
    except FileNotFoundError:
        log.warning("Config not found: %s — using defaults", path)
        return {}
    except yaml.YAMLError as exc:
        log.error("YAML parse error in %s: %s", path, exc)
        return {}


def _load_json(path: Path) -> Dict[str, Any]:
    """Load a JSON file; return empty dict on missing file or parse error."""
    try:
        with path.open("r", encoding="utf-8") as fh:
            return json.load(fh)
    except FileNotFoundError:
        log.warning("JSON data file not found: %s", path)
        return {}
    except json.JSONDecodeError as exc:
        log.error("JSON parse error in %s: %s", path, exc)
        return {}


def _parse_project_meta(key: str, raw: Dict[str, Any]) -> ProjectMeta:
    """Hydrate a ProjectMeta from a single projects.json entry."""
    valid_volumes: List[VolumeRef] = []
    for v in raw.get("active_volumes", []):
        try:
            valid_volumes.append(VolumeRef(v))
        except ValueError:
            log.debug("Unknown VolumeRef '%s' in project '%s' — skipped", v, key)

    try:
        owner = ProjectOwner(raw.get("owner", "rossaedwards"))
    except ValueError:
        owner = ProjectOwner.ROSS

    return ProjectMeta(
        key=key,
        name=key.replace("_", " ").title(),
        description=raw.get("description", ""),
        owner=owner,
        active_volumes=valid_volumes,
        active_axioms=raw.get("active_axioms", []),
        active_dualities=raw.get("active_dualities", []),
        status=raw.get("status", "active"),
    )


# ─────────────────────────────────────────────────────────────────────────────
# MemoryEngine
# ─────────────────────────────────────────────────────────────────────────────


class MemoryEngine:
    """
    Central orchestrator for all Memoree read and write operations across the
    9-cell TSLCA matrix.
    """

    def __init__(
        self,
        config_path: Path | str = CONFIG_PATH,
        projects_path: Path | str = PROJECTS_PATH,
    ) -> None:
        cfg = _load_yaml(Path(config_path))
        self._cfg = cfg

        # Vector backend
        self.vector = VectorBackend(config_path=str(config_path))

        # MemoriBridge
        self.mirror: bool = cfg.get("backend", {}).get("mirror_to_primary", False)
        self.memori = MemoriBridge()

        # Project registry
        raw_projects = _load_json(Path(projects_path))
        global_settings: Dict[str, Any] = raw_projects.get("global_settings", {})

        self.default_project: str = global_settings.get(
            "default_project", _DEFAULT_PROJECT
        )
        self.default_llm: str = global_settings.get("default_llm", _DEFAULT_LLM)
        self.context_window_days: int = int(
            global_settings.get("context_window_days", 30)
        )

        self.projects: Dict[str, ProjectMeta] = {
            key: _parse_project_meta(key, val)
            for key, val in raw_projects.get("projects", {}).items()
        }

        # Dualities & invariants
        raw_dualities = _load_json(DUALITIES_PATH)
        raw_invariants = _load_json(INVARIANTS_PATH)
        self.global_dualities: List[str] = raw_dualities.get("dualities", [])
        self.global_invariants: List[str] = raw_invariants.get("invariants", [])

        # Runtime counters
        self._start_time: datetime = datetime.now(tz=timezone.utc)
        self._active_sessions: int = 0
        self._working_buffer: List[WorkingMemory] = []

        log.info(
            "[MemoryEngine] Initialized v4.0 — %d projects loaded | 9 TSL collections ready",
            len(self.projects),
        )

    def _project_meta(self, project: str) -> Optional[ProjectMeta]:
        return self.projects.get(project)

    def _resolve_llm(self, llm: str | LLMProvider) -> LLMProvider:
        try:
            return LLMProvider(llm) if isinstance(llm, str) else llm
        except ValueError:
            return LLMProvider.UNKNOWN

    def _mirror_guard(self, fn_name: str) -> bool:
        if self.mirror:
            log.debug("[mirror] %s → MemoriBridge", fn_name)
        return self.mirror

    # ── Write: 9-Cell TSLCA Memory Operations ─────────────────────────────────

    def write_sensory(self, mem: SensoryMemory) -> str:
        """Persist a SensoryMemory (SIX ⊗ SIX — perception & resonance)."""
        mem.touch()
        self.vector.upsert(
            collection="sensory",
            doc_id=mem.id,
            text=f"Sensory ({mem.modality}): {mem.content}",
            metadata={
                "project": mem.project,
                "session_id": mem.session_id or "",
                "modality": mem.modality,
                "dominant_freq_hz": str(mem.dominant_frequency_hz or ""),
                "geometry": mem.sacred_geometry_pattern or "",
                "resonance_score": str(mem.resonance_score),
                "blessed_by_mama_bear": str(mem.blessed_by_mama_bear),
                "timestamp": mem.timestamp.isoformat(),
            },
        )
        return mem.id

    def write_working(self, mem: WorkingMemory) -> str:
        """Persist a WorkingMemory (SIX ⊗ SCX — active session buffer)."""
        mem.touch()
        self._working_buffer.append(mem)
        self.vector.upsert(
            collection="working",
            doc_id=mem.id,
            text=f"Working Focus: {mem.active_focus or 'general'}\n{mem.content}",
            metadata={
                "project": mem.project,
                "session_id": mem.session_id,
                "open_loops": str(mem.open_loops),
                "cured": str(mem.cured),
                "timestamp": mem.timestamp.isoformat(),
            },
        )
        return mem.id

    def write_event(self, mem: EpisodicMemory) -> str:
        """Persist an EpisodicMemory (SIX ⊗ ICX — conversation turn)."""
        mem.touch()
        self.vector.upsert(
            collection="episodic",
            doc_id=mem.id,
            text=mem.content,
            metadata={
                "session_id": mem.session_id,
                "project": mem.project,
                "role": mem.role,
                "llm": str(mem.llm),
                "turn_index": str(mem.turn_index),
                "intent": mem.intent or "",
                "timestamp": mem.timestamp.isoformat(),
            },
        )
        if self._mirror_guard("write_event"):
            self.memori.mirror_episodic(
                role=mem.role,
                content=mem.content,
                llm=mem.llm,
            )
        return mem.id

    def embed_document(self, mem: SemanticMemory) -> str:
        """Persist a SemanticMemory (SCX ⊗ SIX — project knowledge)."""
        mem.touch()
        full_text = (
            f"Project: {mem.project}\n"
            f"Category: {mem.category}\n"
            f"Tags: {', '.join(mem.tags)}\n\n"
            f"{mem.content}"
        )
        self.vector.upsert(
            collection="semantic",
            doc_id=mem.id,
            text=full_text,
            metadata={
                "project": mem.project,
                "category": mem.category,
                "subcategory": mem.subcategory or "",
                "tags": str(mem.tags),
                "confidence": str(mem.confidence),
                "timestamp": mem.timestamp.isoformat(),
            },
        )
        if self._mirror_guard("embed_document"):
            self.memori.mirror_semantic(
                project=mem.project,
                category=mem.category,
                content=mem.content,
                tags=mem.tags,
                relationships=mem.relationships,
            )
        return mem.id

    def store_fact(self, mem: MetaMemory) -> str:
        """Persist a MetaMemory (SCX ⊗ SCX — verified invariant/fact)."""
        mem.touch()
        self.vector.upsert(
            collection="meta",
            doc_id=mem.id,
            text=mem.fact,
            metadata={
                "project": mem.project or "global",
                "verified": str(mem.verified),
                "confidence": str(mem.confidence),
                "deprecated": str(mem.deprecated),
                "superseded_by": mem.superseded_by or "",
                "timestamp": mem.timestamp.isoformat(),
            },
        )
        if self._mirror_guard("store_fact"):
            self.memori.mirror_meta(
                fact=mem.fact,
                confidence=mem.confidence,
                sources=mem.sources,
                verified=mem.verified,
            )
        return mem.id

    def store_quantum(self, mem: QuantumMemory) -> str:
        """Persist a QuantumMemory (SCX ⊗ ICX — simulation & physics state)."""
        mem.touch()
        param_text = "\n".join(f"  {k}: {v}" for k, v in mem.parameters.items())
        full_text = (
            f"Project: {mem.project}\n"
            f"Simulation: {mem.simulation_name or 'unnamed'}\n"
            f"Coherence: {mem.coherence_state}\n"
            f"Lattice: {mem.lattice_type or 'unspecified'}\n"
            + (
                f"Observable: {mem.observable} = {mem.observable_value} {mem.units or ''}\n"
                if mem.observable
                else ""
            )
            + (f"Parameters:\n{param_text}" if param_text else "")
            + (f"\nNotes: {mem.notes}" if mem.notes else "")
        )
        self.vector.upsert(
            collection="quantum",
            doc_id=mem.id,
            text=full_text,
            metadata={
                "project": mem.project,
                "coherence_state": str(mem.coherence_state),
                "lattice_type": mem.lattice_type or "",
                "experiment_id": mem.experiment_id or "",
                "qubit_count": str(mem.qubit_count or ""),
                "timestamp": mem.timestamp.isoformat(),
            },
        )
        return mem.id

    def write_identity(self, mem: IdentityMemory) -> str:
        """Persist an IdentityMemory (ICX ⊗ SIX — SoulJourney pipeline state)."""
        mem.touch()
        full_text = (
            f"BlissID: {mem.bliss_id}\n"
            f"SoulHash: {mem.soul_hash}\n"
            f"Stage: {mem.souljourney_stage}\n"
            f"Content: {mem.content}"
        )
        self.vector.upsert(
            collection="identity",
            doc_id=mem.id,
            text=full_text,
            metadata={
                "project": mem.project,
                "bliss_id": mem.bliss_id,
                "soul_hash": mem.soul_hash,
                "stage": str(mem.souljourney_stage),
                "immutable": str(mem.immutable),
                "timestamp": mem.timestamp.isoformat(),
            },
        )
        return mem.id

    def store_workflow(self, mem: ProceduralMemory) -> str:
        """Persist a ProceduralMemory (ICX ⊗ SCX — repeatable workflow)."""
        mem.touch()
        steps_text = "\n".join(f"{i + 1}. {s}" for i, s in enumerate(mem.steps))
        full_text = f"Task: {mem.task}\n\nSteps:\n{steps_text}"
        self.vector.upsert(
            collection="procedural",
            doc_id=mem.id,
            text=full_text,
            metadata={
                "project": mem.project,
                "task": mem.task,
                "frequency": str(mem.frequency),
                "success_rate": str(mem.success_rate),
                "tools": str(mem.tools_required),
                "timestamp": mem.timestamp.isoformat(),
            },
        )
        if self._mirror_guard("store_workflow"):
            self.memori.mirror_procedural(
                task=mem.task,
                steps=mem.steps,
                frequency=mem.frequency,
                success_rate=mem.success_rate,
            )
        return mem.id

    def store_governance(self, mem: GovernanceMemory) -> str:
        """Persist a GovernanceMemory (ICX ⊗ ICX — voting, mandate, Archivus)."""
        mem.touch()
        full_text = f"Title: {mem.title}\nProject: {mem.project}\nType: {mem.record_type}\n\n{mem.content}"
        self.vector.upsert(
            collection="governance",
            doc_id=mem.id,
            text=full_text,
            metadata={
                "project": mem.project,
                "record_type": mem.record_type,
                "outcome": mem.outcome or "",
                "immutable": str(mem.immutable),
                "ledger_hash": mem.ledger_hash or "",
                "archivus_block_ref": mem.archivus_block_ref or "",
                "timestamp": mem.timestamp.isoformat(),
            },
        )
        return mem.id

    def store_creative(self, mem: CreativeMemory) -> str:
        """Persist a CreativeMemory (media / narrative entry)."""
        mem.touch()
        full_text = f"Title: {mem.title or 'Untitled'}\nMedium: {mem.medium}\n\n{mem.content}"
        self.vector.upsert(
            collection="creative",
            doc_id=mem.id,
            text=full_text,
            metadata={
                "project": mem.project,
                "medium": mem.medium,
                "status": mem.status,
                "language": mem.language,
                "tags": str(mem.tags),
                "timestamp": mem.timestamp.isoformat(),
            },
        )
        return mem.id

    # ── Generic Upsert Dispatch ───────────────────────────────────────────────

    def upsert(self, request: UpsertMemoryRequest) -> str:
        """Route a generic UpsertMemoryRequest to the correct typed write method."""
        base = {
            "project": request.project,
            "content": request.content,
            "tags": request.tags,
            "session_id": request.session_id,
            "llm": request.llm,
            **request.extra,
        }
        mt = MemoryType(request.memory_type)

        dispatch: Dict[MemoryType, Any] = {
            MemoryType.SENSORY: (SensoryMemory, self.write_sensory),
            MemoryType.WORKING: (WorkingMemory, self.write_working),
            MemoryType.EPISODIC: (EpisodicMemory, self.write_event),
            MemoryType.SEMANTIC: (SemanticMemory, self.embed_document),
            MemoryType.META: (MetaMemory, self.store_fact),
            MemoryType.QUANTUM: (QuantumMemory, self.store_quantum),
            MemoryType.IDENTITY: (IdentityMemory, self.write_identity),
            MemoryType.PROCEDURAL: (ProceduralMemory, self.store_workflow),
            MemoryType.GOVERNANCE: (GovernanceMemory, self.store_governance),
            MemoryType.CREATIVE: (CreativeMemory, self.store_creative),
        }

        if mt not in dispatch:
            raise ValueError(f"Unsupported memory_type in upsert: '{mt}'")

        schema_cls, write_fn = dispatch[mt]
        try:
            mem = schema_cls(**base)
        except Exception as exc:
            raise ValueError(
                f"Schema validation failed for {mt}: {exc}"
            ) from exc

        return write_fn(mem)

    def bulk_upsert(self, request: BulkUpsertRequest) -> Dict[str, Any]:
        """Batch upsert with dry-run support."""
        written: List[str] = []
        errors: List[tuple] = []

        for i, rec in enumerate(request.records):
            try:
                if not request.dry_run:
                    mem_id = self.upsert(rec)
                    written.append(mem_id)
                else:
                    written.append(f"dry_run_valid_{i}")
            except Exception as exc:
                errors.append((i, str(exc)))

        return {"written": written, "errors": errors, "dry_run": request.dry_run}

    # ── Read: Context Assembly with SUXS-IFO Contraction ──────────────────────

    def read_context(
        self,
        project: str,
        llm: str | LLMProvider = _DEFAULT_LLM,
        session_id: Optional[str] = None,
        top_k: int = _CONTEXT_RESULTS,
    ) -> ContextResponse:
        """
        Assemble and contract full 9-cell TSLCA context payload for the project.
        """
        self._active_sessions += 1

        def _query(col: str) -> List[Dict[str, Any]]:
            try:
                return self.vector.query(col, query_text=project, n_results=top_k)
            except Exception as exc:
                log.warning("[read_context] query failed for '%s': %s", col, exc)
                return []

        sensory_raw = _query("sensory")
        working_raw = _query("working")
        episodic_raw = _query("episodic")
        semantic_raw = _query("semantic")
        meta_raw = _query("meta")
        quantum_raw = _query("quantum")
        identity_raw = _query("identity")
        procedural_raw = _query("procedural")
        governance_raw = _query("governance")
        creative_raw = _query("creative")

        meta = self._project_meta(project)
        active_volumes: List[str] = (
            [str(v) for v in meta.active_volumes] if meta else []
        )
        active_axioms: List[str] = list(meta.active_axioms) if meta else []
        active_dualities: List[str] = list(meta.active_dualities) if meta else []

        duality_pairs: List[DualityPair] = []
        for ds in active_dualities:
            try:
                duality_pairs.append(DualityPair.from_string(ds))
            except ValueError:
                pass

        related_projects: List[str] = [
            key
            for key, pm in self.projects.items()
            if key != project and bool(set(pm.active_dualities) & set(active_dualities))
        ]

        total = (
            len(sensory_raw)
            + len(working_raw)
            + len(episodic_raw)
            + len(semantic_raw)
            + len(meta_raw)
            + len(quantum_raw)
            + len(identity_raw)
            + len(procedural_raw)
            + len(governance_raw)
            + len(creative_raw)
        )

        self._active_sessions = max(0, self._active_sessions - 1)

        return ContextResponse(
            project=project,
            project_meta=meta,
            llm=self._resolve_llm(llm),
            session_id=session_id,
            # 9 TSLCA memory layers
            sensory=sensory_raw,
            working=working_raw,
            episodic=episodic_raw,
            semantic=semantic_raw,
            meta=meta_raw,
            quantum=quantum_raw,
            identity=identity_raw,
            procedural=procedural_raw,
            governance=governance_raw,
            creative=creative_raw,
            # Invariants
            active_volumes=active_volumes,
            active_axioms=active_axioms,
            active_dualities=active_dualities,
            invariants=self.global_invariants,
            duality_pairs=duality_pairs,
            related_projects=related_projects,
            total_memories=total,
        )

    # ── Read: Structured Query ────────────────────────────────────────────────

    def query(self, request: MemoryQuery) -> List[MemorySearchResult]:
        """Execute a structured search across the specified TSL collections."""
        results: List[MemorySearchResult] = []
        collection_map: Dict[MemoryType, str] = {
            MemoryType.SENSORY: "sensory",
            MemoryType.WORKING: "working",
            MemoryType.EPISODIC: "episodic",
            MemoryType.SEMANTIC: "semantic",
            MemoryType.META: "meta",
            MemoryType.QUANTUM: "quantum",
            MemoryType.IDENTITY: "identity",
            MemoryType.PROCEDURAL: "procedural",
            MemoryType.GOVERNANCE: "governance",
            MemoryType.CREATIVE: "creative",
        }

        for mt in request.memory_types:
            collection = collection_map.get(mt)
            if not collection:
                continue
            try:
                raw = self.vector.query(
                    collection,
                    query_text=request.query_text,
                    n_results=request.top_k,
                    project=request.project,
                )
            except Exception as exc:
                log.warning("[query] collection '%s' failed: %s", collection, exc)
                continue

            for r in raw:
                score = float(r.get("score", 0.0))
                if score < request.min_score:
                    continue
                payload = r.get("payload", {})
                is_deprecated = payload.get("deprecated") in (True, "True", "true")
                if not request.include_deprecated and is_deprecated:
                    continue

                results.append(
                    MemorySearchResult(
                        memory_id=str(r.get("id", "")),
                        memory_type=mt,
                        project=payload.get("project", request.project or "memoree"),
                        score=min(max(score, 0.0), 1.0),
                        content_preview=str(payload.get("text", ""))[:300],
                        tags=payload.get("tags", [])
                        if isinstance(payload.get("tags"), list)
                        else [],
                        created_at=_now(),
                        tier=MemoryTier.WARM,
                        deprecated=is_deprecated,
                        superseded_by=payload.get("superseded_by"),
                    )
                )

        results.sort(key=lambda x: x.score, reverse=True)
        return results[: request.top_k]

    # ── Heartbeat Curing & Curing Buffer ──────────────────────────────────────

    def cure_working_buffer(self) -> Dict[str, int]:
        """Cure active working memories into permanent episodic/semantic layers."""
        cured_count = 0
        uncured: List[WorkingMemory] = []
        for mem in self._working_buffer:
            if not mem.cured:
                # Cure into episodic turn
                ep_mem = EpisodicMemory(
                    session_id=mem.session_id,
                    project=mem.project,
                    role="assistant",
                    content=f"[Cured from Working Buffer]: {mem.content}",
                    tags=mem.tags + ["cured_working"],
                )
                self.write_event(ep_mem)
                mem.cured = True
                mem.cured_into_type = MemoryType.EPISODIC
                mem.cured_into_id = ep_mem.id
                cured_count += 1
            else:
                uncured.append(mem)

        self._working_buffer = uncured
        return {"cured_count": cured_count, "remaining_buffer": len(self._working_buffer)}

    # ── Diagnostics & Lattice Snapshot ────────────────────────────────────────

    def diagnostics(self) -> MemoreeDiagnostics:
        """Return a live MemoreeDiagnostics snapshot."""
        uptime = (datetime.now(tz=timezone.utc) - self._start_time).total_seconds()
        diag = self.vector.get_diagnostics()

        return MemoreeDiagnostics(
            status="healthy" if diag.get("qdrant_connected") else "degraded",
            version="4.0.0",
            uptime_seconds=round(uptime, 2),
            qdrant_connected=diag.get("qdrant_connected", False),
            qdrant_collections=diag.get("collections", []),
            active_sessions=self._active_sessions,
            llm_hooks_active=[p for p in LLMProvider if p != LLMProvider.UNKNOWN],
        )

    def get_lattice_snapshot(self) -> Dict[str, Any]:
        """Return a structured 3x3 Cognitive Field Tensor snapshot."""
        snapshot = LatticeSnapshot.create_empty()
        # Populate counts from vector backend
        total = 0
        for cell_key, cell_state in snapshot.cells.items():
            col_name = cell_state.memory_type.value
            results = self.vector.query(col_name, query_text="", n_results=100)
            cell_state.count = len(results)
            total += len(results)

        six_six = snapshot.cells.get("SIX⊗SIX")
        scx_scx = snapshot.cells.get("SCX⊗SCX")
        icx_icx = snapshot.cells.get("ICX⊗ICX")
        trace = (
            (six_six.count if six_six else 0)
            + (scx_scx.count if scx_scx else 0)
            + (icx_icx.count if icx_icx else 0)
        )
        snapshot.unified_field_trace = float(trace)


        return {
            "version": "4.0.0",
            "timestamp": snapshot.timestamp.isoformat(),
            "unified_field_trace": snapshot.unified_field_trace,
            "total_memories": snapshot.total_memories,
            "cells": {
                k: {
                    "core_i": v.core_i,
                    "core_j": v.core_j,
                    "memory_type": v.memory_type.value,
                    "count": v.count,
                    "mean_hif": v.mean_hif,
                }
                for k, v in snapshot.cells.items()
            },
        }

    # ── Project Registry Accessors ────────────────────────────────────────────

    def list_projects(self) -> List[ProjectMeta]:
        return sorted(self.projects.values(), key=lambda p: p.key)

    def get_project(self, key: str) -> Optional[ProjectMeta]:
        return self._project_meta(key)

    def projects_by_owner(self, owner: ProjectOwner | str) -> List[ProjectMeta]:
        o = ProjectOwner(owner) if isinstance(owner, str) else owner
        return [p for p in self.projects.values() if ProjectOwner(p.owner) == o]

    def projects_by_duality(self, duality: str) -> List[ProjectMeta]:
        return [
            p
            for p in self.projects.values()
            if any(duality in d for d in p.active_dualities)
        ]
