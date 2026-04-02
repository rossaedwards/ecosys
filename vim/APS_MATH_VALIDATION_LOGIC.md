---

### 3. `APS_MATH_VALIDATION_LOGIC.md`

```markdown
# APS Canon Compiler — Math Validation Logic

## 1. Purpose

This document defines how the APS Canon Compiler validates mathematical content across:

- Source `.md` and `.tex` files.
- Extracted math in `extracted_math_v32/`.
- Generated `.tex` files after MD→TeX conversion.

The goal is to ensure:

- No equations are lost.
- No math is malformed.
- No TeX is corrupted.
- Extracted math is consistent with source.

---

## 2. Inputs

The math validation engine consumes:

- Parsed math from source `.md` and `.tex` (see `APS_PARSER_DESIGN.md`).
- Extracted math from:
  - `extracted_math_v32/CODEX_*.md`
  - `extracted_math_v32/summary.json`
- Generated `.tex` files in `latex_output/APS_Volumes/`.

---

## 3. Validation Phases

### 3.1 Source vs Extracted Math

For each source file with math:

1. Collect all math blocks and inline math from the parser.
2. Identify corresponding entries in `extracted_math_v32` (if any).
3. Compare:
   - Count of equations.
   - Structural similarity (e.g., same LaTeX up to whitespace).
4. Flag:
   - Missing equations in extraction.
   - Extra equations in extraction.
   - Structural mismatches.

Write a summary to `APS_MATH_VALIDATION_REPORT.md` and/or JSON.

---

### 3.2 Source vs Generated `.tex`

After MD→TeX conversion:

1. For each `.md` file converted to `.tex`:
   - Parse math from the generated `.tex`.
   - Compare with math parsed from the original `.md`.
2. Check:
   - Equation counts.
   - Inline vs display consistency.
   - Presence of labels.
3. Flag:
   - Lost equations.
   - Broken math environments.
   - Misconverted inline/display math.

---

### 3.3 Extracted vs Generated `.tex`

For files with entries in `extracted_math_v32`:

1. Compare extracted equations with equations in the final `.tex`.
2. Ensure:
   - All extracted equations appear in the final `.tex`.
   - No major structural corruption.

---

## 4. Error Levels

Define severity levels:

- **INFO** — minor formatting differences.
- **WARN** — equation present but structurally altered.
- **ERROR** — equation missing or math environment broken.

The compiler must:

- Log all issues to `APS_MATH_VALIDATION_REPORT.md`.
- Summarize counts of INFO/WARN/ERROR.
- Optionally block final build on severe ERRORs (configurable).

---

## 5. Reporting

Generate:

- `APS_MATH_VALIDATION_REPORT.md`:
  - Summary of validation.
  - Per-file issues.
  - Counts of INFO/WARN/ERROR.
- Optionally:
  - `APS_MATH_VALIDATION_REPORT.json` for machine use.

Example entry:

```json
{
  "path": "vim/vim_section_lxvi.md",
  "issues": [
    {
      "level": "WARN",
      "type": "structural_mismatch",
      "message": "Equation 3 differs between source and extracted math."
    },
    {
      "level": "ERROR",
      "type": "missing_equation",
      "message": "Equation 5 present in source but missing in generated .tex."
    }
  ]
}---

### 3. `APS_MATH_VALIDATION_LOGIC.md`

```markdown
# APS Canon Compiler — Math Validation Logic

## 1. Purpose

This document defines how the APS Canon Compiler validates mathematical content across:

- Source `.md` and `.tex` files.
- Extracted math in `extracted_math_v32/`.
- Generated `.tex` files after MD→TeX conversion.

The goal is to ensure:

- No equations are lost.
- No math is malformed.
- No TeX is corrupted.
- Extracted math is consistent with source.

---

## 2. Inputs

The math validation engine consumes:

- Parsed math from source `.md` and `.tex` (see `APS_PARSER_DESIGN.md`).
- Extracted math from:
  - `extracted_math_v32/CODEX_*.md`
  - `extracted_math_v32/summary.json`
- Generated `.tex` files in `latex_output/APS_Volumes/`.

---

## 3. Validation Phases

### 3.1 Source vs Extracted Math

For each source file with math:

1. Collect all math blocks and inline math from the parser.
2. Identify corresponding entries in `extracted_math_v32` (if any).
3. Compare:
   - Count of equations.
   - Structural similarity (e.g., same LaTeX up to whitespace).
4. Flag:
   - Missing equations in extraction.
   - Extra equations in extraction.
   - Structural mismatches.

Write a summary to `APS_MATH_VALIDATION_REPORT.md` and/or JSON.

---

### 3.2 Source vs Generated `.tex`

After MD→TeX conversion:

1. For each `.md` file converted to `.tex`:
   - Parse math from the generated `.tex`.
   - Compare with math parsed from the original `.md`.
2. Check:
   - Equation counts.
   - Inline vs display consistency.
   - Presence of labels.
3. Flag:
   - Lost equations.
   - Broken math environments.
   - Misconverted inline/display math.

---

### 3.3 Extracted vs Generated `.tex`

For files with entries in `extracted_math_v32`:

1. Compare extracted equations with equations in the final `.tex`.
2. Ensure:
   - All extracted equations appear in the final `.tex`.
   - No major structural corruption.

---

## 4. Error Levels

Define severity levels:

- **INFO** — minor formatting differences.
- **WARN** — equation present but structurally altered.
- **ERROR** — equation missing or math environment broken.

The compiler must:

- Log all issues to `APS_MATH_VALIDATION_REPORT.md`.
- Summarize counts of INFO/WARN/ERROR.
- Optionally block final build on severe ERRORs (configurable).

---

## 5. Reporting

Generate:

- `APS_MATH_VALIDATION_REPORT.md`:
  - Summary of validation.
  - Per-file issues.
  - Counts of INFO/WARN/ERROR.
- Optionally:
  - `APS_MATH_VALIDATION_REPORT.json` for machine use.

Example entry:

```json
{
  "path": "vim/vim_section_lxvi.md",
  "issues": [
    {
      "level": "WARN",
      "type": "structural_mismatch",
      "message": "Equation 3 differs between source and extracted math."
    },
    {
      "level": "ERROR",
      "type": "missing_equation",
      "message": "Equation 5 present in source but missing in generated .tex."
    }
  ]
}---

### 3. `APS_MATH_VALIDATION_LOGIC.md`

```markdown
# APS Canon Compiler — Math Validation Logic

## 1. Purpose

This document defines how the APS Canon Compiler validates mathematical content across:

- Source `.md` and `.tex` files.
- Extracted math in `extracted_math_v32/`.
- Generated `.tex` files after MD→TeX conversion.

The goal is to ensure:

- No equations are lost.
- No math is malformed.
- No TeX is corrupted.
- Extracted math is consistent with source.

---

## 2. Inputs

The math validation engine consumes:

- Parsed math from source `.md` and `.tex` (see `APS_PARSER_DESIGN.md`).
- Extracted math from:
  - `extracted_math_v32/CODEX_*.md`
  - `extracted_math_v32/summary.json`
- Generated `.tex` files in `latex_output/APS_Volumes/`.

---

## 3. Validation Phases

### 3.1 Source vs Extracted Math

For each source file with math:

1. Collect all math blocks and inline math from the parser.
2. Identify corresponding entries in `extracted_math_v32` (if any).
3. Compare:
   - Count of equations.
   - Structural similarity (e.g., same LaTeX up to whitespace).
4. Flag:
   - Missing equations in extraction.
   - Extra equations in extraction.
   - Structural mismatches.

Write a summary to `APS_MATH_VALIDATION_REPORT.md` and/or JSON.

---

### 3.2 Source vs Generated `.tex`

After MD→TeX conversion:

1. For each `.md` file converted to `.tex`:
   - Parse math from the generated `.tex`.
   - Compare with math parsed from the original `.md`.
2. Check:
   - Equation counts.
   - Inline vs display consistency.
   - Presence of labels.
3. Flag:
   - Lost equations.
   - Broken math environments.
   - Misconverted inline/display math.

---

### 3.3 Extracted vs Generated `.tex`

For files with entries in `extracted_math_v32`:

1. Compare extracted equations with equations in the final `.tex`.
2. Ensure:
   - All extracted equations appear in the final `.tex`.
   - No major structural corruption.

---

## 4. Error Levels

Define severity levels:

- **INFO** — minor formatting differences.
- **WARN** — equation present but structurally altered.
- **ERROR** — equation missing or math environment broken.

The compiler must:

- Log all issues to `APS_MATH_VALIDATION_REPORT.md`.
- Summarize counts of INFO/WARN/ERROR.
- Optionally block final build on severe ERRORs (configurable).

---

## 5. Reporting

Generate:

- `APS_MATH_VALIDATION_REPORT.md`:
  - Summary of validation.
  - Per-file issues.
  - Counts of INFO/WARN/ERROR.
- Optionally:
  - `APS_MATH_VALIDATION_REPORT.json` for machine use.

Example entry:

```json
{
  "path": "vim/vim_section_lxvi.md",
  "issues": [
    {
      "level": "WARN",
      "type": "structural_mismatch",
      "message": "Equation 3 differs between source and extracted math."
    },
    {
      "level": "ERROR",
      "type": "missing_equation",
      "message": "Equation 5 present in source but missing in generated .tex."
    }
  ]
}---

### 3. `APS_MATH_VALIDATION_LOGIC.md`

```markdown
# APS Canon Compiler — Math Validation Logic

## 1. Purpose

This document defines how the APS Canon Compiler validates mathematical content across:

- Source `.md` and `.tex` files.
- Extracted math in `extracted_math_v32/`.
- Generated `.tex` files after MD→TeX conversion.

The goal is to ensure:

- No equations are lost.
- No math is malformed.
- No TeX is corrupted.
- Extracted math is consistent with source.

---

## 2. Inputs

The math validation engine consumes:

- Parsed math from source `.md` and `.tex` (see `APS_PARSER_DESIGN.md`).
- Extracted math from:
  - `extracted_math_v32/CODEX_*.md`
  - `extracted_math_v32/summary.json`
- Generated `.tex` files in `latex_output/APS_Volumes/`.

---

## 3. Validation Phases

### 3.1 Source vs Extracted Math

For each source file with math:

1. Collect all math blocks and inline math from the parser.
2. Identify corresponding entries in `extracted_math_v32` (if any).
3. Compare:
   - Count of equations.
   - Structural similarity (e.g., same LaTeX up to whitespace).
4. Flag:
   - Missing equations in extraction.
   - Extra equations in extraction.
   - Structural mismatches.

Write a summary to `APS_MATH_VALIDATION_REPORT.md` and/or JSON.

---

### 3.2 Source vs Generated `.tex`

After MD→TeX conversion:

1. For each `.md` file converted to `.tex`:
   - Parse math from the generated `.tex`.
   - Compare with math parsed from the original `.md`.
2. Check:
   - Equation counts.
   - Inline vs display consistency.
   - Presence of labels.
3. Flag:
   - Lost equations.
   - Broken math environments.
   - Misconverted inline/display math.

---

### 3.3 Extracted vs Generated `.tex`

For files with entries in `extracted_math_v32`:

1. Compare extracted equations with equations in the final `.tex`.
2. Ensure:
   - All extracted equations appear in the final `.tex`.
   - No major structural corruption.

---

## 4. Error Levels

Define severity levels:

- **INFO** — minor formatting differences.
- **WARN** — equation present but structurally altered.
- **ERROR** — equation missing or math environment broken.

The compiler must:

- Log all issues to `APS_MATH_VALIDATION_REPORT.md`.
- Summarize counts of INFO/WARN/ERROR.
- Optionally block final build on severe ERRORs (configurable).

---

## 5. Reporting

Generate:

- `APS_MATH_VALIDATION_REPORT.md`:
  - Summary of validation.
  - Per-file issues.
  - Counts of INFO/WARN/ERROR.
- Optionally:
  - `APS_MATH_VALIDATION_REPORT.json` for machine use.

Example entry:

```json
{
  "path": "vim/vim_section_lxvi.md",
  "issues": [
    {
      "level": "WARN",
      "type": "structural_mismatch",
      "message": "Equation 3 differs between source and extracted math."
    },
    {
      "level": "ERROR",
      "type": "missing_equation",
      "message": "Equation 5 present in source but missing in generated .tex."
    }
  ]
}---

### 3. `APS_MATH_VALIDATION_LOGIC.md`

```markdown
# APS Canon Compiler — Math Validation Logic

## 1. Purpose

This document defines how the APS Canon Compiler validates mathematical content across:

- Source `.md` and `.tex` files.
- Extracted math in `extracted_math_v32/`.
- Generated `.tex` files after MD→TeX conversion.

The goal is to ensure:

- No equations are lost.
- No math is malformed.
- No TeX is corrupted.
- Extracted math is consistent with source.

---

## 2. Inputs

The math validation engine consumes:

- Parsed math from source `.md` and `.tex` (see `APS_PARSER_DESIGN.md`).
- Extracted math from:
  - `extracted_math_v32/CODEX_*.md`
  - `extracted_math_v32/summary.json`
- Generated `.tex` files in `latex_output/APS_Volumes/`.

---

## 3. Validation Phases

### 3.1 Source vs Extracted Math

For each source file with math:

1. Collect all math blocks and inline math from the parser.
2. Identify corresponding entries in `extracted_math_v32` (if any).
3. Compare:
   - Count of equations.
   - Structural similarity (e.g., same LaTeX up to whitespace).
4. Flag:
   - Missing equations in extraction.
   - Extra equations in extraction.
   - Structural mismatches.

Write a summary to `APS_MATH_VALIDATION_REPORT.md` and/or JSON.

---

### 3.2 Source vs Generated `.tex`

After MD→TeX conversion:

1. For each `.md` file converted to `.tex`:
   - Parse math from the generated `.tex`.
   - Compare with math parsed from the original `.md`.
2. Check:
   - Equation counts.
   - Inline vs display consistency.
   - Presence of labels.
3. Flag:
   - Lost equations.
   - Broken math environments.
   - Misconverted inline/display math.

---

### 3.3 Extracted vs Generated `.tex`

For files with entries in `extracted_math_v32`:

1. Compare extracted equations with equations in the final `.tex`.
2. Ensure:
   - All extracted equations appear in the final `.tex`.
   - No major structural corruption.

---

## 4. Error Levels

Define severity levels:

- **INFO** — minor formatting differences.
- **WARN** — equation present but structurally altered.
- **ERROR** — equation missing or math environment broken.

The compiler must:

- Log all issues to `APS_MATH_VALIDATION_REPORT.md`.
- Summarize counts of INFO/WARN/ERROR.
- Optionally block final build on severe ERRORs (configurable).

---

## 5. Reporting

Generate:

- `APS_MATH_VALIDATION_REPORT.md`:
  - Summary of validation.
  - Per-file issues.
  - Counts of INFO/WARN/ERROR.
- Optionally:
  - `APS_MATH_VALIDATION_REPORT.json` for machine use.

Example entry:

```json
{
  "path": "vim/vim_section_lxvi.md",
  "issues": [
    {
      "level": "WARN",
      "type": "structural_mismatch",
      "message": "Equation 3 differs between source and extracted math."
    },
    {
      "level": "ERROR",
      "type": "missing_equation",
      "message": "Equation 5 present in source but missing in generated .tex."
    }
  ]
}
