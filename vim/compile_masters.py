import subprocess
from pathlib import Path

# The specific master files for the Academic Triad
MASTERS = [
    "Volume_I_Geodesic_Evolution.tex",
    "Volume_II_Thermodynamic_Stabilization.tex",
    "Volume_III_Edwards_Unified_Field.tex"
]

def compile_pdf(tex_file):
    if not Path(tex_file).exists():
        print(f"⚠️ Skipping {tex_file} - File not found.")
        return

    base_name = tex_file.replace('.tex', '')
    print(f"\n🔄 Initiating 10-Pass Compilation for: {base_name}")
    
    for pass_num in range(1, 11):
        print(f"   ➤ Executing Pass {pass_num}/10...")
        subprocess.run(
            ["pdflatex", "-interaction=nonstopmode", "-halt-on-error", tex_file], 
            stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL
        )
        
        # Inject BibTeX run after Pass 1 to resolve citations
        if pass_num == 1:
            print(f"   📚 Resolving Bibliography (BibTeX)...")
            subprocess.run(["bibtex", base_name], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
            
    print(f"   ✅ SUCCESS: {base_name}.pdf forged.")

print("⚡ IGNITING THE 10-PASS COMPILER...")
for master in MASTERS:
    compile_pdf(master)
print("\n👑 ALL VOLUMES FORGED. Cross-references locked.")
