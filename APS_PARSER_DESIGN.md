# APS Canon Compiler — Deep Analysis Parser Design

## 1. Purpose

This document specifies the parsing and analysis layer of the APS Canon Compiler.

The parser must:

- Read and structurally analyze all relevant files in `rossaedwards/main`.
- Extract headings, math, terminology, cross-references, and domain signals.
- Produce structured records that feed:
  - APS Master TOC
  - APS Master Index
  - Volume classification
  - Math validation
  - LaTeX generation

The parser is **read-only** and must never modify source files.

---

## 2. File Types and Responsibilities

The parser must handle at least:

- `.md` — primary manuscripts, notes, terminology, specs.
- `.tex` — existing LaTeX manuscripts and fragments.
- `.py` — figure scripts, simulation scripts, docstrings, comments.
- `.ipynb` — notebooks (markdown + code).
- `.json` — math summaries (e.g., `extracted_math_v32/summary.json`).
- `.txt` — file lists, notes, inventories.
- `.svg` — metadata only (e.g., ORCID icon).

For each file, the parser must produce a structured record with:

- `path`
- `ext`
- `title`
- `type` (section, appendix, glossary, figure_script, simulation, terminology, spec, etc.)
- `domain_tags` (e.g., FTQC, TSLCA, SAGES, Standards, Fuxyez, Balance Geometry)
- `headings`
- `math_blocks`
- `inline_math`
- `terminology_refs`
- `figure_refs`
- `cross_refs`
- `candidate_volume`
- `confidence`

---

## 3. Markdown Parsing (`.md`)

For `.md` files:

- Extract:
  - Top-level heading as `title` (if present).
  - All headings (`#`–`######`) with level and text.
  - Code blocks (language-tagged if present).
  - Inline math: `$...$`, `\(...\)`.
  - Display math: `$$...$$`, `\[...\]`.
  - Links and references.
- Detect:
  - Domain tags via keywords (FTQC, TSLCA, SAGES, Standards, Fuxyez, Balance Geometry, Vacuum Field, etc.).
  - Terminology references by matching against:
    - `SCIENTIFIC_TERMINOLOGY.md`
    - `SCIENTIFIC_TERMINOLOGIES.md`
    - `PHYSICS.md`
  - Cross-references (e.g., “see Section LXVI”, “see Appendix A”).
  - Figure references (e.g., `fig_016_016_geodesic_deviation`).

---

## 4. LaTeX Parsing (`.tex`)

For `.tex` files:

- Extract:
  - `\section`, `\subsection`, `\subsubsection` titles.
  - `\begin{equation}`, `\begin{align}`, and other math environments.
  - Inline math `$...$`, `\(...\)`.
  - Display math `$$...$$`, `\[...\]`.
  - `\label{...}` and `\ref{...}`.
- Detect:
  - Domain tags via keywords and macros.
  - Figure environments and `\includegraphics`.
  - Bibliography commands if present.

---

## 5. Python Parsing (`.py`)

For `.py` files:

- Extract:
  - Module docstring.
  - Function and class docstrings.
  - Comments that contain scientific explanations.
- Detect:
  - Figure scripts (e.g., `fig_*.py`).
  - Simulation scripts (e.g., `sim_*.py`).
  - Domain tags via keywords in docstrings and comments.

Classify `.py` files as:

- `figure_script`
- `simulation`
- `utility`
- `spec` (if heavily documented)

---

## 6. Notebook Parsing (`.ipynb`)

For `.ipynb` files:

- Extract:
  - Markdown cells (treated like `.md`).
  - Code cells (for docstrings and comments).
- Detect:
  - Domain tags via keywords.
  - Math in markdown cells.
  - References to figures and simulations.

---

## 7. JSON Parsing (`.json`)

For `extracted_math_v32/summary.json` and similar:

- Extract:
  - Equation IDs.
  - Source file paths (if present).
  - Equation types (inline, display).
  - Any metadata (labels, tags).

This feeds the math validation logic.

---

## 8. TXT and Terminology Parsing (`.txt`, terminology `.md`)

For `.txt` and terminology `.md`:

- Extract:
  - Term → definition pairs.
  - Symbol → meaning pairs.
  - Domain tags.
- Files of interest:
  - `SCIENTIFIC_TERMINOLOGY.md`
  - `SCIENTIFIC_TERMINOLOGIES.md`
  - `PHYSICS.md`
  - HIF files
  - TSL files

These feed the terminology index and classification.

---

## 9. Output Data Model

The parser must produce a list of records like:

```json
{
  "path": "vim/vim_section_lxvi.md",
  "ext": ".md",
  "title": "Section LXVI: Micro-Scale Coherence",
  "type": "section",
  "domain_tags": ["Balance Geometry", "Continuum"],
  "headings": [
    {"level": 1, "text": "Section LXVI: Micro-Scale Coherence"}
  ],
  "math_blocks": [
    {"kind": "display", "source": "$$ ... $$", "label": null}
  ],
  "inline_math": [
    {"source": "$x_t$", "context": "…"}
  ],
  "terminology_refs": ["Balance State Vector", "Equilibrium Manifold"],
  "figure_refs": ["fig_066_066_subsections_lxvi_a_b_c_micro_scale_coherence"],
  "cross_refs": ["Appendix A"],
  "candidate_volume": null,
  "confidence": 0.0
}
