# APS Canon Compiler — Requirements Document

## 1. Purpose

The APS Canon Compiler is a Python-based system that:

- Scans the entire `rossaedwards/main` repository.
- Reads, analyzes, and indexes all scientific content (math, prose, code, terminology).
- Builds a Master Table of Contents (TOC) and Master Content Index for the entire APS canon.
- Classifies all content into APS Volumes using a hybrid map (`aps_volume_map.json`).
- Normalizes symbols and terminology using a symbol map (`aps_symbols_map.json`).
- Converts `.md` → `.tex` with math preservation and APS preamble.
- Builds the canonical APS LaTeX structure (Option C: one folder per APS Volume).
- Prioritizes FTQC (APS Volume XVII) for arXiv readiness.

The compiler does NOT assume any prior structure beyond the current repo layout.

---

## 2. Scope

The compiler must operate over the entire `main` repo, including but not limited to:

- `vim/`
- `ftqc/`
- `tvfd/`
- `tslca/`
- `sages/`
- `standards/`
- `fuxyez/`
- `extracted_math_v32/`
- root-level `.md`, `.tex`, `.py`, `.ipynb`, `.json`, `.txt`

It must NOT ignore:

- `hif_*.md`
- `section_c.md`
- `tsl_*.md`
- `SCIENTIFIC_TERMINOLOGY.md`
- `SCIENTIFIC_TERMINOLOGIES.md`
- `PHYSICS.md`

All `.md` and `.tex` files must be scanned, read, analyzed, and considered for inclusion.

---

## 3. Phase 1 — Scan, Read, Analyze, Index

### 3.1 Full Repository Scan

- Recursively walk the repo from the root.
- Collect all files with extensions:
  - `.md`, `.tex`, `.py`, `.ipynb`, `.json`, `.txt`, `.svg`
- Store file metadata:
  - path, size, modified time, extension, folder.

### 3.2 Content Parsing

For each file:

- `.md`: parse headings, paragraphs, code blocks, math blocks.
- `.tex`: parse sections, math environments, macros.
- `.py`: parse docstrings, comments, function/class names (for figure metadata and scientific logic).
- `.ipynb`: extract markdown cells and code cells.
- `.json`: parse math summaries (e.g., `extracted_math_v32/summary.json`).
- `.txt`: parse lists, notes, file inventories.

### 3.3 Structural Extraction

Extract:

- Section and subsection headers.
- Appendix markers.
- Domain keywords (e.g., “FTQC”, “TSLCA”, “SAGES”, “Balance Geometry”, “Vacuum Field”).
- Cross-references (e.g., “see Section LXVI”, “see Appendix A”).
- Figure references (e.g., `fig_001_001`, `fig_016_016_geodesic_deviation`).
- Citations (if present).

### 3.4 Mathematical Extraction

For `.md` and `.tex`:

- Identify inline math: `$...$`, `\(...\)`.
- Identify display math: `$$...$$`, `\[...\]`.
- Identify raw TeX blocks.
- Extract equation labels and references.

For `extracted_math_v32/`:

- Read `CODEX_*.md` and `summary.json`.
- Map extracted equations back to source files where possible.
- Flag mismatches or missing equations.

### 3.5 Terminology Extraction

From:

- `SCIENTIFIC_TERMINOLOGY.md`
- `SCIENTIFIC_TERMINOLOGIES.md`
- `PHYSICS.md`
- HIF files
- TSL files
- other glossary-like files

Extract:

- term → definition pairs.
- symbol → meaning pairs.
- domain tags (e.g., “FTQC”, “Balance Geometry”, “SAGES”).

---

## 4. Phase 2 — Master TOC and Master Index

### 4.1 Master TOC

Generate:

- `APS_MASTER_TOC.json`
- `APS_MASTER_TOC.md`

Each entry should include:

- file path
- title (from first heading or inferred)
- type (section, appendix, glossary, figure script, etc.)
- domain tags (e.g., FTQC, TSLCA, SAGES)
- candidate APS Volume (initial guess)

### 4.2 Master Index

Generate:

- `APS_MASTER_INDEX.json`
- `APS_MASTER_INDEX.md`

Each entry should include:

- file path
- list of math environments
- list of key terms
- list of figures referenced
- list of cross-references
- list of terminology references
- candidate APS Volume (initial guess)
- confidence score (for classification)

---

## 5. Phase 3 — Hybrid Volume Map (aps_volume_map.json)

The compiler must generate a hybrid map:

- auto-generated fields (based on analysis)
- manual override fields
- TODO notes
- Cursor guide notes

Output:

- `aps_volume_map.json`
- `APS_VOLUME_MAP_GUIDE.md`

The JSON must support:

- `auto_keywords`
- `auto_patterns`
- `auto_folders`
- `manual_files`
- `manual_notes`
- `cursor_guide`

The Markdown guide must explain:

- how to edit the map
- how auto vs manual resolution works
- how to add new volumes

---

## 6. Phase 4 — Symbol Map (aps_symbols_map.json)

The compiler must either:

- read an existing `aps_symbols_map.json`, or
- generate a starter map based on the current symbol usage and the previously defined renaming rules.

Output:

- `aps_symbols_map.json`
- `APS_SYMBOLS_MAP_GUIDE.md`

The symbol map must include:

- prose replacements (e.g., “rÆ” → “Balance State Vector”)
- LaTeX replacements (e.g., `\mathbf{rAE}` → `\mathbf{x}`)
- variable replacements (e.g., `rAE_f` → `x_f`)
- domain-specific symbols (FTQC, TSLCA, SAGES, etc.)

---

## 7. Phase 5 — APS Volume Structure (Option C)

The compiler must create:

`latex_output/APS_Volumes/`

Inside it:

- `APS_Volume_I_Theory_of_Balance/`
- `APS_Volume_II_Balance_Geometry/`
- `APS_Volume_III_Balance_Continuum/`
- `APS_Volume_IV_Gravity_Field/`
- `APS_Volume_V_Vacuum_Field/` (TVFD)
- `APS_Volume_XVII_FTQC/`
- `APS_Volume_XVIII_TSLCA/`
- `APS_Volume_XXV_Fuxyez/`
- `APS_Volume_XXXIII_SAGES/`
- `APS_Volume_XXXIV_Standards/`
- `APS_Volume_XXXVII_Cosmology/`
- (and so on, extensible)

Each volume folder must contain:

- `main.tex`
- `sections/`
- `appendices/`
- `figures/`
- `bibliography.bib` (if needed)
- a small `README.md` describing the volume.

---

## 8. Phase 6 — MD → TeX Conversion

For all `.md` files assigned to a volume:

- Use Pandoc with math-preserving flags:
  - `-f markdown+tex_math_dollars+tex_math_single_backslash+raw_tex`
  - `-t latex --wrap=none`
- Insert APS/arXiv preamble:
  - `\documentclass{article}` or `book` (configurable)
  - `amsmath`, `amssymb`, `amsthm`, `hyperref`, `geometry`, `inputenc`, `fontenc`
- Apply `aps_symbols_map.json` to the generated `.tex`.
- Ensure LaTeX syntax remains valid.

---

## 9. Phase 7 — FTQC Priority (APS Volume XVII)

For `APS_Volume_XVII_FTQC`:

- Generate `ftqc_arxiv.tex` as the main entry point.
- Ensure:
  - abstract
  - author block
  - ORCID
  - APS-style preamble
  - figures included
  - bibliography included (if present)
  - arXiv-safe packages only
- Optionally generate:
  - `FTQC_SECTION_MAP.md`
  - `FTQC_TODO_EXPANSIONS.md` (for analogies, deeper explanations, etc.)

---

## 10. Phase 8 — Non-Destructive Operation

The compiler must:

- Never delete original files.
- Write all outputs to:
  - `APS_MASTER_TOC.*`
  - `APS_MASTER_INDEX.*`
  - `aps_volume_map.json`
  - `aps_symbols_map.json`
  - `latex_output/APS_Volumes/`
- Log operations to:
  - `APS_COMPILER_LOG.txt`
