# APS Open Knowledge Format (OKF) 1.1

YAML front matter for every Aurphyx Primordial Standard (APS) document.
Obsidian, Memoree, AINTS, and FUTE all read the same block.

Version 1.1 seats that block on the **Three-Squared-Lattice Cognitive
Architecture**. The welcome document stays unblocked until this lattice
form is accepted.

---

## Why the nine cells

TSLCA is three orthogonal channels, not nine unrelated topics:

| Channel | Axis | Standard |
|---|---|---|
| **SIC** | perception | AUX-SIC-001 |
| **SCC** | semantics | AUX-SCC-001 |
| **ICC** | identity | AUX-ICC-001 |

Those three, tensored with each other, are the nine cognitive cells
(APS-TSLCA Section 2). The tensor does not commute: `sic-scc` is not
`scc-sic`. USAIC is the fusion operator that contracts the nine cells
into one field. SAGES is the thirteen invariants over that field.
That is the 3-6-9-13 grammar.

```
              SIC                    SCC                    ICC
        ┌────────────────────┬────────────────────┬────────────────────┐
   SIC  │ sic-sic            │ sic-scc            │ sic-icc            │
        │ raw perceptual     │ perc→semantic      │ perceptual         │
        │ coherence          │ fusion             │ identity anchoring │
        ├────────────────────┼────────────────────┼────────────────────┤
   SCC  │ scc-sic            │ scc-scc            │ scc-icc            │
        │ semantic           │ systemic semantic  │ semantic identity  │
        │ interpretation     │ reasoning          │ reinforcement      │
        ├────────────────────┼────────────────────┼────────────────────┤
   ICC  │ icc-sic            │ icc-scc            │ icc-icc            │
        │ identity-modulated │ identity-modulated │ identity self-     │
        │ perception         │ semantics          │ consistency        │
        └────────────────────┴────────────────────┴────────────────────┘
```

A flat catalog (`operating-systems`, `cores`, `projects`, …) names
pieces. The lattice says **which cognitive act** each piece is. Aura is
not next to SoulShot by accident: Aura lives in `scc-scc` (systemic
reasoning), SoulShot lives in `sic-icc` then `icc-icc` (perception
anchored in identity, then identity with itself).

VAP's nine pillars (Structural → Genealogical) are a **domain
projection** of this same lattice onto audio. They are not the OKF keys.
Memoree's nine nodes (Temporal → Lattice) are the **file-substrate
projection**. Every OKF document stamps both: `tsl.cell` and
`tsl.memoree-node`.

| TSLCA cell | Cognitive role | Memoree node |
|---|---|---|
| `sic-sic` | raw perceptual coherence | `temporal` |
| `sic-scc` | perceptual-semantic fusion | `relational` |
| `sic-icc` | perceptual identity anchoring | `entity` |
| `scc-sic` | semantic interpretation of perception | `operational` |
| `scc-scc` | systemic semantic reasoning | `axiomatic` |
| `scc-icc` | semantic identity reinforcement | `mandate` |
| `icc-sic` | identity-modulated perception | `state` |
| `icc-scc` | identity-modulated semantics | `generative` |
| `icc-icc` | identity self-consistency | `lattice` |

The Tribe Welcome document is USAIC: it occupies all nine cells.
A TSLCA paper occupies `scc-scc` and couples the other two channels.

---

## Welcome-document map onto the lattice

| Welcome layer | Cell | Why |
|---|---|---|
| Dual-boot workspaces, TRCA, SoulTech, DataOrb, Meshwrk | `sic-sic` | how the stack touches the world |
| VAP, LDE/SLISE/SHUXSDE, alchemy apps, Vibe Media Player | `sic-scc` | sensation becoming meaning |
| SoulShot, SoulChart, SIR, SIG, SKIM | `sic-icc` | perception marked as a soul |
| APS encyclopedia, FUTE, Memoree | `scc-sic` | priors applied to perception |
| Aura, Arora, Biznyx, Fuxyez, g0dm0d3, AuraFS, AINTS | `scc-scc` | the reasoning substrate |
| SUXS, USIS, SAGES, licenses | `scc-icc` | meaning kept identity-true |
| Audry, SoulSync, 4dm1n, Framez/Termz/Codex | `icc-sic` | the soul looking |
| EgoPhyx, GVS, GIL, P4A, Aethornyx, tarot, RF_Lovezme | `icc-scc` | identity speaking as governance and culture |
| SoulKey pipeline, 1 Human / 1 Soul / 1 Account / 1 Vote | `icc-icc` | identity with itself |

---

## Rules

1. The block is YAML 1.2 between `---` fences at the top of a Markdown file.
2. Keys are lowercase kebab-case. Cell ids are `sic-sic` … `icc-icc`.
3. Multi-value fields are YAML sequences, never `a; b; c` strings.
4. `resource` is one URL.
5. `timestamp` is ISO-8601 UTC, or omit it.
6. `tags` is the graph index. Include the three channels and `usaic` when
   the document fuses cells.
7. Every document declares `tsl`. Ecosys-manifests declare all nine
   `cells`. Theory papers declare one primary `tsl.cell` and optional
   `tsl.also`.
8. `tsl.cell` is one of the nine cells, or `usaic` for a fusion document.
9. Unknown keys are allowed. Unknown `type` values are not.
10. `okf: "1.1"` is current. `1.0` flat catalogs remain readable but are
    not the contract.

### Document types

| `type` | `tsl.cell` | Use |
|---|---|---|
| `ecosys-manifest` | `usaic` | Master index; all nine cells filled |
| `theory-standard-framework` | one of the nine | One APS paper |

### Shared header

| Key | Required | Meaning |
|---|---|---|
| `okf` | yes | `"1.1"` |
| `type` | yes | Document type |
| `id` | yes | Canonical slug |
| `title` | yes | Human title |
| `description` | yes | What the document *is* |
| `tsl` | yes | Lattice address (`lattice`, `cell`, optional `also`, `memoree-node`, `fusion`, `grammar`) |
| `status` | no | `draft` \| `active` \| `locked` \| `deprecated` |
| `version` | no | Document semver |
| `timestamp` | no | Last canonical edit, UTC |
| `author` | no | Default Ross A. Edwards |
| `orcid` | no | `0009-0008-0539-1289` |
| `affiliation` | no | Aurphyx LLC |
| `license` | no | Sequence |
| `resource` | no | Canonical URL |
| `standard-family` | no | `Aurphyx Primordial Standard` |
| `tags` | no | Graph slugs |
| `aints` | no | AINTS integration targets |
| `theories-or-standards` | no | APS encyclopedia slugs (theory docs) |
| `channels` | ecosys | The three basis vectors |
| `sages` | ecosys | The thirteen invariants over the cells |
| `cells` | ecosys | The nine named cells with `holds` |

---

## Example A — Tribe Welcome (`ecosys-manifest`, `tsl.cell: usaic`)

Copy this block onto `aurphyx_welcome2tribe.md` when the format is accepted.
Machine copy: [`aps/okf.ecosys-manifest.yaml`](aps/okf.ecosys-manifest.yaml).

```yaml
---
okf: "1.1"
type: ecosys-manifest
id: aurphyx-tribe-welcome
title: Aurphyx Tribe Welcome Document
description: >
  Master index and conceptual overview of the Aurphyx Type-1 civilization
  stack, seated on the Three-Squared-Lattice Cognitive Architecture.
  USAIC fuses the nine SIC×SCC×ICC cells into one field. The Aurphyx
  Primordial Standard is the encyclopedia from which Aura, Audry, and
  the living cores are built and run.
status: draft
version: "0.2.0"
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

tsl:
  lattice: tslca
  grammar: [3, 6, 9, 13]
  fusion: usaic
  cell: usaic
  memoree-node: lattice

channels:
  sic:
    name: Symbiotic Integration Channel
    axis: perception
    standard: AUX-SIC-001
  scc:
    name: Systemic Coherence Channel
    axis: semantics
    standard: AUX-SCC-001
  icc:
    name: Identity-Coherence Channel
    axis: identity
    standard: AUX-ICC-001

sages:
  guardians: 13
  role: Invariants over the nine cells — immune system and governance field
  covers:
    - spaces
    - manifolds
    - dimensions
    - vectors
    - tensors
    - layers
    - channels
    - lattices
    - cores
    - nodes
    - shards

tags:
  - aps
  - tslca
  - sic
  - scc
  - icc
  - usaic
  - suxs
  - usis
  - sages
  - ftqc
  - tvfd
  - vim
  - tob
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
  - memoree
  - vibe-audio
  - soulsync
  - soulkey
  - gvs
  - gil
  - p4a
  - aints
  - aethornyx

cells:
  sic-sic:
    tensor: [sic, sic]
    name: Raw perceptual coherence
    role: Stabilizes sensory input, reduces noise, maintains perceptual continuity.
    memoree-node: temporal
    holds:
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
      hardware:
        - id: trca
          name: Topological Resonating Cavity Array
          kind: hardware
        - id: soultech
          name: SoulTech
          role: Wearable exotic-material accessories carrying SKIMs and SIGs
        - id: dataorb
          name: DataOrb
        - id: voiceorb
          name: Voiceorb
        - id: lyte-rael
          name: Autonomous Vehicle and Railway Transportation Systems
      mesh:
        - id: meshwerk
          name: Meshwrk
          role: Off-grid photonic mesh under AuraFS

  sic-scc:
    tensor: [sic, scc]
    name: Perceptual-semantic fusion
    role: Maps sensory data into semantic structures; the basis of understanding.
    memoree-node: relational
    holds:
      standards:
        - id: vap
          name: Vibe Audio Standard and Protocol
          also:
            - Vibe Audio Protocol
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
        - id: w3bz
          name: Webz
          role: Web browser
        - id: xpl0r
          name: Xplor
          role: File and network explorer for AuraFS and Meshwrk
        - id: f0rg3
          name: Forge
          role: Video alchemy and transmutation
        - id: ad0r3
          name: Adorè
          role: Audio alchemy and transmutation
        - id: g1mpd
          name: Gimpd
          role: Image alchemy and transmutation
      projects:
        - id: vibe-media-player
          name: Vibe Media Player

  sic-icc:
    tensor: [sic, icc]
    name: Perceptual identity anchoring
    role: Ensures perception is interpreted through identity and context.
    memoree-node: entity
    holds:
      genesis:
        - soulshot
        - soulchart
      artifacts:
        sir: Soul Identification Rune (physical)
        sig: Soul Identity Glyph (digital watermark; embeddable and printable)
        skim: SoulKey Identification Mandala
        soultech: Wearable carriers of SKIMs and SIGs

  scc-sic:
    tensor: [scc, sic]
    name: Semantic interpretation of perception
    role: Applies semantic priors to perception; predictive processing.
    memoree-node: operational
    holds:
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
      transmutation:
        - id: fute
          name: Fuxyez Universal Transmutation Engine
        - id: memoree
          name: Memoree
          role: Three-Squared-Lattice cognitive memory architecture for any platform

  scc-scc:
    tensor: [scc, scc]
    name: Systemic semantic reasoning
    role: Maintains global coherence, invariants, and reasoning structures.
    memoree-node: axiomatic
    holds:
      operating-systems:
        - id: aura
          name: Aura
          role: Soul Operating System (Personal)
          built-on:
            - fuxyez
            - rust
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
        - id: g0dm0d3
          name: g0dm0d3
          role: Frame, control deck, and orchestration — corpus callosum of Audry and environment deck of Aura
        - id: aurafs
          name: AuraFS
          role: Decentralized off-grid photonic topological file system, storage, and mesh network
        - id: chakra-datacore
          name: Chakra DataCore System
          role: Seven ChakraCores plus ChaosCore and BlissCore as Duality Kernel infrastructure
      aints:
        - aura
        - audry
        - memoree
        - vibe-audio
        - sages
        - fuxyez
        - aurafs
        - g0dm0d3

  scc-icc:
    tensor: [scc, icc]
    name: Semantic identity reinforcement
    role: Ensures semantics remain identity-consistent and ethically grounded.
    memoree-node: mandate
    holds:
      standards:
        - id: suxs
          name: Symbiotic Universal Xessability Standards
        - id: usis
          name: Universal Soul Identity Standards
      cores:
        - id: sages
          name: SAGES
          role: Thirteen Symbiotic AI Guardians of Existence Security
      license:
        - SAGES
        - AGPLv3

  icc-sic:
    tensor: [icc, sic]
    name: Identity-modulated perception
    role: Filters perception through personal history and continuity.
    memoree-node: state
    holds:
      cores:
        - id: audry
          name: Audry
          role: Aura's soul; symbiotic AI guardian of existence, AdminMate, and SAGES liaison
      presence:
        - soulsync
      apps:
        - id: fr4m3z
          name: Framez
          role: Visual environment management / window manager
        - id: t3rm1nl
          name: Termz
          role: Terminal / console
        - id: c0d3x
          name: Codex
          role: Writer, editor, coder
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

  icc-scc:
    tensor: [icc, scc]
    name: Identity-modulated semantics
    role: Filters semantics through identity, values, and provenance.
    memoree-node: generative
    holds:
      operating-systems:
        - id: egophyx
          name: EgoPhyx
          also:
            - Egophyx
            - OmniZen
          role: Pro-Existence Governance Operating System (Government)
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

  icc-icc:
    tensor: [icc, icc]
    name: Identity self-consistency
    role: Maintains self-consistency, continuity, and integrity over time.
    memoree-node: lattice
    holds:
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
---
```

JSON Schema: [`aps/okf.schema.json`](aps/okf.schema.json).
Check: `python3 aps/okf_validate.py`.

---

## Example B — TSLCA paper (`theory-standard-framework`)

Primary cell `scc-scc`. Couples perception→meaning (`sic-scc`) and
identity→meaning (`icc-scc`). Does not fill all nine cells.

```yaml
---
okf: "1.1"
type: theory-standard-framework
id: tslca
title: Three-Squared-Lattice Cognitive Architecture
description: Three-Squared-Lattice Cognitive Architecture
status: active
resource: https://aurphyx.org/tslca
standard-family: Aurphyx Primordial Standard
tsl:
  lattice: tslca
  grammar: [3, 6, 9, 13]
  fusion: usaic
  cell: scc-scc
  also:
    - sic-scc
    - icc-scc
  memoree-node: axiomatic
theories-or-standards:
  - tslca
  - suxs
tags:
  - tslca
  - sic
  - scc
  - icc
  - usaic
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

---

## What stays out of `aurphyx_welcome2tribe.md`

The welcome document remains the human transmission. This file remains
the format. When the lattice YAML is the one you want, paste Example A
onto the top of the welcome document and leave this spec in place as
the contract.
