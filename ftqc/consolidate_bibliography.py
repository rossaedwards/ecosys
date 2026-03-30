#!/usr/bin/env python3
# ORCID: 0009-0008-0539-1289
"""
FTQC Bibliography Consolidator
Converts RIS → BibTeX, merges all .bib files, removes duplicates
Generates arxiv_ftqc_complete.bib for main.tex
"""
from pathlib import Path
import re
import hashlib

class BibliographyConsolidator:
    def __init__(self):
        self.citations_dir = Path('citations')
        self.entries = {}  # key → entry dict
        self.duplicates_removed = 0
    
    def log(self, msg, level="INFO"):
        icons = {"INFO": "ℹ️", "SUCCESS": "✅", "WARNING": "⚠️", "ERROR": "❌"}
        print(f"{icons.get(level, '•')} {msg}")
    
    def convert_ris_to_bibtex(self, ris_file):
        """Convert RIS to BibTeX entries (simple parser)"""
        self.log(f"Converting {ris_file.name}...", "INFO")
        
        with open(ris_file, 'r', encoding='utf-8', errors='ignore') as f:
            content = f.read()
        
        # Simple RIS → BibTeX conversion
        entries = []
        current_entry = {}
        entry_type = 'article'
        
        for line in content.split('\n'):
            line = line.strip()
            if not line:
                continue
            
            if line.startswith('TY  -'):
                current_entry = {}
                ris_type = line.split('-')[1].strip()
                entry_type = {'JOUR': 'article', 'BOOK': 'book', 'CONF': 'inproceedings'}.get(ris_type, 'misc')
            
            elif line.startswith('AU  -') or line.startswith('A1  -'):
                author = line.split('-', 1)[1].strip()
                if 'author' not in current_entry:
                    current_entry['author'] = []
                current_entry['author'].append(author)
            
            elif line.startswith('TI  -') or line.startswith('T1  -'):
                current_entry['title'] = line.split('-', 1)[1].strip()
            
            elif line.startswith('JO  -') or line.startswith('T2  -'):
                current_entry['journal'] = line.split('-', 1)[1].strip()
            
            elif line.startswith('PY  -') or line.startswith('Y1  -'):
                current_entry['year'] = line.split('-', 1)[1].strip().split('/')[0]
            
            elif line.startswith('VL  -'):
                current_entry['volume'] = line.split('-', 1)[1].strip()
            
            elif line.startswith('SP  -'):
                current_entry['pages'] = line.split('-', 1)[1].strip()
            
            elif line.startswith('DO  -'):
                current_entry['doi'] = line.split('-', 1)[1].strip()
            
            elif line.startswith('ER  -'):
                if 'title' in current_entry and 'year' in current_entry:
                    # Generate citation key
                    first_author = current_entry.get('author', ['Unknown'])[0].split(',')[0].strip()
                    year = current_entry['year']
                    key = f"{first_author}{year}"
                    key = re.sub(r'[^a-zA-Z0-9]', '', key)
                    
                    # Format as BibTeX
                    bib_entry = f"@{entry_type}{{{key},\n"
                    
                    if 'author' in current_entry:
                        authors = ' and '.join(current_entry['author'])
                        bib_entry += f"  author = {{{authors}}},\n"
                    
                    for field in ['title', 'journal', 'year', 'volume', 'pages', 'doi']:
                        if field in current_entry:
                            value = current_entry[field] if isinstance(current_entry[field], str) else str(current_entry[field])
                            bib_entry += f"  {field} = {{{value}}},\n"
                    
                    bib_entry += "}\n"
                    entries.append((key, bib_entry))
        
        return entries
    
    def parse_bibtex_entry(self, entry_text):
        """Parse a BibTeX entry to extract key and fields"""
        # Extract citation key
        key_match = re.search(r'@\w+\{([^,]+),', entry_text)
        if not key_match:
            return None, None
        
        key = key_match.group(1).strip()
        return key, entry_text
    
    def load_bibtex_file(self, bib_file):
        """Load entries from a .bib file"""
        self.log(f"Loading {bib_file.name}...", "INFO")
        
        with open(bib_file, 'r', encoding='utf-8', errors='ignore') as f:
            content = f.read()
        
        # Split into entries
        entries = re.split(r'\n@', content)
        loaded = 0
        
        for entry in entries:
            if not entry.strip():
                continue
            
            # Add @ back if it was split
            if not entry.startswith('@'):
                entry = '@' + entry
            
            key, full_entry = self.parse_bibtex_entry(entry)
            if key:
                # Check for duplicates by title hash
                title_match = re.search(r'title\s*=\s*\{([^}]+)\}', full_entry, re.IGNORECASE)
                if title_match:
                    title = title_match.group(1).strip().lower()
                    title_hash = hashlib.md5(title.encode()).hexdigest()[:8]
                    
                    if title_hash in self.entries:
                        self.duplicates_removed += 1
                        continue
                    
                    self.entries[title_hash] = (key, full_entry)
                    loaded += 1
                else:
                    # No title, store by key
                    if key not in [k for h, (k, e) in self.entries.items()]:
                        self.entries[key] = (key, full_entry)
                        loaded += 1
        
        self.log(f"  Loaded {loaded} unique entries", "SUCCESS")
    
    def consolidate(self):
        """Main consolidation process"""
        self.log("\n" + "="*70, "INFO")
        self.log("FTQC BIBLIOGRAPHY CONSOLIDATOR", "INFO")
        self.log("="*70 + "\n", "INFO")
        
        if not self.citations_dir.exists():
            self.log("citations/ directory not found!", "ERROR")
            return False
        
        # Step 1: Convert RIS files
        ris_files = list(self.citations_dir.glob('*.ris'))
        if ris_files:
            self.log(f"Found {len(ris_files)} RIS files", "INFO")
            for ris_file in ris_files:
                entries = self.convert_ris_to_bibtex(ris_file)
                for key, entry in entries:
                    title_match = re.search(r'title\s*=\s*\{([^}]+)\}', entry, re.IGNORECASE)
                    if title_match:
                        title = title_match.group(1).strip().lower()
                        title_hash = hashlib.md5(title.encode()).hexdigest()[:8]
                        if title_hash not in self.entries:
                            self.entries[title_hash] = (key, entry)
        
        # Step 2: Load existing .bib files
        bib_files = list(self.citations_dir.glob('*.bib'))
        if bib_files:
            self.log(f"\nFound {len(bib_files)} BibTeX files", "INFO")
            for bib_file in bib_files:
                self.load_bibtex_file(bib_file)
        
        # Step 3: Also check root directory
        root_bibs = [f for f in Path('.').glob('*.bib') if f.name != 'arxiv_ftqc_complete.bib']
        if root_bibs:
            self.log(f"\nFound {len(root_bibs)} BibTeX files in root", "INFO")
            for bib_file in root_bibs:
                self.load_bibtex_file(bib_file)
        
        # Step 4: Write consolidated bibliography
        output_file = Path('arxiv_ftqc_complete.bib')
        self.log(f"\n📝 Writing {output_file.name}...", "INFO")
        
        with open(output_file, 'w', encoding='utf-8') as f:
            f.write("% FTQC Master Bibliography\n")
            f.write(f"% Auto-generated: {Path(__file__).name}\n")
            f.write(f"% Total entries: {len(self.entries)}\n")
            f.write(f"% Duplicates removed: {self.duplicates_removed}\n\n")
            
            for hash_or_key, (key, entry) in sorted(self.entries.items()):
                f.write(entry)
                f.write("\n\n")
        
        self.log(f"✅ Wrote {len(self.entries)} entries to {output_file}", "SUCCESS")
        self.log(f"   Removed {self.duplicates_removed} duplicates", "INFO")
        
        # Step 5: Update main.tex
        if Path('main.tex').exists():
            self.log("\n📝 Updating main.tex...", "INFO")
            with open('main.tex', 'r', encoding='utf-8') as f:
                content = f.read()
            
            # Update bibliography command
            content = re.sub(
                r'\\bibliography\{[^}]+\}',
                r'\\bibliography{arxiv_ftqc_complete}',
                content
            )
            
            with open('main.tex', 'w', encoding='utf-8') as f:
                f.write(content)
            
            self.log("  Updated \\bibliography{arxiv_ftqc_complete}", "SUCCESS")
        
        self.log("\n" + "="*70, "SUCCESS")
        self.log("🎉 CONSOLIDATION COMPLETE!", "SUCCESS")
        self.log("="*70, "SUCCESS")
        self.log(f"\n✅ {output_file}: {len(self.entries)} references", "SUCCESS")
        self.log("\n💡 Next step: python fix_citations.py", "INFO")
        
        return True

def main():
    consolidator = BibliographyConsolidator()
    consolidator.consolidate()

if __name__ == '__main__':
    main()
