#!/usr/bin/env python3
"""
VIM_FIGURE_FORGE.py

Self-contained generator for VIM figure scripts.

- Reads:  VIM_FIGURE_GUIDE.md
- Parses:
  * SECTION I–X with explicit bullets
  * Range blocks XI–XX, XXI–XXX, XXXI–XL, XLI–L, LI–LXX
    - Strict header detection (R1)
    - Hybrid reconciliation (Option C)
    - Auto-generated figures use suffix (Option A)
  * LXXI–C section headers with single figures
  * Appendices A–Z, Ω, ∞
- Writes: main/vim/fig_scripts/*.py (one per figure)
"""

from __future__ import annotations

import re
from pathlib import Path
from textwrap import dedent

# ---------------------------------------------------------------------
# Paths
# ---------------------------------------------------------------------

ROOT = Path(__file__).resolve().parents[2] if len(Path(__file__).resolve().parents) >= 2 else Path(".")
VIM_DIR = ROOT / "main" / "vim"
GUIDE_PATH = VIM_DIR / "VIM_FIGURE_GUIDE.md"
OUT_DIR = VIM_DIR / "fig_scripts"


# ---------------------------------------------------------------------
# Roman numeral helpers
# ---------------------------------------------------------------------

_ROMAN_MAP = [
    (1000, "M"),
    (900, "CM"),
    (500, "D"),
    (400, "CD"),
    (100, "C"),
    (90, "XC"),
    (50, "L"),
    (40, "XL"),
    (10, "X"),
    (9, "IX"),
    (5, "V"),
    (4, "IV"),
    (1, "I"),
]

_ROMAN_TO_INT = {
    "I": 1,
    "II": 2,
    "III": 3,
    "IV": 4,
    "V": 5,
    "VI": 6,
    "VII": 7,
    "VIII": 8,
    "IX": 9,
    "X": 10,
    "XI": 11,
    "XII": 12,
    "XIII": 13,
    "XIV": 14,
    "XV": 15,
    "XVI": 16,
    "XVII": 17,
    "XVIII": 18,
    "XIX": 19,
    "XX": 20,
    "XXI": 21,
    "XXII": 22,
    "XXIII": 23,
    "XXIV": 24,
    "XXV": 25,
    "XXVI": 26,
    "XXVII": 27,
    "XXVIII": 28,
    "XXIX": 29,
    "XXX": 30,
    "XXXI": 31,
    "XXXII": 32,
    "XXXIII": 33,
    "XXXIV": 34,
    "XXXV": 35,
    "XXXVI": 36,
    "XXXVII": 37,
    "XXXVIII": 38,
    "XXXIX": 39,
    "XL": 40,
    "XLI": 41,
    "XLII": 42,
    "XLIII": 43,
    "XLIV": 44,
    "XLV": 45,
    "XLVI": 46,
    "XLVII": 47,
    "XLVIII": 48,
    "XLIX": 49,
    "L": 50,
    "LI": 51,
    "LII": 52,
    "LIII": 53,
    "LIV": 54,
    "LV": 55,
    "LVI": 56,
    "LVII": 57,
    "LVIII": 58,
    "LIX": 59,
    "LX": 60,
    "LXI": 61,
    "LXII": 62,
    "LXIII": 63,
    "LXIV": 64,
    "LXV": 65,
    "LXVI": 66,
    "LXVII": 67,
    "LXVIII": 68,
    "LXIX": 69,
    "LXX": 70,
    "LXXI": 71,
    "LXXII": 72,
    "LXXIII": 73,
    "LXXIV": 74,
    "LXXV": 75,
    "LXXVI": 76,
    "LXXVII": 77,
    "LXXVIII": 78,
    "LXXIX": 79,
    "LXXX": 80,
    "LXXXI": 81,
    "LXXXII": 82,
    "LXXXIII": 83,
    "LXXXIV": 84,
    "LXXXV": 85,
    "LXXXVI": 86,
    "LXXXVII": 87,
    "LXXXVIII": 88,
    "LXXXIX": 89,
    "XC": 90,
    "XCI": 91,
    "XCII": 92,
    "XCIII": 93,
    "XCIV": 94,
    "XCV": 95,
    "XCVI": 96,
    "XCVII": 97,
    "XCVIII": 98,
    "XCIX": 99,
    "C": 100,
}


def roman_to_int(s: str) -> int:
    s = s.strip().upper()
    if s in _ROMAN_TO_INT:
        return _ROMAN_TO_INT[s]
    # Fallback generic parser (not really needed for 1–100, but safe)
    i = 0
    result = 0
    while i < len(s):
        if i + 1 < len(s) and s[i : i + 2] in ("CM", "CD", "XC", "XL", "IX", "IV"):
            pair = s[i : i + 2]
            for val, sym in _ROMAN_MAP:
                if sym == pair:
                    result += val
                    break
            i += 2
        else:
            for val, sym in _ROMAN_MAP:
                if s[i] == sym:
                    result += val
                    break
            i += 1
    return result


def int_to_roman(num: int) -> str:
    result = []
    for val, sym in _ROMAN_MAP:
        while num >= val:
            result.append(sym)
            num -= val
    return "".join(result)


# ---------------------------------------------------------------------
# Normalization helpers
# ---------------------------------------------------------------------

def normalize_shortname(text: str) -> str:
    t = text.strip().lower()
    t = re.sub(r"[^\w]+", "_", t)
    t = re.sub(r"_+", "_", t).strip("_")
    return t or "figure"


def normalize_appendix_token(tok: str) -> str:
    tok = tok.strip().upper()
    if tok == "Ω":
        return "appendix_omega"
    if tok in ("∞", "INFINITE"):
        return "appendix_infinite"
    if len(tok) == 1 and tok.isalpha():
        return f"appendix_{tok.lower()}"
    return f"appendix_{normalize_shortname(tok)}"


# ---------------------------------------------------------------------
# Parsing
# ---------------------------------------------------------------------

def parse_guide() -> list[dict]:
    """
    Parse VIM_FIGURE_GUIDE.md into a list of figure entries.

    Each entry:
        {
            "index": int,
            "id": str,
            "section_id": str,
            "label": str,
            "title": str,
            "auto": bool,
        }
    """
    text = GUIDE_PATH.read_text(encoding="utf-8")
    lines = text.replace("\r\n", "\n").split("\n")

    entries: list[dict] = []

    current_section_id: str | None = None
    current_section_label: str | None = None

    # Range block state (R1 + hybrid C + auto A)
    in_range = False
    range_start_roman: str | None = None
    range_end_roman: str | None = None
    range_start_num: int | None = None
    range_end_num: int | None = None
    range_explicit_nums: list[int] = []
    range_buffer_entries: list[dict] = []

    def flush_range():
        nonlocal in_range, range_start_num, range_end_num, range_explicit_nums, range_buffer_entries
        if not in_range or range_start_num is None or range_end_num is None:
            # Nothing to do
            in_range = False
            range_start_num = None
            range_end_num = None
            range_explicit_nums = []
            range_buffer_entries = []
            return

        # Add explicit entries first
        for e in range_buffer_entries:
            entries.append(e)

        # Auto-generate missing numbers (Option C + A)
        full_set = set(range(range_start_num, range_end_num + 1))
        missing = sorted(full_set.difference(range_explicit_nums))
        if missing:
            base_id = f"section_{range_start_roman}_{range_end_roman}_auto"
            for num in missing:
                title = f"Auto-generated figure {num}"
                section_id = base_id
                entry_id = f"{base_id}_fig{num}"
                entries.append(
                    {
                        "index": None,  # filled later
                        "id": entry_id,
                        "section_id": section_id,
                        "label": f"Figure {num}",
                        "title": title,
                        "auto": True,
                    }
                )

        # Reset range state
        in_range = False
        range_start_roman = None
        range_end_roman = None
        range_start_num = None
        range_end_num = None
        range_explicit_nums = []
        range_buffer_entries = []

    # Regexes
    re_section_header = re.compile(r"^##\s+\*\*(SECTION\s+([IVXLCDM]+)\s+—\s+(.+))\*\*")
    re_section_header_roman_only = re.compile(r"^##\s+\*\*([IVXLCDM]+)\s+—\s+(.+)\*\*")
    re_range_header = re.compile(r"^###\s+\*\*SECTIONS\s+([IVXLCDM]+)–([IVXLCDM]+)")
    re_figure_bullet = re.compile(r"^- \*\*Figure\s+(\d+)\s*:\s*(.+)\*\*?")
    re_numbered_figure = re.compile(r"^(\d+)\.\s+(.*)$")
    re_appendix_bullet = re.compile(r"^- \*\*Appendix\s+(.+?)\s*:\s*(.+)\*\*?")

    for raw_line in lines:
        line = raw_line.rstrip("\n")

        # Range boundary detection (R1): flush when we hit a new header or a horizontal rule
        if in_range:
            if line.startswith("#") or line.strip().startswith("---"):
                flush_range()
                # continue processing this line as a potential header
                # (do not "continue" here)

        stripped = line.strip()
        if not stripped:
            continue

        # 1. Range header (R1)
        m_range = re_range_header.match(stripped)
        if m_range:
            # Flush any previous range
            flush_range()
            start_roman = m_range.group(1).strip().upper()
            end_roman = m_range.group(2).strip().upper()
            start_num = roman_to_int(start_roman)
            end_num = roman_to_int(end_roman)

            in_range = True
            range_start_roman = start_roman
            range_end_roman = end_roman
            range_start_num = start_num
            range_end_num = end_num
            range_explicit_nums = []
            range_buffer_entries = []
            current_section_id = None
            current_section_label = None
            continue

        # 2. SECTION headers with "SECTION <ROMAN> — Title"
        m_sec = re_section_header.match(stripped)
        if m_sec:
            flush_range()
            roman = m_sec.group(2).strip().upper()
            title = m_sec.group(3).strip()
            current_section_id = f"section_{roman}"
            current_section_label = title
            continue

        # 3. SECTION headers with "<ROMAN> — Title" (LXXI–C, etc.)
        m_sec2 = re_section_header_roman_only.match(stripped)
        if m_sec2:
            flush_range()
            roman = m_sec2.group(1).strip().upper()
            title = m_sec2.group(2).strip()
            current_section_id = f"section_{roman}"
            current_section_label = title
            continue

        # 4. Explicit figure bullets: "- **Figure 1: ...**"
        m_fig_bullet = re_figure_bullet.match(stripped)
        if m_fig_bullet:
            num = int(m_fig_bullet.group(1))
            title = m_fig_bullet.group(2).strip()
            label = f"Figure {num}"
            section_id = current_section_id or f"section_{int_to_roman(num)}"
            entry_id = f"{section_id}_fig{num}"
            entry = {
                "index": None,
                "id": entry_id,
                "section_id": section_id,
                "label": label,
                "title": title,
                "auto": False,
            }
            if in_range:
                range_explicit_nums.append(num)
                range_buffer_entries.append(entry)
            else:
                entries.append(entry)
            continue

        # 5. Numbered list figures inside ranges: "11. Symmetry orbits"
        m_num = re_numbered_figure.match(stripped)
        if m_num:
            num = int(m_num.group(1))
            title = m_num.group(2).strip()
            label = f"Figure {num}"
            if in_range and range_start_num is not None and range_end_num is not None:
                # Explicit figure inside range: each gets its own section ID
                roman = int_to_roman(num)
                section_id = f"section_{roman}"
                entry_id = f"{section_id}_fig{num}"
                entry = {
                    "index": None,
                    "id": entry_id,
                    "section_id": section_id,
                    "label": label,
                    "title": title,
                    "auto": False,
                }
                range_explicit_nums.append(num)
                range_buffer_entries.append(entry)
            else:
                # Numbered figure outside a range (not really present in this guide, but safe)
                roman = int_to_roman(num)
                section_id = f"section_{roman}"
                entry_id = f"{section_id}_fig{num}"
                entries.append(
                    {
                        "index": None,
                        "id": entry_id,
                        "section_id": section_id,
                        "label": label,
                        "title": title,
                        "auto": False,
                    }
                )
            continue

        # 6. Appendix bullets: "- **Appendix A: ...**"
        m_app = re_appendix_bullet.match(stripped)
        if m_app:
            app_token = m_app.group(1).strip()
            title = m_app.group(2).strip()
            section_id = normalize_appendix_token(app_token)
            label = f"Figure {app_token}"
            entry_id = f"{section_id}_fig"
            entries.append(
                {
                    "index": None,
                    "id": entry_id,
                    "section_id": section_id,
                    "label": label,
                    "title": title,
                    "auto": False,
                }
            )
            continue

        # Everything else is ignored by the parser.

    # Flush any trailing range
    flush_range()

    # Assign global indices and ensure stable ordering
    for idx, e in enumerate(entries, start=1):
        e["index"] = idx

    return entries


# ---------------------------------------------------------------------
# Script generation
# ---------------------------------------------------------------------

SCRIPT_TEMPLATE = """\
\"\"\"Auto-generated figure script for VIM.

Section ID : {section_id}
Figure ID  : {figure_id}
Label      : {label}
Title      : {title}
Auto       : {auto_flag}
\"\"\"

from pathlib import Path


def run_simulation(output_dir: Path | None = None) -> Path:
    \"\"\"Placeholder simulation for: {title}

    This function should be replaced with the actual physics / simulation
    corresponding to this figure in the VIM manuscript.
    \"\"\"
    if output_dir is None:
        try:
            output_dir = Path(__file__).resolve().parent
        except NameError:
            output_dir = Path.cwd()

    output_dir.mkdir(parents=True, exist_ok=True)
    out_path = output_dir / "{png_name}"

    # TODO: replace this with real simulation / plotting code.
    # For now, we just write a tiny placeholder file so the pipeline is testable.
    out_path.write_text(
        "Placeholder for {title}\\n"
        "Section ID: {section_id}\\n"
        "Figure ID : {figure_id}\\n",
        encoding="utf-8",
    )

    print(f"[VIM FIGURE] Generated placeholder: {{out_path}}")
    return out_path


if __name__ == "__main__":
    run_simulation()
"""


def generate_scripts(entries: list[dict]) -> int:
    OUT_DIR.mkdir(parents=True, exist_ok=True)
    count = 0
    for e in entries:
        idx = e["index"]
        section_id = e["section_id"]
        figure_id = e["id"]
        label = e["label"]
        title = e["title"]
        auto_flag = "yes" if e["auto"] else "no"

        short = normalize_shortname(f"{idx:03d}_{title}")
        script_name = f"fig_{idx:03d}_{short}.py"
        png_name = f"fig_{idx:03d}_{short}.png"

        script_text = SCRIPT_TEMPLATE.format(
            section_id=section_id,
            figure_id=figure_id,
            label=label,
            title=title.replace('"', '\\"'),
            auto_flag=auto_flag,
            png_name=png_name,
        )

        (OUT_DIR / script_name).write_text(dedent(script_text), encoding="utf-8")
        count += 1
    return count


# ---------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------

def main() -> None:
    if not GUIDE_PATH.exists():
        raise SystemExit(f"Guide not found: {GUIDE_PATH}")

    entries = parse_guide()
    n = generate_scripts(entries)
    print(f"Generated {n} figure scripts in {OUT_DIR}")


if __name__ == "__main__":
    main()
