#!/usr/bin/env python3
"""
bib_consolidator.py

Reads citation-list.md (structured citation list from ftqc thesis),
parses all 200 entries, fetches BibTeX from doi.org for each,
deduplicates, and writes ms.bib.

Format parsed:
  Title | BibTeX_Key | URL | DOI | Category | Priority
"""

import re, time, shutil, sys
from pathlib import Path

try:
    import requests
except ImportError:
    sys.exit("Missing requests. Run: pip install requests")

INPUT_FILE  = "citation-list.md"
OUTPUT_FILE = "ms.bib"
BACKUP_FILE = "ms_ORIGINAL_backup.bib"

# ── 0. Backup existing ms.bib ─────────────────────────────────────────────
if Path(OUTPUT_FILE).exists():
    shutil.copy(OUTPUT_FILE, BACKUP_FILE)
    print(f"Backed up {OUTPUT_FILE} -> {BACKUP_FILE}")

# ── 1. Parse citation-list.md ────────────────────────────────────────────
raw = Path(INPUT_FILE).read_text(encoding="utf-8", errors="replace")

# Each line: Title | Key | URL | DOI | Category | Priority
line_re = re.compile(
    r"^(.+?)\s*\|\s*(\S+?)\s*\|\s*(.+?)\s*\|\s*(.+?)\s*\|\s*(\S+?)\s*\|\s*(\S+?)\s*$",
    re.MULTILINE
)

ISBN_RE = re.compile(r"ISBN:", re.IGNORECASE)
DOI_RE  = re.compile(r"^10\.\d{4,}/\S+")

parsed = []
seen_keys = set()

for m in line_re.finditer(raw):
    title, key, url, doi, category, priority = [x.strip() for x in m.groups()]
    if key == "BibTeX_Key":
        continue
    # strip trailing annotation (e.g. " ---raebit")
    key = re.split(r"\s+---", key)[0].strip()
    if key in seen_keys:
        print(f"  Duplicate skipped: {key}")
        continue
    seen_keys.add(key)
    parsed.append({"title": title, "key": key, "url": url,
                   "doi": doi, "category": category, "priority": priority})

print(f"Parsed {len(parsed)} unique citations from {INPUT_FILE}")

# ── 2. Fetch BibTeX from doi.org or build fallback ───────────────────────
headers = {
    "Accept": "application/x-bibtex",
    "User-Agent": "bib-consolidator/1.0"
}

entries_out  = []
failed       = []
manual_count = 0

for i, item in enumerate(parsed):
    key   = item["key"]
    doi   = item["doi"]
    title = item["title"]
    url   = item["url"]

    # Extract raw URL from markdown link syntax [text](url)
    url_m     = re.search(r"\((https?://[^)]+)\)", url)
    url_clean = url_m.group(1) if url_m else url.strip()

    # ---- No real DOI (ISBN, WorldCat, Caltech thesis) --------------------
    if ISBN_RE.search(doi) or not DOI_RE.match(doi):
        isbn_m = re.search(r"ISBN:?(\S+)", doi, re.IGNORECASE)
        isbn   = isbn_m.group(1).strip() if isbn_m else ""
        lines_e = [f"@misc{{{key},"]
        lines_e.append(f"  title        = {{{title}}},")
        if isbn:
            lines_e.append(f"  note         = {{ISBN: {isbn}}},")
        if url_clean.startswith("http"):
            lines_e.append(f"  howpublished = {{\\url{{{url_clean}}}}},")
        lines_e.append("}")
        entries_out.append("\n".join(lines_e))
        manual_count += 1
        print(f"  [{i+1:03d}] MANUAL  {key}")
        continue

    # ---- Real DOI — fetch from doi.org -----------------------------------
    try:
        r = requests.get(f"https://doi.org/{doi}", headers=headers,
                         timeout=12, allow_redirects=True)
        if r.status_code == 200 and r.text.strip().startswith("@"):
            bib = re.sub(
                r"@(\w+)\{[^,]+,",
                lambda mo: f"@{mo.group(1)}{{{key},",
                r.text.strip(), count=1
            )
            entries_out.append(bib)
            print(f"  [{i+1:03d}] OK      {key}")
        else:
            raise ValueError(f"HTTP {r.status_code}")
    except Exception as e:
        failed.append((key, doi, str(e)))
        lines_e = [f"@article{{{key},"]
        lines_e.append(f"  title = {{{title}}},")
        lines_e.append(f"  doi   = {{{doi}}},")
        if url_clean.startswith("http"):
            lines_e.append(f"  url   = {{{url_clean}}},")
        lines_e.append(f"  note  = {{FETCH FAILED - verify manually}},")
        lines_e.append("}")
        entries_out.append("\n".join(lines_e))
        print(f"  [{i+1:03d}] FAILED  {key}  ({e})")

    time.sleep(0.4)

# ── 3. Write ms.bib ────────────────────────────────────────────────────
Path(OUTPUT_FILE).write_text("\n\n".join(entries_out) + "\n", encoding="utf-8")
size_kb = Path(OUTPUT_FILE).stat().st_size / 1024

print()
print("=" * 50)
print("DONE")
print(f"  Total entries : {len(entries_out)}")
print(f"  Fetched OK    : {len(entries_out) - len(failed) - manual_count}")
print(f"  Manual (ISBN) : {manual_count}")
print(f"  Fetch failed  : {len(failed)}")
print(f"  Written to    : {OUTPUT_FILE}  ({size_kb:.1f} KB)")
print("=" * 50)

if failed:
    print()
    print("Entries needing manual attention:")
    for fk, fd, fe in failed:
        print(f"  {fk:<40s}  DOI: {fd}")
        print(f"    Error: {fe}")
