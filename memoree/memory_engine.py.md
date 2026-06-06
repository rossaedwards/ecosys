memory_engine.py.md
"""
Memoree — Memory Engine
═══════════════════════════════════════════════════════════════════════════════
Orchestrates all storage backends and assembles full context payloads.
AuraFS is disabled pending integration — all AuraFS call-sites are preserved
as commented stubs so the re-enable is a one-line uncomment per method.

  Path   : c:\\memoree\\memory_engine.py
  Owner  : Ross Edwards / Aurphyx LLC
  GitHub : rossaedwards | aurphyx
  ORCiD  : 0009-0008-0539-1289

Backend Map
───────────────────────────────────────────────────────────────────────────────
  VectorBackend   → Chroma / Qdrant  (always active)
  MemoriBridge    → Aurphyx Memori REST mirror  (toggled by config mirror flag)
  AuraFSBackend   → AuraFS shard layer  [DISABLED — awaiting integration]

projects.json powers
───────────────────────────────────────────────────────────────────────────────
  • ProjectMeta hydration on context assembly
  • Active axiom / duality injection into ContextResponse
  • Default project / LLM resolution via global_settings
═══════════════════════════════════════════════════════════════════════════════
f0rg3d in l0v3 by Ross Edwards
"""

from __future__ import annotations

import json
import logging
import os
import yaml
from datetime import datetime, timezone
from pathlib import Path
from typing import Any, Dict, List, Optional

from schemas import (
    # Core memory types
    EpisodicMemory,
    SemanticMemory,
    ProceduralMemory,
    MetaMemory,
    QuantumMemory,
    CreativeMemory,
    GovernanceMemory,
    AnyMemory,
    # Domain objects
    ProjectMeta,
    ProjectOwner,
    VolumeRef,
    DualityPair,
    # Context & session
    ContextResponse,
    ThreadSummary,
    # Search
    MemoryQuery,
    MemorySearchResult,
    # Upsert
    UpsertMemoryRequest,
    BulkUpsertRequest,
    # Diagnostics
    MemoreeDiagnostics,
    # Enums
    MemoryType,
    LLMProvider,
    MemoryTier,
)

# from aurafs_backend import AuraFSBackend   # [DISABLED] awaiting AuraFS integration
from vector_backend import VectorBackend
from memori_bridge import MemoriBridge

log = logging.getLogger("memoree.engine")

# ─────────────────────────────────────────────────────────────────────────────
# Constants
# ─────────────────────────────────────────────────────────────────────────────

BASE_DIR        = Path(__file__).parent
CONFIG_PATH     = BASE_DIR / "config.yaml"
PROJECTS_PATH   = BASE_DIR / "projects.json"
DUALITIES_PATH  = BASE_DIR / "dualities.json"
INVARIANTS_PATH = BASE_DIR / "invariants.json"

_DEFAULT_CHROMA_DIR   = str(BASE_DIR / "embeddings")
_DEFAULT_EMBED_MODEL  = "all-MiniLM-L6-v2"
_DEFAULT_PROJECT      = "memoree"
_DEFAULT_LLM          = "supergrok"
_CONTEXT_RESULTS      = 5   # default top-k per collection in read_context


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
    """
    Hydrate a ProjectMeta from a single projects.json entry.

    VolumeRef values that are not recognised are silently dropped so a
    stale projects.json never hard-crashes the engine on startup.
    """
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
    Central orchestrator for all Memoree read and write operations.

    Responsibilities
    ────────────────
    • Write all seven memory types to VectorBackend (and optionally mirror
      to MemoriBridge).
    • Assemble full ContextResponse payloads enriched with ProjectMeta,
      active axioms, active dualities, and cross-project references pulled
      from projects.json, dualities.json, and invariants.json.
    • Expose a type-safe bulk upsert path and a structured query interface.
    • Surface a MemoreeDiagnostics snapshot for the /diagnostics route.

    AuraFS stubs are preserved throughout.  Re-enabling is a one-line
    uncomment per method once aurafs_backend.py is integrated.
    """

    # ── Lifecycle ────────────────────────────────────────────────────────────

    def __init__(
        self,
        config_path: Path | str = CONFIG_PATH,
        projects_path: Path | str = PROJECTS_PATH,
    ) -> None:
        cfg = _load_yaml(Path(config_path))
        self._cfg = cfg

        # ── Vector backend ───────────────────────────────────────────────────
        chroma_dir  = cfg.get("chroma", {}).get("persist_dir",  _DEFAULT_CHROMA_DIR)
        embed_model = cfg.get("chroma", {}).get("embedding_model", _DEFAULT_EMBED_MODEL)
        self.vector = VectorBackend(persist_dir=chroma_dir, model_name=embed_model)

        # ── MemoriBridge (optional mirror) ───────────────────────────────────
        self.mirror: bool = cfg.get("backend", {}).get("mirror_to_primary", True)
        self.memori = MemoriBridge()

        # ── AuraFS (DISABLED) ────────────────────────────────────────────────
        # aurafs_root  = cfg.get("aurafs", {}).get("root", "~/.memoree")
        # self.aurafs  = AuraFSBackend(root_path=aurafs_root)

        # ── Project registry ─────────────────────────────────────────────────
        raw_projects = _load_json(Path(projects_path))
        global_settings: Dict[str, Any] = raw_projects.get("global_settings", {})

        self.default_project: str = global_settings.get("default_project", _DEFAULT_PROJECT)
        self.default_llm:     str = global_settings.get("default_llm",     _DEFAULT_LLM)
        self.context_window_days: int = int(global_settings.get("context_window_days", 30))

        self.projects: Dict[str, ProjectMeta] = {
            key: _parse_project_meta(key, val)
            for key, val in raw_projects.get("projects", {}).items()
        }

        # ── Dualities & invariants ────────────────────────────────────────────
        raw_dualities  = _load_json(DUALITIES_PATH)
        raw_invariants = _load_json(INVARIANTS_PATH)
        self.global_dualities:  List[str] = raw_dualities.get("dualities",  [])
        self.global_invariants: List[str] = raw_invariants.get("invariants", [])

        # ── Runtime counters ─────────────────────────────────────────────────
        self._start_time: datetime = datetime.now(tz=timezone.utc)
        self._active_sessions: int = 0

        log.info(
            "[MemoryEngine] Initialized — %d projects loaded | AuraFS disabled | mirror=%s",
            len(self.projects),
            self.mirror,
        )

    # ── Internal helpers ──────────────────────────────────────────────────────

    def _project_meta(self, project: str) -> Optional[ProjectMeta]:
        """Return ProjectMeta for the given key, or None if not registered."""
        meta = self.projects.get(project)
        if meta is None:
            log.debug("Project key '%s' not found in projects.json", project)
        return meta

    def _resolve_llm(self, llm: str | LLMProvider) -> LLMProvider:
        """Coerce a raw string or LLMProvider into a validated LLMProvider."""
        try:
            return LLMProvider(llm) if isinstance(llm, str) else llm
        except ValueError:
            log.warning("Unknown LLM provider '%s' — falling back to UNKNOWN", llm)
            return LLMProvider.UNKNOWN

    def _mirror_guard(self, fn_name: str) -> bool:
        """Returns True (and logs) only when mirroring is active."""
        if self.mirror:
            log.debug("[mirror] %s → MemoriBridge", fn_name)
        return self.mirror

    # ── Write: Core Memory Types ──────────────────────────────────────────────

    def write_event(self, mem: EpisodicMemory) -> str:
        """
        Persist an EpisodicMemory (single conversation turn).

        Vector index key: collection='episodic'
        Metadata stored: session_id, project, role, llm, turn_index, timestamp
        """
        mem.touch()
        # self.aurafs.write_episodic(mem)   # [DISABLED]
        self.vector.upsert(
            collection="episodic",
            doc_id=mem.id,
            text=mem.content,
            metadata={
                "session_id":  mem.session_id,
                "project":     mem.project,
                "role":        mem.role,
                "llm":         mem.llm,
                "turn_index":  str(mem.turn_index),
                "intent":      mem.intent or "",
                "timestamp":   mem.timestamp.isoformat(),
            },
        )
        if self._mirror_guard("write_event"):
            self.memori.mirror_episodic(
                role=mem.role,
                content=mem.content,
                llm=mem.llm,
            )
        log.debug("[write_event] %s  project=%s  session=%s", mem.id, mem.project, mem.session_id)
        return mem.id

    def embed_document(self, mem: SemanticMemory) -> str:
        """
        Persist a SemanticMemory (project knowledge unit).

        The vectorised text prefixes project/category/tags for richer
        nearest-neighbour retrieval at context assembly time.
        """
        mem.touch()
        # self.aurafs.write_semantic(mem)   # [DISABLED]
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
                "project":    mem.project,
                "category":   mem.category,
                "subcategory": mem.subcategory or "",
                "tags":       str(mem.tags),
                "confidence": str(mem.confidence),
                "timestamp":  mem.timestamp.isoformat(),
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
        log.debug("[embed_document] %s  project=%s  category=%s", mem.id, mem.project, mem.category)
        return mem.id

    def store_workflow(self, mem: ProceduralMemory) -> str:
        """
        Persist a ProceduralMemory (repeatable workflow / automation recipe).

        Steps are numbered and concatenated for full-text semantic search.
        Pre/postconditions and required tools are included in metadata.
        """
        mem.touch()
        # self.aurafs.write_procedural(mem)   # [DISABLED]
        steps_text = "\n".join(f"{i + 1}. {s}" for i, s in enumerate(mem.steps))
        pre_text   = "\n".join(f"• {p}" for p in mem.preconditions)  if mem.preconditions  else ""
        post_text  = "\n".join(f"• {p}" for p in mem.postconditions) if mem.postconditions else ""
        full_text  = (
            f"Task: {mem.task}\n\n"
            f"Steps:\n{steps_text}"
            + (f"\n\nPreconditions:\n{pre_text}"  if pre_text  else "")
            + (f"\n\nPostconditions:\n{post_text}" if post_text else "")
        )
        self.vector.upsert(
            collection="procedural",
            doc_id=mem.id,
            text=full_text,
            metadata={
                "project":      mem.project,
                "task":         mem.task,
                "frequency":    str(mem.frequency),
                "success_rate": str(mem.success_rate),
                "tools":        str(mem.tools_required),
            },
        )
        if self._mirror_guard("store_workflow"):
            self.memori.mirror_procedural(
                task=mem.task,
                steps=mem.steps,
                frequency=mem.frequency,
                success_rate=mem.success_rate,
            )
        log.debug("[store_workflow] %s  project=%s  task=%s", mem.id, mem.project, mem.task)
        return mem.id

    def store_fact(self, mem: MetaMemory) -> str:
        """
        Persist a MetaMemory (verified fact / cross-session invariant).

        Deprecated records are still written — they form the audit trail
        and are filtered at query time via `include_deprecated=False`.
        """
        mem.touch()
        # self.aurafs.write_meta(mem)   # [DISABLED]
        self.vector.upsert(
            collection="meta",
            doc_id=mem.id,
            text=mem.fact,
            metadata={
                "project":    mem.project or "global",
                "verified":   str(mem.verified),
                "confidence": str(mem.confidence),
                "deprecated": str(mem.deprecated),
                "timestamp":  mem.timestamp.isoformat(),
            },
        )
        if self._mirror_guard("store_fact"):
            self.memori.mirror_meta(
                fact=mem.fact,
                confidence=mem.confidence,
                sources=mem.sources,
                verified=mem.verified,
            )
        log.debug("[store_fact] %s  project=%s  verified=%s", mem.id, mem.project, mem.verified)
        return mem.id

    def store_quantum(self, mem: QuantumMemory) -> str:
        """
        Persist a QuantumMemory (physics / simulation state snapshot).

        Observable, coherence state, and lattice type are all stored in
        metadata for structured filtering independent of vector similarity.
        """
        mem.touch()
        # self.aurafs.write_quantum(mem)   # [DISABLED]
        param_text = "\n".join(f"  {k}: {v}" for k, v in mem.parameters.items())
        full_text  = (
            f"Project: {mem.project}\n"
            f"Simulation: {mem.simulation_name or 'unnamed'}\n"
            f"Coherence: {mem.coherence_state}\n"
            f"Lattice: {mem.lattice_type or 'unspecified'}\n"
            + (f"Observable: {mem.observable} = {mem.observable_value} {mem.units or ''}\n" if mem.observable else "")
            + (f"Parameters:\n{param_text}" if param_text else "")
            + (f"\nNotes: {mem.notes}" if mem.notes else "")
        )
        self.vector.upsert(
            collection="quantum",
            doc_id=mem.id,
            text=full_text,
            metadata={
                "project":         mem.project,
                "coherence_state": str(mem.coherence_state),
                "lattice_type":    mem.lattice_type or "",
                "experiment_id":   mem.experiment_id or "",
                "qubit_count":     str(mem.qubit_count or ""),
                "timestamp":       mem.timestamp.isoformat(),
            },
        )
        # MemoriBridge does not yet support quantum mirroring — stub reserved
        log.debug("[store_quantum] %s  project=%s  state=%s", mem.id, mem.project, mem.coherence_state)
        return mem.id

    def store_creative(self, mem: CreativeMemory) -> str:
        """
        Persist a CreativeMemory (art, music, narrative, divination entry).

        Title, universe, and medium are prepended to the content vector
        so creative search operates on both context and raw content.
        """
        mem.touch()
        # self.aurafs.write_creative(mem)   # [DISABLED]
        full_text = (
            (f"Title: {mem.title}\n" if mem.title else "")
            + f"Medium: {mem.medium}\n"
            + (f"Universe: {mem.universe}\n" if mem.universe else "")
            + f"Project: {mem.project}\n\n"
            + mem.content
        )
        self.vector.upsert(
            collection="creative",
            doc_id=mem.id,
            text=full_text,
            metadata={
                "project":  mem.project,
                "medium":   mem.medium,
                "status":   mem.status,
                "language": mem.language,
                "tags":     str(mem.tags),
                "timestamp": mem.timestamp.isoformat(),
            },
        )
        log.debug("[store_creative] %s  project=%s  medium=%s", mem.id, mem.project, mem.medium)
        return mem.id

    def store_governance(self, mem: GovernanceMemory) -> str:
        """
        Persist a GovernanceMemory (vote, policy, mandate, ledger entry).

        Immutable records (ILS-archived) are written once and must never
        be called again with the same ID — the backend will reject the
        overwrite if the record already exists with immutable=True.
        """
        mem.touch()
        # self.aurafs.write_governance(mem)   # [DISABLED]
        full_text = (
            f"Title: {mem.title}\n"
            f"Project: {mem.project}\n"
            f"Type: {mem.record_type}\n\n"
            f"{mem.content}"
        )
        self.vector.upsert(
            collection="governance",
            doc_id=mem.id,
            text=full_text,
            metadata={
                "project":      mem.project,
                "record_type":  mem.record_type,
                "outcome":      mem.outcome or "",
                "immutable":    str(mem.immutable),
                "ledger_hash":  mem.ledger_hash or "",
                "timestamp":    mem.timestamp.isoformat(),
            },
        )
        log.debug(
            "[store_governance] %s  project=%s  type=%s  immutable=%s",
            mem.id, mem.project, mem.record_type, mem.immutable,
        )
        return mem.id

    # ── Generic Upsert ────────────────────────────────────────────────────────

    def upsert(self, request: UpsertMemoryRequest) -> str:
        """
        Route a generic UpsertMemoryRequest to the correct typed write method.

        `request.extra` is unpacked and merged with the core fields so callers
        can pass specialised fields (e.g. `qubit_count` for QuantumMemory)
        without a separate endpoint per memory class.
        """
        base = {
            "project":    request.project,
            "content":    request.content,
            "tags":       request.tags,
            "session_id": request.session_id,
            "llm":        request.llm,
            **request.extra,
        }
        mt = MemoryType(request.memory_type)

        dispatch: Dict[MemoryType, Any] = {
            MemoryType.EPISODIC:   (EpisodicMemory,   self.write_event),
            MemoryType.SEMANTIC:   (SemanticMemory,   self.embed_document),
            MemoryType.PROCEDURAL: (ProceduralMemory, self.store_workflow),
            MemoryType.META:       (MetaMemory,       self.store_fact),
            MemoryType.QUANTUM:    (QuantumMemory,    self.store_quantum),
            MemoryType.CREATIVE:   (CreativeMemory,   self.store_creative),
            MemoryType.GOVERNANCE: (GovernanceMemory, self.store_governance),
        }

        if mt not in dispatch:
            raise ValueError(f"Unsupported memory_type in upsert: '{mt}'")

        schema_cls, write_fn = dispatch[mt]
        try:
            mem = schema_cls(**base)
        except Exception as exc:
            raise ValueError(
                f"Schema validation failed for {mt} — check 'extra' fields: {exc}"
            ) from exc

        return write_fn(mem)

    def bulk_upsert(self, request: BulkUpsertRequest) -> Dict[str, Any]:
        """
        Batch upsert with optional dry-run validation.

        Returns a result dict with keys:
            written  → list of memory IDs successfully written
            errors   → list of (index, error_message) tuples
            dry_run  → bool echoing the request flag
        """
        written: List[str]         = []
        errors:  List[tuple]       = []

        for i, rec in enumerate(request.records):
            try:
                if not request.dry_run:
                    mem_id = self.upsert(rec)
                    written.append(mem_id)
                else:
                    # Validate only — instantiate the schema but don't write
                    self.upsert.__func__  # ensure method is reachable
                    base = {"project": rec.project, "content": rec.content,
                            "tags": rec.tags, **rec.extra}
                    schema_cls = {
                        MemoryType.EPISODIC:   EpisodicMemory,
                        MemoryType.SEMANTIC:   SemanticMemory,
                        MemoryType.PROCEDURAL: ProceduralMemory,
                        MemoryType.META:       MetaMemory,
                        MemoryType.QUANTUM:    QuantumMemory,
                        MemoryType.CREATIVE:   CreativeMemory,
                        MemoryType.GOVERNANCE: GovernanceMemory,
                    }[MemoryType(rec.memory_type)]
                    schema_cls(**base)
                    written.append(f"dry_run_valid_{i}")
            except Exception as exc:
                errors.append((i, str(exc)))
                log.warning("[bulk_upsert] record[%d] failed: %s", i, exc)

        log.info(
            "[bulk_upsert] project=%s  written=%d  errors=%d  dry_run=%s",
            request.project, len(written), len(errors), request.dry_run,
        )
        return {"written": written, "errors": errors, "dry_run": request.dry_run}

    # ── Read: Context Assembly ────────────────────────────────────────────────

    def read_context(
        self,
        project:    str,
        llm:        str | LLMProvider = _DEFAULT_LLM,
        session_id: Optional[str] = None,
        top_k:      int = _CONTEXT_RESULTS,
    ) -> ContextResponse:
        """
        Assemble a full ContextResponse for the given project.

        Layers assembled in order:
          1. Vector queries across all seven collections (episodic → governance)
          2. ProjectMeta hydrated from projects.json
          3. Active axioms, dualities, and volumes from ProjectMeta
          4. DualityPair objects parsed from active_dualities strings
          5. Related projects identified by shared domain/duality overlap
          6. Global invariants from invariants.json

        The resulting payload is ready for direct injection into any
        LLM system-prompt hook (perplexity_hook, supergrok_hook, etc.).
        """
        self._active_sessions += 1

        # ── Vector recall ────────────────────────────────────────────────────
        def _query(collection: str) -> List[Dict[str, Any]]:
            try:
                return self.vector.query(collection, query_text=project, n_results=top_k)
            except Exception as exc:
                log.warning("[read_context] vector query failed for '%s': %s", collection, exc)
                return []

        episodic_raw   = _query("episodic")
        semantic_raw   = _query("semantic")
        procedural_raw = _query("procedural")
        meta_raw       = _query("meta")
        quantum_raw    = _query("quantum")
        creative_raw   = _query("creative")
        governance_raw = _query("governance")

        # ── Project metadata ─────────────────────────────────────────────────
        meta = self._project_meta(project)

        active_volumes:   List[str] = [str(v) for v in meta.active_volumes]   if meta else []
        active_axioms:    List[str] = list(meta.active_axioms)                 if meta else []
        active_dualities: List[str] = list(meta.active_dualities)              if meta else []

        # ── DualityPair objects ──────────────────────────────────────────────
        duality_pairs: List[DualityPair] = []
        for ds in active_dualities:
            try:
                duality_pairs.append(DualityPair.from_string(ds))
            except ValueError as exc:
                log.debug("[read_context] duality parse skip: %s", exc)

        # ── Related projects (shared duality or domain) ───────────────────────
        related_projects: List[str] = [
            key for key, pm in self.projects.items()
            if key != project
            and bool(set(pm.active_dualities) & set(active_dualities))
        ]

        total = (
            len(episodic_raw) + len(semantic_raw) + len(procedural_raw)
            + len(meta_raw)   + len(quantum_raw)  + len(creative_raw)
            + len(governance_raw)
        )

        self._active_sessions = max(0, self._active_sessions - 1)

        return ContextResponse(
            project=project,
            project_meta=meta,
            llm=self._resolve_llm(llm),
            session_id=session_id,
            # Memory layers
            episodic=episodic_raw,
            semantic=semantic_raw,
            procedural=procedural_raw,
            meta=meta_raw,
            quantum=quantum_raw,
            creative=creative_raw,
            governance=governance_raw,
            # Active invariants
            active_volumes=active_volumes,
            active_axioms=active_axioms,
            active_dualities=active_dualities,
            invariants=self.global_invariants,
            duality_pairs=duality_pairs,
            # Cross-project
            related_projects=related_projects,
            # Diagnostics
            total_memories=total,
        )

    # ── Read: Structured Query ────────────────────────────────────────────────

    def query(self, request: MemoryQuery) -> List[MemorySearchResult]:
        """
        Execute a structured MemoryQuery across specified collections.

        Filters by project, memory type, minimum similarity score, and
        deprecation status. Returns a list of MemorySearchResult sorted
        by descending score.
        """
        results: List[MemorySearchResult] = []

        collection_map: Dict[MemoryType, str] = {
            MemoryType.EPISODIC:   "episodic",
            MemoryType.SEMANTIC:   "semantic",
            MemoryType.PROCEDURAL: "procedural",
            MemoryType.META:       "meta",
            MemoryType.QUANTUM:    "quantum",
            MemoryType.CREATIVE:   "creative",
            MemoryType.GOVERNANCE: "governance",
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
                    filters=request.filters,
                )
            except Exception as exc:
                log.warning("[query] collection '%s' failed: %s", collection, exc)
                continue

            for r in raw:
                score = float(r.get("score", 0.0))
                if score < request.min_score:
                    continue
                if not request.include_deprecated and r.get("metadata", {}).get("deprecated") == "True":
                    continue
                if request.project and r.get("metadata", {}).get("project") != request.project:
                    continue

                results.append(
                    MemorySearchResult(
                        memory_id=r.get("id", ""),
                        memory_type=mt,
                        project=r.get("metadata", {}).get("project", request.project or "unknown"),
                        score=min(score, 1.0),
                        content_preview=str(r.get("document", ""))[:300],
                        tags=r.get("metadata", {}).get("tags", []),
                        created_at=datetime.now(tz=timezone.utc),
                        tier=MemoryTier.WARM,
                    )
                )

        results.sort(key=lambda x: x.score, reverse=True)
        return results[: request.top_k]

    # ── Diagnostics ───────────────────────────────────────────────────────────

    def diagnostics(self) -> MemoreeDiagnostics:
        """
        Return a live MemoreeDiagnostics snapshot.

        Called by the /diagnostics FastAPI route in routes.py.
        `uptime_seconds` is derived from the engine's own start timestamp
        so it reflects true daemon uptime, not process uptime.
        """
        uptime = (datetime.now(tz=timezone.utc) - self._start_time).total_seconds()

        qdrant_ok         = False
        qdrant_collections: List[str] = []
        try:
            qdrant_collections = self.vector.list_collections()
            qdrant_ok = True
        except Exception as exc:
            log.warning("[diagnostics] vector backend unreachable: %s", exc)

        return MemoreeDiagnostics(
            status="healthy" if qdrant_ok else "degraded",
            version="0.1.0",
            uptime_seconds=round(uptime, 2),
            qdrant_connected=qdrant_ok,
            qdrant_collections=qdrant_collections,
            active_sessions=self._active_sessions,
            llm_hooks_active=[p for p in LLMProvider if p != LLMProvider.UNKNOWN],
        )

    # ── Project Registry Helpers ──────────────────────────────────────────────

    def list_projects(self) -> List[ProjectMeta]:
        """Return all registered projects, sorted alphabetically by key."""
        return sorted(self.projects.values(), key=lambda p: p.key)

    def get_project(self, key: str) -> Optional[ProjectMeta]:
        """Return the ProjectMeta for `key`, or None if not registered."""
        return self._project_meta(key)

    def projects_by_owner(self, owner: ProjectOwner | str) -> List[ProjectMeta]:
        """Filter the project registry by owner (rossaedwards | aurphyx)."""
        o = ProjectOwner(owner) if isinstance(owner, str) else owner
        return [p for p in self.projects.values() if ProjectOwner(p.owner) == o]

    def projects_by_duality(self, duality: str) -> List[ProjectMeta]:
        """
        Return all projects whose active_dualities list contains `duality`.
        Accepts partial matches (e.g. 'coherence' matches 'coherence/decoherence').
        """
        return [
            p for p in self.projects.values()
            if any(duality in d for d in p.active_dualities)
        ]