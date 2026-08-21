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

This file is the **cross-stack** plan. Each top-level folder has its own `SUMZ-SUGGZ.md`. Organism law remains `.cursorrules`. Protocol truth is `vasp/`. Lattice truth is `tslca/`.

**Priorities (Ross):** 1) Vibe Audio Player 2) Vibe Media Player 3) Fuxyez. AuraFS is next only as a **local put/get** for those three.

Lab: `rossaedwards/ecosys`. Do not push `aurphyx/ecosys` unless asked.

---

## How the stack actually fits

```
vasp/ 3.69 ────────── identity of a track
   │
   ├─ vibeaudioplayer/  phone + web  (orb, local files, Scene)
   └─ vibeplayer/       desktop crown jewel (modules, File menu, Vinyl, Skinz)
            │
            └─ v01d / FUTE ── should live in fuxyez/fute (today: vibeplayer/fute)
                    │
                    ├─ wsz / MM / Kodi / Opera GTX  →  .vskin
                    ├─ Mixxx C++                    →  vmp-vinyl (GPL island)
                    └─ vsix                         →  lapidary / volt

tslca/     3×3 Φ_ij + HIF + U     →  orb math (do not collapse operators)
memoree/   session memory         →  last skin / playlist (optional)
aurafs/    shards                 →  only after AFS-0 put/get
g0dm0d3/   Aura DE                →  parked
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

Missing overlays almost everywhere: `PROJECT_CONTEXT.md`. Flagged per folder. Do not invent physics to fill them.

Empty product folders: `audry/`, `soulshot/`, `soulsync/`, `tob/`. Park.

---

## Execution series (do in this order)

One folder per coding pass. This series is the *sequence*, not a license to stamp the tree.

### Series 0 — Vibe Audio Player v1 (phone + web)

Owner: `vibeaudioplayer/`  
Done when: S24 smoke list in `ANDROID_STUDIO_APK.md` passes; web Orb + local file; no login wall; launcher mipmaps exist.

### Series 1 — Vibe Media Player plays for real

Owner: `vibeplayer/`  
Done when: `tauri:dev` or CLI+UI plays FLAC through the modular shell; File → Open File works; VASP write path exists (even 3.1 shim labeled as such).

### Series 2 — Skinz Dashboard + WinAmp v01d

Owners: `vibeplayer/` UI + FUTE (move toward `fuxyez/fute`)  
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

Owner: `vibeplayer/` + `vmp-vinyl` (GPL island)  
Done when: two decks in UI call the Rust engine; PNGs in `assets/`.

### Series 7 — AuraFS AFS-0

Owner: `aurafs/src` storage path only  
Done when: init / put / get / delete round-trip; Fuxyez bindings can call it; Vibe may persist sidecars **optionally**.

### Series 8 — Kodi-class media (VMP)

Video surface + Kodi skin XML. After audio is loved.

### Parked (not in this campaign)

g0dm0d3 DE, Audry 4dm1n, SoulShot/SoulKey hardware, GVS, Opulence/P4A in the APK, cartoon/420 production, OpenSCAD, FTQC manuscript, expanding AuraFS redteam, VST3 on phone, Play Billing.

---

## Per-folder index

| Folder | SUMZ file | Heat |
|---|---|---|
| `vibeaudioplayer/` | yes | **P1** |
| `vibeplayer/` | yes | **P1** |
| `fuxyez/` | yes | **P1** |
| `aurafs/` | yes | P2 (narrow) |
| `vasp/` | yes | protocol lock |
| `tslca/` | yes | orb law |
| `memoree/` | yes | optional session |
| `lapidary/` | yes | FUTE pattern |
| `vap-serv/` | yes | merge or park |
| `sages/` `suxs/` `vim/` `tvfd/` `ftqc/` `aps/` | yes | cite, don’t code |
| `g0dm0d3-ktrl/` `audry/` `soulshot/` `soulsync/` `tob/` | yes | empty/spec |
| `docs/` `overviews/` `aurphyx_bunker/` | yes | archive/narrative |
| `cartoon-quantum-mechanics/` `the-420-platoon/` `openscad/` `ineffable_ledger_P4A/` | yes | off-path |

---

## Immediate next coding slice (when you say go)

1. VAP: disable auth on the product route + mipmap icons.  
2. VMP: Skinz module → Dashboard shell with a dead **v01d** drop zone wired to existing `transmute_pack`.  
3. Fuxyez: make `compiler` Cargo.toml match `main.rs` until `cargo build` speaks.

Do not start all three in one pass. Pick one folder.
