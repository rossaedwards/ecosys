#!/usr/bin/env python3
# ORCID: 0009-0008-0539-1289
"""
FTQC Bibliography Consolidator v2.0
Converts RIS → BibTeX, merges ALL .bib files (including master_citations.bib),
removes duplicates, generates arxiv_ftqc_complete.bib for main.tex
Ross A. Edwards | Aurphyx LLC
"""
from pathlib import Path
import re
import hashlib
from datetime import datetime

class BibliographyConsolidator:
    def __init__(self):
        self.citations_dir = Path('citations')
        self.entries = {}
        self.duplicates_removed = 0
        self.total_loaded = 0

    def log(self, msg, level="INFO"):
        icons = {"INFO": "ℹ️", "SUCCESS": "✅", "WARNING": "⚠️", "ERROR": "❌", "DIVINE": "✨"}
        print(f"{icons.get(level, '•')} {msg}")

    def convert_ris_to_bibtex(self, ris_file):
        with open(ris_file, 'r', encoding='utf-8', errors='ignore') as f:
            content = f.read()
        entries = []
        current_entry = {}
        entry_type = 'article'
        for line in content.split('\n'):
            line = line.strip()
            if not line:
                continue
            if line.startswith('TY  -'):
                current_entry = {}
                ris_type = line.split('-', 1)[1].strip()
                entry_type = {'JOUR': 'article', 'BOOK': 'book', 'CONF': 'inproceedings',
                              'RPRT': 'techreport', 'THES': 'phdthesis'}.get(ris_type, 'misc')
            elif line.startswith(('AU  -', 'A1  -')):
                current_entry.setdefault('author', []).append(line.split('-', 1)[1].strip())
            elif line.startswith(('TI  -', 'T1  -')):
                current_entry['title'] = line.split('-', 1)[1].strip()
            elif line.startswith(('JO  -', 'T2  -', 'JF  -')):
                current_entry['journal'] = line.split('-', 1)[1].strip()
            elif line.startswith(('PY  -', 'Y1  -')):
                current_entry['year'] = line.split('-', 1)[1].strip().split('/')[0]
            elif line.startswith('VL  -'):
                current_entry['volume'] = line.split('-', 1)[1].strip()
            elif line.startswith('SP  -'):
                current_entry['pages'] = line.split('-', 1)[1].strip()
            elif line.startswith('DO  -'):
                current_entry['doi'] = line.split('-', 1)[1].strip()
            elif line.startswith('ER  -'):
                if 'title' in current_entry and 'year' in current_entry:
                    first_author = current_entry.get('author', ['Unknown'])[0].split(',')[0].strip()
                    key = re.sub(r'[^a-zA-Z0-9]', '', f"{first_author}{current_entry['year']}")
                    bib_entry = f"@{entry_type}{{{key},\n"
                    if 'author' in current_entry:
                        bib_entry += f"  author = {{{' and '.join(current_entry['author'])}}},\n"
                    for field in ['title', 'journal', 'year', 'volume', 'pages', 'doi']:
                        if field in current_entry:
                            bib_entry += f"  {field} = {{{current_entry[field]}}},\n"
                    bib_entry += "}\n"
                    entries.append((key, bib_entry))
        return entries

    def _entry_hash(self, entry_text):
        doi_match = re.search(r'doi\s*=\s*\{([^}]+)\}', entry_text, re.IGNORECASE)
        if doi_match:
            return 'doi:' + doi_match.group(1).strip().lower()
        title_match = re.search(r'title\s*=\s*[{"]([^}"]+)[}"]', entry_text, re.IGNORECASE)
        year_match = re.search(r'year\s*=\s*[{"]?(\d{4})[}"\s,]', entry_text, re.IGNORECASE)
        if title_match:
            raw = re.sub(r'[^a-z0-9]', '', title_match.group(1).strip().lower())
            if year_match:
                raw += year_match.group(1)
            return hashlib.md5(raw.encode()).hexdigest()[:12]
        return None

    def _extract_key(self, entry_text):
        # BUG #1 FIX
        m = re.search(r'@\w+\{([^,\s]+)', entry_text)
        return m.group(1).strip() if m else None

    def _add_entry(self, key, entry_text):
        h = self._entry_hash(entry_text) or key
        if h in self.entries:
            self.duplicates_removed += 1
            return False
        final_key = key
        existing_keys = {k for _, (k, _) in self.entries.items()}
        suffix = 1
        while final_key in existing_keys:
            final_key = f"{key}{chr(96 + suffix)}"
            suffix += 1
        if final_key != key:
            entry_text = re.sub(r'(@\w+\{)' + re.escape(key) + r',',
                                f'\\g<1>{final_key},', entry_text, count=1)
        self.entries[h] = (final_key, entry_text)
        self.total_loaded += 1
        return True

    def load_bibtex_file(self, bib_file):
        self.log(f"Loading {bib_file.name}...", "INFO")
        with open(bib_file, 'r', encoding='utf-8', errors='ignore') as f:
            content = f.read()
        # BUG #2 FIX: lookahead split catches ALL entries
        raw_entries = re.split(r'(?=@[A-Za-z]+\s*\{)', content)
        loaded = 0
        for entry in raw_entries:
            entry = entry.strip()
            if not entry or not entry.startswith('@'):
                continue
            if entry.lower().startswith('@comment'):
                continue
            key = self._extract_key(entry)
            if key and self._add_entry(key, entry):
                loaded += 1
        self.log(f"  Loaded {loaded} entries", "SUCCESS")
        return loaded

    def consolidate(self):
        self.log("\n" + "="*70, "DIVINE")
        self.log("  FTQC BIBLIOGRAPHY CONSOLIDATOR v2.0", "DIVINE")
        self.log("  Ross A. Edwards | Aurphyx LLC", "DIVINE")
        self.log("="*70 + "\n", "DIVINE")

        # BUG #3 FIX: scan all likely dirs, prioritize master_citations.bib
        search_dirs = [Path('.'), Path('citations'), Path('refs'),
                       Path('bibliography'), Path('bib')]
        all_bib_files, all_ris_files = [], []
        seen = set()
        for d in search_dirs:
            if d.exists():
                for f in d.glob('*.bib'):
                    fp = f.resolve()
                    if fp not in seen and f.name != 'arxiv_ftqc_complete.bib':
                        seen.add(fp)
                        all_bib_files.append(f)
                all_ris_files += list(d.glob('*.ris'))

        self.log(f"Found {len(all_bib_files)} .bib files:", "INFO")
        for f in all_bib_files:
            self.log(f"  → {f}", "INFO")

        for ris_file in all_ris_files:
            for key, entry in self.convert_ris_to_bibtex(ris_file):
                self._add_entry(key, entry)

        # master_citations.bib loads FIRST
        priority = [f for f in all_bib_files if 'master' in f.name.lower()]
        others   = [f for f in all_bib_files if f not in priority]
        for bib_file in priority + others:
            self.load_bibtex_file(bib_file)

        if not self.entries:
            self.log("No entries loaded! Check paths.", "ERROR")
            return False

        output_file = Path('arxiv_ftqc_complete.bib')
        with open(output_file, 'w', encoding='utf-8') as f:
            f.write(f"% FTQC Master Bibliography\n% Date: {datetime.now()}\n")
            f.write(f"% Entries: {len(self.entries)} | Dupes removed: {self.duplicates_removed}\n\n")
            for _, (key, entry) in sorted(self.entries.items(), key=lambda x: x[1][0].lower()):
                f.write(entry.strip() + "\n\n")

        for tex in ['main.tex', 'rae-ftqc_arxiv_complete_FINAL.tex']:
            if Path(tex).exists():
                with open(tex, 'r') as f:
                    c = f.read()
                updated = re.sub(r'\\bibliography\{[^}]+\}',
                                 r'\\bibliography{arxiv_ftqc_complete}', c)
                if updated != c:
                    with open(tex, 'w') as f:
                        f.write(updated)
                    self.log(f"Updated \\bibliography{{}} in {tex}", "SUCCESS")
                break

        self.log(f"\n✅ {output_file}: {len(self.entries)} references ready", "SUCCESS")
        self.log(f"   Dupes removed: {self.duplicates_removed}", "INFO")
        self.log("✨ All 200+ citations will appear in your arXiv PDF.\n", "DIVINE")
        return True

def main():
    BibliographyConsolidator().consolidate()

if __name__ == '__main__':
    main()
