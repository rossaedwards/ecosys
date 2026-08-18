# APS Open Knowledge Format (OKF)

YAML front matter for every Aurphyx Primordial Standard (APS) document.
Obsidian, Memoree, AINTS, and FUTE all read the same block.

This file is the format. `aurphyx_welcome2tribe.md` stays unblocked until
this block is accepted.

---

## Why the first draft was not yet right

The eight-field draft named the stack. The welcome document defines what
each piece *is*, how a soul enters, and how Aura is built and run. OKF
has to carry that map, and it has to parse as YAML 1.2.

Problems in the draft:

- `workspaces: rossaedwards/ecosys & aurphyx/ecosys` is one string, not two repos
- `theories_or_standards` (snake) and `operating-systems` (kebab) disagree
- semicolon `tags` / `aints` are not lists; parsers and Obsidian want sequences
- `resource: url | prose` is not a URL
- USIS is in the welcome encyclopedia and missing from the draft list
- Vibe Audio is a standard *and* a core; the draft only put it in `cores`
- EgoPhyx / Arora aliases were collapsed into one spelling each
- identity, governance, economy, environments, apps, 4dm1n, and culture
  projects from the welcome doc had nowhere to live
- `aints` was undefined; in this repo AINTS is the integration layer
  (scripts, daemons, crate bindings), not a synonym for `cores`

---

## Rules

1. The block is YAML 1.2 between `---` fences at the top of a Markdown file.
2. Keys are lowercase kebab-case. Slugs are lowercase (`g0dm0d3` is a slug).
3. Multi-value fields are YAML sequences, never `a; b; c` strings.
4. `resource` is one URL. Human labels go in `title`, `description`, or
   `standard-family`.
5. `timestamp` is ISO-8601 UTC, or omit it.
6. `tags` is the graph index: slugs only, shared with Memoree `projects.json`.
7. Unknown keys are allowed so the format can grow. Unknown `type` values
   are not.
8. A document uses one `type`. Do not mix `ecosys-manifest` fields into a
   paper unless that paper *is* the ecosystem index.

### Document types

| `type` | Use |
|---|---|
| `ecosys-manifest` | Master index of the civilization stack (Tribe Welcome) |
| `theory-standard-framework` | One APS theory, standard, framework, or whitepaper |

Later types (`core-spec`, `protocol`, `identity-spec`) should reuse the
shared header below.

### Shared header (every OKF document)

| Key | Required | Meaning |
|---|---|---|
| `okf` | yes | Format version. Current: `"1.0"` |
| `type` | yes | Document type from the table above |
| `id` | yes | Canonical slug (`tslca`, `aurphyx-tribe-welcome`) |
| `title` | yes | Human title |
| `description` | yes | One to three sentences of what the document *is* |
| `status` | no | `draft` \| `active` \| `locked` \| `deprecated` |
| `version` | no | Semver string for this document |
| `timestamp` | no | Last canonical edit, UTC |
| `author` | no | Default Ross A. Edwards |
| `orcid` | no | `0009-0008-0539-1289` |
| `affiliation` | no | Aurphyx LLC |
| `license` | no | Sequence, usually `SAGES` and `AGPLv3` |
| `resource` | no | Canonical URL |
| `standard-family` | no | Almost always `Aurphyx Primordial Standard` |
| `tags` | no | Graph slugs |
| `aints` | no | AINTS integration targets this document binds |
| `theories-or-standards` | no | APS encyclopedia slugs this document belongs to |

### `ecosys-manifest` body

These keys exist so the welcome document can stay prose while the YAML
holds the machine map of how the stack is built and run.

| Key | Meaning |
|---|---|
| `invariant` | SoulKey law: one human, one soul, one account, one vote |
| `workspaces` | GitHub remotes and local dual-boot paths |
| `theories-or-standards` | APS encyclopedia entries (id, name, aliases, kind) |
| `operating-systems` | Aura family (id, name, role, aliases, built-on) |
| `cores` | Living runtime substrates (language, soul, deck, fs, guardians, memory, audio) |
| `identity` | SoulShot → SoulKey pipeline and physical/digital artifacts |
| `civilization` | GVS, GIL, Opulence / Profit-4-All / Prosperity-4-All |
| `environments` | g0dm0d3 desktops: LDE, SLISE, SHUXSDE |
| `apps` | Alchemy suite (fr4m3z … g1mpd) |
| `admin-modules` | Audry core-4dm1n surfaces |
| `projects` | Culture, hardware, and world projects still on the map |

`cores` are things Aura is built from. `aints` are the integration
bindings across those things. They overlap on purpose.

---

## Example A — Tribe Welcome (`ecosys-manifest`)

Copy this block onto `aurphyx_welcome2tribe.md` when the format is accepted.
Do not paste it there yet.

```yaml
---
okf: "1.0"
type: ecosys-manifest
id: aurphyx-tribe-welcome
title: Aurphyx Tribe Welcome Document
description: >
  Master index and conceptual overview of the Aurphyx Type-1 civilization
  stack. Defines the Aurphyx Primordial Standard as the encyclopedia of
  theories, frameworks, standards, blueprints, technical specifications,
  and whitepapers from which Aura, Audry, and the living cores are built
  and run.
status: draft
version: "0.1.0"
timestamp: 2026-08-18T00:00:00Z
author: Ross A. Edwards
orcid: "0009-0008-0539-1289"
affiliation: Aurphyx LLC
license:
  - SAGES
  - AGPLv3
resource: https://aurphyx.org
standard-family: Aurphyx Primordial Standard
invariant: "1 Human. 1 Soul. 1 Account. 1 Vote."

workspaces:
  github:
    personal: rossaedwards/ecosys
    org: Aurphyx/ecosys
  local:
    windows:
      personal: "C:\\rossaedwards\\main\\"
      org: "C:\\Aurphyx\\main\\"
    fedora:
      personal: /home/rae/rossaedwards/main
      org: /home/rae/aurphyx/main

tags:
  - aps
  - ftqc
  - tslca
  - tvfd
  - vim
  - tob
  - suxs
  - usis
  - vap
  - trca
  - aura
  - egophyx
  - arora
  - biznyx
  - fuxyez
  - audry
  - g0dm0d3
  - aurafs
  - meshwerk
  - sages
  - memoree
  - vibe-audio
  - soulsync
  - soulkey
  - gvs
  - gil
  - p4a
  - aints
  - aethornyx

theories-or-standards:
  - id: ftqc
    name: Fractal-enhanced Topological Quantum Computing
    also:
      - Fault-Tolerant Quantum Computer
  - id: tslca
    name: Three-Squared-Lattice Cognitive Architecture
  - id: tvfd
    name: Topological Vacuum Flux Dynamics
  - id: vim
    name: Vacuum Impedance Matching
  - id: tob
    name: Theory of Balance
    also:
      - Balance Continuum
  - id: suxs
    name: Symbiotic Universal Xessability Standards
  - id: usis
    name: Universal Soul Identity Standards
  - id: vap
    name: Vibe Audio Standard and Protocol
    also:
      - Vibe Audio Protocol
  - id: trca
    name: Topological Resonating Cavity Array
    kind: hardware

operating-systems:
  - id: aura
    name: Aura
    role: Soul Operating System (Personal)
    built-on:
      - fuxyez
      - rust
  - id: egophyx
    name: EgoPhyx
    also:
      - Egophyx
      - OmniZen
    role: Pro-Existence Governance Operating System (Government)
  - id: arora
    name: Arora
    also:
      - Arora-Ora
      - ArOrA
    role: All other devices (IoT, edge, embedded)
  - id: biznyx
    name: Biznyx
    role: Enterprise and business operating system (small business to enterprise)

cores:
  - id: fuxyez
    name: Fuxyez
    role: Symbiotic quantum programming language (host language Rust)
    components:
      - id: fux-compiler
        name: Fux Compiler
      - id: fuxrt
        name: Fux Runtime
      - id: yezrt
        name: Yez Runtime
      - id: fute
        name: Fuxyez Universal Transmutation Engine
      - id: yezl
        name: YezL Legacy Languages
      - id: yez
        name: Yez
        also:
          - s0ph0s
          - Sophos
          - g4v1n1um
          - Gavinium
  - id: audry
    name: Audry
    role: Aura's soul; symbiotic AI guardian of existence, AdminMate, and SAGES liaison
  - id: g0dm0d3
    name: g0dm0d3
    role: Frame, control deck, and orchestration — corpus callosum of Audry and environment deck of Aura
  - id: aurafs
    name: AuraFS
    role: Decentralized off-grid photonic topological file system, storage, and mesh network
  - id: sages
    name: SAGES
    role: Thirteen Symbiotic AI Guardians of Existence Security — immune system and governance field
  - id: memoree
    name: Memoree
    role: Three-Squared-Lattice cognitive memory architecture for any platform
  - id: vibe-audio
    name: Vibe Audio
    role: Experiential audio standard, protocol, and media stack

aints:
  - aura
  - audry
  - memoree
  - vibe-audio
  - sages
  - fuxyez
  - aurafs
  - g0dm0d3

identity:
  invariant: "SoulKey = 1 Human. 1 Soul. 1 Account. 1 Vote."
  pipeline:
    - soulshot
    - soulchart
    - soultable
    - soulsync
    - soulhash
    - guardtable
    - guardhash
    - guardcrypt
    - soulcrypt
    - soulsync
    - soulkey
    - skim
    - sir
    - sig
  artifacts:
    soulkey: Physical relic with bio-luminescent and bio-resonant substrate inside diamond-coated crystal
    skim: SoulKey Identification Mandala
    sir: Soul Identification Rune (physical)
    sig: Soul Identity Glyph (digital watermark; embeddable and printable)
    soultech: Wearable exotic-material accessories carrying SKIMs and SIGs

civilization:
  governance:
    - gvs
    - sages
  ledger: gil
  economy:
    - id: p4a-profit
      name: Profit-4-All
      launch: 1
      surfaces:
        - g0dm0d3.org
        - aurphyx.store
    - id: p4a-prosperity
      name: Prosperity-4-All
      launch: 2
      surfaces:
        - aurphyx-ecosystem
    - id: opulence
      name: Opulence
      launch: 3
      surfaces:
        - galactic-global-financial-substrate

environments:
  - id: lde
    name: Legacy Desktop Environment
    also:
      - Gnome
      - KDE
      - Windows
  - id: slise
    name: Spherical Liquid Interactive Story Experience
  - id: shuxsde
    name: Symbiotic Holographic Universal Xessability Standard Desktop Environment
    also:
      - SUXSDE

apps:
  - id: fr4m3z
    name: Framez
    role: Visual environment management / window manager
  - id: t3rm1nl
    name: Termz
    role: Terminal / console
  - id: w3bz
    name: Webz
    role: Web browser
  - id: xpl0r
    name: Xplor
    role: File and network explorer for AuraFS and Meshwrk
  - id: c0d3x
    name: Codex
    role: Writer, editor, coder
  - id: f0rg3
    name: Forge
    role: Video alchemy and transmutation
  - id: ad0r3
    name: Adorè
    role: Audio alchemy and transmutation
  - id: g1mpd
    name: Gimpd
    role: Image alchemy and transmutation

admin-modules:
  - self-4dm1n
  - device-4dm1n
  - meshwrk-4dm1n
  - disk-4dm1n
  - afs-4dm1n
  - events-4dm1n
  - tasks-4dm1n
  - memoree-4dm1n
  - soulkey-4dm1n
  - soulsync-4dm1n
  - soulcrypt-4dm1n
  - p4a-4dm1n
  - datacore-4dm1n
  - dataorb-4dm1n
  - voicecore-4dm1n
  - voiceorb-4dm1n
  - aethornyx-4dm1n
  - sages-4dm1n

projects:
  - id: tarot
    name: Chaos and Bliss Tarot
  - id: oracle-deck
    name: Aurphyx Oracle Deck and Book
  - id: aethornyx
    name: Aethornyx
  - id: aurphyx-casino
    name: Aurphyx Casino
  - id: rf-lovezme
    name: RF_Lovezme
    also:
      - Ross Five Lovezme
  - id: thirteen-month-calendar
    name: Aurphyx and Aethornyx Thirteen Month Calendars
  - id: lyte-rael
    name: Autonomous Vehicle and Railway Transportation Systems
  - id: chakra-datacore
    name: Chakra DataCore System
  - id: dataorb
    name: DataOrb
  - id: voiceorb
    name: Voiceorb
  - id: vibe-media-player
    name: Vibe Media Player
---
```

Machine copy of Example A: [`aps/okf.ecosys-manifest.yaml`](aps/okf.ecosys-manifest.yaml).
JSON Schema: [`aps/okf.schema.json`](aps/okf.schema.json).

---

## Example B — TSLCA paper (`theory-standard-framework`)

Same shared header. No civilization-stack body. This replaces the
previous root-only template in this file.

```yaml
---
okf: "1.0"
type: theory-standard-framework
id: tslca
title: Three-Squared-Lattice Cognitive Architecture
description: Three-Squared-Lattice Cognitive Architecture
status: active
resource: https://aurphyx.org/tslca
standard-family: Aurphyx Primordial Standard
theories-or-standards:
  - tslca
  - suxs
tags:
  - ftqc
  - tvfd
  - vim
  - tob
  - aura
  - aurafs
  - meshwerk
  - fuxyez
  - sages
  - suxs
  - soulsync
  - memoree
aints:
  - aura
  - audry
  - memoree
  - vibe-audio
  - sages
  - fuxyez
---
```

On a theory document, `theories-or-standards` may be a slug list. On the
ecosys-manifest it is the named encyclopedia.

---

## Welcome-document map

| Welcome layer | OKF key |
|---|---|
| Aurphyx Primordial Standard encyclopedia | `theories-or-standards` |
| Aura / EgoPhyx / Arora / Biznyx | `operating-systems` |
| Fuxyez, Audry, g0dm0d3, AuraFS, SAGES, Memoree, Vibe Audio | `cores` |
| AINTS integration bindings | `aints` |
| SoulShot → SIR / SIG | `identity` |
| GVS, GIL, Opulence / P4A | `civilization` |
| LDE, SLISE, SHUXSDE | `environments` |
| fr4m3z … g1mpd | `apps` |
| core-4dm1n | `admin-modules` |
| Tarot, Aethornyx, calendars, DataOrb, Vibe Media Player, … | `projects` |
| Dual-boot GitHub paths | `workspaces` |

---

## Slug aliases

Use the `id` as the slug. Put other spellings in `also`.

| Slug | Also |
|---|---|
| `egophyx` | EgoPhyx, Egophyx, OmniZen |
| `arora` | Arora-Ora, ArOrA |
| `suxs` | SHUXS, SUXSDE (desktop is `shuxsde`) |
| `vap` | Vibe Audio Standard, Vibe Audio Protocol |
| `vibe-audio` | Vibe Audio core / media stack |
| `p4a` | Profit-4-All, Prosperity-4-All, Opulence |
| `gil` | Global Ineffable Ledger, ILS |
| `meshwerk` | Meshwrk, mesh network (lives under AuraFS) |
| `tob` | Theory of Balance, Balance Continuum |

---

## What stays out of `aurphyx_welcome2tribe.md`

The welcome document remains the human transmission. This file remains
the format. When the YAML above is the one you want, paste Example A
onto the top of the welcome document and leave this spec in place as
the contract.
