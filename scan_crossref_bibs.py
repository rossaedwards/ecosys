import os
import re
from pathlib import Path

ROOT_DIR = Path(".")
TEX_DIR = ROOT_DIR / "vim" / "tex_output" # Adjust based on where your tex files live
BIB_FILE = ROOT_DIR / "references.bib"

cite_pattern = re.compile(r"\\cite\{([^}]+)\}")
label_pattern = re.compile(r"\\label\{([^}]+)\}")
ref_pattern = re.compile(r"\\ref\{([^}]+)\}")

found_labels = set()
found_refs = set()
found_citations = set()

print("🔍 Scanning LaTeX files for Cross-References and Citations...")

if not TEX_DIR.exists():
    print(f"Directory {TEX_DIR} not found. Run transmuter script first.")
    exit(1)

for root, _, files in os.walk(TEX_DIR):
    for file in files:
        if file.endswith(".tex"):
            with open(Path(root) / file, "r", encoding="utf-8") as f:
                content = f.read()
                found_labels.update(cite_pattern.findall(content))
                found_refs.update(ref_pattern.findall(content))
                found_labels.update(label_pattern.findall(content))

# Output Diagnostics
print("\n--- DIAGNOSTICS ---")
print(f"Total Unique Labels Found: {len(found_labels)}")
print(f"Total Unique Cross-Refs Found: {len(found_refs)}")

broken_refs = found_refs - found_labels
if broken_refs:
    print(f"⚠️ WARNING: Found {len(broken_refs)} broken references (\\ref without matching \\label):")
    for br in broken_refs:
        print(f"  - {br}")
else:
    print("✅ All internal cross-references are valid.")

print("\nTo manage Citations, ensure the following keys exist in your references.bib:")
for cite in found_citations:
    print(f"  - @article{{{cite}, ...}}")
    