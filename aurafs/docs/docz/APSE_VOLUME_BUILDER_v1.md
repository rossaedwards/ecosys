APSE_VOLUME_BUILDER_v1

### APSE VOLUME BUILDER v1 — Specification

A deterministic, reproducible builder that materializes APSE volumes from compiler outputs. The builder **stitches fragments**, **wires cross references**, and **emits per‑volume source trees** ready for rendering. It does not reclassify content or change canonical terminology; it assembles and formats.

---

### Inputs and Outputs

**Inputs**

- **Compiler artifacts**
  - `aps_volume_map.json` — fragment → volume routing.
  - `apse_fragment_index.json` — fragment metadata and content spans.
  - `apse_crossref_index.json` — cross‑reference graph.
  - `aps_symbols_map.json` — canonical symbols and locations.
- **Source trees**
  - `C:\rossaedwards\main\aps\` and configured APS roots.
  - `C:\rossaedwards\main\vim\extracted_math_v32\` (equation corpus).
- **Layout rules**
  - `apse_build_config.yaml` (builder options, templates, ordering overrides).
  - `APS_LATEX_PREAMBLE_SPEC.md` and Markdown templates.

**Primary outputs**

- Per‑volume directory trees under `/APSE/volumes/Volume_<ID>/`.
- `volume.yaml` metadata for each volume.
- `index.md` and `sections/*.md` (and optional `latex/` mirror).
- `apse_build_manifest.json` with provenance and hashes.
- Warnings and a `build_report.json` listing unresolved references and confidence scores.

---

### Directory Layout and File Schemas

**Per‑volume layout**

```
APSE/volumes/Volume_APSE-A.7/
  volume.yaml
  index.md
  sections/
    A.7.1.md
    A.7.2.md
  figures/
  latex/
    main.tex
    sections/
      A.7.1.tex
  assets/
  metadata/
    fragments.json
    crossrefs.json
```

**volume.yaml schema (one line per field)**

| **field** | **meaning** |
|---|---|
| `volume_id` | canonical volume id |
| `title` | human title |
| `status` | draft; frozen; published |
| `compiler_version` | compiler git hash |
| `builder_version` | builder git hash |
| `source_commit` | source tree commit |
| `sections` | ordered list of section ids |

**section file conventions**

- **Filename:** `<SectionID>.md` (e.g., `A.7.2.md`)  
- **Header block:** YAML front matter with `section_id`, `title`, `fragments` (ordered list).  
- **Anchors:** every fragment inserted with `<!-- FRAG:FRAG_ID -->` and `#frag-FRAG_ID` anchor line.

**Equation labeling**

- **Equation ID:** `EQ_<VolumeID>_<SectionID>_<Index>`  
- **LaTeX label:** `\label{eq:APSE-A.7:000123}`  
- **Markdown reference:** `[Eq. (A.7.1)](#eq-APSE-A.7-000123)`

---

### Assembly Pipeline and Algorithms

**Stage 1 Discovery**

- Read `aps_volume_map.json` to enumerate volumes and section paths.
- Validate fragment provenance and content availability.

**Stage 2 Section Ordering**

- For each section gather fragments:
  - Primary sort: `volume_map.priority` (higher first).
  - Secondary sort: `source_order` (file line numbers).
  - Tertiary: `confidence` (compiler score).
- Apply manual ordering overrides from `apse_build_config.yaml`.

**Stage 3 Stitching**

- For each fragment:
  - Insert canonical header anchor.
  - Normalize inline symbols using `aps_symbols_map.json`.
  - Preserve original source block formatting for code and math.
- Insert minimal connective text only when fragments are adjacent and a `stitch_hint` exists in fragment metadata; otherwise leave fragments as authored.

**Stage 4 Cross‑reference wiring**

- Resolve `apse_crossref_index.json`:
  - Replace symbolic references with stable anchors.
  - If target missing, mark as `TODO` and add to `build_report.json`.
- Generate inter‑volume link map for Master Index consumption.

**Stage 5 Output generation**

- Emit `sections/*.md` and `index.md` with generated TOC.
- Optionally render LaTeX mirror:
  - Convert Markdown math blocks to LaTeX environments.
  - Emit `latex/main.tex` with `\input{sections/*.tex}`.

**Determinism rules**

- All ordering decisions are deterministic and recorded in `volume.yaml`.
- Builder must be idempotent: same inputs → identical outputs.

---

### Build Tools Automation and CLI

**Primary script**

- `build_volumes_from_vim.py` orchestrates:
  - `--compile-only` (run compiler stage)
  - `--build-only` (consume existing compiler outputs)
  - `--full` (compile + build)
  - `--volume APSE-A.7` (single volume)
  - `--outdir /path/to/APSE/volumes/`

**Suggested CI steps**

1. Checkout source at pinned commit.
2. Run `python build_volumes_from_vim.py --full --outdir ./APSE/volumes/`.
3. Validate `apse_build_manifest.json` and `build_report.json`.
4. Archive `/APSE/volumes/` with manifest.

**Configuration example `apse_build_config.yaml`**

```yaml
default_template: "aps_markdown_v1"
stitch_policy: "preserve" # preserve | stitch | annotate
latex_enabled: true
manual_overrides:
  APSE-A.7:
    sections:
      A.7.1:
        order: [FRAG_A7_000123, FRAG_A7_000124]
```

---

### Quality Assurance Governance and Reproducibility

**Automated checks**

- **Link integrity:** verify all anchors resolve; list unresolved in `build_report.json`.
- **Symbol consistency:** ensure every symbol usage maps to a canonical entry; flag unmapped symbols.
- **Equation parity:** compare extracted equation hashes with emitted LaTeX labels.
- **Provenance audit:** every fragment must include `source_path` and `hash`; otherwise fail.

**Human review workflow**

- **Confidence threshold:** fragments with `confidence < 0.6` are placed in a `quarantine` section for editorial review.
- **Review queue:** `build_report.json` drives a review dashboard listing:
  - ambiguous fragments
  - unresolved crossrefs
  - deprecated legacy terms (e.g., `ZPE_CORE`) occurrences
- **Approval gating:** a volume can be marked `frozen` only after editorial signoff recorded in `volume.yaml`.

**SAGES compliance**

- Builder enforces SAGES invariants at assembly time:
  - **Non‑maleficence:** detect and flag content that proposes biological modification or prohibited operations.
  - **Transparency:** ensure provenance metadata is present for every fragment.
  - **Consent and Provenance:** any fragment claiming human subject data must include provenance and consent metadata or be quarantined.

**Reproducibility**

- Every build emits `apse_build_manifest.json` with:
  - compiler and builder git hashes
  - source commit hashes
  - timestamps
  - output checksums
- Use manifest to reproduce or verify builds.

---

### Next Steps and Integration

- Implement `APSE_VOLUME_BUILDER_v1` as a Python module with clear unit tests for:
  - fragment ordering
  - anchor insertion
  - crossref resolution
  - manifest generation
- Wire builder into CI with reproducible environments (pinned Python, LaTeX toolchain).
- Provide a lightweight review UI that consumes `build_report.json` for editorial workflows.
- After initial runs, iterate builder to support:
  - per‑volume glossaries auto‑generated from `aps_symbols_map.json`
  - thematic route generation (spines)
  - staged publishing hooks for post‑publication ingestion

---

If you want, I will now generate the **builder CLI skeleton** and a **unit test plan** for the critical assembly functions so you can start implementing `build_volumes_from_vim.py` immediately. Which would you like first: **CLI skeleton** or **unit test plan**?