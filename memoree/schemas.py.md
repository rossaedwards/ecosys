schemas.py.md
"""
Memoree — Core Memory Schemas
═══════════════════════════════════════════════════════════════════════════════
Sovereign memory substrate for the Aurphyx LLC ecosystem.

  Path   : c:\\memoree\\schemas.py
  Owner  : Ross Edwards / Aurphyx LLC
  GitHub : rossaedwards | aurphyx
  ORCiD  : 0009-0008-0539-1289

Memory Architecture
───────────────────
  ┌─ Episodic    → Conversation turns, session-bound interactions
  ├─ Semantic    → Project knowledge, facts, relationships, dualities
  ├─ Procedural  → Repeatable workflows, task recipes, automation sequences
  ├─ Meta        → Verified facts, confidence-tracked beliefs, axioms
  ├─ Quantum     → Physics/simulation state, lattice snapshots, coherence logs
  ├─ Creative    → Art, music, narrative, tarot, oracle, worldbuilding entries
  └─ Governance  → Voting records, policy decisions, guardian mandates (SAGES/GVS)

Project Registry
────────────────
  rossaedwards → memoree, ftqc, tvfd, vim, tslca, soulshot, rae,
                 majorana_1, zpe_core, rf_lovezme

  aurphyx      → aurafs, g0dm0d3, arora, audry, sages, standards,
                 ineffable, gvs, opulence, fuxyez, blissid, adore,
                 egophyx, chakra_datacore, duality_kernel, dataorb,
                 aints, aethornyx, aurphyx_casino, irra, uavrt,
                 auraorb, vibe_audio, symbiotic_channels, tarot,
                 oracle_deck, thirteen_month_calendar, aurafs_devices

Backend Registry (c:\\memoree\\)
────────────────────────────────
  memory_engine.py   → MemoryEngine orchestrator (Chroma + MemoriBridge)
  vector_backend.py  → Qdrant / Chroma vector persistence
  memori_bridge.py   → Aurphyx Memori REST mirror
  aurafs_backend.py  → AuraFS shard layer  [disabled — awaiting integration]
  routes.py          → FastAPI route handlers
  memoree_service.py → Uvicorn daemon entry point (port 7042)
  perplexity_hook.py → Perplexity AI context-injection hook
  supergrok_hook.py  → SuperGrok hook
  gemini_hook.py     → Gemini hook
  lmstudio_hook.py   → LM Studio / MCP hook
  heartbeat.py       → Daemon health pulse          [disabled]
  powersync_client.py→ PowerSync live-sync bridge
  memos_overlay.py   → Overlay renderer
  scaffold_memoree.py→ One-shot project scaffolder
  aurphyx_memori.py  → Memori API client
═══════════════════════════════════════════════════════════════════════════════
"""

from __future__ import annotations

import uuid
from datetime import datetime, timezone
from enum import Enum
from typing import Any, Dict, List, Literal, Optional, Union

from pydantic import BaseModel, Field, field_validator, model_validator


# ─────────────────────────────────────────────────────────────────────────────
# Helpers
# ─────────────────────────────────────────────────────────────────────────────

def _now() -> datetime:
    """UTC-aware timestamp factory — used by every schema default."""
    return datetime.now(tz=timezone.utc)


def _uuid() -> str:
    """UUID v4 factory."""
    return str(uuid.uuid4())


# ─────────────────────────────────────────────────────────────────────────────
# Enumerations
# ─────────────────────────────────────────────────────────────────────────────

class MemoryType(str, Enum):
    EPISODIC   = "episodic"
    SEMANTIC   = "semantic"
    PROCEDURAL = "procedural"
    META       = "meta"
    QUANTUM    = "quantum"
    CREATIVE   = "creative"
    GOVERNANCE = "governance"


class ProjectOwner(str, Enum):
    ROSS    = "rossaedwards"
    AURPHYX = "aurphyx"


class ProjectDomain(str, Enum):
    # Physics / Research
    QUANTUM_COMPUTING = "quantum_computing"
    THERMODYNAMICS    = "thermodynamics"
    VACUUM_DYNAMICS   = "vacuum_dynamics"
    COHERENCE_THEORY  = "coherence_theory"
    PHOTONICS         = "photonics"
    SIMULATION        = "simulation"
    # OS / Infrastructure
    OPERATING_SYSTEM  = "operating_system"
    FILE_SYSTEM       = "file_system"
    KERNEL            = "kernel"
    DAEMON            = "daemon"
    INTEGRATION       = "integration"
    HARDWARE          = "hardware"
    # AI / Intelligence
    AI_ORCHESTRATION  = "ai_orchestration"
    AI_COMPANION      = "ai_companion"
    AI_SECURITY       = "ai_security"
    MEMORY            = "memory"
    # Identity / Security
    IDENTITY          = "identity"
    BIOMETRICS        = "biometrics"
    CRYPTOGRAPHY      = "cryptography"
    # Governance / Society
    GOVERNANCE        = "governance"
    VOTING            = "voting"
    LEDGER            = "ledger"
    ECONOMICS         = "economics"
    ACCESSIBILITY     = "accessibility"
    TRANSPORTATION    = "transportation"
    # Creative / Media
    LANGUAGE          = "language"
    MUSIC             = "music"
    GAME              = "game"
    NARRATIVE         = "narrative"
    DIVINATION        = "divination"
    CALENDAR          = "calendar"
    # Standards / Protocol
    STANDARDS         = "standards"
    PROTOCOL          = "protocol"
    AUDIO             = "audio"
    NETWORK           = "network"


class LLMProvider(str, Enum):
    SUPERGROK  = "supergrok"
    PERPLEXITY = "perplexity"
    GEMINI     = "gemini"
    CLAUDE     = "claude"
    LMSTUDIO   = "lmstudio"
    OPENAI     = "openai"
    OLLAMA     = "ollama"
    UNKNOWN    = "unknown"


class MemoryTier(str, Enum):
    """Storage / retrieval priority tier."""
    HOT    = "hot"     # active session, in-memory
    WARM   = "warm"    # recent, fast-path vector
    COLD   = "cold"    # archived, batch retrieval
    FROZEN = "frozen"  # immutable ledger / audit trail


class VolumeRef(str, Enum):
    VOLUME_I   = "Volume_I"
    VOLUME_II  = "Volume_II"
    VOLUME_III = "Volume_III"


class ConfidenceLevel(str, Enum):
    VERIFIED   = "verified"    # peer-confirmed or cross-referenced
    HIGH       = "high"        # strong confidence, single source
    MEDIUM     = "medium"      # inferred or partially confirmed
    LOW        = "low"         # speculative or early hypothesis
    DEPRECATED = "deprecated"  # superseded, kept for audit trail


class CoherenceState(str, Enum):
    """Quantum / physics memory coherence descriptor."""
    COHERENT    = "coherent"
    DECOHERENT  = "decoherent"
    SUPERPOSED  = "superposed"
    ENTANGLED   = "entangled"
    COLLAPSED   = "collapsed"
    STABILIZING = "stabilizing"


class NodeTier(str, Enum):
    """AuraFS network participation tier."""
    SURVIVOR = "survivor"
    CITIZEN  = "citizen"
    TITAN    = "titan"


# ─────────────────────────────────────────────────────────────────────────────
# Base Model
# ─────────────────────────────────────────────────────────────────────────────

class AurphyxBase(BaseModel):
    """
    Common fields inherited by every Memoree memory schema.

    All timestamps are UTC-aware. The `version` field tracks schema
    migrations; bump it whenever a breaking field change is introduced.
    The optional `checksum` is a SHA-256 of the serialised record body
    and is populated by the storage backend after write.
    """

    id: str = Field(
        default_factory=_uuid,
        description="UUID v4 — globally unique memory identifier",
    )
    created_at:  datetime   = Field(default_factory=_now)
    updated_at:  datetime   = Field(default_factory=_now)
    memory_tier: MemoryTier = MemoryTier.WARM
    version:     int        = Field(default=1, ge=1, description="Schema version for migration tracking")
    checksum:    Optional[str] = Field(None, description="SHA-256 of serialized content (set by backend)")

    model_config = {
        "use_enum_values": True,
        "json_encoders": {datetime: lambda v: v.isoformat()},
        "populate_by_name": True,
    }

    def touch(self) -> None:
        """Refresh `updated_at` to now — call before every write."""
        self.updated_at = _now()


# ─────────────────────────────────────────────────────────────────────────────
# Domain Objects  (non-memory — used as embedded sub-schemas)
# ─────────────────────────────────────────────────────────────────────────────

class DualityPair(BaseModel):
    """
    A named duality — the foundational ontological unit of the Aurphyx framework.

    The `balance_coefficient` encodes the current tension between poles:
      0.0 → fully pole_a  |  0.5 → balanced  |  1.0 → fully pole_b

    Examples: coherence/decoherence, chaos/bliss, order/entropy
    """
    name:                str   = Field(..., description="e.g. 'coherence/decoherence'")
    pole_a:              str   = Field(..., description="First pole")
    pole_b:              str   = Field(..., description="Second pole")
    balance_coefficient: float = Field(
        default=0.5, ge=0.0, le=1.0,
        description="0.0 = pole_a  |  0.5 = balanced  |  1.0 = pole_b",
    )
    active_in_projects: List[str]    = Field(default_factory=list)
    description:        Optional[str] = None

    @classmethod
    def from_string(cls, duality_str: str) -> "DualityPair":
        """
        Parse the 'pole_a/pole_b' shorthand into a full DualityPair.

        Raises:
            ValueError: if the string does not contain exactly one '/'.
        """
        parts = duality_str.split("/", 1)
        if len(parts) != 2:
            raise ValueError(
                f"Invalid duality string: '{duality_str}'. Expected 'a/b' format."
            )
        return cls(
            name=duality_str,
            pole_a=parts[0].strip(),
            pole_b=parts[1].strip(),
        )


class Axiom(BaseModel):
    """
    A foundational invariant or principle active in one or more projects.
    e.g.  A22_Invariant,  A12_Local_Global
    """
    key:                str   = Field(..., description="Short identifier, e.g. 'A22_Invariant'")
    statement:          str   = Field(..., description="The axiom in natural language or LaTeX")
    formal_expression:  Optional[str] = Field(None, description="Mathematical / formal notation")
    active_in_projects: List[str]     = Field(default_factory=list)
    related_dualities:  List[str]     = Field(default_factory=list)
    confidence:         ConfidenceLevel = ConfidenceLevel.HIGH
    source_volume:      Optional[VolumeRef] = None
    notes:              Optional[str]   = None


class ProjectMeta(BaseModel):
    """
    Rich project descriptor — mirrors `projects.json` with additional
    runtime fields for Memoree context injection.

    `priority` is inverted (1 = highest focus, 10 = background).
    `dependencies` lists other project keys this project depends on;
    `integrates_with` lists project keys this project embeds into
    (e.g. arora embeds audry).
    """
    key:               str           = Field(..., description="Canonical project key, e.g. 'aurafs'")
    name:              str           = Field(..., description="Human-readable project name")
    description:       str
    owner:             ProjectOwner
    domains:           List[ProjectDomain] = Field(default_factory=list)
    active_volumes:    List[VolumeRef]     = Field(default_factory=list)
    active_axioms:     List[str]           = Field(default_factory=list)
    active_dualities:  List[str]           = Field(default_factory=list)
    tags:              List[str]           = Field(default_factory=list)
    repo_urls:         List[str]           = Field(default_factory=list)
    status: Literal["active", "paused", "archived", "planned"] = "active"
    priority:          int  = Field(default=5, ge=1, le=10, description="1=highest focus, 10=background")
    dependencies:      List[str] = Field(default_factory=list, description="Project keys this depends on")
    integrates_with:   List[str] = Field(default_factory=list, description="Project keys this embeds into")
    related_papers:    List[str] = Field(
        default_factory=list,
        description="arXiv IDs, DOIs, or local manuscript paths",
    )
    notes:      Optional[str] = None
    created_at: datetime      = Field(default_factory=_now)
    updated_at: datetime      = Field(default_factory=_now)


# ─────────────────────────────────────────────────────────────────────────────
# Core Memory Types
# ─────────────────────────────────────────────────────────────────────────────

class EpisodicMemory(AurphyxBase):
    """
    A single conversational turn or interaction event.

    Bound to a session, project, and LLM provider. The `parent_id` field
    enables turn threading for multi-hop conversation reconstruction.
    `intent` holds a detected topic label; `sentiment` is a normalised
    float in [-1.0, 1.0]. `embedding_id` links to the Qdrant point UUID
    after vector indexing.
    """
    memory_type: Literal["episodic"] = MemoryType.EPISODIC
    session_id:  str
    project:     str = "memoree"
    role:        Literal["user", "assistant", "system", "tool"] = "user"
    content:     str
    llm:         LLMProvider  = LLMProvider.PERPLEXITY
    model_name:  Optional[str] = Field(None, description="Specific model ID if known, e.g. 'sonar-pro'")
    turn_index:  int           = Field(default=0, ge=0)
    parent_id:   Optional[str] = Field(None, description="Prior turn ID for conversation threading")
    tags:               List[str]        = Field(default_factory=list)
    intent:             Optional[str]    = Field(None, description="Detected user intent / topic label")
    sentiment:          Optional[float]  = Field(None, ge=-1.0, le=1.0,
                                                  description="-1.0 negative | 0.0 neutral | 1.0 positive")
    embedding_id:       Optional[str]   = Field(None, description="Qdrant point UUID after indexing")
    user_preferences:   Dict[str, Any]  = Field(default_factory=dict)
    timestamp:          datetime         = Field(default_factory=_now)


class SemanticMemory(AurphyxBase):
    """
    A factual or conceptual knowledge unit tied to a project.

    The backbone of cross-project context injection. `summary` provides a
    1-sentence fast-retrieval preview; `relationships` stores related memory
    IDs or project keys for graph traversal during context assembly.
    `active_dualities` and `active_axioms` anchor this record to the live
    invariant set loaded from `dualities.json` / `invariants.json`.
    """
    memory_type:      Literal["semantic"] = MemoryType.SEMANTIC
    project:          str
    category:         str  = Field(..., description="Knowledge category, e.g. 'architecture', 'physics'")
    subcategory:      Optional[str]           = None
    content:          str
    summary:          Optional[str]           = Field(None, description="1-sentence fast-retrieval summary")
    tags:             List[str]               = Field(default_factory=list)
    domains:          List[ProjectDomain]     = Field(default_factory=list)
    relationships:    List[str]               = Field(
        default_factory=list,
        description="Related memory IDs or project keys",
    )
    active_dualities: List[str]               = Field(default_factory=list)
    active_axioms:    List[str]               = Field(default_factory=list)
    source:           Optional[str]           = Field(
        None,
        description="Origin: session ID, file path, arXiv ID, URL, etc.",
    )
    confidence:   ConfidenceLevel = ConfidenceLevel.HIGH
    embedding_id: Optional[str]  = None
    timestamp:    datetime        = Field(default_factory=_now)


class ProceduralMemory(AurphyxBase):
    """
    A named, repeatable workflow, task recipe, or automation sequence.

    Tracks execution frequency and rolling success rate for continuous
    optimisation. `preconditions` and `postconditions` enable automated
    pre-flight checks and state assertions. `tools_required` lists CLI
    commands, service names, or Python modules needed at runtime.
    """
    memory_type: Literal["procedural"] = MemoryType.PROCEDURAL
    project:     str
    task:        str
    description: Optional[str] = None
    steps:       List[str]

    preconditions:       List[str] = Field(
        default_factory=list,
        description="Conditions that must be true before execution",
    )
    postconditions:      List[str] = Field(
        default_factory=list,
        description="Expected state after successful execution",
    )
    tools_required:      List[str] = Field(
        default_factory=list,
        description="Tool names, CLI commands, or service dependencies",
    )
    frequency:           int            = Field(default=1, ge=0)
    success_rate:        float          = Field(default=1.0, ge=0.0, le=1.0)
    avg_duration_seconds: Optional[float] = None
    last_executed:       Optional[datetime] = None
    tags:                List[str]      = Field(default_factory=list)
    timestamp:           datetime       = Field(default_factory=_now)


class MetaMemory(AurphyxBase):
    """
    A confidence-tracked belief, verified fact, or system-level invariant.

    Used for cross-session knowledge anchoring and axiom enforcement.
    `project=None` indicates a global invariant active across all projects.
    Deprecated records are retained for audit trail — never deleted.
    `superseded_by` points to the MetaMemory ID that replaces this record.
    """
    memory_type: Literal["meta"] = MemoryType.META
    project:     Optional[str]   = Field(None, description="None = global scope")
    fact:        str
    formal_expression: Optional[str] = Field(None, description="Mathematical / formal notation")
    confidence:        ConfidenceLevel = ConfidenceLevel.HIGH
    confidence_score:  float           = Field(default=1.0, ge=0.0, le=1.0)
    sources:           List[str]       = Field(default_factory=list)
    related_axioms:    List[str]       = Field(default_factory=list)
    related_dualities: List[str]       = Field(default_factory=list)
    verified:          bool            = False
    verified_by:       Optional[str]   = Field(None, description="LLM provider or human reviewer")
    verified_at:       Optional[datetime] = None
    deprecated:        bool            = False
    deprecated_reason: Optional[str]   = None
    superseded_by:     Optional[str]   = Field(None, description="ID of the replacing MetaMemory")
    last_accessed:     datetime        = Field(default_factory=_now)
    access_count:      int             = Field(default=0, ge=0)
    timestamp:         datetime        = Field(default_factory=_now)


# ─────────────────────────────────────────────────────────────────────────────
# Specialised Memory Types
# ─────────────────────────────────────────────────────────────────────────────

class QuantumMemory(AurphyxBase):
    """
    Physics / simulation state snapshot for quantum and thermodynamic projects.

    Scope: ftqc, tvfd, vim, tslca, rae, majorana_1, zpe_core, duality_kernel.

    `lattice_type` describes the geometric substrate (e.g. 'Sierpinski',
    'hexagonal_photonic', 'fractal', 'Majorana').  `topological_invariant`
    stores Chern numbers or Z2 indices. `parameters` is an open dict for
    arbitrary simulation state that doesn't fit a typed field.
    """
    memory_type:          Literal["quantum"] = MemoryType.QUANTUM
    project:              str
    experiment_id:        Optional[str]  = None
    simulation_name:      Optional[str]  = None
    coherence_state:      CoherenceState = CoherenceState.COHERENT
    lattice_type:         Optional[str]  = Field(None, description="e.g. 'Sierpinski', 'Majorana'")
    qubit_count:          Optional[int]  = Field(None, ge=1)
    error_rate:           Optional[float] = Field(None, ge=0.0, le=1.0)
    coherence_time_us:    Optional[float] = Field(None, description="Coherence time in microseconds")
    topological_invariant: Optional[float] = Field(None, description="Chern number or Z2 invariant")
    hamiltonian_ref:      Optional[str]  = Field(None, description="File path, arXiv ID, or DOI")
    observable:           Optional[str]  = Field(None, description="Primary observable being tracked")
    observable_value:     Optional[float] = None
    units:                Optional[str]  = None
    parameters:           Dict[str, Any] = Field(
        default_factory=dict,
        description="Arbitrary key-value simulation parameters",
    )
    figure_paths:    List[str] = Field(default_factory=list, description="Paths to generated plots")
    active_axioms:   List[str] = Field(default_factory=list)
    active_dualities: List[str] = Field(default_factory=list)
    volume_ref:      Optional[VolumeRef] = None
    notes:           Optional[str]       = None
    timestamp:       datetime            = Field(default_factory=_now)


class CreativeMemory(AurphyxBase):
    """
    Creative and narrative memory for art, music, gaming, and divination projects.

    Scope: adore, aethornyx, rf_lovezme, tarot, oracle_deck, fuxyez,
           thirteen_month_calendar, aurphyx_casino.

    `soul_id_ref` links to a BlissID / SoulHash when the work is
    soul-anchored. `aurafs_shard_ref` is set by the storage backend
    after the content is minted into AuraFS.
    """
    memory_type:       Literal["creative"] = MemoryType.CREATIVE
    project:           str
    medium:            str  = Field(
        ...,
        description=(
            "e.g. 'prose', 'music_track', 'tarot_card', 'game_lore', "
            "'oracle_reading', 'code_poem', 'month_design'"
        ),
    )
    title:             Optional[str] = None
    content:           str
    language:          str           = Field(default="en", description="ISO 639-1 language code")
    universe:          Optional[str] = Field(None, description="Canonical lore universe this belongs to")
    characters:        List[str]     = Field(default_factory=list)
    themes:            List[str]     = Field(default_factory=list)
    dualities_expressed: List[str]   = Field(default_factory=list, description="Dualities this work embodies")
    soul_id_ref:       Optional[str] = Field(None, description="BlissID / SoulHash reference")
    aurafs_shard_ref:  Optional[str] = Field(None, description="AuraFS shard ID if minted")
    tags:              List[str]     = Field(default_factory=list)
    status: Literal["draft", "in_progress", "complete", "published", "archived"] = "draft"
    collaborators:     List[str]     = Field(default_factory=list)
    timestamp:         datetime      = Field(default_factory=_now)


class GovernanceMemory(AurphyxBase):
    """
    Governance, voting, policy, and guardian mandate records.

    Scope: gvs, sages, ineffable, egophyx, irra, standards, opulence.

    Once `immutable=True` the record has been archived to the Ineffable
    Ledger System (ILS) and must never be modified or deleted. `ledger_hash`
    is the ILS entry hash. `blissid_refs` enumerates every participating
    soul identity in this governance event.
    """
    memory_type: Literal["governance"] = MemoryType.GOVERNANCE
    project:     str
    record_type: Literal[
        "vote", "policy", "mandate", "guardian_action",
        "ledger_entry", "standard", "accessibility_ruling",
        "economic_event", "proposal",
    ]
    title:         str
    content:       str
    proposer:      Optional[str]  = Field(None, description="BlissID, soul hash, or entity identifier")
    guardian_ids:  List[str]      = Field(default_factory=list, description="SAGES guardian IDs involved")
    votes_for:     int            = Field(default=0, ge=0)
    votes_against: int            = Field(default=0, ge=0)
    votes_abstain: int            = Field(default=0, ge=0)
    quorum_reached: bool          = False
    outcome: Optional[Literal["passed", "rejected", "tabled", "superseded"]] = None
    blissid_refs:  List[str]      = Field(default_factory=list, description="Participating BlissIDs")
    ledger_hash:   Optional[str]  = Field(None, description="Ineffable Ledger entry hash (ILS)")
    immutable:     bool           = Field(default=False, description="True = permanently archived to ILS")
    tags:          List[str]      = Field(default_factory=list)
    effective_date: Optional[datetime] = None
    timestamp:     datetime       = Field(default_factory=_now)

    @model_validator(mode="after")
    def _enforce_immutability(self) -> "GovernanceMemory":
        """Immutable records must carry a ledger_hash."""
        if self.immutable and not self.ledger_hash:
            raise ValueError("immutable GovernanceMemory records must provide a ledger_hash.")
        return self


# ─────────────────────────────────────────────────────────────────────────────
# Identity & Soul Schemas
# ─────────────────────────────────────────────────────────────────────────────

class SoulProfile(BaseModel):
    """
    BlissID / SoulShot identity anchor.

    Links biometric soul data to Memoree context for personalised recall.
    `soul_hash` is the immutable biometric fingerprint produced at
    SoulShot genesis. `quantum_entropy_seed` is the entropy token from
    which the hash is derived and must be stored offline only.
    """
    soul_hash:            str  = Field(..., description="SoulHash — immutable biometric fingerprint")
    bliss_id:             str  = Field(..., description="BlissID — one soul, one voice, one vote, one ID")
    guardian_hash:        Optional[str]  = Field(None, description="GuardHash — guardian-layer verification token")
    soul_chart_ref:       Optional[str]  = Field(None, description="SoulChart path or AuraFS shard reference")
    hrv_signature:        Optional[str]  = Field(None, description="HRV-derived entropy token")
    voice_signature:      Optional[str]  = Field(None, description="Voice biometric hash")
    quantum_entropy_seed: Optional[str]  = Field(None, description="Quantum entropy seed — store offline only")
    arora_profile_id:     Optional[str]  = Field(None, description="Arora OS user profile UUID")
    aethornyx_avatar_id:  Optional[str]  = Field(None, description="In-game avatar ID in Aethornyx")
    created_at:           datetime       = Field(default_factory=_now)
    last_verified:        Optional[datetime] = None


# ─────────────────────────────────────────────────────────────────────────────
# AuraFS Shard Reference
# ─────────────────────────────────────────────────────────────────────────────

class AuraFSShard(BaseModel):
    """
    Reference to an AuraFS distributed storage shard.

    Used by any project that mints, stores, or retrieves shard-backed
    content. `pinned=True` prevents eviction from the hot tier.
    `redundancy_factor` sets the minimum number of storage nodes
    that must hold a full copy of this shard.
    """
    shard_id:   str = Field(default_factory=_uuid)
    shard_type: Literal[
        "memory", "file", "crypto", "nft", "game_asset",
        "ledger", "soul_data", "media", "code",
    ]
    project:          str
    owner_bliss_id:   Optional[str]  = None
    node_tier:        NodeTier       = NodeTier.CITIZEN
    content_hash:     str            = Field(..., description="SHA-256 of shard content")
    size_bytes:       Optional[int]  = None
    encrypted:        bool           = True
    redundancy_factor: int           = Field(default=3, ge=1)
    aurafs_path:      Optional[str]  = Field(None, description="Logical path within AuraFS namespace")
    pinned:           bool           = Field(default=False, description="True = never evicted from hot tier")
    created_at:       datetime       = Field(default_factory=_now)
    last_accessed:    Optional[datetime] = None


# ─────────────────────────────────────────────────────────────────────────────
# Context & Session Schemas
# ─────────────────────────────────────────────────────────────────────────────

class ContextResponse(BaseModel):
    """
    Full context payload returned to an LLM at session start.

    Aggregates all relevant memory types, project metadata, active
    invariants, duality pairs, AuraFS shards, and cross-project
    references into a single injectable payload.

    `context_tokens` is populated by the hook layer after serialisation
    so the caller can make truncation decisions before injection.
    """
    project:      str
    project_meta: Optional[ProjectMeta] = None
    llm:          LLMProvider           = LLMProvider.PERPLEXITY
    session_id:   Optional[str]         = None
    soul_profile: Optional[SoulProfile] = None

    # Memory layers
    episodic:   List[Dict[str, Any]] = Field(default_factory=list)
    semantic:   List[Dict[str, Any]] = Field(default_factory=list)
    procedural: List[Dict[str, Any]] = Field(default_factory=list)
    meta:       List[Dict[str, Any]] = Field(default_factory=list)
    quantum:    List[Dict[str, Any]] = Field(default_factory=list)
    creative:   List[Dict[str, Any]] = Field(default_factory=list)
    governance: List[Dict[str, Any]] = Field(default_factory=list)

    # Active invariants
    active_volumes:   List[str]         = Field(default_factory=list)
    active_axioms:    List[str]         = Field(default_factory=list)
    active_dualities: List[str]         = Field(default_factory=list)
    invariants:       List[str]         = Field(default_factory=list)
    duality_pairs:    List[DualityPair] = Field(default_factory=list)

    # Summaries
    last_summary:     Optional[str]         = None
    thread_summaries: List[Dict[str, Any]]  = Field(default_factory=list)

    # Cross-project context
    related_projects: List[str]       = Field(default_factory=list)
    aurafs_shards:    List[AuraFSShard] = Field(default_factory=list)

    # Diagnostics
    total_memories: int           = 0
    context_tokens: Optional[int] = None
    generated_at:   datetime      = Field(default_factory=_now)


class ThreadSummary(BaseModel):
    """
    Compressed multi-turn session summary for long-horizon context recovery.

    Indexes topic clusters and cross-project references for fast
    re-injection at session resumption. `embedding_cluster_ids` stores
    Qdrant cluster IDs representing the vector neighbourhood of this
    thread, enabling semantic-distance-based context ranking.
    """
    session_id:   str
    project:      str
    llm:          LLMProvider = LLMProvider.PERPLEXITY
    short_summary: str = Field(..., description="1-2 sentence TL;DR")
    long_summary:  str = Field(..., description="Full narrative summary")

    key_topics:         List[str] = Field(default_factory=list)
    decisions_made:     List[str] = Field(default_factory=list, description="Concrete decisions reached")
    open_questions:     List[str] = Field(default_factory=list, description="Unresolved follow-up items")
    projects_mentioned: List[str] = Field(default_factory=list)
    axioms_referenced:  List[str] = Field(default_factory=list)
    dualities_referenced: List[str] = Field(default_factory=list)
    aps_refs:           List[str] = Field(
        default_factory=list,
        description="APS / arXiv / DOI references cited in this thread",
    )
    embedding_cluster_ids: List[str] = Field(
        default_factory=list,
        description="Qdrant cluster IDs for this thread's memory vectors",
    )
    soul_id_ref:         Optional[str] = Field(None, description="BlissID of primary participant")
    aurafs_archive_shard: Optional[str] = Field(None, description="AuraFS shard ID if archived")
    turn_count:  int           = Field(default=0, ge=0)
    token_count: Optional[int] = None
    timestamp:   datetime      = Field(default_factory=_now)


# ─────────────────────────────────────────────────────────────────────────────
# Search & Retrieval
# ─────────────────────────────────────────────────────────────────────────────

class MemoryQuery(BaseModel):
    """
    Structured query for cross-project memory retrieval.

    `memory_types` defaults to the four highest-value types for general
    context assembly. `min_score` is a cosine similarity floor; results
    below this threshold are discarded by the vector backend.
    `include_deprecated` allows audit-trail access to superseded records.
    """
    query_text:   str
    project:      Optional[str]       = None
    memory_types: List[MemoryType]    = Field(
        default_factory=lambda: [
            MemoryType.EPISODIC,
            MemoryType.SEMANTIC,
            MemoryType.META,
            MemoryType.QUANTUM,
        ]
    )
    domains:            List[ProjectDomain] = Field(default_factory=list)
    tags:               List[str]           = Field(default_factory=list)
    top_k:              int                 = Field(default=10, ge=1, le=100)
    min_score:          float               = Field(default=0.65, ge=0.0, le=1.0)
    include_deprecated: bool                = False
    session_id:         Optional[str]       = None
    llm:                LLMProvider         = LLMProvider.PERPLEXITY
    filters:            Dict[str, Any]      = Field(default_factory=dict)


class MemorySearchResult(BaseModel):
    """A single ranked result returned from a MemoryQuery."""
    memory_id:       str
    memory_type:     MemoryType
    project:         str
    score:           float    = Field(..., ge=0.0, le=1.0)
    content_preview: str      = Field(..., description="First 300 characters of content")
    tags:            List[str] = Field(default_factory=list)
    created_at:      datetime
    tier:            MemoryTier = MemoryTier.WARM


# ─────────────────────────────────────────────────────────────────────────────
# Upsert Requests
# ─────────────────────────────────────────────────────────────────────────────

class UpsertMemoryRequest(BaseModel):
    """
    Generic upsert payload — accepts any memory type.

    Routes to the correct storage backend based on `memory_type`.
    `extra` is forwarded directly to the typed schema constructor,
    allowing callers to pass specialised fields (e.g. `qubit_count`
    for QuantumMemory) without a separate typed endpoint per memory class.
    """
    memory_type: MemoryType
    project:     str
    content:     str
    tags:        List[str]     = Field(default_factory=list)
    session_id:  Optional[str] = None
    llm:         LLMProvider   = LLMProvider.PERPLEXITY
    extra:       Dict[str, Any] = Field(
        default_factory=dict,
        description="Additional fields forwarded to the typed schema constructor",
    )


class BulkUpsertRequest(BaseModel):
    """
    Batch upsert of multiple memory records in a single call.

    Set `dry_run=True` to validate all records against their schemas
    without writing to any storage backend — useful for import previews
    and CI schema validation pipelines.
    """
    records:    List[UpsertMemoryRequest]
    project:    str
    session_id: Optional[str] = None
    dry_run:    bool           = Field(
        default=False,
        description="Validate without writing to storage if True",
    )


# ─────────────────────────────────────────────────────────────────────────────
# Health & Diagnostics
# ─────────────────────────────────────────────────────────────────────────────

class MemoreeDiagnostics(BaseModel):
    """
    System health snapshot for the Memoree daemon (port 7042).

    Populated by the `/diagnostics` route and the heartbeat loop.
    `qdrant_connected` reflects the live state of the Qdrant collection
    at the time of the snapshot. `llm_hooks_active` lists every provider
    hook that has registered itself since service start.
    """
    status:  Literal["healthy", "degraded", "error"] = "healthy"
    version: str   = "0.1.0"
    uptime_seconds: float = 0.0
    total_memories: int   = 0

    memories_by_type:    Dict[str, int] = Field(default_factory=dict)
    memories_by_project: Dict[str, int] = Field(default_factory=dict)

    qdrant_connected:    bool       = False
    qdrant_collections:  List[str]  = Field(default_factory=list)
    active_sessions:     int        = 0
    llm_hooks_active:    List[str]  = Field(default_factory=list)
    aurafs_shards_indexed: int      = 0

    last_gc_at:  Optional[datetime] = None
    errors:      List[str]          = Field(default_factory=list)
    warnings:    List[str]          = Field(default_factory=list)
    checked_at:  datetime           = Field(default_factory=_now)


# ─────────────────────────────────────────────────────────────────────────────
# Union type for type-safe dispatch
# ─────────────────────────────────────────────────────────────────────────────

AnyMemory = Union[
    EpisodicMemory,
    SemanticMemory,
    ProceduralMemory,
    MetaMemory,
    QuantumMemory,
    CreativeMemory,
    GovernanceMemory,
]

__all__ = [
    # Enums
    "MemoryType", "ProjectOwner", "ProjectDomain", "LLMProvider",
    "MemoryTier", "VolumeRef", "ConfidenceLevel", "CoherenceState", "NodeTier",
    # Domain objects
    "DualityPair", "Axiom", "ProjectMeta",
    # Base
    "AurphyxBase",
    # Core memory
    "EpisodicMemory", "SemanticMemory", "ProceduralMemory", "MetaMemory",
    # Specialised memory
    "QuantumMemory", "CreativeMemory", "GovernanceMemory",
    # Identity / storage
    "SoulProfile", "AuraFSShard",
    # Context & session
    "ContextResponse", "ThreadSummary",
    # Search
    "MemoryQuery", "MemorySearchResult",
    # Upsert
    "UpsertMemoryRequest", "BulkUpsertRequest",
    # Diagnostics
    "MemoreeDiagnostics",
    # Union
    "AnyMemory",
]