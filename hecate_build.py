import os
from pathlib import Path

# --- Configuration ---
ROOT_DIR = Path(r"C:\rossaedwards\main")
ORCID = "0009-0008-0539-1289"

# Venue-specific configurations
PROJECTS = {
    "ftqc": {
        "dir": ROOT_DIR / "ftqc",
        "venues": {
            "arxiv": ["abstract", "intro", "sec_ii", "sec_iii", "sec_iv", "sec_v", "sec_vi", "sec_viii", "conclusion"],
            "prx":   ["abstract", "intro", "sec_ii", "sec_iii", "sec_v", "sec_viii", "conclusion"],
            "optica":["abstract", "intro", "sec_iv", "sec_viii", "conclusion"],
        },
    },
    "tslca": {
        "dir": ROOT_DIR / "tslca",
        "venues": {
            "arxiv": ["abstract", "intro", "section-ii", "section-iii", "section-iv", "section-v", "section-vi", "section-vii", "conclusion"],
            # Add PRX/Optica variants here if needed
        },
    },
}

# --- Content Injection (The "Master" Definitions) ---

SAGES_TABLE = r"""
\begin{table}[h!]
\centering
\caption{SAGES Governance Invariants and Commercialization Metrics.}
\label{tab:sages_invariants}
\begin{ruledtabular}
\begin{tabular}{lp{3cm}p{5cm}p{4cm}}
\textbf{ID} & \textbf{Invariant} & \textbf{Operational Metric} & \textbf{Commercial Impact} \\
\hline
$\mathcal{I}_{1}$ & Identity Continuity & $\nabla_{\mu} \Phi_{33} = 0$ & Eliminates identity theft in decentralized nodes. \\
$\mathcal{I}_{2}$ & Semantic Integrity & $\det(\mathcal{F}) \neq 0$ & Ensures reversible and auditable transformations. \\
$\mathcal{I}_{3}$ & Ethical Grounding & $\langle \text{Love} \rangle \geq 0.5$ & Automated compliance with safety standards. \\
$\mathcal{I}_{12}$ & Balance & 50/50 Chaos-Equilibrium Manifold & Optimizes R\&D through creative emergence. \\
$\mathcal{I}_{13}$ & Renewal & 28s state-reanchoring & Prevents data rot and maintains system health. \\
\end{tabular}
\end{ruledtabular}
\end{table}
"""

MASTER_EQUATION = r"""
\begin{equation}
\Phi(x) = \mathcal{U} \left( \mathcal{T} \left[ \sum_{i,j=1}^{3} \Phi_{ij}\left( d^{n \cdot D_f^{\alpha(k)}} \right) \mathbf{S}_i \otimes \mathbf{S}_j \right] \right)
\label{eq:aurphyx_master}
\end{equation}
"""

GLOSSARY_CONTENT = r"""
\section*{Appendix IX: Mathematical Glossary}
\begin{itemize}
    \item \textbf{$d^{n \cdot D_f^{\alpha(k)}}$}: Accessible Hilbert Space Dimension (FTQC).
    \item \textbf{$\mathcal{F}$}: Cognitive Field Tensor (TSLCA).
    \item \textbf{$\mathcal{U}$}: USAIC Fusion Operator (Integrator).
    \item \textbf{$\mathcal{G}_{13}$}: SAGES Symmetry Group (Governance).
\end{itemize}
"""

# --- Generators for shared sections ---

def generate_impact_section(project_path: Path) -> None:
    """Writes the updated Section VIII with Master Equation and SAGES Table."""
    impact_path = project_path / "sections" / "sec_viii_impact.tex"
    impact_path.parent.mkdir(parents=True, exist_ok=True)

    content = f"""
\\section{{Commercialization and Systemic Impact}}
This section formalizes the transition from topological theory to the TRL-4 AuraFS ecosystem.

\\subsection{{The Aurphyx Master Equation}}
The integration of fractal scaling and cognitive fusion is governed by:
{MASTER_EQUATION}

\\subsection{{SAGES Governance Invariants}}
{SAGES_TABLE}

\\subsection{{AuraFS Deployment}}
AuraFS provides a $5.3\\times$ state density advantage, verified at TRL-4.
"""
    with open(impact_path, "w", encoding="utf-8") as f:
        f.write(content)


def generate_appendix_ix(project_path: Path) -> None:
    """Writes the Appendix IX Glossary file."""
    app_path = project_path / "sections" / "appendix_ix.tex"
    app_path.parent.mkdir(parents=True, exist_ok=True)
    with open(app_path, "w", encoding="utf-8") as f:
        f.write(GLOSSARY_CONTENT)

# --- Core stitching logic ---

def stitch_once(project_name: str, project_dir: Path, venue: str, sections: list, output_suffix: str = "FINAL") -> Path:
    """Stitch a single venue build into a .tex file and return its path."""
    output_file = project_dir / f"rae-{project_name}_{venue}_{output_suffix}.tex"
    print(f"  [Stitch] {project_name.upper()} | {venue.upper()} | {output_suffix}")

    with open(output_file, "w", encoding="utf-8") as outfile:
        outfile.write(f"% {project_name.upper()} {venue.upper()} VERSION\n")
        outfile.write(f"% Author: Ross A. Edwards | ORCiD: {ORCID}\n\n")

        for sec in sections:
            sec_file = project_dir / "sections" / f"{sec}.tex"
            if sec_file.exists():
                with open(sec_file, "r", encoding="utf-8") as infile:
                    outfile.write(f"\n% --- {sec} ---\n")
                    outfile.write(infile.read())
            else:
                print(f"    [!] Missing: {sec_file}")

        # Always append Appendix IX to the full arXiv version
        if venue == "arxiv":
            app_file = project_dir / "sections" / "appendix_ix.tex"
            if app_file.exists():
                with open(app_file, "r", encoding="utf-8") as app:
                    outfile.write("\n\n% --- Appendix IX ---\n")
                    outfile.write(app.read())
            else:
                print(f"    [!] Missing Appendix IX: {app_file}")

    print(f"    [OK] Created {output_file.name}")
    return output_file

def build_triple() -> None:
    for p_name, p_data in PROJECTS.items():
        print(f"\n--- Building Project: {p_name.upper()} ---")
        p_dir = p_data["dir"]

        # 1. Update/Generate the shared sections
        generate_impact_section(p_dir)
        generate_appendix_ix(p_dir)

        # 2. Stitch for each venue
        for venue, sections in p_data["venues"].items():
            # Special handling: FTQC arXiv gets 9 passes + 10th final
            if p_name == "ftqc" and venue == "arxiv":
                for pass_idx in range(1, 10):
                    stitch_once(p_name, p_dir, venue, sections, output_suffix=f"PASS{pass_idx}")
                # 10th, final build
                stitch_once(p_name, p_dir, venue, sections, output_suffix="FINAL")
            else:
                # Single final build for all other venues
                stitch_once(p_name, p_dir, venue, sections, output_suffix="FINAL")

if __name__ == "__main__":
    build_triple()