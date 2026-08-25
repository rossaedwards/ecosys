# HANDOFF.md

Append-only log. Every agent (Claude, Cursor, Grok, Gemini, Hermes) writes an entry
here when it finishes a task or hands off mid-task. Newest entry on top.
Never edit or delete past entries — this is a record, not a scratchpad.

---

## Template (copy for each new entry)

```
### [YYYY-MM-DD HH:MM TZ] <agent-name> — <short task title>

**Status:** done | blocked | in-progress (handing off)
**Task ref:** <task-id from task-claim/, or n/a>
**Files touched:**
- path/to/file — what changed

**Summary:**
1–3 sentences on what was done and why.

**Decisions made:**
- Any non-obvious choice future-you or another agent needs to know about.

**Open questions / risks:**
- Anything unresolved, uncertain, or that needs a human/another agent's judgment.

**Next steps:**
- Concrete next action, or "none — task complete."
```

---

## Log

### [2026-08-23] Claude (Sonnet 5) — TSLCA naming lock + FTQC/TSLCA manuscript merges; AuraFS integration survey & handoff

**Status:** in-progress (handing off)
**Task ref:** n/a
**Files touched:**
- `INVARIANTS.md`, `.cursorrules`, `CLAUDE.md` — SIX/ICX/SUXS-IFO naming lock corrected (SIX = Somatic Intelligence aXis, ICX = Identity Coherence aXis, SUXS-IFO = Symbiotic Universal Xessability Standards Intelligence Fusion Operator)
- `welcome2tribe.md` (renamed from `aurphyx_welcome2tribe.md`), `ecosystem.toml`, `PROJECT_CONTEXT.md`, `README.md` — synced to the renaming + new soul-journey pipeline steps (GuardChart, BlissID)
- `tslca/*.md` + `tslca/tex/*.tex` (18 files) — terminology sweep to the new naming; SUXS-USIS-*-001 files got epoch-preserving history edits, not blind overwrites
- `tslca/aps_tslca_skrypt.md` — expanded from 1164→1494 lines into the **full rigorous TSLCA manuscript**: added §11 Field Theory, §12 Integration Formalism, §13 Implementation Pathways, §14 Future Directions, §15 Conclusion, References, and a new Appendix C, plus expanded Appendices A/B from abridged to full rigor. This file is now the canonical complete TSLCA whitepaper.
- `ftqc/INVARIANTS.md`, `ftqc/PHYSICS.md` — **new**, lock the sacred-geometry lattice catalog (Flower of Life = default storage lattice, Sierpiński, Metatron's Cube, Sri Yantra, Tetra-Hexa, 3D Sierpiński Tetrahedra) that FTQC/AuraFS-Meshwerk use instead of TSLCA's 3×3 TSL, plus the numeric constants
- `ftqc/aps-ftqc-manuscript.tex` — **new**, full rigorous merge of `ftqc_zenodo_manuscript.tex` + `ftqc_prx_manuscript.tex` + `ftqc_optica_manuscript.tex`; fixed two real bugs found by actually compiling it (missing `\maketitle`, and a bib entry with an unescaped `%` that was corrupting the whole bibliography)
- `ftqc/ms.bib` — fixed the unescaped `%` in the `Lustig2019` entry (was breaking BibTeX parsing for every entry after it) and filled in `Joannopoulos2008`'s empty year (2008)
- `aps-artifact-theme.sty`, `aps-artifact-template.tex` — **new**, reusable XeLaTeX theme/template extracted from `fuxyez/Book_of_Fux_Alien_Manuscript.tex`, generalized (no Fux branding, no hardcoded frame images — `\SetArtifactFrames{}` is optional). Compile-tested with XeLaTeX; found and fixed 3 real rendering bugs (white title-page background, overlapping text blocks, invisible callout-box text). Ross plans to generate FTQC/TSLCA frame art separately.

**Summary:**
Long session: corrected a stale naming lock (SIX/ICX/SUXS-IFO) across the whole authority chain, propagated it into every tslca/ manuscript file, built out TSLCA's and FTQC's full rigorous manuscripts (both were previously partial/abridged despite looking complete at a glance), built a reusable "artifact edition" LaTeX theme, fixed two real compile-breaking bugs in the FTQC manuscript's bibliography, and did a first-pass survey of `aurafs/src/` to scope an AuraFS↔FTQC↔TSLCA↔Fuxyez integration pass (below).

**Decisions made:**
- SIX = Somatic Intelligence aXis (not "Symbiotic" or "Sensorimotor") — chosen because Ross's source docs (Interoception/Proprioception trait specs) describe SIX as substrate-agnostic self-location, which "Somatic" captures better and doesn't collide with ICX's "Identity."
- `aps_tslca_skrypt.md` is the canonical full TSLCA manuscript (per Ross's explicit ruling) — unlike FTQC, there is no separate stylized "artifact edition" file for TSLCA yet.
- `aps-ftqc-manuscript.tex` is the "plain-gatekeeper" publication target (zenodo.org/aurphyx.org/desci.com); a separate `aps_ftqc_skrypt.tex` (Book-of-Fux-style stylized edition) is explicitly deferred, not built this session.
- The `aps-artifact-theme.sty`/`aps-artifact-template.tex` pair lives at repo root (not under `fuxyez/`) since it's now a cross-volume shared asset, same tier as `aps-okf.yaml`.

**Open questions / risks:**
- **TVFD status unknown.** Ross's handoff message says "if FTQC is done and TSLCA is done, and TVFD is done" — this session never touched `tvfd/` at all. Do not assume TVFD alignment is complete; verify before building on that assumption.
- `tslca/tex/aps_tslca_skrypt.tex` is now out of sync with the expanded `.md` version (still mirrors the old ~8-page draft). Not regenerated this session.
- FTQC's `aps-ftqc-manuscript.tex` bib has two entries with `&amp;` (HTML-entity-encoded, from a scraped source) that will print literally as "&amp;" instead of "&" in two journal names — cosmetic, not breaking, not fixed.

**Next steps — AuraFS integration survey (for whichever of Hermes Agent / Grok Build / Cursor picks this up):**

Ross asked for a deep read of `aurafs/src/` (large Rust crate, Cursor-authored, ~400+ files across crypto/gov/network/redteam/whitehat/storage/etc.) and a plan to align it with FTQC, TSLCA, and Fuxyez. Full read wasn't possible in remaining context — this is a **scoped survey of the alignment-critical files only**, not a full audit. Findings and concrete follow-up tasks below; each is self-contained enough to pick up cold.

**Good news first — a lot already matches.** `aurafs/src/physics/mod.rs` and `aurafs.toml` already define the exact canonical constants this session just locked into `ftqc/PHYSICS.md` independently: η (hilbert_scaling_bias) = 5.3, T2 (coherence_window_us) = 1600μs, d_s (spectral_dimension) = 1.37 (±0.05 tolerance, i.e. [1.32, 1.42] — matches `ftqc/PHYSICS.md`'s IPR variance band exactly), PBG (photonic_band_gap) = 0.21. `aurafs/src/network/meshwerk/topology_engine.rs` already implements the replica formula `⌈log_η(N)⌉` and PBG-based routing overhead. `aurafs/src/crypto/pqc/dilithium_sig.rs` + `aurafs/src/shard/{data,mod}.rs` already implement Dilithium-5 shard signing. `AURAFS_PROJECT_CONTEXT.md` already documents D_f=1.585, d_s≈1.37, Sierpiński gasket — it's a peer source, not something that needs to be brought up to speed.

**T1 — Implement HIF math in `aurafs/src/tslca/hif.rs`.** File exists but is empty. `aurafs/src/tslca/lattice.rs` already has the right data shapes (`NodeFields{c,r,a,hif,hif_nbr,active}`, `TslLattice` as a 3×3×3 array) — port the HIF field equation, Triple Threshold Gate, and node/neighborhood/layer/lattice-level HIF from `tslca/aps_tslca_skrypt.md` §5.1–5.5 and §6.1–6.6 (and the full protocol in Appendix A) into this file.

**T2 — Implement propagation/stability/continuity.** `aurafs/src/tslca/mod.rs` and `modes.rs` are both empty. Port §7 (Propagation Rules), §8 (Stability Conditions), §9 (Continuity Conditions) from `tslca/aps_tslca_skrypt.md`. `NodeContinuity{mem, tag, invariants}` already matches Ξ_{i,j,k} from §9.3 — wire it to the renewal-safe-reset behavior described there ("forgetting without amnesia").

**T3 — Make the SIX/SCX/ICX correspondence explicit in code.** `lattice.rs`'s `Channel::{Coherence, Resonance, Alignment}` are HIF's C/R/A subfields, which map to SCX/SIX/ICX per `tslca/aps_tslca_skrypt.md` §5.2's table. Right now that mapping only exists in the manuscript. Add doc comments or a parallel mapping so a reader of the Rust code sees the correspondence without cross-referencing the whitepaper.

**T4 — Reconcile `ftqc/PHYSICS.md`'s AuraFS section against the real implementation.** It was written without reading this code first. Check it against `aurafs/src/physics/mod.rs` + `aurafs.toml` (add the missing `lock_acquisition_timeout_us` = 100μs FUSE constant, which `ftqc/PHYSICS.md` doesn't mention) and against `AURAFS_PROJECT_CONTEXT.md` §4.2 (the likely original source — `ftqc/PHYSICS.md` should cite it, not risk drifting from it).

**T5 — Resolve a "lattice" naming collision.** Three unrelated things are called "lattice" in this codebase: TSLCA's 3×3×3 cognitive lattice (`tslca/lattice.rs`), FTQC/Meshwerk's physical Sierpiński/C6v storage geometry (`network/meshwerk/topology_engine.rs`), and LLL cryptographic lattice-basis reduction for compression (`compression/lattice.rs` — currently a stub, unrelated to the other two). Decide whether they need distinct type names before more code gets written against any of them under the ambiguous shared name.

**T6 — Stub-file audit.** `aurafs/src/core/lattice.rs` contains only the literal text "sss" — a placeholder never filled in. This survey only checked a handful of files; there are likely other stubs like it. Worth a grep pass (`find src -name '*.rs' -size -50c` or similar) before assuming any given module is implemented.

**T7 — Wire the Void-Shard / Trap-State / Aura-Shard lifecycle into `aurafs/src/shard/`.** `ftqc/PHYSICS.md` §2 names this three-stage lifecycle (mutable write buffer → coherent hold during the 1600μs window → final Dilithium-5-signed immutable shard). Dilithium-5 signing already exists; check whether `shard/mod.rs`'s existing state model already captures the three stages implicitly, and if not, add the explicit lifecycle enum/states.

**T8 — Confirm TVFD status before treating it as a fourth "done" framework** to align against (see Open questions above).

### [example] system — file created
Initialized HANDOFF.md. First real entry goes above this line.
