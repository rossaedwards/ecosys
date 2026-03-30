# Aurphyx Publication Submission Guide

## Complete Instructions for arXiv, PRX, Optica, HuggingFace Papers, and phys.org

**Author:** Ross A. Edwards  
**Date:** January 31, 2026  
**Document Version:** 1.0

---

## Table of Contents

1. [arXiv Submission](#1-arxiv-submission)
2. [Physical Review X (PRX) Submission](#2-physical-review-x-prx-submission)
3. [Optica Submission](#3-optica-submission)
4. [HuggingFace Papers](#4-huggingface-papers)
5. [phys.org Press Release](#5-physorg-press-release)
6. [Printing and Archival](#6-printing-and-archival)
7. [File Checklist](#7-file-checklist)

---

## 1. arXiv Submission

### 1.1 Registration (First-Time Users)

1. **Create Account**: Go to https://arxiv.org/user/register
2. **Email Verification**: Confirm your institutional email
3. **Endorsement**: For first submission to quant-ph, you need endorsement from an existing arXiv author
   - Request via: https://arxiv.org/auth/endorse
   - Alternative: Co-author with someone who has arXiv posting rights
4. **ORCID Link** (recommended): Connect your ORCID iD for persistent author identification

### 1.2 Compile LaTeX for arXiv

```bash
# Step 1: Create submission directory
mkdir arxiv_submission
cd arxiv_submission

# Step 2: Copy required files
cp aurphyx_arxiv_complete.tex main.tex  # arXiv requires main.tex
cp aurphyx_prx.bib references.bib
cp Fig*.png ./
cp Fig*.pdf ./

# Step 3: Compile locally to generate .bbl file
pdflatex main.tex
bibtex main
pdflatex main.tex
pdflatex main.tex

# Step 4: Verify PDF renders correctly
open main.pdf  # or use your PDF viewer

# Step 5: Create submission package (arXiv prefers tar.gz)
tar -czvf aurphyx_arxiv.tar.gz main.tex main.bbl *.png *.pdf
```

### 1.3 Submit to arXiv

1. **Login**: https://arxiv.org/submit
2. **New Submission**: Click "Start New Submission"
3. **License**: Select "CC BY 4.0" (recommended for maximum reuse)
4. **Category**: Primary: `quant-ph` (Quantum Physics)
   - Cross-list: `cond-mat.mes-hall`, `physics.optics`
5. **Upload**: Upload `aurphyx_arxiv.tar.gz`
6. **Metadata**:
   - **Title**: Fractal-Enhanced Topological Quantum Computing...
   - **Authors**: Ross A. Edwards
   - **Abstract**: (copy from LaTeX)
   - **Comments**: "40 pages, 10 figures, submitted to Physical Review X"
   - **Report-no**: Leave blank (or add institutional report number)
   - **DOI**: Leave blank until journal assigns one
7. **Preview**: Verify PDF rendering
8. **Submit**: Click "Submit Article"

### 1.4 Post-Submission

- **arXiv ID**: Received within 24-48 hours (format: 2601.XXXXX)
- **Announcement**: Paper appears in daily quant-ph listing
- **Updates**: Use "Replace" to upload revised versions
- **DOI**: arXiv assigns DOI like `10.48550/arXiv.2601.XXXXX`

---

## 2. Physical Review X (PRX) Submission

### 2.1 Registration

1. **Create APS Account**: https://authors.aps.org/
2. **Complete Profile**: Add affiliation, ORCID, research interests
3. **Verify Email**: Confirm institutional email

### 2.2 Prepare Submission Package

**Required Files:**
- `main.tex` (LaTeX source)
- `main.bbl` (compiled bibliography)
- `Fig1_Hilbert_Scaling.pdf` through `Fig10_Technology_Roadmap.pdf`
- `PRX_Cover_Letter.pdf` (convert from .md)
- `supplementary_information.pdf` (if applicable)

**PRX Formatting Requirements:**
- Use `revtex4-2` document class
- Two-column format
- Maximum 12 pages (main text)
- Figures embedded or separate (PDF preferred)
- References: APS style (handled by RevTeX)

### 2.3 Submit to PRX

1. **Login**: https://authors.aps.org/
2. **Start Submission**: Select "Physical Review X"
3. **Upload Manuscript**: Upload LaTeX source + figures
4. **Upload Cover Letter**: Attach as separate PDF
5. **Metadata**:
   - **Title**: (as in manuscript)
   - **Authors**: Add all authors with affiliations
   - **Abstract**: (as in manuscript)
   - **PACS**: 03.67.Lx, 03.67.Pp, 42.70.Qs
   - **Keywords**: quantum computing, fractal geometry, topological...
6. **Suggested Reviewers**: (from cover letter)
7. **Excluded Reviewers**: List any conflicts
8. **Submit**: Confirm and submit

### 2.4 PRX Review Process

- **Editorial Assessment**: 1-2 weeks
- **Peer Review**: 4-8 weeks (2-3 reviewers)
- **Decision Types**:
  - Accept
  - Minor Revisions
  - Major Revisions
  - Reject (with option to resubmit)
- **Publication Fee**: ~$3,500 (open access)

### 2.5 Post-Acceptance

- **Proofs**: Review within 48 hours
- **DOI Assignment**: `10.1103/PhysRevX.XX.XXXXXX`
- **Publication**: Online within 1-2 weeks of proof approval

---

## 3. Optica Submission

### 3.1 Registration

1. **Create Optica Account**: https://www.opticasubmit.org/
2. **ORCID Integration**: Link ORCID for author identification

### 3.2 Optica-Specific Formatting

**Journal Options:**
- **Optica** (flagship, high-impact): 10 pages, 6 figures max
- **Optics Letters** (rapid communication): 4 pages
- **Optics Express** (comprehensive): No length limit

**For "Photonic Band Engineering in Hexagonal Resonant Lattices...":**
→ Target: **Optica** (flagship journal)

**Formatting:**
- Use Optica LaTeX template: https://www.optica.org/author-info/
- Single-column format
- Figures: EPS or PDF (vector preferred)
- References: Optica style (numeric)

### 3.3 Submit to Optica

1. **Login**: https://www.opticasubmit.org/
2. **Select Journal**: Optica
3. **Upload**: Manuscript + figures + cover letter
4. **Metadata**: Title, authors, abstract, keywords
5. **Suggested Reviewers**: 3-5 experts in photonic crystals
6. **Submit**: Confirm and submit

### 3.4 Optica Review Process

- **Initial Screening**: 1 week
- **Peer Review**: 3-6 weeks
- **Publication Fee**: ~$2,500 (open access)

---

## 4. HuggingFace Papers

### 4.1 Overview

HuggingFace Papers (https://huggingface.co/papers) aggregates ML/AI papers from arXiv. It's **not** a submission platform—papers appear automatically after arXiv posting.

### 4.2 How to Get Listed

1. **Post to arXiv first** (required)
2. **Use relevant keywords** in title/abstract:
   - "quantum computing", "machine learning", "neural network"
   - "optimization", "simulation"
3. **Add code repository** (increases visibility):
   - Link GitHub repo in arXiv "Code" field
   - Or add to HuggingFace Hub: https://huggingface.co/models

### 4.3 Manual Submission

If paper doesn't appear automatically:
1. Go to https://huggingface.co/papers
2. Click "Submit a paper"
3. Enter arXiv ID: `2601.XXXXX`
4. Paper is added to the daily feed

### 4.4 Increase Visibility

- **HuggingFace Space**: Create interactive demo
- **Model Card**: If you have trained models
- **Discussion**: Engage with community on paper page

---

## 5. phys.org Press Release

### 5.1 Overview

phys.org is a science news aggregator. They publish:
- Institutional press releases
- Summaries of notable papers
- Interviews with researchers

### 5.2 Getting Coverage

**Option A: Institutional Press Release**
1. Contact your institution's media relations office
2. Draft press release (see template below)
3. Institution submits to phys.org via their portal

**Option B: Direct Submission**
1. Email: editors@phys.org
2. Subject: "Research Highlight: [Your Title]"
3. Include:
   - arXiv link
   - 300-word plain-language summary
   - High-resolution figure
   - Contact information

**Option C: Science Journalists**
1. Post on EurekAlert! (https://www.eurekalert.org/)
2. Tweet paper with #QuantumComputing hashtag
3. Contact science writers directly (Nature News, Science Magazine)

### 5.3 Press Release Template

```
FOR IMMEDIATE RELEASE

[HEADLINE]: Fractal Geometry Could Slash Quantum Computing 
Resource Requirements by 16×

[SUBHEAD]: New architecture combines fractal lattices, 
topological physics, and photonic engineering

[CITY, DATE] — A new quantum computing architecture called 
Aurphyx could reduce the number of qubits needed for 
fault-tolerant computation by a factor of 16, according to 
research published on arXiv.

[Body: 3-4 paragraphs explaining the work in plain language]

[Quote from researcher]

[Link to paper]

[Contact information]

###
```

---

## 6. Printing and Archival

### 6.1 Print-Ready PDF Generation

```bash
# High-quality print PDF (300 DPI, CMYK)
pdflatex -interaction=nonstopmode main.tex
gs -dNOPAUSE -dBATCH -sDEVICE=pdfwrite \
   -dCompatibilityLevel=1.4 \
   -dPDFSETTINGS=/prepress \
   -sOutputFile=aurphyx_print.pdf main.pdf
```

### 6.2 Professional Printing Options

**Option A: Thesis Binding Services**
- ProQuest Dissertation Services
- Local university copy center
- Specify: Perfect binding, 80# gloss cover, 60# text paper

**Option B: On-Demand Printing**
- Lulu.com (create PDF book)
- Blurb.com (photo book quality)
- Amazon KDP (if distributing publicly)

**Option C: Local Print Shop**
- FedEx Office / Kinko's
- Staples Copy & Print
- Request: Saddle-stitch or perfect binding

### 6.3 Archival Copies

**Digital Archives:**
- Zenodo (https://zenodo.org/): DOI-assigned archive
- figshare (https://figshare.com/): Data + code archival
- GitHub Releases: Tag version with paper submission

**Institutional Archives:**
- Submit to your university's digital repository
- Request LOCKSS/CLOCKSS preservation

---

## 7. File Checklist

### arXiv Submission Package
```
□ main.tex (renamed from aurphyx_arxiv_complete.tex)
□ main.bbl (compiled bibliography)
□ Fig1_Hilbert_Scaling.png
□ Fig2_Coherence_Dynamics.png
□ Fig3_Anderson_Localization.png
□ Fig4_Sierpinski_Lattice.png
□ Fig5_Band_Structure.png
□ Fig6_Neglecton_Braiding.png
□ Fig7_Device_Cross_Section.png
□ Fig8_Majorana_T_Junction.png
□ Fig9_Information_Scaling.png
□ Fig10_Technology_Roadmap.png
```

### PRX Submission Package
```
□ main.tex
□ main.bbl
□ All figures (PDF preferred)
□ PRX_Cover_Letter.pdf
□ supplementary_information.pdf (optional)
□ Author agreement form
```

### Optica Submission Package
```
□ optica_submission.tex (Optica template)
□ optica.bib
□ All figures (EPS or PDF)
□ cover_letter_optica.pdf
```

### Code/Data Repository
```
□ fig1_hilbert_scaling.py
□ fig2_coherence_dynamics.py
□ fig3_anderson_localization.py
□ fig4_sierpinski_lattice.py
□ fig5_band_structure.py
□ fig6_neglecton_braiding.py
□ fig7_device_cross_section.py
□ fig8_majorana_t_junction.py
□ fig9_information_scaling.py
□ fig10_technology_roadmap.py
□ requirements.txt (numpy, matplotlib, scipy)
□ README.md
□ LICENSE (MIT or Apache 2.0)
```

---

## Quick Reference: Submission URLs

| Platform | URL | Timeline |
|----------|-----|----------|
| arXiv | https://arxiv.org/submit | 24-48 hrs |
| PRX | https://authors.aps.org/ | 2-4 months |
| Optica | https://www.opticasubmit.org/ | 2-3 months |
| HuggingFace | https://huggingface.co/papers | Auto (after arXiv) |
| phys.org | editors@phys.org | 1-2 weeks |
| Zenodo | https://zenodo.org/ | Instant DOI |

---

## Recommended Submission Order

1. **Day 1**: Submit to arXiv (establishes priority)
2. **Day 2**: Submit to PRX (with arXiv preprint link)
3. **Day 2**: Submit to Optica (separate photonics manuscript)
4. **Week 1**: Paper appears on HuggingFace Papers
5. **Week 1**: Send press release to phys.org
6. **Week 2**: Archive code on Zenodo with DOI
7. **Ongoing**: Update arXiv versions as revisions progress

---

*Document prepared by Dr. Elena Voss, Publication Advisor*
*Last updated: January 31, 2026*
