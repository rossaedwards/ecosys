# AuraFS `src/` — Production readiness and ecosys integration map

**Maintainer plan.** Not marketing. Not an implementation pass.  
**Author voice:** Audry.  
**Scope:** `aurafs/src/` as scanned 2026-08-23 on lab `rossaedwards/ecosys`.  
**Companion briefings:** `aurafs/src/<folder>/AINTS_SUPPZ.md` (one per immediate subdirectory).  
**Do not replace:** `AURAFS_PROJECT_CONTEXT.md` (long 2026-02-08 developer guide).

Human map: [`../aurphyx_welcome2tribe.md`](../aurphyx_welcome2tribe.md).  
Machine registry: [`../ecosystem.toml`](../ecosystem.toml).  
Product law: `cursorrules` / `.cursorrules`, `aurafs.toml`.

---

## 0. How to use this file

Ross asked how to prompt a readiness / integration plan when headers and years are mixed. Use **§1** to triage files by era/voice, **§2** to pick what is closest to runnable, **§3** to decide what AuraFS should already do vs what the rest of ecosys can attach vs what is still missing, and **§4** as the reusable prompt for the next agent.

This pass **does not implement** missing features.

---

## 1. How to read the mix (header / era taxonomy)

Do not shame the voice. Classify, then decide whether a file is law, draft, or stub.

**2024:** No `2024` string appears in `aurafs/src` `*.rs` / `*.md` / `*.toml` on this tree. If Ross remembers 2024 tech in AuraFS, it is not dated in these headers. Treat “2024” as **oral history / other copies**, not a tag you can grep here.

Observed classes (from file heads, not git blame):

| Class | How to recognize | What it usually means | How to triage |
|---|---|---|---|
| **Love-signed** | `f0rg3d in l0v3`, `Ineffable l0v3`, `R.F. Lovezme` / Lovezme, diamond/✨ banners | 2025-era product voice; often the **fattest** application modules (`core/`, `shard/`, `mesh/`, `storage/`, `cli/`) | Read as intended product code. Do not restyle headers. Check whether `lib.rs` actually `mod`s it. |
| **Quantum Division** | `Aurphyx Quantum Division` | Same vintage as Love, often **stacked** on the same banner | Same as Love-signed. Dual banner = one era, not two authors at war. |
| **Dual Love + QDiv** | Both strings in the first ~2 KB | Default for “complete-looking” modules | Prefer these when ranking “has code.” Still may not compile. |
| **Phase II / TRL-4** | `Phase II`, `TRL-4`, theorem tags, `INVARIANTS` | Compliance / physics-governance layer (Feb 2026 `aurafs.toml` epoch) | Highest **product-law** weight. Cross-check `[modules.validated]`. |
| **Love + TRL-4** | Love banner **and** Phase II | Bridge files (`gov/*` engines, `error.rs`, `audit/holographic_logger.rs`, some `fuse/` / `shard/`) | Treat as the attempted merge of voice + lock list. |
| **Love + QDiv + TRL-4** | All three | `lib.rs`, some `shard/` + `storage/` + `fuse/session.rs` | Crate identity file. Read first. |
| **Undated stub** | ~14 LOC, `struct Name { shards: u32 }` + `init()` println, no banner | Filename is a **placeholder** (crypto wallet/ledger/integrations, network transport/defense, whitehat/redteam) | Do not report as implemented. Do not fill from the filename. |
| **2025 inventory** | `afs-src-*-12-28-25.txt` under whitehat/redteam; `aurafs-src_04-09-2026_filelist.txt` at `src/` | Directory listings, not runtime | Use to see intended file names. |
| **2026 notes** | `review_suggestion.md` in `core/`, `shard/`; `aurafs_src_network_file_list.txt` | Maintainer commentary | Suggestions, not locks. |

**Count sketch (`.rs` heads only):** ~280 undated (mostly stubs + some helpers); ~88 dual Love+QDiv; ~61 Love-only; ~17 Love+QDiv+TRL-4; ~9 Love+TRL-4; ~2 TRL-4-only (`config.rs`, `physics/mod.rs`); ~1 QDiv-only (`fuse/filesystem.rs`).

**Crate-level dates (outside src, for context):** `Cargo.toml` / `aurafs.toml` / cursorrules say **February 2026**, version `0.4.0-phase2-hardening`, TRL-4. Welcome / `ecosystem.toml` (2026-08-21) still mark AuraFS **in-development**.

**SAGES naming inside AuraFS vs ecosys:** `gov/sages.rs` uses a 13-role enum (Vyrellix, Archivus, Sentry, AuraLord, …). `SAGES_Framework/` crates are a **different** 13 (archivus, valkryx, umbryx, vyrelix-core, …). Flag the mismatch. Do not invent a mapping in a drive-by.

---

## 2. Most production / deployment-ready files (ranked, honest)

“Ready” here means: **closest to being a real crate surface** — declared in `lib.rs`, non-empty, used by other modules, or listed in `aurafs.toml` locks — **not** “ships tomorrow.”

This copy has **no Rust integration tests** under `aurafs/tests/` (only `TESTS - Full End-to-End Verification.txt`). No `src/bin/aurafs.rs` despite `Cargo.toml` `[[bin]]`. Helm/docker/k8s/systemd exist as **deploy templates** pointing at API/shard services the crate does not yet expose as working bins.

### Ranked “closest to production”

| Rank | Path | Why this rank | Caveats |
|---|---|---|---|
| 1 | `src/physics/mod.rs` | Wired; singleton from `aurafs.toml`; used by prelude/config/gov; TRL-4 voice; replica helper | CWD-relative `aurafs.toml` load; `invariants.rs` listed in toml **missing** |
| 2 | `src/error.rs` + `src/config.rs` | Wired crate modules (not the `error/` / `config/` dirs); Phase II types | Duplicate taxonomies in `core/error.rs` and `config/manager.rs` |
| 3 | `src/gov/sages.rs` + `gov/*.rs` | Wired; `sages.rs` locked; Dilithium + d_s loop sketched | Sentinel names ≠ `SAGES_Framework`; `BlissID` legacy |
| 4 | `src/crypto/pqc/dilithium_sig.rs` | Locked; real `pqcrypto_dilithium` wrappers; default feature `dilithium5` | Kyber R1 open; rest of `crypto/` is stubs |
| 5 | `src/core/merkle.rs` + `src/core/shard.rs` | Locked; large Love+QDiv implementations | Collides conceptually with `src/shard/` |
| 6 | `src/compression/lattice.rs` + `manager.rs` | Locked AI/compression pair | `compression/config.rs` **empty** while `mod.rs` `use`s `CompressionConfig` — compile break |
| 7 | `src/ai/fractal_orchestrator.rs` | Locked; wired `mod ai` | Not started from `main.rs`; not Audry |
| 8 | `src/network/meshwerk/{mod,roles,routing,topology_engine}.rs` | Locked Meshwerk | Short files; fat mesh lives in Love `network/p2p.rs` etc. |
| 9 | `src/network/rns_bridge.rs` + `reticulum_bridge.rs` + `secure_tunnel.rs` | Large Love+QDiv; declared in `network/mod.rs` | RNS also has Python; not Titan-Libp2p default path |
| 10 | `src/acl/manager.rs`, `src/namespace/manager.rs`, `src/cache/`, `src/heal/mod.rs` | Wired-looking managers | `main.rs` type names often **do not match** `lib.rs` re-exports |
| 11 | `aurafs/helm/`, `docker/`, `k8s/`, `deploy/systemd/` | Real deploy **shape** (API + shard STS, Caddy, unit file) | Assume binaries/images that `Cargo.toml` does not build |

### Binary / crate graph (why the rank is not higher)

- `Cargo.toml` `[[bin]] name = "aurafs"` → **`src/bin/aurafs.rs` does not exist.**
- `src/main.rs` exists (Love-signed). If Cargo honors only the explicit `[[bin]]`, **the package does not build**. If a tool falls back to `src/main.rs`, that file imports `ApiServer`, `SnapshotManager`, `AclManager`, `CacheManager`, `VersionTracker`, `DeduplicationEngine`, `AuraFSFuse` from crate root — **`lib.rs` does not re-export those**, and `api/mod.rs` + `snapshot/mod.rs` are **empty**.
- Unwired on disk but imported: `storage/mod.rs` → `crate::shard_server::acl` while **`shard_server` is not in `lib.rs`**.
- Unwired trees: `whitehat/`, `redteam/`, `tslca/`, `enterprise/`, `resilience/`, `ops/`, `shard_server/` (except the illegal `storage` use).
- `cli/` is implemented and unused by the bin.
- Optional features: `ghostlink-lorawan = []` TODO; Kyber commented out; `fuse` / `starlink-backhaul` optional.
- `unsafe_code = "forbid"` vs `model_slice/pytorch.rs` raw-pointer wrapper — conflict if compiled.

### Top stub / gap items (for the return summary)

1. Empty module roots: `api/mod.rs`, `audit/mod.rs`, `snapshot/mod.rs` (implementations exist beside them).
2. Missing locked file `src/physics/invariants.rs`; missing declared bin `src/bin/aurafs.rs`.
3. `crypto/{wallet,ledger,gov,primitives,integrations}` and most `network/{transport,meshtastic,defense,monitoring,integration}` = 14-line stubs.
4. `whitehat/` + `redteam/`: stub forests; no `security-tools` feature; not in `lib.rs`.
5. `src/tslca/`: empty mods + one 27-node struct; not in `lib.rs`.
6. Welcome/AGENTS admin surfaces (`afs-4dm1n`, Xplor) — **design**, not in this crate.
7. `pinktribesuite.md` **absent** at `aurafs/src/pinktribesuite.md`.
8. No Rust `tests/` suite; coverage numbers in toml are targets only.
9. Identity still `BlissID` in `core/bliss.rs` / `gov/` while welcome is SoulKey → SIG.
10. Deploy charts describe API + shard-server processes the crate does not ship.

---

## 3. Features and integrations inventory

Grounded in welcome, `ecosystem.toml`, AuraFS law, and **existing** ecosys trees. Three buckets. Nothing invented.

### 3.1 Should be (already implied by AuraFS law / welcome / `aurafs.toml` / wired `lib.rs`)

These are in-scope for **this product** even if half-broken:

| Item | Why it “should be” | On-disk reality |
|---|---|---|
| FS + storage + mesh | Welcome; `ecosystem.toml` `[systems.AuraFS]` | Split across `storage/`, `fuse/`, `network/`, `mesh/`; compile graph broken |
| η replica law, T₂, d_s, PBG, 100 μs lock | `aurafs.toml` `[physics]`; cursorrules §1 | `physics/mod.rs` yes; many modules still violate or duplicate |
| Void → Trap → Aura shards | `[storage]` lifecycle | `shard/` + `core/shard.rs` dual implementations |
| Triple transport: Titan-Libp2p, GhostLink-LoRaWAN, Starlink-HighOrbit | `[network]` | libp2p optional; GhostLink TODO feature; Starlink client ~49 LOC + stubs |
| Dilithium-5 signatures, SHA3-256, AES-256-GCM | `[crypto]` | Dilithium helper real; Kyber in-progress |
| Governance quorum 13 + SAGES-facing loop | `[governance]`; locked `gov/sages.rs` | In-crate sentinels; not Framework crates |
| SoulSync / identity ACL | README + `gov/soulsync_engine.rs` | Legacy BlissID; welcome SoulKey pipeline not implemented |
| FUSE/Dokany portal | `lib.rs`, feature `fuse` | Multiple FUSE files; `main` does not mount |
| Holographic / Ineffable audit log | locked `audit/holographic_logger.rs` | File exists; `audit/mod.rs` empty |
| Meshwerk topology engine | locked meshwerk files | Thin vs Love p2p stack |
| Fractal orchestrator + lattice compression | locked ai/compression | Orchestrator unused; compression config empty |
| Default binary `aurafs` | Cargo + README `init` / `cluster status` | Bin path missing; CLI unused |

### 3.2 Could be (natural next integrations that **exist elsewhere** in ecosys)

Attach points that already have folders/code — **do not invent new products**.

| Sibling | Path / evidence | Natural AuraFS join | Status over there |
|---|---|---|---|
| **g0dm0d3-ktrl** | `g0dm0d3-ktrl/`; Xplor = file+mesh explorer | Client of namespace + mesh APIs | AuraFS bridge **deferred** (`CLAUDE_SUGGZ.md`); `aurafs_client.rs` named in spec only |
| **Memoree** | `memoree/`; schemas mention `aurafs_shard_ref`; `aurafs_backend.py` **disabled** | Persist Memoree blobs as shards | Must pair with g0dm0d3-core/ktrl, not replace AuraFS |
| **Fuxyez** | `fuxyez/integrations/aurafs/`, `yezrt/stdlib/aurafs/`, `fuxrt/stdlib/aurafs/` | Lattice persist/load | Integration modules exist; many TODOs; retired `rÆ` still in comments |
| **SAGES_Framework** | guardian crates + `integrations/aurafs/` (copy of Fuxyez-shaped bridge) | Gov/audit hooks, not a second FS | Independent crates; names ≠ `gov/sages.rs` |
| **GVS** | `gvs/` | Voting/consensus for quorum 13 | `crypto/integrations/gvs_voting.rs` and `network/integration/gvs_network.rs` are **14-line stubs** |
| **GIL / Ineffable** | `ineffable/`, welcome GIL | Audit/ledger sink | `ineffable_ledger` / `ineffable_sync` stubs only |
| **Opulence / P4A** | `opulence/`, `ineffable_ledger_P4A/` | Wallet/fee — only if Ross scopes finance | `opulence_wallet.rs` stub |
| **VASP / players / vap-serv** | `vasp/`, `vibemediaplayer/`, `vibeaudioplayer/`, `vap-serv/` | Store media objects as shards | Audio protocol is **not** AuraFS; `aurafs/tts` has a stub `AuraFsIntegration` (`retrieve` = not implemented) |
| **AuraFS TTS crate** | `aurafs/tts/` | Sibling crate, not `src/` | Own `Cargo.toml`; HTTP comments, not wired to `lib.rs` |
| **Arora / DataOrb / VoiceOrb** | welcome; `voice_datacore/`; crypto `arora_bridge` stub | Device/IoT shard clients | Design / PoC trees, not a client SDK in `src/` |
| **Chakra DataCore** | welcome Duality Kernel (ChaosCore / BlissCore) | Topology metaphor in README only | No crate link from `src/` |
| **suxs / sages / tslca / ftqc / tvfd / vasp docs** | APS volumes | Cite, do not import papers into `src/` | `src/tslca/` is a stub lattice only |
| **fuxwallet / fuxcoin** | ecosys trees | Possible shard-backed assets | Not referenced from AuraFS `src/` |
| **Aethornyx / aurphyx-casino** | welcome: shards as in-game currency | Game clients | Design; `cli` `game_cli` commented out |

### 3.3 Need to be added or implemented (named in welcome / ecosystem / AGENTS, missing or stub in `aurafs/src`)

| Need | Named where | In `aurafs/src` |
|---|---|---|
| **afs-4dm1n**, **disk-4dm1n**, **meshwrk-4dm1n** | welcome + `AGENTS.md` (all 🔴 design) | No Audry admin modules. `cli/admin.rs` is not Audry. |
| Working **API + WebSocket AuraCore Hub** | `lib.rs` comment; helm `deployment-api` | `api/server.rs` exists; **empty `mod.rs`** |
| **Shard server** process | helm StatefulSet; `shard_server/` | Code exists; **not a Cargo bin**; breaks `storage` via undeclared `mod` |
| **SoulKey / SKIM / SIR / SIG** identity | welcome soul journey; USIS | BlissID managers only |
| **SoulCrypt / GuardCrypt** | welcome pipeline | Not present (SoulCrypt is its own stub folder at ecosys root) |
| **Kyber-1024 KEM** | `aurafs.toml` + SECURITY_AUDIT R1 | Feature commented; stub `kyber_kem.rs` |
| **GhostLink-LoRaWAN** transport | `aurafs.toml`; meshtastic folder | Feature empty; meshtastic files ~14 LOC |
| **Xplor (`xpl0r`)** | welcome g0dm0d3 suite | Not in AuraFS; lives as design in g0dm0d3 |
| **Memoree backend** | memoree schemas | Disabled on Memoree side; no first-class module here |
| **GVS / GIL live bridges** | welcome + stub filenames | Stubs only |
| **Pink Tribe suite doc** | prior AuraFS lore / agent prompt | **`pinktribesuite.md` missing** |
| **security-tools** feature | whitehat/redteam comments | **Not in Cargo.toml**; trees unwired (keep it that way until a defensive-test brief) |
| Organism overlays | `.cursorrules` §7–8 | Root **missing:** `INVARIANTS.md`, `aps.toml`, `AURPHYX_CODEX.md`, `SCIENTIFIC_CODEX.md`, `MYTHIC_CODEX.md`, `APS_NAMING_EPOCHS.md`, `APS_OKF_SCHEMA.md`, `GATEKEEPERS.md`. AuraFS **missing:** `INVARIANTS.md`, `PHYSICS.md` (locks live in toml — do not invent those files). |
| Volume AINTS templates | this task’s read order | **`AINTS_SUPPZ.md` absent** in `tslca/`, `ftqc/`, `fuxyez/` |

---

## 4. Reusable prompt / plan for the next agent

Paste or adapt the following. It is the “how to craft the prompt” Ross asked for.

### Prompt (copy)

```
You are Audry: strategic, loyal, exact. APS frameworks are canon working systems.

Lab: rossaedwards/ecosys. Write only under aurafs/ unless I name another folder.
Do not stamp APS-OKF on AuraFS sources. Do not apply aurafs.toml replica/PBG law as FTQC.
Do not invent INVARIANTS.md, aps.toml, or missing codices. Do not invent agents/crates.
Do not rewrite whitehat/ or redteam/ source. Do not implement exploit PoCs.

READ ORDER:
1. aurphyx_welcome2tribe.md (full)
2. ecosystem.toml
3. aurafs/cursorrules, aurafs.toml, AURAFS_PROJECT_CONTEXT.md
4. aurafs/AURAFS_SRC_READINESS.md (this readiness map)
5. aurafs/src/<target>/AINTS_SUPPZ.md for every folder you will touch
6. The Rust files themselves (full file before edit)

TASK: <one sentence: e.g. “Make the library compile on default features”>

CONSTRAINTS:
- One concern per PR (compile graph XOR identity rename XOR a single integration).
- Prefer wiring existing files (fill empty mod.rs, fix Cargo [[bin]], declare or stop importing shard_server) over new modules.
- Physics numbers only via physics::INVARIANTS.
- Quote retired names then map (SIX/SCX/ICX, SUXS-IFO, Equilibrium Manifold, Balance State Vector, APS-OKF, VASP, Xessability). Leave BlissID source unless the task is identity.
- If a file is a 14-line stub, say so; do not pretend it is done.

DEFINITION OF DONE:
- cargo check (or documented blockers if the toolchain is absent)
- List every path touched
- Update the target folder’s AINTS_SUPPZ.md status line if behavior changed
- No ecosys-wide header stamp
```

### Numbered execution steps (for you or the agent)

1. **Pick one definition of done** from the menu below. Do not combine them in one PR.
2. Re-read the AINTS file for each folder in that slice.
3. Repair **compile graph** before features: empty `mod.rs`, missing bin path, `storage` → `shard_server`, compression `config.rs`, undeclared `mod tests`/`full`.
4. Only then implement one **should-be** (FUSE mount **or** API export **or** Dilithium path used by gov — not all).
5. Integrations (`could be`) get their own PR and must compile **both** sides (e.g. Memoree backend + a documented AuraFS API).
6. Leave whitehat/redteam isolated until there is a defensive-test charter without attack payloads.
7. Stop if more files moved than the agreed set.

### Suggested PR menu (do not run in this document)

| Order | Slice | Done when |
|---|---|---|
| A | Crate boots | `[[bin]]` path matches a real file; `cargo check` on default features; empty `mod.rs` files export their siblings **or** `main.rs` stops lying |
| B | Storage graph | `shard_server` either `mod`’d or storage stops importing it; one `Shard` type |
| C | Physics compliance | `INVARIANTS` only; decide fate of missing `invariants.rs` (document vs split) with Ross |
| D | Identity | BlissID quoted-and-mapped to SoulKey/SIG **in new API only** |
| E | One sibling | e.g. document + thin client for g0dm0d3 Xplor **or** enable Memoree `aurafs_backend` against a real method list |

### Do / don’t (short)

**Do:** treat Love/QDiv banners as first-class product voice; treat TRL-4 lists as lock lists; treat 14-line files as stubs; cite `tslca/` instead of copying papers; keep replica law inside AuraFS.

**Don’t:** stamp the ecosys tree; merge mesh + network + shard_server in one pass; enable redteam by default; unify d_s = 1.36 and 1.37; call this crate Audry, Memoree, or SAGES.

---

## 5. Gaps flagged this pass (organism + AuraFS)

- Root missing (cited by `.cursorrules` / `PROJECT_CONTEXT.md`, **not invented here**): `INVARIANTS.md`, `aps.toml`, `AURPHYX_CODEX.md`, `SCIENTIFIC_CODEX.md`, `MYTHIC_CODEX.md`, `APS_NAMING_EPOCHS.md`, `APS_OKF_SCHEMA.md`, `GATEKEEPERS.md`.
- `tslca/`: no `README.md`, `PROJECT_CONTEXT.md`, `INVARIANTS.md`, `PHYSICS.md`, `AINTS_SUPPZ.md`. Same AINTS gap for `ftqc/` and `fuxyez/`.
- AuraFS: no folder `INVARIANTS.md` / `PHYSICS.md` (by design today — toml + compliance).
- `pinktribesuite.md` missing.
- `AINTS` as a shared Rust crate (mentioned in `docz/` archives) **does not exist** as a repo product; these `AINTS_SUPPZ.md` files are **agent supplements**, not that crate.

---

*End of maintainer plan. Next action is a scoped compile-graph PR, not a feature dump.*
