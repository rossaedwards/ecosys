import os
import re
from pathlib import Path

VIM_DIR = Path("main/vim")

ROMAN_PATTERN = re.compile(r"^vim_section_([ivxlcdm]+)(.*)\.md$", re.IGNORECASE)
APPENDIX_PATTERN = re.compile(r"^appendix_([a-z0-9]+)\.md$", re.IGNORECASE)

SPECIAL_APPENDICES = {
    "omega": "Ω",
    "infinity": "∞",
    "infinite": "∞"
}

ROMAN_VALUES = {
    'I': 1, 'V': 5, 'X': 10,
    'L': 50, 'C': 100, 'D': 500, 'M': 1000
}

def roman_to_int(s):
    s = s.upper()
    total = 0
    prev = 0
    for ch in reversed(s):
        val = ROMAN_VALUES.get(ch, 0)
        if val < prev:
            total -= val
        else:
            total += val
        prev = val
    return total

def main():
    print("\n🔍 SCANNING VIM FOLDER (NO CHANGES WILL BE MADE)\n")

    sections = []
    appendices = []
    engines = []
    unknown = []

    for file in VIM_DIR.iterdir():
        name = file.name

        # VIM sections
        m = ROMAN_PATTERN.match(name)
        if m:
            roman = m.group(1)
            suffix = m.group(2)
            index = roman_to_int(roman)
            sections.append((index, roman.upper(), suffix, name))
            continue

        # Appendices
        a = APPENDIX_PATTERN.match(name)
        if a:
            key = a.group(1).lower()
            label = SPECIAL_APPENDICES.get(key, key.upper())
            appendices.append((label, name))
            continue

        # Engines (HIF/TSL)
        if name.startswith(("hif_", "tsl_")):
            engines.append(name)
            continue

        # Everything else
        unknown.append(name)

    # Sort sections by Roman numeral
    sections.sort(key=lambda x: x[0])

    print("📘 ORDERED MANUSCRIPT SECTIONS (I–C):\n")
    for idx, roman, suffix, fname in sections:
        print(f"  {roman}{suffix}  →  {fname}")

    print("\n📙 APPENDICES (Balance State Vector Alphabet, Metrics & Dynamics):\n")
    for label, fname in appendices:
        print(f"  {label}  →  {fname}")

    print("\n📗 ENGINE FILES (HIF / TSL):\n")
    for fname in engines:
        print(f"  {fname}")

    if unknown:
        print("\n⚠️ OTHER FILES (left untouched):\n")
        for fname in unknown:
            print(f"  {fname}")

    print("\n✨ DONE. No files were renamed or modified.\n")

if __name__ == "__main__":
    main()
    