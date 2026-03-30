#!/usr/bin/env python3
# ORCID: 0009-0008-0539-1289
"""
Compile citation sources for the FTQC arXiv build.

- Reads all .ris and .bib files from ./ftqc
- Converts RIS records into BibTeX entries
- Merges everything into:
  - ./master_citations.bib
  - ./arxiv_ftqc.bib
"""

from __future__ import annotations

from collections import defaultdict
from pathlib import Path
import re
import sys
from typing import DefaultDict


SCRIPT_DIR = Path(__file__).resolve().parent
CITATIONS_DIR = SCRIPT_DIR
MASTER_OUTPUT = CITATIONS_DIR / "master_citations.bib"
ARXIV_OUTPUT = SCRIPT_DIR / "arxiv_ftqc.bib"

RIS_EXT = ".ris"
BIB_EXT = ".bib"


def read_text_with_fallback(path: Path) -> str:
    for encoding in ("utf-8-sig", "utf-8", "cp1252", "latin-1"):
        try:
            return path.read_text(encoding=encoding)
        except UnicodeDecodeError:
            continue
    raise UnicodeDecodeError("unknown", b"", 0, 1, f"Could not decode: {path}")


def sanitize_value(value: str) -> str:
    value = value.replace("\n", " ").strip()
    value = re.sub(r"\s+", " ", value)
    return value


def escape_bibtex(value: str) -> str:
    value = sanitize_value(value)
    value = value.replace("\\", "\\\\")
    value = value.replace("{", "\\{").replace("}", "\\}")
    return value


def parse_ris_file(path: Path) -> list[dict[str, list[str]]]:
    text = read_text_with_fallback(path)
    records: list[dict[str, list[str]]] = []
    current: DefaultDict[str, list[str]] | None = None
    last_tag: str | None = None

    for raw_line in text.splitlines():
        line = raw_line.rstrip("\r\n")
        if not line.strip():
            continue

        # RIS canonical line: "XX  - value"
        match = re.match(r"^([A-Z0-9]{2})\s{2}-\s?(.*)$", line)
        if match:
            tag = match.group(1).strip()
            value = match.group(2).strip()

            if tag == "TY":
                current = defaultdict(list)
                current["TY"].append(value)
                last_tag = "TY"
                continue

            if current is None:
                # Skip preamble noise before first TY.
                continue

            if tag == "ER":
                records.append(dict(current))
                current = None
                last_tag = None
                continue

            current[tag].append(value)
            last_tag = tag
            continue

        # Continuation lines are appended to the previous tag.
        if current is not None and last_tag is not None:
            continuation = line.strip()
            if continuation:
                prior = current[last_tag][-1] if current[last_tag] else ""
                current[last_tag][-1] = f"{prior} {continuation}".strip()

    if current:
        records.append(dict(current))

    return records


def first_value(record: dict[str, list[str]], *tags: str) -> str:
    for tag in tags:
        values = record.get(tag, [])
        if values:
            value = sanitize_value(values[0])
            if value:
                return value
    return ""


def parse_year(record: dict[str, list[str]]) -> str:
    candidates = [
        first_value(record, "PY"),
        first_value(record, "Y1"),
        first_value(record, "DA"),
    ]
    for candidate in candidates:
        match = re.search(r"(19|20)\d{2}", candidate)
        if match:
            return match.group(0)
    return ""


def infer_entry_type(ris_type: str) -> str:
    mapping = {
        "JOUR": "article",
        "JFULL": "article",
        "BOOK": "book",
        "CHAP": "incollection",
        "CONF": "inproceedings",
        "CPAPER": "inproceedings",
        "THES": "phdthesis",
        "RPRT": "techreport",
    }
    return mapping.get(ris_type.upper(), "misc")


def sanitize_key(raw: str) -> str:
    key = re.sub(r"[^A-Za-z0-9:_-]+", "", raw)
    return key or "ref"


def extract_existing_keys(bib_text: str) -> set[str]:
    return {
        match.group(1).strip()
        for match in re.finditer(r"@\w+\s*\{\s*([^,\s]+)", bib_text, flags=re.IGNORECASE)
    }


def normalize_existing_bib_keys(bib_text: str, used_keys: set[str]) -> str:
    pattern = re.compile(r"(@\w+\s*\{\s*)([^,]+?)(\s*,)", flags=re.IGNORECASE)

    def replace(match: re.Match[str]) -> str:
        prefix, raw_key, comma = match.groups()
        base = sanitize_key(raw_key.strip())
        key = base
        suffix = 2
        while key in used_keys:
            key = f"{base}_{suffix}"
            suffix += 1
        used_keys.add(key)
        return f"{prefix}{key}{comma}"

    return pattern.sub(replace, bib_text)


def make_key(record: dict[str, list[str]], used_keys: set[str], fallback_seed: str) -> str:
    preferred = first_value(record, "ID")
    if preferred:
        base = sanitize_key(preferred)
    else:
        doi = first_value(record, "DO")
        if doi:
            base = sanitize_key(doi.replace("/", "_"))
        else:
            authors = record.get("AU", []) or record.get("A1", [])
            first_author = authors[0] if authors else fallback_seed
            author_token = re.sub(r"[^A-Za-z0-9]+", "", first_author.split(",")[0]) or "author"
            year = parse_year(record) or "noyear"
            title = first_value(record, "TI", "T1", "CT")
            title_token = re.sub(r"[^A-Za-z0-9]+", "", title.split(" ")[0]) if title else "title"
            base = sanitize_key(f"{author_token}{year}{title_token}")

    key = base
    suffix = 2
    while key in used_keys:
        key = f"{base}_{suffix}"
        suffix += 1
    used_keys.add(key)
    return key


def ris_record_to_bibtex(
    record: dict[str, list[str]],
    used_keys: set[str],
    fallback_seed: str,
) -> str:
    entry_type = infer_entry_type(first_value(record, "TY") or "MISC")
    key = make_key(record, used_keys, fallback_seed)

    fields: list[tuple[str, str]] = []

    authors = record.get("AU", []) or record.get("A1", [])
    if authors:
        fields.append(("author", " and ".join(sanitize_value(a) for a in authors if sanitize_value(a))))

    title = first_value(record, "TI", "T1", "CT")
    if title:
        fields.append(("title", title))

    journal = first_value(record, "JO", "JF", "T2")
    if journal:
        fields.append(("journal", journal))

    year = parse_year(record)
    if year:
        fields.append(("year", year))

    volume = first_value(record, "VL")
    if volume:
        fields.append(("volume", volume))

    number = first_value(record, "IS")
    if number:
        fields.append(("number", number))

    start_page = first_value(record, "SP")
    end_page = first_value(record, "EP")
    if start_page and end_page:
        fields.append(("pages", f"{start_page}--{end_page}"))
    elif start_page:
        fields.append(("pages", start_page))

    doi = first_value(record, "DO")
    if not doi:
        note_lines = record.get("N1", [])
        for note in note_lines:
            doi_match = re.search(r"doi:\s*(\S+)", note, flags=re.IGNORECASE)
            if doi_match:
                doi = doi_match.group(1)
                break
    if doi:
        fields.append(("doi", doi))

    url = first_value(record, "UR")
    if url:
        fields.append(("url", url))

    issn = first_value(record, "SN")
    if issn:
        fields.append(("issn", issn))

    publisher = first_value(record, "PB")
    if publisher:
        fields.append(("publisher", publisher))

    abstract = first_value(record, "AB", "N2")
    if abstract:
        fields.append(("abstract", abstract))

    notes = [sanitize_value(n) for n in record.get("N1", []) if sanitize_value(n)]
    if notes:
        note = "; ".join(notes)
        if "doi:" in note.lower() and doi:
            note = re.sub(r"doi:\s*\S+", "", note, flags=re.IGNORECASE).strip(" ;")
        if note:
            fields.append(("note", note))

    lines = [f"@{entry_type}{{{key},"]
    for field_name, field_value in fields:
        lines.append(f"  {field_name} = {{{escape_bibtex(field_value)}}},")
    if len(lines) > 1:
        lines[-1] = lines[-1].rstrip(",")
    lines.append("}")
    return "\n".join(lines)


def collect_source_files() -> tuple[list[Path], list[Path]]:
    if not CITATIONS_DIR.exists():
        raise FileNotFoundError(f"Missing citations directory: {CITATIONS_DIR}")

    bib_files = sorted(
        [
            p
            for p in CITATIONS_DIR.glob(f"*{BIB_EXT}")
            if p.name.lower() != MASTER_OUTPUT.name.lower()
        ],
        key=lambda p: p.name.lower(),
    )
    ris_files = sorted(CITATIONS_DIR.glob(f"*{RIS_EXT}"), key=lambda p: p.name.lower())
    return bib_files, ris_files


def build_merged_bib() -> tuple[str, int, int, int]:
    bib_files, ris_files = collect_source_files()

    merged_chunks: list[str] = []
    used_keys: set[str] = set()

    # Include existing .bib files first, normalizing malformed entry keys.
    for bib_path in bib_files:
        bib_text = read_text_with_fallback(bib_path).strip()
        if not bib_text:
            continue
        normalized_text = normalize_existing_bib_keys(bib_text, used_keys=used_keys)
        merged_chunks.append(f"% --- source: {bib_path.name} ---\n{normalized_text}\n")

    ris_record_count = 0
    converted_entry_count = 0

    for ris_path in ris_files:
        records = parse_ris_file(ris_path)
        ris_record_count += len(records)
        converted_entries: list[str] = []
        for idx, record in enumerate(records, start=1):
            if not first_value(record, "TY"):
                continue
            converted_entries.append(
                ris_record_to_bibtex(record, used_keys=used_keys, fallback_seed=f"{ris_path.stem}_{idx}")
            )

        if converted_entries:
            converted_entry_count += len(converted_entries)
            block = "\n\n".join(converted_entries)
            merged_chunks.append(f"% --- converted from: {ris_path.name} ---\n{block}\n")

    output_header = (
        "% Auto-generated by compile_citations.py\n"
        "% Do not edit manually; modify source files in ./citations instead.\n\n"
    )
    merged_text = output_header + "\n".join(merged_chunks).strip() + "\n"
    return merged_text, len(bib_files), ris_record_count, converted_entry_count


def write_outputs(merged_text: str) -> None:
    MASTER_OUTPUT.write_text(merged_text, encoding="utf-8", newline="\n")
    ARXIV_OUTPUT.write_text(merged_text, encoding="utf-8", newline="\n")


def main() -> int:
    used_key_count = 0
    try:
        merged_text, bib_count, ris_records, converted_entries = build_merged_bib()
        write_outputs(merged_text)
        used_key_count = len(extract_existing_keys(merged_text))
    except Exception as exc:  # noqa: BLE001
        print(f"[ERROR] Citation compile failed: {exc}", file=sys.stderr)
        return 1

    print("[OK] Citation merge complete.")
    print(f"  Source .bib files: {bib_count}")
    print(f"  Source .ris records: {ris_records}")
    print(f"  Converted RIS entries: {converted_entries}")
    print(f"  Approx unique BibTeX keys: {used_key_count}")
    print(f"  Wrote: {MASTER_OUTPUT}")
    print(f"  Wrote: {ARXIV_OUTPUT}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
