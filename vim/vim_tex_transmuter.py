import os
import re
import subprocess
from pathlib import Path


def safe_print(msg):
    """Print with fallback for Windows cp1252 Unicode issues."""
    try:
        print(msg)
    except UnicodeEncodeError:
        print(msg.encode("ascii", "replace").decode())


# Target directory (script's directory, works when run from anywhere)
WORK_DIR = Path(__file__).resolve().parent
TEX_DIR = WORK_DIR / "tex_output"
os.makedirs(TEX_DIR, exist_ok=True)

patterns = {
    "section": re.compile(r"vim_section[-_]([a-zA-Z]+)(_[a-c])?\.md$", re.IGNORECASE),
    "appendix": re.compile(r"appendix_([a-zA-Z0-9_\-∞Ω]+)\.md$", re.IGNORECASE)
}

safe_print("⚡ Starting VIM Markdown -> LaTeX Transmutation...")

for filename in os.listdir(WORK_DIR):
    if not filename.endswith(".md"):
        continue
        
    filepath = WORK_DIR / filename
    new_name = None

    if match := patterns["section"].match(filename):
        roman = match.group(1).upper()
        sub = match.group(2).upper() if match.group(2) else ""
        new_name = f"Section_{roman}{sub}.tex"
    elif match := patterns["appendix"].match(filename):
        identifier = match.group(1).upper()
        new_name = f"Part_{identifier}.tex"

    if new_name:
        target_path = TEX_DIR / new_name
        try:
            # Basic Pandoc conversion from markdown to LaTeX
            subprocess.run(["pandoc", str(filepath), "-o", str(target_path)], check=True)
            safe_print(f"Transmuted: {filename} -> {new_name}")
        except Exception as e:
            safe_print(f"Error converting {filename}: {e}")

safe_print(f"✅ VIM files successfully transmuted to {TEX_DIR}")
