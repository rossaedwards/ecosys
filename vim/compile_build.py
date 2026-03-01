import subprocess, shutil, os, sys
from pathlib import Path

def build():
    print("--- Ross A. Edwards BUILD CHAIN ---")
    
    # Run Sims (from simulations/ so ../figures resolves correctly)
    print("Running Simulations...")
    for s in ["sim_governor.py", "sim_vacuum_collapse.py", "sim_voice.py"]:
        subprocess.run([sys.executable, s], cwd="simulations", check=True)
    
    # Compile LaTeX
    print("Compiling PDF...")
    os.chdir("src")
    # Copy figures to src for pdflatex (expects flat structure)
    for fig in Path("../figures").glob("*.png"):
        shutil.copy(fig, ".")
    subprocess.run(["pdflatex", "-interaction=nonstopmode", "Vacuum_Impedance_Matching_Thesis_II.tex"], check=True)
    subprocess.run(["biber", "Vacuum_Impedance_Matching_Thesis_II"], check=True)
    subprocess.run(["pdflatex", "-interaction=nonstopmode", "Vacuum_Impedance_Matching_Thesis_II.tex"], check=True)
    
    # arXiv Prep
    print("Creating arXiv submission...")
    out = Path("../arxiv_build")
    out.mkdir(exist_ok=True)
    for f in ["Vacuum_Impedance_Matching_Thesis_II.tex", "Vacuum_Impedance_Matching_Thesis_II.bbl", "references.bib"]:
        shutil.copy(f, out)
    for fig in Path("../figures").glob("*.png"):
        shutil.copy(fig, out)
    shutil.make_archive("../Vacuum_Impedance_Matching_Thesis_II_arXiv", 'zip', out)
    print("DONE.")

if __name__ == "__main__": build()