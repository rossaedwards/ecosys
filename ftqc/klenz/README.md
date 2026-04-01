# 🗃️ klenz/ — Archived Source Material

> **ARCHIVED — Do not use these files for compilation.**
> All canonical citations live in `../master_citations.bib`.

---

## What is this directory?

`klenz/` is the **raw provenance vault** for the FTQC paper submission pipeline. Every file here is an **original downloaded, exported, or auto-generated citation artifact** that predates the Phases 1–3 bibliography consolidation (completed 2026-04-01).

These files were the *source material* from which `master_citations.bib` was built — they are kept here for:
- **Provenance** — traceable record of every citation's origin
- **Recovery** — if a field in `master_citations.bib` ever needs to be re-verified against the original publisher export
- **Audit** — demonstrate to reviewers that all 55 canonical entries were sourced from authoritative databases (APS, IOP, Nature, arXiv, IEEE Xplore, ACS, Optica, Cambridge Core)

---

## ⚠️ Rules

| Rule | Detail |
|---|---|
| **DO NOT** `\cite{}` from any file in this directory | Use `../master_citations.bib` exclusively |
| **DO NOT** edit files here | They are read-only historical artifacts |
| **DO NOT** delete files here | Provenance must be preserved through submission |
| **DO** reference here if you need to re-verify a field | Cross-check against the original export |

---

## Directory Contents

### Raw Citation Exports (by publisher)

| Prefix / Pattern | Source | Format |
|---|---|---|
| `PhysRevA.*.bibtex`, `PhysRevB.*.bibtex`, `PhysRevLett.*.bibtex`, `RevModPhys.*.bibtex` | American Physical Society (APS) | BibTeX |
| `IOPEXPORT_BIB*.bib` | IOP Publishing | BibTeX |
| `10.1038_*.ris`, `10.1007_*.ris` | Nature / Springer | RIS |
| `acsphotonics.*.ris` | American Chemical Society | RIS |
| `csp_*.bib` | CrossRef / Semantic Scholar | BibTeX |
| `S00034916*.bib`, `S03701573*.bib` | Elsevier (ScienceDirect) | BibTeX |
| `0960077995*.bib` | Elsevier (Chaos, Solitons & Fractals) | BibTeX |
| `citation-228695348*.bib` | Semantic Scholar | BibTeX |
| `citations-*.bibtex` | Zotero bulk export | BibTeX |
| `IEEE Xplore Citation*.bib` | IEEE Xplore | BibTeX |
| `CambridgeCore_Citation*.txt` | Cambridge Core | Plain text |
| `10.1515_nanoph*.bib` | De Gruyter (Nanophotonics) | BibTeX |
| `PhysRevA.69.062320.bibtex` | APS | BibTeX |

### Figures (original renders)

`Fig1_Hilbert_Scaling.*`, `Fig2_Coherence_Dynamics.*`, `Fig3_Anderson_Localization.*`,
`Fig3_Fractal_Localization.*`, `Fig4_Sierpinski_Lattice.*`, `Fig5_Band_Structure.*`,
`Fig6_Neglecton_Braiding.*`, `Fig6_ZPE_Majorana_Stability.*`, `Fig7_Device_Cross_Section.*`,
`Fig8_Majorana_T_Junction.*`, `Fig9_Information_Scaling.*`, `Fig10_Technology_Roadmap.*`,
`aurphyx_aurafs_*.png`

### Build & Diagnostic Scripts (superseded)

`build_and_fix.py`, `build_arxiv.ps1`, `build_arxiv_package.py`, `build_arxiv_utf8.ps1`,
`build_ftqc.py`, `compile_citations.py`, `consolidate_bibliography.py`, `definitive_fix.py`,
`diagnose_ftqc.py`, `emergency_fix.py`, `fig1_hilbert_scaling.py` … `fig10_technology_roadmap.py`

### Working Bibliography Files (superseded)

`arxiv_ftqc.bib`, `arxiv_ftqc_complete.bib`, `arxiv_ftqc_completeNotes.bib`,
`arxiv_missing.bib`, `citation-list.md`, `context.json`

### Manuscript / Submission Docs (archival copies)

`arxiv_ftqc_complete.tex`, `arxiv_ftqc_complete.pdf`, `AOF.pdf`,
`DARPA_Executive_Summary_Aurphyx.md.pdf`, `PRX_Abstract_SectionI_SHAREABLE.md.pdf`,
`PRX_Cover_Letter.md.pdf`, `Submission_Guide_Complete.md`, `FTQC_INTEGRATION_GUIDE.md`,
`ORCID-iD_icon_vector.svg`

---

## Canonical Output

The single authoritative bibliography for all three venues is:

```
ftqc/master_citations.bib
```

55 entries • 0 duplicates • 0 stubs • Author+Year keys • Cleaned 2026-04-01

---

*Archived by Hecate Build Pipeline — Phase 5 — 2026-04-01*  
*Ross A. Edwards | Aurphyx LLC | ORCID: 0009-0008-0539-1289*
