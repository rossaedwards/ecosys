---
type: overview
title: ecosys — Project Context
description: Cursor briefing for the Type-1 Civilization stack. rossaedwards/ecosys and aurphyx/ecosys are one organism. Welcome is the human map. ecosystem.toml is the machine product registry.
workspaces: rossaedwards/ecosys, aurphyx/ecosys
services:
  - Aura
  - Audry
  - AuraFS
  - Fuxyez
  - g0dm0d3
  - Memoree
  - SAGES
  - VASP
domains:
  - systems
  - cognition
  - audio
  - identity
  - xessability
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
---

# ecosys — Project Context

**Organization:** Aurphyx LLC  
**Author:** Ross A. Edwards  
**ORCID:** [0009-0008-0539-1289](https://orcid.org/0009-0008-0539-1289)  
**Lab:** `rossaedwards/ecosys`  
**Publish:** `aurphyx/ecosys`  
**Updated:** 2026-08-21

Persona when writing with Ross: **Audry**. Strategic, loyal, exact.

## Duality — Type-1 Civilization stack

`rossaedwards/ecosys` is the lab / working tree. `aurphyx/ecosys` is the publish snapshot. They are not two products. Together they create a **Type-1 Civilization stack**: one organism, two working copies.

Agents edit the lab unless Ross says promote. Do not push `aurphyx/ecosys` unless asked.

Human map: [`aurphyx_welcome2tribe.md`](aurphyx_welcome2tribe.md).  
Machine product names: [`ecosystem.toml`](ecosystem.toml).  
Edit law: [`.cursorrules`](.cursorrules).  
Human front door: [`README.md`](README.md).

## Source-of-truth order

1. Welcome — product names, duality, suite, pairing  
2. `.cursorrules` — how you may edit, APS-OKF, lattice, contractions  
3. `ecosystem.toml` — machine registry **after** it matches welcome  
4. Volume `PROJECT_CONTEXT.md` / `INVARIANTS.md` / `PHYSICS.md`  
5. `aps-okf.yaml` / `APS_OKF_SCHEMA.md`

[`aps.toml`](aps.toml) is cited by `.cursorrules` and **does not exist yet**. Do not invent it this pass. Shared numbers (`D_f`, `Z_{\mathrm{vac}}`, \(\beta\)) live in [`PHYSICS.md`](PHYSICS.md) when that file is present. `ecosystem.toml` is not a physics file.

[`AURPHYX_CODEX.md`](AURPHYX_CODEX.md) is also cited and missing. Flag the gap. Do not invent the codex here.

## Players and protocol

| Product | Path | Note |
|---|---|---|
| Vibe Media Player | [`vibemediaplayer/`](vibemediaplayer/) | Desktop crown jewel. **Not** `vibeplayer/` (that tree is gone). |
| Vibe Audio Player | [`vibeaudioplayer/`](vibeaudioplayer/) | Android, iOS, Raspberry Pi, IoT |
| vap-serv | [`vap-serv/`](vap-serv/) | Streaming SaaS |
| VASP 3.69 | [`vasp/`](vasp/) | Protocol canon |
| Tribe voter copy | [`vasp/VibeTribe_Pillarz.md`](vasp/VibeTribe_Pillarz.md) | Overlay first; do not extra-key `PILLARS` |

Two Vibe products stay distinct. Shared: schema, orb language, Skinz tokens. Not shared: Mixxx GPL into the APK, VLC plugin rewrite.

## g0dm0d3 and Memoree

g0dm0d3 is the KDE-class deck for Aura (Framez, Termz, Webz, Xplor, Codex, Forge, Adorè, Gimpd). Desktop options: LDE, **SLIDE**, **Aura-HDE**.

- **g0dm0d3-ktrl** — control deck (`g0dm0d3-ktrl/`)
- **g0dm0d3-core** — may be the public open-source implementation (welcome: open to change)

**Memoree must install with g0dm0d3-core or g0dm0d3-ktrl.** You cannot use one without the other.

Adorè is DAW-class audio alchemy. It is not Vibe Audio Player.

## docz/ vs docs/

| Folder | Role |
|---|---|
| [`docz/`](docz/) | Years of source documents copied from the Aurphyx tree. Archive. Duplicates. Bliss-era titles. **Not** VASP 3.69 canon. |
| [`docs/`](docs/) | Existing catch-all (assets, misc notes, historical VAP copies). Also not protocol truth. |

Do not implement players from `docz/`. Current names live in welcome.

## Naming

Quote retired terms, then map: SIC/SCC/ICC → SIX/SCX/ICX; rÆ → Balance State Vector; Vibe-OKF → APS-OKF.

Product titles that contain “Bliss” (Chaos & Bliss tarot; Chakra Duality Kernel **ChaosCore** / **BlissCore**) stay as welcome product names. Physics “Bliss manifold” → **Equilibrium Manifold**. Do not put Mythic Naming or Claims keys in YAML.

Machine map and vim two-phase writers: [`vim/aps_nomenclature_map.yaml`](vim/aps_nomenclature_map.yaml), [`vim/aps_nomenclature_apply.py`](vim/aps_nomenclature_apply.py) (`--dry-run` then `--apply`), [`vim/aps_okf_stamp.py`](vim/aps_okf_stamp.py) (`--plan` then `--stamp`). Root table: [`AURPHYX_TERMINOLOGY.md`](AURPHYX_TERMINOLOGY.md). Volume briefing: [`vim/PROJECT_CONTEXT.md`](vim/PROJECT_CONTEXT.md).

## Folder overlays

Each volume should have README, PROJECT_CONTEXT, PHYSICS, INVARIANTS. Many still lack them. Do not invent physics to fill gaps.

Empty / parked product stubs: `audry/`, `soulshot/`, `soulsync/`, `tob/`.

## Related

- [`SUMZ-SUGGZ.md`](SUMZ-SUGGZ.md) — cross-stack execution series
- [`INVARIANTS.md`](INVARIANTS.md) — organism locks (when present)
- [`PHYSICS.md`](PHYSICS.md) — shared constants (when present)
- [`aps-okf.yaml`](aps-okf.yaml) — public header contract
