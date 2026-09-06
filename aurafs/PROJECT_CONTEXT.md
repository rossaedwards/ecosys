# AuraFS — Cursor / Agent Briefing

**Organization:** Aurphyx LLC  
**Author:** Ross A. Edwards  
**ORCID:** [0009-0008-0539-1289](https://orcid.org/0009-0008-0539-1289)  
**Folder:** `aurafs/`  
**Updated:** 2026-08-23  
**Persona:** Audry — strategic, loyal, exact.

This file is the **folder overlay briefing** for agents working in `aurafs/`. It is not the AuraFS thesis, not an ecosys physics volume, and not a reprint of FTQC or TSLCA papers.

The older long developer guide still lives at [`AURAFS_PROJECT_CONTEXT.md`](AURAFS_PROJECT_CONTEXT.md) (2026-02-08). Prefer **this** file for Cursor context. Do not create a third parallel briefing.

---

## 1. What AuraFS is / is not

**Is (welcome + `ecosystem.toml`):** AuraFS is the **file system, storage system, and mesh network** of the Type-1 Civilization stack. Decentralized, off-grid, photonic, topological. It is a **product** crate (Rust) inside the lab monorepo `rossaedwards/ecosys`. Publish snapshot of the organism is `aurphyx/ecosys`; do not push there unless Ross asks.

**Is (product claim, AuraFS law):** Physics-informed distributed storage. Fractal lattice, Anderson localization, post-quantum signatures, Meshwerk / triple-topology transport. Runtime constants are owned by **this** product (`aurafs.toml`), not by ecosys APS-OKF headers.

**Is not:**

- An ecosys **physics volume** (`tslca/`, `ftqc/`, `tvfd/`, `vim/`, `suxs/`, `sages/`, `vasp/`). Those folders hold APS canon. AuraFS **uses** some of that geometry; it does not host the papers.
- Aura OS, Audry, Fuxyez, g0dm0d3, Memoree, VASP, or SAGES_Framework. Those are sibling products/layers. AuraFS is one OS substrate among: Fuxyez, AuraFS, Audry, SAGES, Chakra DataCore, g0dm0d3, GVS, GIL, OPE/P4A, Memoree.
- AuraFS product law is **not** ecosys root `.cursorrules`. Root `.cursorrules` is organism edit law for APS docs. Do not stamp APS-OKF YAML onto AuraFS sources unless a file already carries that contract.
- Do not apply `aurafs.toml` / AuraFS replica-count law back onto ecosys root or onto `ftqc/` as FTQC theory.

**Audry / SAGES / g0dm0d3 (from welcome, for orientation only):**

| Actor | Role relative to AuraFS |
|---|---|
| **Audry** | Aura's soul; AdminMate. `afs-4dm1n` and `disk-4dm1n` / `meshwrk-4dm1n` are the intended admin surfaces. Those modules are still **design**. |
| **SAGES** | 13 guardians. Governance field includes **Shards**, **Nodes**, **Lattices**, storage, filesystem, meshwrk. AuraFS `gov/` is the product-side hook, not the 13 guardian crates. |
| **g0dm0d3** | Control deck. **Xplor** (`xpl0r`) is the file + meshwrk explorer over AuraFS. Design. |
| **Fuxyez** | Language/runtime hosted on Rust. Bindings to AuraFS are integrations, not this crate's core. |
| **Memoree** | TSLCA cognitive memory. Pairs with g0dm0d3-core / g0dm0d3-ktrl — **not** a substitute for AuraFS shards. |
| **Aethornyx / Casino** | Welcome: AuraFS shards as in-game currency. Design; do not invent ledger rules here. |

---

## 2. Authority inside this folder (product law)

Read **this chain** before editing AuraFS code. Do **not** substitute ecosys APS-OKF stamping.

1. [`cursorrules`](cursorrules) and [`.cursorrules`](.cursorrules) — AuraFS development rules (duplicate copies of the same product law as of 2026-02-08).
2. [`aurafs.toml`](aurafs.toml) — **single source of truth** for runtime `[physics]`, governance quorum, transports, crypto names, storage ceilings, TRL-4 locked file list.
3. [`compliance/PHYSICS_INVARIANTS.json`](compliance/PHYSICS_INVARIANTS.json) — CI bounds (mirror of `[physics]`).
4. [`compliance/ALGORITHM_PROOFS.md`](compliance/ALGORITHM_PROOFS.md) — product theorems / propositions as written here.
5. [`compliance/SECURITY_AUDIT.md`](compliance/SECURITY_AUDIT.md) — PQC posture (R1–R6).
6. [`compliance/DARPA_TRL_VALIDATION.md`](compliance/DARPA_TRL_VALIDATION.md) — TRL mapping.
7. [`VALIDATION_REPORT.md`](VALIDATION_REPORT.md) — lab-validation writeup cited by `aurafs.toml`.
8. [`README.md`](README.md) — human product door (still carries some retired names; see §4).

Access physics numbers in Rust **only** through `physics::INVARIANTS`. Hardcoding `5.3`, `1600`, `1.37`, `0.21`, or `100` as those constants is a product compliance violation (`physics_audit.yml`).

**This overlay does not invent** an `aurafs/INVARIANTS.md` or `aurafs/PHYSICS.md`. Those files are **absent**. Locks live in `aurafs.toml` + `cursorrules` + `compliance/`.

---

## 3. Type-1 stack placement

From [`aurphyx_welcome2tribe.md`](../aurphyx_welcome2tribe.md) and [`ecosystem.toml`](../ecosystem.toml):

```
Aura OS family (Aura / Egophyx / Biznyx / Arora)
    ├── Fuxyez (language + FUTE + Yez / Gavinium)
    ├── AuraFS  ← this folder: FS + storage + mesh
    ├── Audry   (soul / admin / SAGES liaison)
    ├── SAGES   (13 guardians; immune-system firewall for Audry)
    ├── g0dm0d3 (+ Xplor over AuraFS)
    ├── Memoree (must pair with g0dm0d3-core or g0dm0d3-ktrl)
    └── GVS / GIL / P4A / Chakra DataCore
```

`ecosystem.toml` `[systems.AuraFS]`: File System, Storage System & Mesh Network; features decentralized, off-grid, photonic, topological; status **in-development**.

Lab vs publish: edit here (`rossaedwards/ecosys`). Do not treat `https://github.com/aurphyx/aurafs` README badges as a second source of naming law.

---

## 4. Nomenclature (new prose)

Quote retired names, then map. Do **not** resurrect retired terms as current in this briefing or in new comments you write. Do **not** silently rewrite whole historical files to chase names.

| Retired (quote) | Current |
|---|---|
| SIC / SCC / ICC | **SIX** Sensorimotor Integration aXis / **SCX** Systemic Coherence aXis / **ICX** Soul Identity aXis |
| USAIC | **SUXS-IFO** (fusion operator \(\mathcal{U}\) — SUXS volume, not an AuraFS replica formula) |
| Bliss manifold / BlissID / Bliss attractor *as physics* | **Equilibrium Manifold**; identity-continuity via **SoulSync** / SoulHash / **SIG** (USIS pipeline) |
| rÆ / rAE / rAE_* | **Balance State Vector** / \(x_*\) |
| Vibe-OKF / vibe-okf | **APS-OKF** (ecosys document headers — **not** applied to this product unless already present) |
| V.A.P. | **VASP** (audio protocol; not AuraFS) |
| Accessibility | **Xessability** (SUXS) |

**Product names that stay:** **BlissCore** / **ChaosCore** (Chakra Duality Kernel). Chaos & Bliss tarot. Source still has `BlissID` types in `gov/` and `core/bliss.rs` — that is **legacy identity code**, not permission to call a manifold “Bliss.” New identity prose: SoulKey → SKIM → SIR → SIG; one soul, one account, one vote.

README ACL line still says “SoulSync/BlissID.” Flag only; do not rewrite README in this pass.

`aurafs/integrations/README.md` still says “Universal Accessibility” and is a **Fuxyez** integrations stub copied into this tree — treat as stale; do not treat it as AuraFS API truth.

---

## 5. Product locks (cite, do not fork)

From `aurafs.toml` `[physics]` — **AuraFS replica / coherence law**:

| Symbol | Role in this product | Value (tolerance) |
|---|---|---|
| η | Hilbert scaling bias. `Replicas = ceil(log_η(Nodes))` | 5.3 (±0.05) |
| T₂ | Coherence window (sync ops) | 1600 μs (±100) |
| d_s | Spectral dimension (implementation clamp) | 1.37 (±0.05); exact cited as \(2\log 3/\log 5 \approx 1.365\) |
| PBG | Photonic band gap / Meshwerk routing overhead | 0.21 (±0.03) |
| — | FUSE lock timeout | 100 μs (T₂/16) |

Shard lifecycle (do not rename Trap-State to “cache”): **Void-Shard → Trap-State → Aura-Shard**.

Transports: **Titan-Libp2p** (primary), **GhostLink-LoRaWAN** (secondary), **Starlink-HighOrbit** (tertiary).

Governance: `min_quorum = 13`. Public API changes on `[modules.validated]` files need `PHYSICS OVERRIDE`.

**Shared organism numbers (cite root / FTQC — do not invent a third 1.585):**

- Hausdorff \(D_f = \log 3 / \log 2 \approx 1.585\) appears in `aurafs.toml` as a **derived comment** (`fractal_density_Df`) and in root [`PHYSICS.md`](../PHYSICS.md). AuraFS Hilbert formula in comments: \(\mathrm{dim}(H_{\mathrm{acc}}) = d^{n \cdot D_f^{\alpha(k)}}\). That is **product documentation of η-biased replica count**. It is **not** license to rewrite `ftqc/` or to treat replica count as FTQC theory.
- Root `PHYSICS.md` lists \(d_s = 1.36\) with a **different** formula \(2 D_f / (D_f + 1)\). AuraFS clamps **1.37** from Rammal–Toulouse \(2\log 3/\log 5\). **Do not silently unify.** Use AuraFS values inside this crate; cite the discrepancy if you touch both.

**TSLCA (only because source exists):** `src/tslca/lattice.rs` is a 27-node (\(3\times3\times3\)) activation lattice with HIF / \(\Psi_{i,j,k}\) / continuity \(\Xi\) and a comment tag `SoulHash / BlissID`. It is **not** declared in `src/lib.rs`. Do not dump TSLCA papers here. Cite [`tslca/`](../tslca/). Do not collapse the 3×3 tensor; do not treat \(\mathcal{U}\), \(\mathrm{Tr}(\mathcal{F})\), and HIF as the same operator. Do not treat AuraFS replica \(\lceil\log_\eta N\rceil\) as TSLCA fusion.

---

## 6. Map of `src/` (what is actually on disk)

Crate root: [`src/lib.rs`](src/lib.rs). Phase II TRL-4 filesystem prelude. **`whitehat/`, `redteam/`, `tslca/`, `enterprise/`, `resilience/`, `ops/`, `shard_server/` exist on disk and are not `mod`’d from `lib.rs`.**

### Declared in `lib.rs`

| Module | Job |
|---|---|
| `physics` | `INVARIANTS` singleton from `aurafs.toml`; violation errors; replica helper |
| `prelude` | anyhow / tracing / physics re-exports |
| `gov` | SAGES-facing governance: SoulSync, voting, proposals, identity (legacy `BlissID*` types) |
| `config` | Hot-reload config (`config/` + `config.rs`) |
| `error` | `RafsError` |
| `core` | Types, merkle, soulproof, metrics, persistence, circuit breaker |
| `shard` | Void → Trap → Aura shard types |
| `storage` | HAL, inode/FS, journal, quota, local shard store, in-tree FUSE helper |
| `snapshot` | **Declared; `snapshot/mod.rs` is empty** |
| `cache` | Coherence-window trap-state monitor (not a generic cache rename) |
| `dedup` | CDC / dedup |
| `compression` | Codecs / lattice compression |
| `network` | Meshwerk, transports, RNS/Reticulum, autoheal, discovery, replication |
| `mesh` | Separate fractal P2P orchestrator (Chord/XOR/routing/swarm) |
| `crypto` | PQC, wallet, ledger, primitives, ecosystem integration hooks |
| `acl` | ACL / identity mapping |
| `namespace` | Virtual paths |
| `ai` | Fractal / sentinel orchestration hooks |
| `model_slice` | Distributed model slice (Audry/Arora hooks) |
| `quantum` | Quantum / Majorana-style hooks |
| `fuse` | FUSE / Dokany presentation |
| `cli` | Management CLI |
| `monitoring` | Observability / \(d_s\) variance |
| `heal` | Holographic redistribution |
| `audit` | **Declared; `audit/mod.rs` is empty** |
| `api` | **Declared; `api/mod.rs` is empty** — `main.rs` still imports `api::{ApiServer, AppState}` |

Also at `src/` top level: [`main.rs`](src/main.rs) (in-crate binary-style entry; TRL-locked in `aurafs.toml`) and [`autoheal_daemon.rs`](src/autoheal_daemon.rs). Manifest `[[bin]]` points at **`src/bin/aurafs.rs`, which is missing** (see §10).

### On disk, not in `lib.rs`

| Path | Briefing note |
|---|---|
| `src/tslca/` | Single file `lattice.rs` — TSLCA activation lattice. Cite `tslca/`; do not expand into a paper. |
| `src/shard_server/` | Orchestrator: ACL, gossip, IPFS/gRPC helpers |
| `src/enterprise/` | Metrics, tiered storage, pool, key manager |
| `src/resilience/` | Circuit breaker / retry / recovery (overlaps `core/`) |
| `src/ops/` | Stub `init()` |
| `src/whitehat/` | Pink Tribe **defensive** suite (`security-tools` feature per module docs) |
| `src/redteam/` | Pink Tribe **adversarial test theater** (authorized testing only) |

### Network (under `src/network/`)

`peer`, `mesh`, `gossip`, `p2p`, `node_manager`, `orchestrator`, `discovery`, `replication`, `secure_tunnel`, `firewall`, `autoheal_daemon`, **`meshwerk/`** (roles, routing, topology_engine — TRL-locked), `meshtastic_integration`, `transport/` (Starlink, UDP multicast, …), `monitoring`, `defense`, `integration`, `reticulum_bridge`, `rns_bridge`, `rns_client`, `packet`.

### Crypto (under `src/crypto/`)

`pqc/` (Dilithium implemented; Kyber still R1 / commented dep), `wallet`, `ledger`, `gov`, `primitives`, `integrations` (Ineffable, Arora, Opulence, SAGES, GVS **hooks**).

### Other trees in `aurafs/` (not `src/`)

| Path | Role |
|---|---|
| `sdk/` | `aurafs-sdk` — `version.workspace = true` but **no root `[workspace]`** |
| `tts/` | Voice packs. **Not** the filesystem. Same workspace-version problem. |
| `ui/desktop/` | Tauri shell fragment |
| `compliance/` | Product physics/security/TRL |
| `tests/`, `benches/`, `simulations/` | Test / sim |
| `helm/`, `k8s/`, `docker/`, `deploy/` | Deploy sketches |
| `docs/` | Mixed deploy notes + `docs/docz/` archive (not protocol truth) |
| `integrations/` | Stale Fuxyez-oriented README; not crate API |

---

## 7. Pink Tribe (briefing only)

Suite lives under **`src/whitehat/`** (defense: chaos, exploit mitigation, net, gov, audit_simulator, quantum_breaker) and **`src/redteam/`** (chaos, exploit, net, gov, audit_simulator, quantum_breaker, cli, fuzzers). Parallel subdirectory names; different intent.

**`pinktribesuite.md` is not in this tree.** Detailed inventory is **not** this briefing. Do not rewrite whitehat/redteam in a context pass. Do not expand redteam surface for Vibe/VASP persistence work.

---

## 8. What agents must not do

- Do **not** import AuraFS **replica-count** law \(\lceil\log_\eta N\rceil\) into `ftqc/` as FTQC theory (organism lock; also `SUMZ-SUGGZ.md`).
- Do **not** fork \(D_f \approx 1.585\) or \(d_s\) into a third local constant file. Cite `aurafs.toml` here; cite root `PHYSICS.md` / `ftqc/` for shared geometry.
- Do **not** apply ecosys **APS-OKF** header stamping to AuraFS unless the target file already uses that contract. This briefing has **no** OKF YAML on purpose.
- Do **not** apply AuraFS `cursorrules` as law for `tslca/`, `vim/`, or repo root.
- Do **not** collapse SUXS-IFO \(\mathcal{U}\), diagonal \(\mathrm{Tr}(\mathcal{F})\), and HIF \(\Phi(C,R,A)\).
- Do **not** call Trap-State a cache/buffer/staging area in new prose.
- Do **not** invent missing `INVARIANTS.md`, `aps.toml`, `AURPHYX_CODEX.md`, `GATEKEEPERS.md`, or `pinktribesuite.md`.
- Do **not** invent new guardian crates, admin modules, or binaries.
- Skip binaries, PDFs, images, wasm, zip.
- One folder per documentation pass: **`aurafs/` only** when the task is this product.

---

## 9. Pointers (cite, don’t reprint)

| Need | Cite |
|---|---|
| Product names, duality, Audry, SAGES, g0dm0d3, Fuxyez, mesh, Soul journey | [`../aurphyx_welcome2tribe.md`](../aurphyx_welcome2tribe.md) |
| Machine registry | [`../ecosystem.toml`](../ecosystem.toml) |
| Organism edit law / lattice contractions (APS docs) | [`../.cursorrules`](../.cursorrules) — **not** for stamping this folder |
| Shared \(D_f\), organism \(d_s\) listing | [`../PHYSICS.md`](../PHYSICS.md) |
| Fractal / Hilbert **theory** (not replica count) | [`../ftqc/`](../ftqc/) — no `PROJECT_CONTEXT.md` there yet |
| 3×3 tensor, HIF, \(\Psi\), continuity \(\Xi\) | [`../tslca/`](../tslca/) — no `PROJECT_CONTEXT.md` there yet |
| Honest `src/` status / Vibe put-get plan | [`SUMZ-SUGGZ.md`](SUMZ-SUGGZ.md) |

---

## 10. Honest engineering status (do not paper over)

Recorded so agents do not assume a green cluster:

- **Does not currently build** as declared: `Cargo.toml` `[[bin]]` → missing `src/bin/aurafs.rs`. `edition = "2024"` / `rust-version = "1.93.0"` vs README “Rust 1.82+” vs `aurafs.toml` `[ci].msrv = "1.82.0"`.
- Empty `mod` roots: `api`, `snapshot`, `audit`. `main.rs` still expects `ApiServer`.
- `sdk/` and `tts/` expect a Cargo workspace that is commented out.
- Kyber is documented as in-progress; `pqcrypto-kyber` is commented in the manifest.
- `status()` in `lib.rs` returns a formatted string. That is not a live cluster.
- `whitehat/` + `redteam/` dominate file count vs `storage/` + `shard/`. Out of path for a first Vibe sidecar store.

A media player needs roughly: init, put, get, delete. Mesh, LoRa, governance votes, and TTS are not v1 player requirements (`SUMZ-SUGGZ.md`).

---

## 11. Gaps flagged (do not invent)

**Repo root (cited by organism law, missing):** `INVARIANTS.md`, `aps.toml`, `AURPHYX_CODEX.md`, `SCIENTIFIC_CODEX.md`, `MYTHIC_CODEX.md`, `APS_NAMING_EPOCHS.md`, `APS_OKF_SCHEMA.md`, `GATEKEEPERS.md`. `PHYSICS.md` **exists**. `aps-okf.yaml` may exist at root; it is **not** AuraFS product law.

**This folder:** no `INVARIANTS.md`, no `PHYSICS.md` overlay (by design — `aurafs.toml` + `compliance/` hold the numbers). No `pinktribesuite.md`. Overlay `PROJECT_CONTEXT.md` is **this file**.

**Sibling volumes:** `ftqc/PROJECT_CONTEXT.md` and `tslca/PROJECT_CONTEXT.md` were not present when this briefing was written.

**Naming drift still in-tree (do not mass-rewrite this pass):** README BlissID; `gov/blissid_manager.rs` and related; `src/tslca/lattice.rs` tag comment; `integrations/README.md` “Accessibility”; g0dm0d3-ktrl briefing elsewhere still uses BlissID/USIS pipeline language.
)
