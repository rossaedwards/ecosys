---
type: implementation-note
title: ecosys — Cross-stack Summary and Phased Series
description: Synthesis of the lab tree after a folder-by-folder pass, with a single execution series to finish Vibe Audio Player, Vibe Media Player, Fuxyez, and a narrow AuraFS put/get.
workspaces: rossaedwards/ecosys, aurphyx/ecosys
services:
  - VASP
  - Fuxyez
  - AuraFS
  - Audry
  - SAGES
domains:
  - audio
  - systems
  - cognition
nodes:
  - SIX⊗SIX
  - SIX⊗SCX
  - SCX⊗SCX
  - SCX⊗ICX
  - ICX⊗ICX
cores:
  - SIX
  - SCX
  - ICX
fields:
  - cognitive-field-tensor
  - harmonic-integrity-field
---

# ecosys — SUMZ / SUGGZ (master)

This file is the **cross-stack** plan. Each top-level folder has its own `SUMZ-SUGGZ.md`. Organism law remains `.cursorrules`. Protocol truth is `vasp/`. Lattice truth is `tslca/`. Human map is [`aurphyx_welcome2tribe.md`](aurphyx_welcome2tribe.md). Cursor briefing is [`PROJECT_CONTEXT.md`](PROJECT_CONTEXT.md).

**Priorities (Ross):** 1) Vibe Audio Player 2) Vibe Media Player 3) Fuxyez. AuraFS is next only as a **local put/get** for those three.

Lab: `rossaedwards/ecosys`. Publish: `aurphyx/ecosys`. Dual-monorepos = one Type-1 organism. Do not push `aurphyx/ecosys` unless asked.

---

## Findings (2026-08-21) — Aurphyx copy + welcome SoT

Folders and files were copied from local `aurphyx/ecosys/`. Welcome is now the human source of truth. [`ecosystem.toml`](ecosystem.toml) was drifting; it is aligned this pass.

| Finding | Action |
|---|---|
| `vibeplayer/` gone; VMP lives in `vibemediaplayer/` | Rename all execution pointers |
| Welcome DE names **SLIDE** / **Aura-HDE** (not SLISE / SHUXSDE) | toml + README + PROJECT_CONTEXT |
| IOT OS is **Arora** (not Ora) | toml + README |
| Windows paths `C:\rossaedwards\ecosys` + `C:\aurphyx\ecosys`; NTFS `D:` | toml |
| Memoree **must** pair with g0dm0d3-core or g0dm0d3-ktrl | toml + PROJECT_CONTEXT |
| Root `PROJECT_CONTEXT.md` missing | created (APS-OKF, Type-1 duality) |
| `aps.toml` still cited by `.cursorrules` and **absent** | flag only; do not invent physics |
| `AURPHYX_CODEX.md` absent | flag only |
| `docz/` ~124 markdown files (AuraOS blueprints, Audry, g0dm0d3, RFLovezme, tarot) | archive attic; [`docz/SUMZ-SUGGZ.md`](docz/SUMZ-SUGGZ.md) |
| `docs/` still exists | catch-all; not VASP 3.69 |
| `vasp/VibeTribe_Pillarz.md` | Tribe voter copy; overlay, not schema extras |

Copied arrivals **without** a per-folder SUMZ this pass (do not stamp the tree in one go):

`docz/` · `chakra_core_system/` · `aurphyx-casino/` · `aurphyx_com/` · `aurphyxstore/` · `gvs/` · `SAGES_Framework/` · `fuxcoin/` · `fuxwallet/` · `opulence/` · `ineffable/` · `ual/` · `brand_and_design/`

`docz/` is the only new SUMZ this pass. Other arrivals wait for a one-folder pass each.

---

## How the stack actually fits

```
vasp/ 3.69 ────────── identity of a track
   │
   ├─ vibeaudioplayer/   phone + web + Android/iOS/RPi/IoT
   └─ vibemediaplayer/   desktop crown jewel (modules, File menu, Vinyl, Skinz)
            │
            └─ v01d / FUTE ── should live in fuxyez/fute (today: vibemediaplayer/fute)
                    │
                    ├─ wsz / MM / Kodi / Opera GTX  →  .vskin
                    ├─ Mixxx C++                    →  vmp-vinyl (GPL island)
                    └─ vsix                         →  lapidary / volt

tslca/     3×3 Φ_ij + HIF + U     →  orb math (do not collapse operators)
memoree/   cognitive memory       →  required pair with g0dm0d3-ktrl / g0dm0d3-core
aurafs/    shards                 →  only after AFS-0 put/get
g0dm0d3-ktrl/  Aura DE deck       →  KDE-class suite; not a chat app
```

Two Vibe products stay **distinct**. Shared: VASP schema, orb uniforms, Skinz token JSON, v01d packs. Not shared: Grok auth template, VLC plugin rewrite, Mixxx GPL into the APK.

## Three favorite hosts for the v01d button

Inside VMP Skinz Customization Dashboard:

1. **WinAmp Classic** (`.wsz`) — chrome language the world already speaks.  
2. **MediaMonkey** — library-first themes.  
3. **Kodi** — full media XML skins.

**Opera GTX** = web/WASM chrome pack, same button, different `PackageKind`.

Today: v01d has a **wsz→vskin skeleton**. Skinz UI is four CSS palettes. That gap *is* the crown-jewel work.

## Remaining assets (players)

| Surface | Missing / unused |
|---|---|
| VMP | Vinyl PNGs in `skins/decks/vibe_default/assets/`; `.vskin` packs for Void/Sodium/Lattice; Tauri icons |
| VAP web | PWA sizes from `app-icon-1024.jpg`; unused `assets/icons/*.svg` until Skinz |
| VAP Android | mipmap densities; PATHS.md vs Studio tree equality |
| Both | Real WinAmp BMP atlas after unzip (do not invent menu chrome) |

## Honest blockers

| Project | Blocker |
|---|---|
| VAP | Auth template; WebGL vs Canvas orb; Android no FFT / no shuffle; demo pause broken; Studio tree outside git |
| VMP | `cargo test --workspace` **29/29 pass**; vis ≠ orb; VASP 3.1 vs 3.69; Skinz = CSS chips; `v01d_info` unused in React |
| Fuxyez | Corrupted EOF tails; workspace members ≠ dirs; **two FUTEs** (language scaffold here vs pack v01d in VMP); compiler does not build |
| AuraFS | Missing `src/bin/aurafs.rs`; empty `api`/`snapshot`/`audit`; sdk/tts not in a workspace — **does not compile** |
| Memoree | `schemas.py` YAML → SyntaxError; VectorBackend constructor mismatch |
| TSLCA sims | Missing `lattice_kernel.py` |
| Lapidary | CMake required for `libz-ng-sys` on this host |
| vap-serv | Rotate leaked YouTube/Spotify keys (removed from source) |

Root `PROJECT_CONTEXT.md` now exists. Volume overlays still missing in most folders. Do not invent physics to fill them.

Empty product folders: `audry/`, `soulshot/`, `soulsync/`, `tob/`. Park.

---

## Execution series (do in this order)

One folder per coding pass. This series is the *sequence*, not a license to stamp the tree.

### Series 0 — Vibe Audio Player v1 (phone + web)

Owner: `vibeaudioplayer/`  
Done when: S24 smoke list in `ANDROID_STUDIO_APK.md` passes; web Orb + local file; no login wall; launcher mipmaps exist.

### Series 1 — Vibe Media Player plays for real

Owner: `vibemediaplayer/`  
Done when: `tauri:dev` or CLI+UI plays FLAC through the modular shell; File → Open File works; VASP write path exists (even 3.1 shim labeled as such).

### Series 2 — Skinz Dashboard + WinAmp v01d

Owners: `vibemediaplayer/` UI + FUTE (move toward `fuxyez/fute`)  
Done when: drop `.wsz` → preview mapped onto main / EQ / playlist / vis hole; Void / Sodium / Lattice as files; **v01d** button visible.

### Series 3 — Fuxyez compiles hello

Owner: `fuxyez/`  
Done when: workspace builds; `fuxyez` parses one `.fux`; `REVIEW.md` exists; language FUTE and pack v01d are federated, not blindly overwritten.

### Series 4 — VAP consumes `.vskin`

Owner: `vibeaudioplayer/`  
Done when: CSS vars + Compose scheme hot-swap; still no `.wsz` parser in the APK.

### Series 5 — MediaMonkey + Opera GTX packs

Owner: FUTE pack kinds + VMP Dashboard  
Done when: theme zip / GX theme produce tokens the Library module and WasmShell can apply.

### Series 6 — Vinyl Vibez UI + vinyl assets

Owner: `vibemediaplayer/` + `vmp-vinyl` (GPL island)  
Done when: two decks in UI call the Rust engine; PNGs in `assets/`.

### Series 7 — AuraFS AFS-0

Owner: `aurafs/src` storage path only  
Done when: init / put / get / delete round-trip; Fuxyez bindings can call it; Vibe may persist sidecars **optionally**.

### Series 8 — Kodi-class media (VMP)

Video surface + Kodi skin XML. After audio is loved.

### Parked (not in this campaign)

g0dm0d3 DE (beyond ktrl hull), Audry 4dm1n, SoulShot/SoulKey hardware, GVS, Opulence/P4A in the APK, cartoon/420 production, OpenSCAD, FTQC manuscript, expanding AuraFS redteam, VST3 on phone, Play Billing, sorting `docz/`.

---

## Per-folder index

| Folder | SUMZ file | Heat |
|---|---|---|
| `vibeaudioplayer/` | yes | **P1** | Android / iOS / RPi / IoT |
| `vibemediaplayer/` | yes | **P1** | Vibe Media Player; Windows / Linux / macOS |
| `fuxyez/` | yes | **P1** | |
| `aurafs/` | yes | P2 (narrow) | |
| `vasp/` | yes | protocol lock | includes VibeTribe_Pillarz.md |
| `tslca/` | yes | orb law | |
| `memoree/` | yes | pair with g0dm0d3 | |
| `lapidary/` | yes | FUTE pattern | |
| `vap-serv/` | yes | SaaS | |
| `sages/` `suxs/` `vim/` `tvfd/` `ftqc/` `aps/` | yes | cite, don’t code | |
| `g0dm0d3-ktrl/` `audry/` `soulshot/` `soulsync/` `tob/` | yes | empty/spec | |
| `docs/` `overviews/` `aurphyx_bunker/` | yes | archive/narrative | |
| `docz/` | yes | attic from aurphyx copy | not protocol |
| `cartoon-quantum-mechanics/` `the-420-platoon/` `openscad/` `ineffable_ledger_P4A/` | yes | off-path | |
| `chakra_core_system/` `aurphyx-casino/` `gvs/` `opulence/` `fuxcoin/` `fuxwallet/` `ual/` `brand_and_design/` `aurphyx_com/` `aurphyxstore/` `SAGES_Framework/` `ineffable/` | **no** | copied; SUMZ later | |

---

## Immediate next coding slice (when you say go)

1. VAP: disable auth on the product route + mipmap icons.  
2. VMP: Skinz module → Dashboard shell with a dead **v01d** drop zone wired to existing `transmute_pack`.  
3. Fuxyez: make `compiler` Cargo.toml match `main.rs` until `cargo build` speaks.

Do not start all three in one pass. Pick one folder.
