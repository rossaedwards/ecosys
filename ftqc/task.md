# FTQC Manuscript Execution Tasks

## Phase 1: Scan, Read, and Understand the Source Material
- [ ] Scan, read, and understand the FTQC repository. You have my permission to read all files, including the root directory and all subdirectories.
- [ ] I want my add my ORCiD to all headers, author, footers, and wherever it is needed. My ORCiD is: 0009-0008-0539-1289, you can find my information in 'rae_orcid.txt' and 'rae_aurphyx_mailing_and_orcid.txt'.
- [ ] There are three distinct parts to the source FTQC repository:
    - [ ]  The first part is the arxiv.org manuscript & submission package
    - [ ]  The second part is the PRX submission package
    - [ ]  The third part is the Optica submission
- [ ] Previously, you were working on Phase 2 and Phase 3 below, but I did not get to plugin my laptop before the battery died. So we are starting fresh.
- [ ] The context.json file was made during the FTQC Deep Scan.
- [ ] The 'master_citations.bib' has over 200+ citations. The individual submission packages can have their own .bib files, but this is my first time submitting to these journals, so I am not sure how to handle the citations. 
- [ ] There are many python scripts in the FTQC directory that I created last month to help with the submission process. The FTQC directory should be flat, no subfolders. Some of the scripts are:'fix_citations.py', 'compile_citations.py', 'ftqc_divine_pipeline.py', 'diagnose_ftqc.py', 'hecate_master_build.py', etc.
- [ ] The 'hecate_master_build.py' script is the main script that will build all three manuscripts. It will build the arXiv manuscript first, then the PRX manuscript, then the Optica manuscript.

## Phase 2: Optica LaTeX Conversion
- [ ] Fix preamble (remove optica.sty, add authblk, fix bst)
- [ ] Convert abstract from optica_submission.md
- [ ] Convert Section 1: Introduction
- [ ] Convert Section 2: C6v Lattice Design
- [ ] Convert Section 3: Band Structure + Table 1
- [ ] Convert Section 4: Localization + Decoherence
- [ ] Convert Section 5: Fabrication + Table 2
- [ ] Convert Section 6: Discussion + Table 3
- [ ] Convert Section 7: Conclusion + Funding/Disclosures
- [ ] Map references to optica_ftqc.bib keys
- [ ] Fix corrupt opticajnl.bst (was HTML, replaced from latex-supplemental-document/)
- [ ] Add \JournalTitle and \bibfield compatibility commands

## Phase 3: PRX Content Population
- [ ] Fix preamble (\\order → \\OrderOp, \\tr removal, figure paths)
- [ ] Convert abstract from prx_abstract_section_i.md
- [ ] Convert Section I introduction (subsections A-F)
- [ ] Compress Section II from section_ii_fractal_hilbert_scaling.tex
- [ ] Compress Section III from section_iii_non_semisimple_tqft.tex
- [ ] Compress Section IV from section_iv_photonic_band_engineering.tex
- [ ] Compress Section V from section_v_experimental_validation.tex
- [ ] Compress Section VI from section_vi_discussion.tex
- [ ] Compress Section VII from section_vii_conclusion.tex
- [ ] Replace \\cite{SM} with footnotes (no Supplementary Material bib entry)
- [ ] Add Joannopoulos2008 to prx_ftqc.bib

## Phase 4: 
- [ ] Review all three manuscripts and suggest any changes for each manuscript based on the target journal's requirements.
- [ ] Build a plan to implement the changes you suggest and ask for my approval to proceed from here.
- [ ] Execute the Implementation Plan.
- [ ] Review everything done so far with me, then we will move on to Phase 5.

## Phase 5: Build Verification
- [ ] Review the 'hecate_master_build.py' script and make sure it is correct.
- [ ] Review the 'master_citations.bib' file and make sure it is correct.
- [ ] Check and review all sections of each submission manuscript for LaTeX formatting, citations, figures, tables, etc.
- [ ] Check and review each manuscript for target journal requirements.
- [ ] Build a plan to fix and implement the changes or fixes you suggest and ask for my approval to proceed from here.
- [ ] Execute the Implementation or Fix Plan.
- [ ] Run hecate_master_build.py
- [ ] Confirm 3/3 targets compile
- [ ] PRX:prx_ftqc.tex
- [ ] Optica:optica_ftqc.tex
- [ ] arXiv:arxiv_ftqc.tex

## Phase 6: Submission Preparation
- [ ] Review arXiv:arxiv_ftqc.tex
- [ ] Review Optica:optica_ftqc.tex
- [ ] Review PRX:prx_ftqc.tex
- [ ] Plan a build, compile, and submission process for arXiv.org first.
- [ ] Plan a build, compile, and submission process for Optica second.
- [ ] Plan a build, compile, and submission process for PRX third.
- [ ] Review the build, compile, and submission process for arXiv.org first.
- [ ] Review the build, compile, and submission process for Optica second.
- [ ] Review the build, compile, and submission process for PRX third.
