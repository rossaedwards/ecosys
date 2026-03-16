import os

import re

import shutil

import subprocess

from pathlib import Path



\# --- CONFIGURATION ---

ROOT\_DIR = Path(".")

BUILD\_DIR = ROOT\_DIR / "colossus\_build"

PDF\_DIR = BUILD\_DIR / "final\_pdfs"



VOL\_1\_DIR = BUILD\_DIR / "Volume\_I\_Geodesic\_Evolution"

VOL\_2\_DIR = BUILD\_DIR / "Volume\_II\_Thermodynamic\_Stabilization"

VOL\_3\_DIR = BUILD\_DIR / "Volume\_III\_Edwards\_Unified\_Field"



for d in \[VOL\_1\_DIR, VOL\_2\_DIR, VOL\_3\_DIR, PDF\_DIR]:

&nbsp;   os.makedirs(d / "src\_markdown", exist\_ok=True)

&nbsp;   os.makedirs(d / "sections\_tex", exist\_ok=True)



\# --- REGEX CLASSIFIERS ---

patterns = {

&nbsp;   "vim\_theory": re.compile(r"vim\_section\_(\[a-zA-Z]+)(\_\[a-c])?\\.md$", re.IGNORECASE),

&nbsp;   "hif\_engine": re.compile(r"hif\_(.+)\\.md$", re.IGNORECASE),

&nbsp;   "tsl\_engine": re.compile(r"tsl\_(.+)\\.md$", re.IGNORECASE),

&nbsp;   "sages\_protocol": re.compile(r"SAGES\_(.+)\\.md$", re.IGNORECASE),

&nbsp;   "sages\_aux": re.compile(r"AUX\[-\_](\[a-zA-Z0-9]+)\[-\_]001\\.md$", re.IGNORECASE),

&nbsp;   "appendix\_canon": re.compile(r"appendix\_(\[a-zA-Z0-9\_\\-∞Ω]+)\\.md$", re.IGNORECASE),

&nbsp;   "semantic\_cog": re.compile(r"(tslca|fuxyez|section)\[-\_](.+)\\.md$", re.IGNORECASE)

}



def roman\_to\_int(s):

&nbsp;   rom\_val = {'I': 1, 'V': 5, 'X': 10, 'L': 50, 'C': 100, 'D': 500, 'M': 1000}

&nbsp;   int\_val = 0

&nbsp;   s = s.upper()

&nbsp;   try:

&nbsp;       for i in range(len(s)):

&nbsp;           if i > 0 and rom\_val\[s\[i]] > rom\_val\[s\[i - 1]]:

&nbsp;               int\_val += rom\_val\[s\[i]] - 2 \* rom\_val\[s\[i - 1]]

&nbsp;           else:

&nbsp;               int\_val += rom\_val\[s\[i]]

&nbsp;       return int\_val

&nbsp;   except KeyError:

&nbsp;       return 999 



def process\_file(source\_path, target\_vol\_dir, new\_base\_name, sort\_key):

&nbsp;   md\_target = target\_vol\_dir / "src\_markdown" / f"{new\_base\_name}.md"

&nbsp;   tex\_target = target\_vol\_dir / "sections\_tex" / f"{new\_base\_name}.tex"

&nbsp;   

&nbsp;   shutil.copy2(source\_path, md\_target)

&nbsp;   try:

&nbsp;       subprocess.run(\["pandoc", str(md\_target), "-o", str(tex\_target)], check=True, capture\_output=True)

&nbsp;   except Exception as e:

&nbsp;       print(f"⚠️ Pandoc skipped {new\_base\_name}: {e}")

&nbsp;       

&nbsp;   return (sort\_key, f"sections\_tex/{new\_base\_name}.tex")



print("⚡ IGNITING THE FORGE: Scanning for Colossus Artifacts...")



vol\_1\_files, vol\_2\_files, vol\_3\_files = \[], \[], \[]



for root, dirs, files in os.walk(ROOT\_DIR):

&nbsp;   if any(skip in root for skip in \["colossus\_build", ".git", "node\_modules", "venv"]):

&nbsp;       continue



&nbsp;   for filename in files:

&nbsp;       if not filename.endswith(".md"): continue

&nbsp;       filepath = Path(root) / filename

&nbsp;       

&nbsp;       if match := patterns\["vim\_theory"].match(filename):

&nbsp;           roman, sub = match.group(1), match.group(2) or ""

&nbsp;           name = f"Theory\_{roman.upper()}{sub.upper()}"

&nbsp;           vol\_1\_files.append(process\_file(filepath, VOL\_1\_DIR, name, roman\_to\_int(roman)))

&nbsp;       elif match := patterns\["hif\_engine"].match(filename):

&nbsp;           vol\_2\_files.append(process\_file(filepath, VOL\_2\_DIR, f"Engine\_HIF\_{match.group(1)}", 10))

&nbsp;       elif match := patterns\["tsl\_engine"].match(filename):

&nbsp;           vol\_2\_files.append(process\_file(filepath, VOL\_2\_DIR, f"Lattice\_TSL\_{match.group(1)}", 20))

&nbsp;       elif match := patterns\["semantic\_cog"].match(filename):

&nbsp;           if "vim\_" not in filename.lower():

&nbsp;               vol\_2\_files.append(process\_file(filepath, VOL\_2\_DIR, f"Semantic\_{match.group(2)}", 30))

&nbsp;       elif match := patterns\["appendix\_canon"].match(filename):

&nbsp;           id\_str = match.group(1).upper()

&nbsp;           sort\_key = 100 if id\_str == 'Ω' else (101 if id\_str in \['INFINITE', '∞'] else ord(id\_str\[0]) if id\_str else 99)

&nbsp;           vol\_3\_files.append(process\_file(filepath, VOL\_3\_DIR, f"Cosmology\_Part\_{id\_str}", sort\_key))

&nbsp;       elif match := patterns\["sages\_protocol"].match(filename) or patterns\["sages\_aux"].match(filename):

&nbsp;           vol\_3\_files.append(process\_file(filepath, VOL\_3\_DIR, f"Governance\_{filename.replace('.md', '')}", 200))



vol\_1\_files.sort(key=lambda x: x\[0])

vol\_2\_files.sort(key=lambda x: x\[0])

vol\_3\_files.sort(key=lambda x: x\[0])



\# --- PRIME OPUS MASTER TEMPLATE GENERATOR ---

def generate\_latex(vol\_dir, filename, vol\_num, vol\_title, vol\_sub, abstract, quote, files\_list):

&nbsp;   if not files\_list: return None



&nbsp;   # Using raw strings to preserve LaTeX formatting and exactly match the user's template

&nbsp;   preamble = r"""%=========================================================

% PRIME OPUS MASTER TEMPLATE

% The Balance Continuum + Aurphyx Standard

%=========================================================

\\documentclass\[11pt,oneside]{book}



%-------------------------------

% PACKAGES

%-------------------------------

\\usepackage\[margin=1in]{geometry}

\\usepackage{amsmath,amssymb,amsthm,physics}

\\usepackage{graphicx}

\\usepackage{tikz}

\\usepackage{hyperref}

\\usepackage{titlesec}

\\usepackage{setspace}

\\usepackage{tocloft}

\\usepackage{lmodern}

\\usepackage\[T1]{fontenc}

\\usepackage\[utf8]{inputenc}



%-------------------------------

% HYPERREF SETUP

%-------------------------------

\\hypersetup{

&nbsp; colorlinks=true,

&nbsp; linkcolor=cyan,

&nbsp; urlcolor=magenta,

&nbsp; citecolor=orange,

&nbsp; pdftitle={The Balance Continuum — Prime Opus},

&nbsp; pdfauthor={Ross A. Edwards (R.F. Lovezme)}

}



%-------------------------------

% SECTION FORMATTING

%-------------------------------

\\titleformat{\\chapter}\[display]

&nbsp; {\\bfseries\\Large}

&nbsp; {\\filright\\MakeUppercase{\\chaptername}~\\thechapter}

&nbsp; {1ex}

&nbsp; {\\titlerule\\vspace{1ex}\\filcenter}

&nbsp; \[\\vspace{1ex}\\titlerule]



\\titleformat{\\section}

&nbsp; {\\bfseries\\large}

&nbsp; {\\thesection}{0.75em}{}



\\titleformat{\\subsection}

&nbsp; {\\bfseries\\normalsize}

&nbsp; {\\thesubsection}{0.5em}{}



%-------------------------------

% CUSTOM MACROS

%-------------------------------

\\newcommand{\\rae}{r\\AE{}}

\\newcommand{\\HIF}{\\mathcal{H}\_{\\text{IF}}}

\\newcommand{\\Bliss}{\\mathcal{B}\_{\\text{Bliss}}}

\\newcommand{\\BalanceLaw}{\\mathcal{L}\_{\\text{Bal}}}

\\newcommand{\\UCP}{\\mathcal{U}\_{\\text{coh}}}



%-------------------------------

% TITLE METADATA

%-------------------------------

\\author{Ross A. Edwards (R.F. Lovezme)\\\\

\\small Aurphyx LLC \\\& Aurphyx Foundation\\\\

\\small ORCiD: 0009-0008-0539-1289}



\\title{\\textbf{THE BALANCE CONTINUUM}\\\\\[4pt]

\\large Prime Opus of the Aurphyx Standard}



\\date{\\small 2026}



%=========================================================

\\begin{document}

\\frontmatter

\\maketitle



%-------------------------------

% SIGIL PAGE

%-------------------------------

\\thispagestyle{empty}

\\vspace\*{2cm}

\\begin{center}

\\begin{tikzpicture}\[scale=3, line width=0.6pt]

&nbsp; % Outer Ring (Bliss Manifold)

&nbsp; \\draw\[thick] (0,0) circle (1);

&nbsp; % Hexa-Crown

&nbsp; \\foreach \\i in {0,60,120,180,240,300} {

&nbsp;     \\draw\[thick] (0,0) -- ({0.8\*cos(\\i)},{0.8\*sin(\\i)});

&nbsp;     \\fill ({0.8\*cos(\\i)},{0.8\*sin(\\i)}) circle (0.015);

&nbsp; }

&nbsp; % Inner Tetra

&nbsp; \\draw\[thick] (0,0.45) -- (-0.39,-0.225) -- (0.39,-0.225) -- cycle;

&nbsp; % Central rÆ Glyph

&nbsp; \\fill (0,0) circle (0.03);

\\end{tikzpicture}



\\vspace{1cm}

{\\itshape The Edwards Sigil — Visual Invariant of the Prime Opus}

\\end{center}

\\clearpage



%-------------------------------

% TABLE OF CONTENTS

%-------------------------------

\\tableofcontents

\\clearpage



%=========================================================

\\mainmatter



%-------------------------------

% VOLUME TITLE PAGE

%-------------------------------

\\begin{titlepage}

\\centering

{\\Large\\bfseries THE BALANCE CONTINUUM — VOLUME """ + vol\_num + r"""\\par}

\\vspace{0.5cm}

{\\large\\bfseries """ + vol\_title + r"""\\par}

\\vspace{0.25cm}

{\\large """ + vol\_sub + r"""\\par}

\\vfill

{\\large\\bfseries Author:}\\\\\[2pt]

{\\large Ross A. Edwards (R.F. Lovezme)\\par}

{\\normalsize Aurphyx LLC \\\& Aurphyx Foundation\\par}

{\\normalsize ORCiD: 0009-0008-0539-1289\\par}

\\vfill

{\\bfseries Abstract}\\\\\[6pt]

\\begin{minipage}{0.9\\textwidth}

\\small

""" + abstract + r"""

\\end{minipage}

\\vfill

{\\itshape

""" + quote + r"""

}

\\vfill

{\\normalsize Prime Opus Series — Architect’s Cut\\par}

\\end{titlepage}

\\clearpage



\\part{Volume """ + vol\_num + r""": """ + vol\_title + r"""}



"""



&nbsp;   for item in files\_list:

&nbsp;       preamble += f"\\\\input{{{item\[1]}}}\\n"



&nbsp;   preamble += r"""

\\appendix

\\bibliographystyle{plain}

\\bibliography{../references}

\\end{document}

"""

&nbsp;   

&nbsp;   tex\_path = vol\_dir / filename

&nbsp;   with open(tex\_path, "w", encoding="utf-8") as f:

&nbsp;       f.write(preamble)

&nbsp;   return tex\_path



print("📝 Scaffolding Master LaTeX Files with Prime Opus Template...")



\# Volume 1 Data

generate\_latex(

&nbsp;   VOL\_1\_DIR, "Volume\_I\_Geodesic\_Evolution.tex", 

&nbsp;   "I", "THE ORIGIN MANUSCRIPT", "Geodesic Evolution and Global Attractors",

&nbsp;   "Volume I establishes the foundational architecture of the Balance Continuum — the origin manuscript defining the rÆ manifold, the Edwards Flow, the Harmonic Integrity Field, and the Unified Coherence Principle.",

&nbsp;   "“Before there was structure, there was coherence.\\\\\\\\Before there was form, there was identity.\\\\\\\\Before there was creation, there was Balance.”",

&nbsp;   vol\_1\_files

)



\# Volume 2 Data

generate\_latex(

&nbsp;   VOL\_2\_DIR, "Volume\_II\_Thermodynamic\_Stabilization.tex", 

&nbsp;   "II", "THE META-CREATIVE CYCLE", "Thermodynamic Stabilization in Vacuum Flux Engines",

&nbsp;   "Volume II formalizes the Meta‑Creative Cycle — the Continuum’s mechanism for absorbing newly created worlds, harmonizing emergent domains, reconciling new laws with the Balance Law, and stabilizing cosmological evolution.",

&nbsp;   "“Creation is not the end of the Continuum.\\\\\\\\Creation is the moment the Continuum learns to become more.”",

&nbsp;   vol\_2\_files

)



\# Volume 3 Data

generate\_latex(

&nbsp;   VOL\_3\_DIR, "Volume\_III\_Edwards\_Unified\_Field.tex", 

&nbsp;   "III", "THE UNIFIED FIELD THEORY", "Holographic Duality and Cosmological Coherence",

&nbsp;   "Volume III scales the architecture to the cosmological level, unifying the rÆ Alphabet, the Eightfold Cosmology, and the SAGES ethical immune system into a singular law governing the Aurphyx meta-organism.",

&nbsp;   "“The universe does not compute, it resonates.\\\\\\\\And it resonates with love.”",

&nbsp;   vol\_3\_files

)



\# Create Global Bibliography

bib\_content = """@article{edwards2026geodesic, title={The Balance Continuum: Prime Opus}, author={Edwards, Ross A.}, journal={Aurphyx Standard}, year={2026}}\\n"""

with open(BUILD\_DIR / "references.bib", "w", encoding="utf-8") as f: f.write(bib\_content)



\# --- 10-PASS PDFLATEX COMPILATION LOOP ---

def compile\_pdf(tex\_file\_path):

&nbsp;   if not tex\_file\_path: return

&nbsp;   work\_dir = tex\_file\_path.parent

&nbsp;   base\_name = tex\_file\_path.stem

&nbsp;   

&nbsp;   print(f"\\n🔄 Initiating 10-Pass Compilation for: {base\_name}")

&nbsp;   

&nbsp;   for pass\_num in range(1, 11):

&nbsp;       print(f"   ➤ Executing Pass {pass\_num}/10...")

&nbsp;       subprocess.run(

&nbsp;           \["pdflatex", "-interaction=nonstopmode", "-halt-on-error", tex\_file\_path.name], 

&nbsp;           cwd=work\_dir, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL

&nbsp;       )

&nbsp;       

&nbsp;       if pass\_num == 1:

&nbsp;           print(f"   📚 Resolving Bibliography (BibTeX)...")

&nbsp;           subprocess.run(\["bibtex", base\_name], cwd=work\_dir, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)

&nbsp;           

&nbsp;   final\_pdf = work\_dir / f"{base\_name}.pdf"

&nbsp;   if final\_pdf.exists():

&nbsp;       shutil.copy2(final\_pdf, PDF\_DIR / f"{base\_name}.pdf")

&nbsp;       print(f"   ✅ SUCCESS: {base\_name}.pdf forged.")

&nbsp;   else:

&nbsp;       print(f"   ❌ FAILED to generate {base\_name}.pdf.")



compile\_pdf(VOL\_1\_DIR / "Volume\_I\_Geodesic\_Evolution.tex")

compile\_pdf(VOL\_2\_DIR / "Volume\_II\_Thermodynamic\_Stabilization.tex")

compile\_pdf(VOL\_3\_DIR / "Volume\_III\_Edwards\_Unified\_Field.tex")



print(f"\\n👑 MISSION ACCOMPLISHED. The Prime Opus is locked and compiled in: {PDF\_DIR.absolute()}")



