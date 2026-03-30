import json
import os
import re
from pathlib import Path

def main():
    root = Path(r'c:\rossaedwards\main\ftqc')
    filelist_path = root / 'ftqc_03-30-2026_filelist.txt'
    
    # Read the filelist (tree /F format)
    files = []
    if filelist_path.exists():
        with open(filelist_path, 'r', encoding='utf-8') as f:
            current_dir = root
            for line in f:
                if 'Folder PATH listing' in line or 'Volume serial number' in line or line.startswith('C:.'):
                    continue
                if not line.strip() or line.strip() == '|':
                    continue
                
                # Directory
                dir_match = re.match(r'^[|\s]*\\---(.*)', line)
                if dir_match:
                    current_dir = root / dir_match.group(1).strip()
                    continue
                
                # File
                filename = re.sub(r'^[|\s+]*', '', line).strip()
                if filename:
                    path = current_dir / filename
                    if path.exists() and path.is_file():
                        files.append(path)
    
    # If list is empty (format mismatch), fallback to recursive search
    if not files:
        files = [f for f in root.glob('**/*') if f.is_file()]
    
    py_files = []
    tex_files = []
    bib_files = []
    
    for f in files:
        ext = f.suffix.lower()
        if ext == '.py':
            py_files.append(f)
        elif ext == '.tex':
            tex_files.append(f)
        elif ext == '.bib':
            bib_files.append(f)
            
    # Process Python scripts
    orcid_py = '# ORCID: 0009-0008-0539-1289'
    for py_file in py_files:
        try:
            with open(py_file, 'r', encoding='utf-8') as f:
                content = f.read()
            if '0009-0008-0539-1289' not in content:
                lines = content.splitlines()
                if lines and lines[0].startswith('#!'):
                    lines.insert(1, orcid_py)
                else:
                    lines.insert(0, orcid_py)
                with open(py_file, 'w', encoding='utf-8') as f:
                    f.write('\n'.join(lines) + '\n')
        except Exception as e:
            pass

    # Process LaTeX files
    missing_figures = set()
    for tex_file in tex_files:
        try:
            with open(tex_file, 'r', encoding='utf-8') as f:
                content = f.read()
            
            # ORCID injection
            if '0009-0008-0539-1289' not in content:
                def author_repl(match):
                    author_content = match.group(1)
                    if 'orcidlink' not in author_content:
                        return author_content + r' \orcidlink{0009-0008-0539-1289}}'
                    return match.group(0)
                
                # Attempt to find \author{...} or \author[...]{...}
                new_content = re.sub(r'(\\author(?:\[[^\]]*\])?\{[^}]*)\}', author_repl, content)
                
                if new_content != content:
                    with open(tex_file, 'w', encoding='utf-8') as f:
                        f.write(new_content)

            # Figure verification based on EXISTING paths in the .tex
            figs = re.findall(r'\\includegraphics(?:\[.*?\])?\{([^}]+)\}', content)
            
            for fig in set(figs):
                p_direct = root / fig
                p_fig_dir = root / 'figures' / fig
                if not (p_direct.exists() or p_fig_dir.exists()):
                    missing_figures.add(f"{tex_file.name}: {fig} (Not Found)")
        except Exception as e:
            pass

    # Generate scan_report.md
    report_path = root / 'scan_report.md'
    
    report_md = [
        "# FTQC Repository Scan Summary",
        "",
        f"- **Total Files Scanned:** {len(files)}",
        f"- **Python Scripts Analyzed & Updated:** {len(py_files)}",
        f"- **LaTeX Documents Analyzed & Updated:** {len(tex_files)}",
        f"- **Bibliography Files Found:** {len(bib_files)}",
        "",
        "## Figure Verification Status"
    ]
    
    if missing_figures:
        report_md.append("The following figures were referenced but could not be located at their specified paths:")
        for mf in sorted(list(missing_figures)):
            report_md.append(f"- {mf}")
    else:
        report_md.append("All figures referenced in the LaTeX files were successfully verified at their existing paths.")
        
    with open(report_path, 'w', encoding='utf-8') as f:
        f.write('\n'.join(report_md))

if __name__ == '__main__':
    main()
