import os
import glob
import re

def fix_literals(filepath):
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            content = f.read()
    except Exception as e:
        print(f"Failed reading {filepath}: {e}")
        return
    
    # Check if the file contains an actual newline
    if '\n' in content and len(content.splitlines()) > 5:
        print(f"File {filepath} already has normal newlines.")
        # But maybe it has some literal \n to fix?
    
    if r'\n' in content:
        # replace literal \n with real newline
        content = content.replace('\\n', '\n')
        with open(filepath, 'w', encoding='utf-8') as f:
            f.write(content)
        print(f"Fixed {filepath}")

if __name__ == '__main__':
    for file in glob.glob('*.tex'):
        fix_literals(file)
