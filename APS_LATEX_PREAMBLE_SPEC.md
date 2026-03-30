---

### 4. `APS_LATEX_PREAMBLE_SPEC.md`

```markdown
# APS Canon Compiler — LaTeX Preamble Specification

## 1. Purpose

This document defines the standard LaTeX preamble for APS Volumes and arXiv-ready outputs.

The preamble must be:

- APS-consistent.
- arXiv-safe.
- Compatible with:
  - math-heavy content
  - figures
  - references
  - multiple volumes.

---

## 2. Document Class

Default:

```latex
\documentclass[11pt]{article}
\usepackage[T1]{fontenc}
\usepackage[utf8]{inputenc}
\usepackage{lmodern}

\usepackage{amsmath}
\usepackage{amssymb}
\usepackage{amsthm}

\usepackage{geometry}
\usepackage{hyperref}
\usepackage{graphicx}
\usepackage{xcolor}
