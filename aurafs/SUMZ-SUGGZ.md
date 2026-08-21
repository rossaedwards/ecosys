---
type: implementation-note
title: AuraFS — Summary and Suggestions
description: Honest map of aurafs/src for a physics-informed store, and a put/get-first plan so Vibe players can persist VASP sidecars without expanding redteam surface.
workspaces: rossaedwards/ecosys, aurphyx/ecosys
services:
  - AuraFS
  - SoulSync
  - SAGES
domains:
  - systems
  - identity
nodes:
  - SCX⊗ICX
cores:
  - SCX
  - ICX
fields:
  - SAGES-governance-field
---

# AuraFS — SUMZ / SUGGZ

**Folder:** `aurafs/`  
**Role:** File system, storage, mesh. Fractal shard distribution. AuraFS has **its own** product rules (`aurafs.toml`, compliance). This note does not replace them.  
**Scope of this pass:** `src/` plus README / sdk / tts at high level. Not a rewrite.

**Gap:** Folder overlay `PROJECT_CONTEXT.md` is missing in this tree (product docs live under `compliance/` and README).

---

## What `src/` claims to be

From `src/lib.rs`, the crate is a Phase II TRL-4 distributed filesystem: physics prelude, governance, storage, mesh, PQC, AI hooks, FUSE/Dokany, CLI, heal, audit, API.

**Locked numbers (cite, do not fork):** `D_f`, `d_s`, scaling bias, T₂ window live in `aurafs.toml` / `compliance/PHYSICS_INVARIANTS.json`. Root `PHYSICS.md` is organism-wide; this crate must cite, not duplicate a third 1.585.

README still says **BlissID** in the ACL line. Current name is identity-continuity / SoulSync. Do not silently rewrite the README in this pass — flag it for a dedicated naming edit.

## Module map (`src/` — names only)

| Region | Modules | Job |
|---|---|---|
| Physics | `physics` | Invariants singleton, violation errors, decoherence recovery |
| Core | `core`, `config`, `error` | Types, hot-reload config, errors |
| Gov | `gov` | SAGES-facing governance |
| Storage | `shard`, `storage`, `snapshot`, `cache`, `dedup`, `compression` | Shards, backends, CDC, codecs |
| Net | `network`, `mesh` | Meshwerk, transport, replication |
| Crypto | `crypto`, `acl`, `namespace` | PQC, ACL, virtual paths |
| AI | `ai`, `model_slice`, `quantum` | Orchestration hooks, model slice, quantum hooks |
| Ops | `fuse`, `cli`, `monitoring`, `heal`, `audit`, `api` | Mount, CLI, metrics, heal, ledger log, hub |
| Also present in tree | `shard_server`, `enterprise`, `whitehat`, `redteam`, `autoheal_daemon` | Server, enterprise, defensive tests, adversarial test theater |

`sdk/` and `tts/` are separate crates. TTS is voice packs, not the filesystem.

**~485 Rust files under `aurafs/`.** Breadth is the risk. A Vibe player needs about four calls: init, put, get, delete.

## Honest status

- **Does not currently build.** `Cargo.toml` `[[bin]]` points at `src/bin/aurafs.rs` — file missing. `edition = "2024"` / `rust-version = "1.93.0"` vs README 1.82+.
- Empty module roots still declared: `api/mod.rs`, `snapshot/mod.rs`, `audit/mod.rs`. `shard_server/`, `redteam/`, `whitehat/` exist on disk and are **not** `mod`’d in `lib.rs`.
- `sdk/` and `tts/` use `version.workspace = true` with **no root `[workspace]`**.
- Architecture README is a full stack (VFS, REST, Meshwerk, PQC, SAGES). `status()` returns a formatted string. That is not a cluster.
- `whitehat/` + `redteam/` dominate file count relative to `storage/` + `shard/`. Out of path for Vibe. Do not expand them.
- No Vibe player source references AuraFS yet. Fuxyez stdlib bindings and Memoree `aurafs_backend.py` are in-memory / local-JSON stand-ins.
- Kyber call sites vs commented Kyber dep; axum/tower/lru referenced in code and missing from the manifest.

## What Vibe / Fuxyez need (narrow)

```
track.flac + track.vap.json  →  aurafs put  →  shard id
library index                →  aurafs query
skin pack .vskin             →  aurafs put (optional, local-first still wins)
```

No mesh, no LoRa, no governance vote, no TTS for v1 players.

## Phased series (AuraFS in service of top-3)

### Series AFS-0 — One local store

0. Make **lib** compile: restore or drop `src/bin/aurafs.rs`; stop declaring empty `api`/`snapshot`/`audit` until they have code; do not pull `shard_server` into the lib until storage round-trip works.
1. `aurafs init --data-dir <path>` creates a real directory layout.
2. `put` bytes → shard id. `get` shard id → bytes. Round-trip test.
3. Physics checks are **warnings** on this path, not a blocker for a sidecar write.
4. Public Rust API small enough for `vmp-core` and a Python/HTTP stub for Memoree.

### Series AFS-1 — Vibe bindings

1. Implement the Fuxyez native list that already exists (connect / persist / load / delete) against AFS-0.
2. Optional: mountless “library folder” mode so VAP web never needs a daemon.

### Series AFS-2 — Identity without the whole OS

1. ACL keyed by a local SoulSync token **or** a single-user default. Not GVS.
2. Rename BlissID in comments when that file is already open.

### Series AFS-3 — Mesh later

Replication, Dokany/FUSE, Meshwerk radios: after players ship. Keep `compliance/` as the physics gate.

## What not to do

- Do not treat redteam/whitehat file volume as product completeness.
- Do not import AuraFS replica-count law into `ftqc/` (organism lock).
- Do not make Vibe Audio Player depend on a cluster.
- Do not dump exploit procedures into this note or into player code.
