---
type: theory-standard
title: Aurphyx OKF YAML Schema
description: Nine-pillar YAML frontmatter for Aurphyx Primordial Standard documents (type, title, description, workspaces, services, domains, nodes, cores, fields).
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
  - systems-governance
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
  - HIF
---

# Aurphyx OKF YAML Schema

Every markdown standard in `tslca/` and `suxs/` uses exactly these nine pillars. Do not add a tenth key. Do not use `ecosys_identity`, `class`, `resource`, `tags`, `timestamp`, or `eco-sys` in place of these keys.

```yaml
---
type: standard-framework-foundational
title: Document title
description: One-sentence scope.
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
  - systems-governance
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
  - HIF
---
```

## Pillar meanings

| Pillar | Role |
|---|---|
| `type` | Document class (`standard-framework-foundational`, `standard-section`, `standard-axis`, `standard-operator`, `identity-standard`, `protocol-spec`, `theory-standard`, `standard-appendix`, `standard-reference`) |
| `title` | Human-readable document title |
| `description` | One-sentence scope |
| `workspaces` | Canonical repos (`rossaedwards/ecosys, aurphyx/ecosys`) |
| `services` | Instantiating systems (Audry, Aura, AuraFS, Fuxyez, SAGES, SoulSync, GVS, and identity services where named) |
| `domains` | Problem categories the document addresses |
| `nodes` | Nine TSLCA lattice cells |
| `cores` | THREE orthonormal aXes: SIX, SCX, ICX |
| `fields` | Cognitive field tensor, unified cognitive field, SAGES governance field, HIF |

## Math in markdown

Use `$...$` for inline math and `$$...$$` for display math. Do not wrap equations in `latex` code fences. Brace multi-character subscripts (`\Phi_{ij}`, `\delta_{ij}`, `\Phi_{\mathrm{unified}}`).
