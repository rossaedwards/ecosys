### 2. `APS_CLASSIFICATION_HEURISTICS.md`

```markdown
# APS Canon Compiler — Classification Heuristics

## 1. Purpose

This document defines how the APS Canon Compiler assigns files to APS Volumes using:

- Parsed content (from the parser).
- `aps_volume_map.json` (hybrid map: auto + manual).
- Terminology and domain tags.
- Math and structural signatures.

The goal is to produce a stable mapping:

- file → APS Volume
- with a confidence score
- and clear override rules.

---

## 2. Inputs

The classification engine consumes:

- Parsed records from the parser (see `APS_PARSER_DESIGN.md`).
- `aps_volume_map.json`:
  - `auto_keywords`
  - `auto_patterns`
  - `auto_folders`
  - `manual_files`
  - `manual_notes`
  - `cursor_guide`
- Terminology index (from terminology parsing).
- Math signatures (e.g., heavy FTQC math vs Balance Geometry math).

---

## 3. Core Principles

1. **Manual overrides always win.**  
   If a file is listed in `manual_files` for a volume, that assignment is authoritative.

2. **Auto classification is suggestive, not final.**  
   Auto fields (`auto_keywords`, `auto_patterns`, `auto_folders`) propose candidate volumes.

3. **Confidence scoring is required.**  
   Each file must have:
   - `candidate_volume`
   - `confidence` in `[0.0, 1.0]`

4. **Multi-domain files are allowed.**  
   A file may contribute to multiple volumes, but must have a primary volume.

---

## 4. Heuristic Signals

For each file, compute signals:

- **Keyword match score**:
  - Count matches of volume-specific keywords.
- **Terminology match score**:
  - Count matches of terms associated with a volume.
- **Folder origin score**:
  - If file is under `ftqc/`, boost FTQC volume.
  - If under `tvfd/`, boost Vacuum Field volume.
- **Math signature score**:
  - FTQC-like math (quantum, entanglement, topological) boosts FTQC.
  - Geometry-heavy math boosts Balance Geometry.
- **Cross-reference score**:
  - References to known sections/appendices of a volume boost that volume.

Combine these into a `confidence` score.

---

## 5. Volume Assignment Algorithm (High-Level)

For each file:

1. Initialize an empty score map: `volume_scores = {}`.
2. For each APS Volume in `aps_volume_map.json`:
   - Apply:
     - keyword matches
     - pattern matches
     - folder matches
   - Add to `volume_scores[volume]`.
3. Apply terminology-based boosts.
4. Apply math-signature-based boosts.
5. Apply cross-reference-based boosts.
6. If file appears in `manual_files` for any volume:
   - Set `candidate_volume` to that volume.
   - Set `confidence = 1.0`.
7. Else:
   - Choose the volume with the highest score.
   - Set `candidate_volume` and `confidence` accordingly.
8. If all scores are low:
   - Set `candidate_volume = null`.
   - Set `confidence` low (e.g., `0.1`).
   - Mark for manual review.

---

## 6. Special Cases

### 6.1 FTQC (APS Volume XVII)

- Any file under `ftqc/` gets a strong FTQC prior.
- Any file with heavy quantum/topological terminology gets FTQC boosts.
- FTQC is priority for arXiv; classification should err on inclusion.

### 6.2 TVFD (Vacuum Field)

- Any file under `tvfd/` gets a strong Vacuum Field prior.
- Thermodynamic vacuum language boosts this volume.

### 6.3 TSLCA

- Files under `tslca/` get TSLCA prior.
- Lattice, activation, three-squared logic boost TSLCA.

### 6.4 SAGES and Standards

- Files under `sages/` and `standards/` get governance/standards priors.
- Ethical, governance, standardization language boosts these volumes.

---

## 7. Outputs

For each file, classification must produce:

```json
{
  "path": "ftqc/rae-ftqc_arxiv_FINAL.tex",
  "candidate_volume": "APS_Volume_XVII_FTQC",
  "confidence": 0.98,
  "signals": {
    "keywords": 0.5,
    "terminology": 0.2,
    "folder": 0.2,
    "math_signature": 0.08
  },
  "manual_override": false
}
