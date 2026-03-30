import os, glob

def main():
    for f in glob.glob('*.tex'):
        with open(f, 'r', encoding='utf-8') as file:
            content = file.read()
            
        if r'\orcidlink' in content and r'\usepackage{orcidlink}' not in content:
            lines = content.splitlines()
            new_lines = []
            for line in lines:
                if line.strip() == r'\begin{document}':
                    new_lines.append(r'\usepackage{orcidlink}')
                new_lines.append(line)
            
            with open(f, 'w', encoding='utf-8') as file:
                file.write('\n'.join(new_lines) + '\n')

if __name__ == '__main__':
    main()
