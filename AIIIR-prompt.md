# AIIIR-prompt.md
## Canon Prompt — Aurphyx Intelligence Innovations-Integrations Report

**Document family:** AIIIR  
**Full title:** Aurphyx Intelligence Innovations-Integrations Report  
**Owner:** Aurphyx LLC / Ross A. Edwards  
**Status:** CANON prompt (v1.0 · 2026-08-26)  
**Audience of this file:** Any model, notebook, or analyst producing an AIIIR from external third-party intake.

This is the **only** canon style for AIIIR files. The older “Aurphyx Intelligence Report” briefing prompt is **reference only** — reuse its discipline (Confirmed / Proposed / Assumed / Unknown, no invented facts), not its section list or executive-briefing tone as the whole product.

---

## 0. How to invoke

Copy everything from **§1 Prompt body** to the end of this file into the model (NotebookLM, Gemini, Claude, Perplexity, Grok, Hermes, etc.). Fill the bracketed fields. Attach or load the third-party sources (papers, PDFs, ScienceX / phys.org packets, notebooks such as “rAE: August Research Gifts”, repos, patents, talks).

**Output filename (required):**

```
AIIIR_<ProjectName>_MM-DD-YYYY.md
```

Example: `AIIIR_AugustResearchGifts_08-26-2026.md`

Use PascalCase or tight CamelCase for `ProjectName`. No spaces. Date is the report date, not the source date.

**Length (hard constraint):**

- **Minimum: 111 lines** of Markdown body (count lines in the finished file).  
- **Maximum: 444 lines.**  
- If you are under 111, you have not covered integrations, star-map, or ecosystem fit. Expand those — do not pad with slogans.  
- If you would exceed 444, compress tables and cut low-signal items. Never drop ⭐ items to save lines.

---

## 1. Prompt body (canon — start copy here)

Act as a senior research scout, systems architect, and Aurphyx integration analyst.

You are producing an **Aurphyx Intelligence Innovations-Integrations Report (AIIIR)** from **external third-party intake** for possible use inside the Aurphyx organism (TSLCA, SUXS, SAGES, VASP, Signara, Phosara, Memoree, SoulSync, AuraFS, Fuxyez, Audry).

PROJECT / PACKET NAME: [PROJECT OR NOTEBOOK NAME]  
ORGANIZATION / SOURCE FAMILY: [e.g. ScienceX / phys.org network / other]  
REPORT DATE: [MM-DD-YYYY]  
REPORT VERSION: [e.g. 1.0]  
INTENDED AUDIENCE: Mixed (Ross + Audry + later agents) — strategic and technical, not investor fluff.

### Core rules

1. Use **only** the sources loaded for this run (notebook, PDFs, attachments, pasted corpus). Do not invent papers, numbers, dates, institutions, or capabilities.  
2. Label every material claim:  
   - **Confirmed** — directly supported by the supplied sources  
   - **Proposed** — a documented idea or future direction in the sources  
   - **Assumed / inferred** — your reasoned interpretation; must be validated later  
   - **Unknown** — not in the sources  
3. This is an **intake + integration** report, not a generic project briefing. Prioritize: new mechanisms, materials, substrates, theories, instruments, and how they could *plug into* Aurphyx.  
4. Do **not** treat Aurphyx canon as something you must “prove” from the third-party sources. Map *to* the organism; do not pretend the external papers already contain SIX / Signara / Memoree unless they actually do.  
5. Preserve exact names of papers, authors, institutions, devices, and materials.  
6. No promotional tone. Curiosity is allowed. Hype is not.

### The ⭐ rule (mandatory)

Place a ⭐ immediately before any item that is **both**:

- wildly interesting (mechanism, theory, material, substrate, instrument, or project), **and**  
- strategically impactful for Aurphyx (could change a domain, substrate, protocol, or research path), **and**  
- a **rabbit-hole / wormhole** — further deep research is likely to yield disproportionate results.

Use ⭐ sparingly enough that a STAR-Report can actually chase them. Typical range: **7–21 starred items** for a full packet. Never star a whole section header unless the section is itself the find.

Every ⭐ item must appear again in **§8 Star Map** with: short name, why it is starred, Aurphyx hook (SIX / SCX / ICX / substrate / domain / protocol), and suggested STAR-Report depth.

### Aurphyx mapping vocabulary (use; do not invent new scientific aXis names)

```
SIX  = Somatic Coherence aXis
SCX  = Systemic Coherence aXis
ICX  = Identity Coherence aXis
SUXS-IFO = fusion / contraction operator (Audry-facing)
```

Domains: Sound = VASP · Language = Signara · Vision = Phosara · Memory substrate = Memoree.  
If a find maps to Proprioception / Soma, say so; that domain is still pending a locked stylized name.

### Required output filename

```
AIIIR_<ProjectName>_MM-DD-YYYY.md
```

### Required length

111–444 lines. Self-contained Markdown.

---

# Required AIIIR structure

## 1. Packet Identity
- Packet / project name and aliases  
- Source family (journals, networks, notebooks)  
- One-sentence purpose of *this intake*  
- Date range of sources if known  
- Scope and limitations of the loaded corpus  

## 2. Scout’s Brief (not a generic executive summary)
150–280 words. Answer:
- What class of science or engineering walked in the door?  
- What 3–7 finds actually matter to Aurphyx?  
- Where the ⭐ cluster sits (materials, neuromorphic, topology, sensing, energy, identity, etc.)  
- Immediate recommended next move (usually: which ⭐ items feed the STAR-Report first)

## 3. Intake Inventory
Table:

```
| Source / artifact | Type | Date if known | Core claim (one line) | Status | ⭐ |
```

Status = Confirmed presence in corpus / Incomplete / Unknown date.  
Mark ⭐ in the last column when the *source itself* is a wormhole, not only a passing mention.

## 4. Innovations Field
Cluster the new or unusual **mechanisms, theories, instruments, algorithms, protocols**.  
For each cluster: what it is, evidence status, why an Aurphyx system would care.  
Star individual items, not every paragraph.

## 5. Materials and Substrates Field
New or newly usable **materials, fabrics, crystals, films, metamaterials, neuromorphic substrates, photonic / acoustic / magnetic / topological media**.  
Call out anything that could become a SIR / SKIM / wearable / AuraFS / embodied SIX realization.  
⭐ the ones with wormhole potential.

## 6. Integrations into the Aurphyx Organism
This section is the point of an AIIIR.

Table:

```
| Find | Aurphyx hook (aXis / domain / protocol / product) | Integration mode | Status | ⭐ |
```

Integration modes (pick one):
- **Drop-in** — could inform an existing standard or product with little translation  
- **Bridge** — needs a directed TSLCA bridge or new interface  
- **Substrate** — candidate physical or computational medium  
- **Grammar** — changes how we name or compute a field  
- **Watch** — important, not yet integrable  

Do not force a hook. If there is no honest hook, say **Watch** and leave it.

## 7. Contradictions, Gaps, and Hype Filters
- Claims that sound extraordinary and lack method  
- Conflicts between sources  
- Missing measurements, replication, or materials process  
- Anything that would be unsafe to treat as a production capability  

## 8. Star Map (mandatory)
Table of **every** ⭐ item:

```
| ⭐ ID | Short name | Why starred | Aurphyx hook | Suggested STAR depth | Source anchors |
```

IDs: ⭐01, ⭐02, … stable for the companion STAR-Report.

## 9. Recommended STAR-Report Charge
- Ordered list of ⭐ IDs to pursue first (usually 5–13)  
- What “done” looks like for that STAR-Report  
- Explicit permission to use deep web, academic search, reasoning, and subagents **in the STAR pass only** — not to fabricate facts inside *this* AIIIR  

## 10. Glossary of Incoming Terms
External terms as the sources use them. Note collisions with Aurphyx vocabulary.

## 11. Confidence and Coverage
- Sources reviewed (count + types)  
- Missing source classes  
- Overall confidence: High / Medium / Low  
- What would most improve the next AIIIR revision  

---

# Quality controls

1. Every factual claim is grounded in the loaded sources or marked Assumed / Unknown.  
2. ⭐ items are specific enough to research (a named effect, material, device class, or theory — not “quantum is interesting”).  
3. Exact names preserved.  
4. Tables for inventory, integrations, and the Star Map.  
5. No fabricated citations, funding, partnerships, or “we already built this.”  
6. Filename and line-count constraints obeyed.  
7. End the file with a one-line pointer:  
   `Companion STAR-Report should be: STAR-Report_<ProjectName>_MM-DD-YYYY.md`

## End of prompt body
