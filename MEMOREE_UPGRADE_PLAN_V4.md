---
type: protocol-spec
title: Memoree TSLCA Memory Upgrade Plan v4.0
description: Plan to instantiate TSLCA as Memoree's cognitive grammar, the same move VASP makes for audio. Supersedes v3.69 — grounds every phase against the real memoree/ codebase, the TSLCA/HIF/SAGES/SUXS-IFO source documents, and the Sovereign Identity pipeline.
workspaces: rossaedwards/ecosys, aurphyx/ecosys
services:
  - mcp
  - acp
  - apc
  - hooks
  - chains
  - rituals
  - links
  - Audry
  - Aura
  - AuraFS
  - Fuxyez
  - SAGES
  - SoulSync
  - Memoree
domains:
  - cognition
  - memory
  - xessability
  - identity
  - semantics
  - provenance
  - ethics
  - systems
  - governance
nodes:
  - SIX⊗SIX
  - SIX⊗SCX
  - SIX⊗ICX
  - SCX⊗SIX
  - SCX⊗SCX
  - SCX⊗ICX
  - ICX⊗SIX
  - ICX⊗SCX
  - ICX⊗ICX
cores:
  - SIX
  - SCX
  - ICX
fields:
  - cognitive-field-tensor
  - unified-cognitive-field
  - SAGES-governance-field
  - harmonic-integrity-field
  - memory-field
---

## ** APS‑TSLCA-MEMOREE-PLAN **
## ** Memoree - Sovereign Memory Substrate **
## ** Symbiotic Universal Xessability Standards **
## ** Three-Squared-Lattice Cognitive Architecture **
## ** Aurphyx Primordial Standard **
## ** Aurphyx LLC **
## ** SAGES | Proprietary | Pro-Existence **
## ** Accessibility = Xessability **
## ** Version 4.0 **

# Changelog from v3.69

This version does not change the thesis, the locked 9-type table, or any
operator definition from v3.69 — per `.cursorrules` §10, changing a locked
operator's definition needs to be asked for explicitly, and nothing here
does. What changed is grounding: v3.69 was written against the *idea* of
Memoree, TSLCA, HIF, and SAGES. v4.0 is written after actually reading
`tslca/`, `sages/`, `tvfd/`, `suxs/`, `ftqc/`, `fuxyez/`, the current
`memoree/` source (`memory_engine.py`, `schemas.py`, `heartbeat.py`,
`routes.py`), and the *Aurphyx Sovereign Identity* deck.

1. **Heartbeat is fixed, not "disabled" as a design choice.** v3.69 listed
   "heartbeat disabled" under Broken/incomplete, which reads like it was
   never turned on. In fact `heartbeat.py`'s `_tick()` already implements
   real logic matching its own docstring's intent (invariant-space check +
   active-project state check) — it just couldn't run: `from
   core.memory_engine import MemoryEngine` pointed at a `core/` package
   that doesn't exist (`memory_engine.py` is at the project root), and the
   tick body called `self.engine.aurafs.read_meta()` /
   `.read_state()`, neither of which exists — `memory_engine.py`'s own
   docstring says AuraFS is disabled pending integration and only imports
   `AuraFSBackend` as a commented stub. Both bugs are fixed as of
   2026-08-25 (heartbeat now uses the real, live `MemoryEngine.query()`
   and `MemoryEngine.list_projects()`). It is still commented out in
   `memoree_service.py`'s `lifespan()` (three lines, all marked
   `[DISABLED]`) — re-enabling it is a decision, not a bug fix, so it was
   left as-is. See Phase 7 below.
2. **`MemorySearchResult` has no `deprecated` field.** `MemoryEngine.query()`
   filters deprecated `MetaMemory` records internally when
   `include_deprecated=False`, but even with it `True`, the returned
   `MemorySearchResult` objects never carry the flag — there is currently
   no way for a caller to ask "which of these are deprecated." This blocks
   real invariant-enforcement (heartbeat can only report *how many* meta
   records answered, not which are stale). Added as a Phase 1 schema fix
   below; it wasn't visible from the outside in v3.69.
3. **HIF is now specified, not just named.** v3.69 says "no HIF / TTG gate
   on write or recall" as a gap but doesn't give the gate. The real
   definition (from `tslca/aps_tslca_hif_protocol_spec.md`) is:
   $$\text{HIF}(x,t) = \sqrt[3]{C(x,t)\cdot R(x,t)\cdot A(x,t)} \cdot \Phi(C,R,A)$$
   with $\Phi = 1$ only when Coherence, Resonance, and Alignment each clear
   their own threshold (the Triple Threshold Gate), and three named
   thresholds — $H_{\text{create}}$, $H_{\text{integrate}}$,
   $H_{\text{renew}}$ — gating write, recall, and dissolve respectively.
   Phase 2 below is now written against this exact formula instead of "add
   an HIF gate" as a black box.
4. **The Identity cell (ICX⊗SIX) has a real pipeline to point at.** The
   *Aurphyx Sovereign Identity* deck lays out the actual SoulJourney
   pipeline already declared in `ecosystem.toml`:
   SoulShot → SoulChart → SoulTable → SoulSync → SoulHash → GuardTable/
   GuardHash → SoulCrypt/GuardCrypt → SoulKey & SKIM → SIR & SIG → BlissID.
   `IdentityMemory` (Phase 1) should carry a pointer into that pipeline
   (which stage produced/verified the record), not just generic
   "provenance tags." Governance-cell (ICX⊗ICX) writes should be able to
   reference a GVS Archivus Ledger Block entry the same way — GVS's own
   diagram (`User + SoulKey → Dilithium-5 verification → Archivus Ledger
   Block`) is the real target `GovernanceMemory` records should be able to
   cite.
5. **"The 13" is ambiguous across three different things and needs a Phase
   0 decision, not an assumption.** v3.69 Phase 5 says "expand
   invariants.json to the 13." Cross-referencing `sages/SAGES_Overview.md`
   against root `CLAUDE.md` turns up three different "13"s in the wild:
   (a) the 13 SAGES guardian *cores* — literal crates under
   `SAGES_Framework/*-core/` (archivus, bliss_engine, chaos_engine,
   cryptanyx, nullivar, nunclex, ophiuchus, praelum, prophetyx, umbryx,
   valkryx, orricshade-core, vyrelix-core); (b) the "13-node identity
   graph" the Sovereign Identity deck says SKIM etches into the SoulKey
   crystal; (c) the 3-6-9-13 grammar's outer layer, which
   `SAGES_Overview.md` names as "GuardHash / SAGES Invariants" without
   enumerating 13 separate invariant statements — SAGES itself only
   formally defines **five** primordial invariants (Unity, Love, Cognitive
   Integrity, Ego-less Stewardship, Interdependence & Balance). These are
   not obviously the same 13. Do not silently pick one — see Phase 0 §0.4.
6. Confirmed real engine methods to build Phases 3–4 against (previously
   assumed, now verified in `memory_engine.py`): `query()`,
   `read_context()`, `list_projects()`, `get_project()`,
   `projects_by_owner()`, `projects_by_duality()`, `diagnostics()`. No
   `read_meta`, `read_state`, or `.aurafs` accessor exists anywhere in the
   live engine.

# Thesis

*(unchanged from v3.69)*

VASP is TSLCA applied to audio. Memoree is TSLCA applied to memory.

TSLCA is the smallest cognitive architecture that stays stable. It cannot
run on a flat vector store. It needs a memory organism with the same three
cores, nine cells, six dual-triad arrows, and thirteen SAGES invariants
(see Changelog §5 — "thirteen" needs a Phase 0 definition before it's
implemented against, it isn't being redefined here).

Today Memoree *names* TSLCA in the README and then stores seven loosely
typed collections. That is a filing cabinet wearing a lattice costume. The
upgrade makes the lattice the runtime.

Canon cores (OKF): SIX / SCX / ICX. Manuscript aliases: SIC / SCC / ICC.
Fusion: SUXS-IFO (alias USAIC).

# Locked 9-type memory architecture

*(unchanged from v3.69 — this table is the locked contract)*

| Cell | Type | Job |
| --- | --- | --- |
| SIX⊗SIX | Sensory | Perception traces, embodiment, xessability transforms |
| SIX⊗SCX | Working | Active context, open loops, uncured session buffer |
| SIX⊗ICX | Episodic | Session-bound turns, timed contact with a self |
| SCX⊗SIX | Semantic | Project knowledge, relationships, dualities |
| SCX⊗SCX | Meta | Verified facts, axioms, confidence-tracked beliefs |
| SCX⊗ICX | Quantum | Lattice / simulation / physics snapshots |
| ICX⊗SIX | Identity | Ξ continuity, provenance tags, self-consistency — **now: a pointer into the SoulJourney pipeline stage that produced/verified the record (SoulShot…BlissID), not a free-text tag** |
| ICX⊗SCX | Procedural | Repeatable workflows, rituals, chains |
| ICX⊗ICX | Governance | Votes, mandates, SAGES / GVS / ILS records — **now: can cite a GVS Archivus Ledger Block entry** |

Non-commutativity is law: Sensory→Semantic is not Semantic→Sensory. Write
paths and recall paths must keep direction.

Creative stays as a *medium flag* on Semantic / Sensory / Identity, not a
tenth type. That kills the README's old 9-node names (Temporal,
Relational, Operational, Axiomatic, State, Generative, Mandate, Entity,
Lattice). Those names are deprecated.

# VASP analogy (do not invent a second grammar)

*(unchanged from v3.69)*

| TSLCA | VASP / audio | Memoree / memory |
| --- | --- | --- |
| SIX | hear / feel / access the waveform | ingest / sense / transform the trace |
| SCX | motif, key, structure, invariant | meaning, axiom, project graph |
| ICX | listener / artist / soul continuity | Ξ, BlissID, provenance |
| SUXS-IFO | mix / master / render a playable field | contract the 9 cells into one prompt |
| HIF / TTG | allow a tone to sound | allow a memory to write, recall, or dissolve |
| SAGES 13 | ethical playback constraints | ethical recall / mutation constraints |

# Current vs target (verified against the live codebase 2026-08-25)

Exists and stays:

- FastAPI daemon `:7042`, SSE, MCP (`memoree_service.py`, `routes.py`)
- `VectorBackend` (Qdrant) — confirmed live in `memory_engine.py`
- Typed Pydantic schemas (`schemas.py`) — large, already has
  `EpisodicMemory`, `SemanticMemory`, `ProceduralMemory`, `MetaMemory`,
  `QuantumMemory`, `CreativeMemory`, `GovernanceMemory`, `ProjectMeta`
- `projects.json` / `dualities.json` / `invariants.json`, loaded via
  `_load_json()` in `memory_engine.py`
- Hooks (perplexity, supergrok, gemini, lmstudio)
- AuraFS stubs (keep commented until integration — confirmed still
  commented as of this pass)
- `heartbeat.py` — **now runnable** (see Changelog §1), still not wired
  into `memoree_service.py`'s startup

Broken / incomplete (re-verified):

- 7 types, not 9 (no Working, Sensory, Identity) — confirmed, `MemoryType`
  enum in `schemas.py` has no `WORKING`, `SENSORY`, or `IDENTITY` members
- `read_context` queries by project string, not by lattice cell — confirmed
- no HIF / TTG gate on write or recall — confirmed; now spec'd, see Phase 2
- no USAIC contractor (prompt is a bag of lists) — confirmed
- `invariants.json` is 5 facts, not the 13 — confirmed; "the 13" needs
  Phase 0 definition first, see Changelog §5
- `ConfidenceLevel` enum vs float `confidence` mismatch — confirmed:
  `MetaMemory.confidence: ConfidenceLevel` alongside a separate
  `confidence_score: float` field already coexist in `schemas.py`; this
  may already be resolved by having both — verify in Phase 1 whether this
  gap is real or v3.69 was looking at a stale copy
- README TSL names ≠ schema names — confirmed
- GibsonAI Memori path still in tree (`aurphyx_memori.py`) — confirmed present
- `*.py.md` twins, empty `vector_backend.py.md`, 20-byte `lmstudio_hook.py.md`
  — confirmed present
- `summarize_thread` is 501 — not re-verified this pass, carry forward
- **`MemorySearchResult` has no `deprecated` field** — new finding, see
  Changelog §2

# Cleanup (do this first in Cursor)

*(unchanged from v3.69)*

Delete:

- `aurphyx_memori.py`
- `aurphyx_memori.py.md`
- every `*.py.md` twin (the live `.py` is canonical)
- `vector_backend.py.md` (empty)
- `lmstudio_hook.py.md`
- `memoree_06-21-2026_tree.txt` (stale — note: a newer
  `memoree_08-19-2026_tree.txt` now also exists and is itself already
  stale relative to the current tree; regenerate rather than hand-edit,
  see `rossaedwards/ecosys/devops/scripts/ecosystem_check.py regen-tree`)
- runtime logs (`memoree_service.log`) — gitignore, do not commit

Keep, then retire later:

- `memori_bridge.py` behind `mirror_to_primary: false`
- `memos_overlay.py`, `powersync_client.py` as optional extras
- `hermes_*.md` as adapter notes

# File header law

*(unchanged from v3.69)*

Every source and data file gets:

1. Vibe-OKF nine keys (type, title, description, workspaces, services, domains, nodes, cores, fields)
2. The nine APS header lines
3. Then the body

Python: YAML frontmatter lives inside the module docstring, matching
`tslca/lattice_kernel.py`. JSON: `_okf` object as the first key, then
payload. YAML: real frontmatter if the loader allows it; otherwise a
`_okf` block at top.

Do not invent extra frontmatter keys. Nine keys. Fixed order.

# Implementation sequence

## Phase 0 — Canon lock

- Adopt this mapping table
- Bump version to 4.0 everywhere (`__init__.py`, diagnostics, README)
- Path in headers: `~/memoree/...` and `rossaedwards/ecosys`, `aurphyx/ecosys`
- **New: resolve "the 13" (Changelog §5) before Phase 5 touches
  `invariants.json`.** Pick one: (a) the 13 SAGES guardian cores, (b) a
  13-entry invariant list distinct from the 5 primordial axioms and not
  yet written down anywhere, or (c) something else. This is a canon
  decision, not an engineering one — flag it, don't infer it.
- `Add and integrate IPFS to align with AuraFS`
- `Integrate with FTQC and AuraFS`

## Phase 1 — Schemas

- Add `WorkingMemory`, `SensoryMemory`, `IdentityMemory`
- Keep Quantum, drop Creative as a first-class `MemoryType`
- Every memory carries: `node`, `cores`, `hif`, `coherence`, `resonance`,
  `alignment`, `direction` (`i⊗j` not `j⊗i`)
- `SoulProfile` remains a domain object, not a memory type
- **New:** `IdentityMemory` carries a `souljourney_stage` field
  (`soulshot | soulchart | soultable | soulsync | soulhash | guardtable |
  guardhash | soulcrypt | guardcrypt | soulkey | sir | sig | blissid`) —
  see Changelog §4
- **New:** `GovernanceMemory` carries an optional `archivus_block_ref`
  pointing at a GVS Archivus Ledger Block entry
- **New:** add a `deprecated: bool` (and `superseded_by: Optional[str]`)
  field to `MemorySearchResult` itself, sourced from the same metadata
  `MemoryEngine.query()` already reads and currently discards — this is
  the fix that makes heartbeat's invariant check meaningful instead of a
  count
- Verify (don't assume) whether the `ConfidenceLevel` vs `confidence_score`
  gap from v3.69 is still real — both already coexist on `MetaMemory` in
  the current `schemas.py`

## Phase 2 — Lattice runtime

- Import / vendor a memory-facing wrapper of `tslca/lattice_kernel.py`
- New `tsl_memory_kernel.py` implementing the real HIF formula from
  `tslca/aps_tslca_hif_protocol_spec.md`:
  $$\text{HIF}(x,t) = \sqrt[3]{C(x,t)\cdot R(x,t)\cdot A(x,t)} \cdot \Phi(C,R,A)$$
  with $\Phi=1$ only when $C \ge C_\theta$, $R \ge R_\theta$, $A \ge A_\theta$
  (Triple Threshold Gate) — plus activation, propagation, stability
  ($S = \nabla^2\text{HIF}$), and continuity per the same spec
- Write is refused if $\text{HIF} < H_{\text{create}}$
- Recall is refused if $\text{HIF} < H_{\text{integrate}}$
- Dissolve / deprecate only if $\text{HIF} < H_{\text{renew}}$

## Phase 3 — Engine

- Nine collections, one per cell
- `read_context` becomes USAIC contraction: weighted 3x3 → bounded prompt,
  built on the currently-live `read_context()` in `memory_engine.py`
  rather than a new method
- Inject active axioms, dualities, volumes, related projects *after*
  contraction — `list_projects()` / `get_project()` / `projects_by_duality()`
  already exist and are the right hooks
- Working memory is the uncured buffer; heartbeat cures Working → Episodic
  / Semantic / Meta (heartbeat now has a working, if minimal, tick to
  extend — see Phase 7)

## Phase 4 — Routes + MCP

- `POST /memories/working|sensory|identity`
- `GET /lattice` → 3x3 field snapshot
- `GET /hif` → live HIF
- MCP tools: `memoree_lattice`, `memoree_contract_context`

## Phase 5 — Data

- Replace `projects.json` with welcome-to-tribe registry (see companion file)
- Expand `invariants.json` **per the Phase 0 §0 decision on "the 13"** —
  do not guess which 13
- Dualities stay, add missing welcome dualities

## Phase 6 — Hooks

- Hooks request contracted field, not raw layers
- Tag every turn with node + llm + session

## Phase 7 — Heartbeat

- **Status as of this pass:** `heartbeat.py` is fixed and runnable
  (correct import, correct engine calls — see Changelog §1), but still
  commented out in `memoree_service.py`'s `lifespan()`. Re-enabling it is a
  one-line-import + two-line-uncomment change, deliberately left for you
  to do (or ask for) rather than done silently, since it starts a
  background asyncio task against the live daemon.
- The full pipeline this phase originally called for — ingest → HIF →
  embed → propagate → summarize → invariant check → cure Working — is NOT
  yet in `_tick()`. What's live now is invariant-space pulse + active
  project state, which is the "invariant check" and part of "state
  update" from the original one-line description, not the whole loop.
  Ingest/embed/propagate/summarize/cure-Working still depend on Phase 1–3
  landing first (Working memory type, HIF gate, USAIC contraction) — this
  phase's real work starts once those exist.

# What not to do

*(unchanged from v3.69)*

- Do not re-add SoulHash *profiles* to Vibe / player paths (SAGES /
  Pro-Existence reject)
- Do not treat Creative or Quantum as extra cores
- Do not flatten SIX/SCX/ICX into one embedding space without keeping cell
  metadata
- Do not implement AuraFS this pass — keep stubs
- Do not push from this sandbox; Cursor edits local `C:\rossaedwards\main\memoree`
  and `/home/rae/rossaedwards/main` — **note (2026-08-25): this session
  now has direct write access to `C:\rossaedwards\ecosys\memoree\` via the
  device bridge, which is a different situation than when this line was
  written. `heartbeat.py` was fixed directly through that bridge this
  pass, at your explicit request. Whether that access extends to the
  schema/engine changes in Phases 1–6 is your call, not an assumption —
  ask before each phase, don't treat this note as blanket permission.**

# Cursor prompt pack

See section at end of the chat response. Run prompts in order 1→7. One
concern per prompt. Do not let Cursor rewrite theory.
