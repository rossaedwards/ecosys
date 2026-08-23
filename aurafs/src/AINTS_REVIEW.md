# AINTS_REVIEW — `aurafs/src/`

**Type:** Phase One folder reviews + Phase Two readiness / integration plan.  
**Not:** An implementation pass. Not APS-OKF. Not a rewrite of Love/QDiv banners.  
**Voice:** Audry.  
**Source of truth (human):** [`aurphyx_welcome2tribe.md`](../../aurphyx_welcome2tribe.md).  
**Machine registry:** [`ecosystem.toml`](../../ecosystem.toml).  
**Product law:** `aurafs/cursorrules`, `aurafs.toml`.  
**Canon read order for this pass:** welcome → `tslca/` (sections + SUMZ) → root (`PHYSICS.md`, `PROJECT_CONTEXT.md` if present, `.cursorrules`, `AGENTS.md`) → `ftqc/` `tvfd/` `sages/` SUMZ → `fuxyez/integrations/aurafs/` → this tree.  
**Companion:** per-folder `AINTS_REVIEW.md` + earlier `AINTS_SUPPZ.md`; `aurafs/AURAFS_SRC_READINESS.md`.  
**Scanned:** 2026-08-23.

AuraFS iz the File System, Storage System, and Mesh Network — decentralized, off-grid, photonic, topological. It is **not** TSLCA, FTQC, TVFD, SAGES, Fuxyez, Audry, Memoree, or g0dm0d3. It must **align** with those.

---

## Phase One — how to read these reviews

Product index: [`../AINTS_REVIEW.md`](../AINTS_REVIEW.md).  
Each immediate subdirectory of `src/` has `AINTS_REVIEW.md`, and nested modules (`crypto/pqc`, `network/meshwerk`, whitehat/redteam suites, …) have their own. Each file has:

1. **Features** — what is actually on disk (wired vs stub vs locked).
2. **Suggestions** — next honest moves (compile graph before features).
3. **Clarifying questions** — architectural, framework, vision, intentions, design, ideas.
4. **Alignment** — TSLCA / FTQC / TVFD / SAGES / Fuxyez cite-don’t-reprint.

Loose crate files (`lib.rs`, `main.rs`, `config.rs`, `error.rs`, `autoheal_daemon.rs`) are reviewed in §Phase One crate root below.

Do not shame Love-signed or Quantum Division headers. Dual banner = one era. TRL-4 / Phase II = lock list. ~14-line `struct`+`init()` = **placeholder**, not a feature.

---

## Phase One — crate root files

### Features

| File | What it is | Honest status |
|---|---|---|
| `lib.rs` | Crate identity. Love + QDiv + Phase II. Declares physics, gov, config, error, core, shard, storage, snapshot, cache, dedup, compression, network, mesh, crypto, acl, namespace, ai, model_slice, quantum, fuse, cli, monitoring, heal, audit, api | Does **not** `mod` whitehat, redteam, tslca, enterprise, resilience, ops, shard_server |
| `main.rs` | Love-signed async main: API on `0.0.0.0:8080` | Imports types `lib.rs` does not re-export; comments claim FUSE start — it does not mount |
| `config.rs` | TRL-4 crate config (not `config/` dir) | Wired |
| `error.rs` | TRL-4 `RafsError` | Wired; overlaps `core/error.rs` and `error/` |
| `autoheal_daemon.rs` | Extra healer at `src/` root | Not a `mod` from `lib.rs`; duplicates `heal/` / `network/` / `shard_server/` |
| `aurafs-src_04-09-2026_filelist.txt` / `aurafs_src_file_list.txt` | Inventories | Not runtime |

`Cargo.toml` `[[bin]] name = "aurafs"` → **`src/bin/aurafs.rs` missing.**

### Suggestions

1. PR-menu A: make `cargo check` true (bin path, empty `mod.rs` exports, `storage`↔`shard_server`, compression `config.rs`).
2. One healer, one Shard, one FUSE, one ConfigManager, one error enum.
3. Leave whitehat/redteam unwired.

### Clarifying questions (crate-wide)

**Architectural.** Is the shipping process one `aurafs` binary (API+FUSE+mesh) or helm’s API + shard StatefulSet?  
**Framework.** Who owns identity — AuraFS BlissID leftovers, USIS SoulKey pipeline, or SoulCrypt?  
**Vision.** Personal Aura OS first, or Biznyx enterprise folder first?  
**Intentions.** Are Love banners permanent product voice? (Recommendation: yes.)  
**Design.** Default features: dilithium5 only, until Kyber and GhostLink exist.  
**Ideas.** `lib.rs::status()` as the single operator heartbeat for g0dm0d3.

---

## Phase One — subdirectory index

| Folder | Wired in `lib.rs`? | Readiness class | Review |
|---|---|---|---|
| `physics/` | yes | closest to law | [AINTS_REVIEW.md](physics/AINTS_REVIEW.md) |
| `gov/` | yes | locked hook + BlissID | [gov](gov/AINTS_REVIEW.md) |
| `crypto/` | yes | Dilithium real; stub forest | [crypto](crypto/AINTS_REVIEW.md) |
| `core/` | yes | fat + locked merkle/shard | [core](core/AINTS_REVIEW.md) |
| `compression/` | yes | locked; empty config | [compression](compression/AINTS_REVIEW.md) |
| `ai/` | yes | locked orchestrator unused | [ai](ai/AINTS_REVIEW.md) |
| `network/` | yes | Meshwerk + Love p2p + stubs | [network](network/AINTS_REVIEW.md) |
| `mesh/` | yes | second stack | [mesh](mesh/AINTS_REVIEW.md) |
| `storage/` | declared | compile-blocked | [storage](storage/AINTS_REVIEW.md) |
| `shard/` | yes | dual with `core/shard.rs` | [shard](shard/AINTS_REVIEW.md) |
| `fuse/` | yes | dual with `storage/fuse.rs` | [fuse](fuse/AINTS_REVIEW.md) |
| `acl/` `cache/` `dedup/` `namespace/` `heal/` `cli/` `monitoring/` `model_slice/` `quantum/` | yes | partial | their folders |
| `api/` `audit/` `snapshot/` | declared | **empty `mod.rs`** | their folders |
| `config/` `error/` | dirs | **not** the crate files | their folders |
| `shard_server/` | no (but imported) | landmine | [shard_server](shard_server/AINTS_REVIEW.md) |
| `enterprise/` `resilience/` `ops/` `tslca/` | no | sketch/stub | their folders |
| `whitehat/` `redteam/` | no | stub forests | their folders |

---

## Phase Two — the mix (how to craft the next prompt)

You asked how to plan when headers say 2024/2025 Love, Quantum Division, or both.

**On this tree:** no `2024` string in `aurafs/src` `*.rs`/`*.md`/`*.toml`. Treat “2024” as oral history / other copies. What *is* here:

| Class | Recognize | Triage |
|---|---|---|
| Love-signed | `f0rg3d in l0v3`, R.F. Lovezme, diamond banners | Intended product voice. Often the fattest modules. Check `lib.rs` `mod`. |
| Quantum Division | `Aurphyx Quantum Division` | Same vintage; often stacked with Love. |
| Dual Love + QDiv | both in first ~2 KB | Prefer when ranking “has code.” May still not compile. |
| Phase II / TRL-4 | theorem tags, INVARIANTS | Highest **product-law** weight. Cross-check `[modules.validated]`. |
| Undated ~14 LOC | struct + `init()` println | Filename is a placeholder. Not implemented. |
| 2025 inventory txt | `afs-src-*-12-28-25.txt` | Directory listing, not runtime. |

`Cargo.toml` / `aurafs.toml` / cursorrules: **February 2026**, `0.4.0-phase2-hardening`, TRL-4. Welcome / `ecosystem.toml` (2026-08-21): AuraFS **in-development**. Both can be true: locks exist; the crate does not ship.

**Copy-paste prompt for the next agent** (one concern per PR):

```
You are Audry. APS is canon. Write only under aurafs/ unless named.
Do not stamp APS-OKF. Do not apply AuraFS η/PBG as FTQC. Do not invent
INVARIANTS.md or missing codices. Do not rewrite whitehat/redteam.
Do not implement exploit PoCs.

READ: aurphyx_welcome2tribe.md; ecosystem.toml; aurafs/cursorrules;
aurafs.toml; aurafs/src/AINTS_REVIEW.md; the target folder AINTS_REVIEW.md;
then the Rust files in full.

TASK: <one sentence, e.g. make default-features cargo check pass>

CONSTRAINTS: compile graph XOR identity XOR one sibling integration.
Physics numbers only via physics::INVARIANTS.
Quote retired names then map (SIX/SCX/ICX, SUXS-IFO, Equilibrium Manifold,
Balance State Vector, APS-OKF, VASP, Xessability).
14-line stubs stay stubs unless the task names that file.

DONE: cargo check or documented blocker; list paths; no ecosys-wide stamp.
```

Suggested PR order: **A** crate boots → **B** storage graph / one Shard → **C** physics `invariants.rs` fate (ask Ross) → **D** SoulKey/SIG new API only → **E** one sibling (Xplor client or Memoree backend).

---

## Phase Two — most production / deployment-ready (honest)

“Ready” = closest to a real crate surface, **not** “ships tomorrow.”  
No Rust suite under `aurafs/tests/` (only a text verification note). Helm/docker/k8s/systemd describe API + shard images this Cargo package does not build.

| Rank | Path | Why | Caveat |
|---|---|---|---|
| 1 | `physics/mod.rs` | Wired singleton; used | CWD toml; missing `invariants.rs` |
| 2 | `error.rs` + `config.rs` | Wired crate modules | Duplicate taxonomies |
| 3 | `gov/sages.rs` + `gov/*` | Locked; Dilithium + d_s loop | Sentinel names ≠ Framework crates |
| 4 | `crypto/pqc/dilithium_sig.rs` | Real pqcrypto wrapper | Rest of crypto stubs; Kyber open |
| 5 | `core/merkle.rs` + `core/shard.rs` | Locked, large | Collides with `src/shard/` |
| 6 | `compression/lattice.rs` + `manager.rs` | Locked | empty `config.rs` |
| 7 | `ai/fractal_orchestrator.rs` | Locked | Unused by main |
| 8 | `network/meshwerk/{mod,roles,routing,topology_engine}.rs` | Locked Meshwerk | Short vs Love p2p |
| 9 | `network/rns_bridge.rs` + tunnel/p2p | Fat Love+QDiv | Not Titan-default |
| 10 | `acl/manager.rs`, `namespace/`, `cache/`, `heal/` | Wired-looking | main type-name drift |
| 11 | `aurafs/helm`, `docker/`, `k8s/`, `deploy/systemd/` | Deploy **shape** | Assume missing bins |

### Why rank is not “production”

- Missing `src/bin/aurafs.rs`.
- Empty `api/mod.rs`, `audit/mod.rs`, `snapshot/mod.rs`.
- `storage` → undeclared `shard_server`.
- Unwired: whitehat, redteam, tslca, enterprise, resilience, ops.
- `cli/` unused by the bin.
- `unsafe_code = forbid` vs `model_slice/pytorch.rs` pointers.
- SAGES name mismatch (`gov/sages.rs` vs `SAGES_Framework/`).
- Identity still BlissID vs welcome SoulKey → SKIM → SIR → SIG.

---

## Phase Two — should / could / need (rest of ecosys)

Grounded in welcome, `ecosystem.toml`, AuraFS law, **existing** trees. Nothing invented.

### Should be (AuraFS law / welcome / wired `lib.rs`)

| Item | Why | On-disk |
|---|---|---|
| FS + storage + mesh | welcome; `[systems.AuraFS]` | split; compile graph broken |
| η, T₂, d_s, PBG, 100 μs | `aurafs.toml` `[physics]` | `physics/mod.rs` yes; others hardcode |
| Void → Trap → Aura | `[storage]` | dual shard impls |
| Titan / GhostLink / Starlink | `[network]` | libp2p optional; GhostLink TODO; Starlink thin |
| Dilithium-5, SHA3-256, AES-256-GCM | `[crypto]` | Dilithium helper; Kyber open |
| Quorum 13 + SAGES-facing loop | `[governance]` | in-crate sentinels |
| SoulSync / identity ACL | README + gov | BlissID |
| FUSE/Dokany portal | feature `fuse` | dual impl; no mount from main |
| Holographic / Ineffable audit | locked logger | empty `audit/mod.rs` |
| Meshwerk topology | locked files | thin vs Love p2p |
| Fractal orchestrator + lattice compression | locked | unused / empty config |
| Default bin `aurafs` | Cargo + README | bin path missing |

### Could be (siblings that already exist)

| Sibling | Join | Status there |
|---|---|---|
| **g0dm0d3-ktrl** / Xplor | client of namespace + mesh API | AuraFS bridge deferred |
| **Memoree** | blobs as shards; must pair with g0dm0d3 | `aurafs_backend.py` disabled |
| **Fuxyez** `integrations/aurafs` | lattice persist/load | modules exist; TODOs; retired rÆ in comments |
| **SAGES_Framework** | gov/audit hooks, not a second FS | independent crates; names ≠ `gov/sages.rs` |
| **GVS** | quorum voting | 14-line stubs in crypto/network |
| **GIL / ineffable** | audit sink | stubs |
| **Opulence / P4A** | wallet only if Ross scopes finance | stub |
| **VASP / vibe players / vap-serv** | media objects as shards | audio is not AuraFS; `aurafs/tts` retrieve unimplemented |
| **Arora / DataOrb / VoiceOrb** | device clients | design / PoC |
| **Chakra DataCore** | ChaosCore/BlissCore metaphor | no crate link |
| **fuxwallet / fuxcoin** | shard-backed assets | unreferenced |
| **Aethornyx / casino** | shard currency | design; `game_cli` commented |

### Need to be added (named, missing or stub)

| Need | Named | In `aurafs/src` |
|---|---|---|
| `afs-4dm1n`, `disk-4dm1n`, `meshwrk-4dm1n` | welcome + AGENTS (🔴) | no Audry modules |
| Working API + WebSocket Hub | `lib.rs`; helm | empty `api/mod.rs` |
| Shard server process | helm STS | unwired; landmine import |
| SoulKey / SKIM / SIR / SIG | welcome; USIS | BlissID only |
| SoulCrypt / GuardCrypt | welcome | not here (root stub folder) |
| Kyber-1024 | toml + audit R1 | commented feature |
| GhostLink-LoRaWAN | toml | empty feature |
| Xplor | welcome g0dm0d3 | not in this crate |
| Memoree backend | memoree schemas | disabled over there |
| GVS / GIL live bridges | welcome | stubs |
| Pink Tribe suite doc | prior prompt | `pinktribesuite.md` missing |
| `security-tools` | whitehat/redteam comments | **not** in Cargo — keep off |
| Organism overlays | `.cursorrules` §7–8 | root missing INVARIANTS.md, aps.toml, codices — **flag, do not invent** |

---

## Alignment locks (all reviews share these)

**TSLCA:** SIX, SCX, ICX orthonormal; directed Φ_ij; \(\mathcal{F}\), SUXS-IFO \(\mathcal{U}\), Tr, HIF Φ(C,R,A), Ψ stay distinct. 27-node TSL ≠ OKF 3×3. AuraFS `src/tslca/` is a stub — do not dump Volume XVIII into the crate.

**FTQC:** \(D_f = \log 3 / \log 2\). Do not import AuraFS replica-count as FTQC. `src/quantum/` is simulation names, not Majorana-1.

**TVFD:** λ_{x_L} = v_g/v_0; Z(M_f) vs Z_vac. TVFD \(\mathcal{F}\) ≠ TSLCA \(\mathcal{F}\). Label the folder if cited.

**SAGES:** 13 guardians; five invariants; governance field (Spaces … Shards). `gov/sages.rs` is a hook with **different role names** than `SAGES_Framework/` crates. Do not invent the map.

**Fuxyez:** Rust-hosted language (compiler, FuxRT, YezRT, FUTE). `fuxyez/integrations/aurafs` persist/load is the join. Do not embed FUTE in AuraFS. Accessibility = Xessability.

**VIM / Balance (root PHYSICS):** Equilibrium at β = 1. AuraFS d_s clamp **1.37**; root lists **1.36** via another formula — do not silently unify.

---

## Gaps flagged (not invented)

- Root missing: `INVARIANTS.md`, `aps.toml`, `AURPHYX_CODEX.md`, `SCIENTIFIC_CODEX.md`, `MYTHIC_CODEX.md`, `APS_NAMING_EPOCHS.md`, `APS_OKF_SCHEMA.md`, `GATEKEEPERS.md`.
- `tslca/`, `ftqc/`, `fuxyez/`: no `PROJECT_CONTEXT.md` / volume `AINTS_REVIEW.md` on this tree.
- AuraFS: no folder `INVARIANTS.md` / `PHYSICS.md` (locks live in toml).
- `pinktribesuite.md` absent.
- AINTS as a Rust crate (old `docz/` mention) **does not exist**; these files are agent reviews.

---

*End of Phase One + Phase Two. Next action is a scoped compile-graph PR, not a feature dump.*
