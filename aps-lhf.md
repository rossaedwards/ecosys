---
type: standard-framework-foundational
title: Aurphyx Primordial Standard — Lattice Header Format
description: Official nine-key YAML header that turns any document into a lattice-addressable object. Machine contract is aps-lhf.yaml. Aurphyx APS objects use the aps-tslca profile.
workspaces: rossaedwards/ecosys, aurphyx/ecosys
services:
  - Audry
  - Aura
  - AuraFS
  - Fuxyez
  - SAGES
  - SoulSync
  - GVS
domains:
  - cognition
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
---

# APS-LHF

**Aurphyx Primordial Standard — Lattice Header Format**  
Version 1.0.0 · Status stable  
Machine contract: [`aps-lhf.yaml`](./aps-lhf.yaml)  
Aurphyx-internal law: [`aps-lhf.md`]

Format license: **Apache-2.0**  
Implement, validate, emit, and ship this header in any project.  
Document bodies keep whatever license they already have.

**This format is aps-lhf. It is not Vibe.**  
Vibe / VASP is the audio standard and player. Do not name this header Vibe-OKF.

## 1. What this is

A document without a header is text. A document with aps-lhf is an object.

The header answers seven questions before anyone reads the body:

| Question                          | Key                        |
| --------------------------------- | -------------------------- |
| What kind of object is this?      | `type`                     |
| What is it called?                | `title`                    |
| What is it, in one sentence?      | `description`              |
| Who owns the source?              | `workspaces`               |
| What runs or consumes it?         | `services`                 |
| What subject does it govern?      | `domains`                  |
| Where does it sit on the lattice? | `nodes`, `cores`, `fields` |

Nine keys. Fixed order. No extras in v1.

This is the public formalization of the TSLCA frontmatter. Aurphyx APS documents are the `aps-tslca` profile of this contract. [`aps-lhf.md`](./aps-lhf.md) remains the Aurphyx-internal law. This file is the public spec.

## 2. Official header (drop-in)

Portable form. Any project. String or list values are both valid against [`aps-lhf.yaml`](./aps-lhf.yaml).

```yaml
---
type: "kind-of-project-you-have"
title: "your-project-name-here"
description: "your-choice-project-descriptions"
workspaces: "your-choice-workspace/foldernames"
services: "your-choice-services-listed-here"
domains: "your-choice-company-domains-folders-repos"
nodes: "your-choice-projects-internals"
cores: "your-choice-hardware-devices"
fields: "your-choice-regions-markets-industries"
---
```

Typed list form for a small protocol object:

```yaml
---
type: protocol-spec
title: Harmonic Integrity Thresholds
description: Triple-threshold gate that permits creation, integration, or renewal.
workspaces: rossaedwards/ecosys, aurphyx/ecosys
services:
  - Audry
  - SAGES
domains:
  - systems
  - governance
nodes:
  - SCX⊗SCX
  - ICX⊗SCX
cores:
  - SCX
  - ICX
fields:
  - harmonic-integrity-field
---
```

## 3. TSLCA gold example

Default profile `aps-tslca`. Cores locked to SIX, SCX, ICX. Nodes are the full nine-cell lattice, row-major, Unicode `⊗`. Off-diagonal cells are ordered: `SIX⊗SCX` is not `SCX⊗SIX`.

```yaml
---
type: standard-framework-foundational
title: Aurphyx Primordial Standard — Object Knowledge Frontmatter
description: Official nine-key YAML header that turns any document into a lattice-addressable object. Machine contract is aps-lhf.yaml. Aurphyx APS objects use the aps-tslca profile.
workspaces: rossaedwards/ecosys, aurphyx/ecosys
services:
  - Audry
  - Aura
  - AuraFS
  - Fuxyez
  - SAGES
  - SoulSync
  - GVS
domains:
  - cognition
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
---
```

That header is the gold example. Copy it. Do not invent a tenth key.

## 4. The nine keys

Order is law. Emit keys in this sequence.

| Key           | Meaning                                                                                                                                   |
| ------------- | ----------------------------------------------------------------------------------------------------------------------------------------- |
| `type`        | Kind of object. Must be a registry value.                                                                                                 |
| `title`       | Human-readable name.                                                                                                                      |
| `description` | One-sentence scope.                                                                                                                       |
| `workspaces`  | Source ownership. Aurphyx default: `rossaedwards/ecosys, aurphyx/ecosys`. Lab first, publish repo second. Do not push from this lab task. |
| `services`    | What runs or consumes the object.                                                                                                         |
| `domains`     | Subject, company domains, folders, or repos.                                                                                              |
| `nodes`       | Lattice cells. TSLCA writes `CORE⊗CORE`.                                                                                                  |
| `cores`       | Basis triad. TSLCA locks SIX, SCX, ICX.                                                                                                   |
| `fields`      | Named fields, regions, markets, or industries.                                                                                            |

`additionalProperties` is false. Extra keys fail v1.

## 5. Type registry

`type` must be one of:

- `standard-framework-foundational`
- `theory-standard`
- `protocol-spec`
- `standard-section`
- `standard-appendix`
- `implementation-note`
- `overview`

## 6. Profiles

| Profile     | Use                                           | Cores                      | Nodes                                                       |
| ----------- | --------------------------------------------- | -------------------------- | ----------------------------------------------------------- |
| `aps-tslca` | Default. Aurphyx APS / TSLCA objects. Locked. | SIX, SCX, ICX              | Full 3×3 row-major, or a subset whose sides are those cores |
| `aps-core`  | Portable triad for other projects.            | Any named triad, max three | `CORE⊗CORE` whose sides are listed cores                    |

Invariants for both profiles:

- Every node side must be a listed core.
- A full 3×3 is written row-major.
- Tensor products are non-commutative.

## 7. Application law

1. Prepend the header. Do not rewrite the body.
2. Skip binaries.
3. Do not stamp unrelated folders as a side effect of adopting this spec.
4. `rossaedwards/ecosys` is the lab. `aurphyx/ecosys` is the later publish repo.
5. Document bodies keep their existing license. The format license is Apache-2.0.

## 8. What this is not

**This public format is aps-lhf.**  
[`aps-lhf.md`](./aps-lhf.md) remains the Aurphyx-internal law.
