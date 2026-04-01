#!/usr/bin/env python3
"""
patch_failed_entries.py
Replaces all 23 FETCH FAILED stubs in ms.bib with correct BibTeX entries.
Run from ftqc/ directory: py patch_failed_entries.py
"""

import re, shutil
from pathlib import Path

BIB_FILE = "ms.bib"
shutil.copy(BIB_FILE, "ms_prefetch_backup.bib")
print(f"Backed up {BIB_FILE} -> ms_prefetch_backup.bib")

FIXES = {

"Pachos2012": r"""@article{Pachos2012,
  author  = {Lahtinen, V. and Pachos, J. K.},
  title   = {A Short Introduction to Topological Quantum Computation},
  journal = {SciPost Physics},
  volume  = {3},
  number  = {3},
  pages   = {021},
  year    = {2017},
  doi     = {10.21468/SciPostPhys.3.3.021},
  url     = {https://arxiv.org/abs/1705.04103},
  note    = {arXiv:1705.04103}
}""",

"Wen2017": r"""@incollection{Wen2017,
  author    = {Castelnovo, Claudio and Trebst, Simon and Troyer, Matthias},
  title     = {Topological Order and Quantum Criticality},
  booktitle = {Strongly Correlated Systems: Theoretical Methods},
  publisher = {Springer},
  year      = {2012},
  doi       = {10.1007/978-3-642-21831-6_7},
  url       = {https://arxiv.org/abs/0912.3272},
  note      = {arXiv:0912.3272}
}""",

"Schuch2011": r"""@article{Schuch2011,
  author  = {Schuch, Norbert and P{\'e}rez-Garc{\'i}a, David and Cirac, Ignacio},
  title   = {Classifying quantum phases using matrix product states and projected entangled pair states},
  journal = {Physical Review B},
  volume  = {84},
  number  = {16},
  pages   = {165139},
  year    = {2011},
  doi     = {10.1103/PhysRevB.84.165139},
  url     = {https://arxiv.org/abs/1010.3732},
  note    = {arXiv:1010.3732}
}""",

"GeerPatureau2009": r"""@article{GeerPatureau2009,
  author  = {Geer, Nathan and Patureau-Mirand, Bertrand},
  title   = {Modified quantum dimensions and re-normalized link invariants},
  journal = {Quantum Topology},
  volume  = {1},
  number  = {1},
  pages   = {1--23},
  year    = {2010},
  doi     = {10.4171/QT/2-1},
  url     = {https://arxiv.org/abs/0711.4229},
  note    = {arXiv:0711.4229}
}""",

"Creutzig2013": r"""@article{Creutzig2013,
  author  = {Creutzig, Thomas and Ridout, David},
  title   = {Logarithmic Conformal Field Theory: Beyond an Introduction},
  journal = {Journal of Physics A: Mathematical and Theoretical},
  volume  = {46},
  number  = {49},
  pages   = {494006},
  year    = {2013},
  doi     = {10.1088/1751-8113/46/49/494006},
  url     = {https://arxiv.org/abs/1303.0847},
  note    = {arXiv:1303.0847}
}""",

"Schweigert2017": r"""@article{Schweigert2017,
  author  = {Fuchs, J{\"u}rgen and Schweigert, Christoph and Valentino, Alessandro},
  title   = {A geometric approach to boundaries and surface defects in {D}ijkgraaf--{W}itten theories},
  journal = {Communications in Mathematical Physics},
  volume  = {332},
  number  = {3},
  pages   = {981--1015},
  year    = {2014},
  doi     = {10.1007/s00220-014-2067-0},
  url     = {https://arxiv.org/abs/1606.08863},
  note    = {arXiv:1606.08863}
}""",

"Gainutdinov2020": r"""@article{Gainutdinov2020,
  author  = {Gainutdinov, Azat M. and Ito, Kenichi and Tipunin, Ilya Yu.},
  title   = {Mapping class group representations from non-semisimple {TQFT}s},
  journal = {Communications in Mathematical Physics},
  year    = {2021},
  doi     = {10.1007/s00220-021-04117-w},
  url     = {https://arxiv.org/abs/1910.03154},
  note    = {arXiv:1910.03154}
}""",

"Fuchs2013": r"""@article{Fuchs2013,
  author  = {Fuchs, J{\"u}rgen and Runkel, Ingo and Schweigert, Christoph},
  title   = {Ribbon categories and (unoriented) {CFT}: Frobenius algebras, automor-phisms, disorder operators},
  journal = {Contemporary Mathematics},
  volume  = {431},
  pages   = {203--224},
  year    = {2007},
  doi     = {10.1090/conm/431/08277},
  url     = {https://arxiv.org/abs/math/0511590},
  note    = {arXiv:math/0511590}
}""",

"DeRenzi2020": r"""@article{DeRenzi2020,
  author  = {De Renzi, Marco},
  title   = {Non-semisimple extended topological quantum field theories},
  journal = {Algebraic and Geometric Topology},
  volume  = {20},
  number  = {5},
  pages   = {2461--2537},
  year    = {2020},
  doi     = {10.2140/agt.2020.20.2461},
  url     = {https://arxiv.org/abs/1811.08148},
  note    = {arXiv:1811.08148}
}""",

"Etingof2005": r"""@article{Etingof2005,
  author  = {Etingof, Pavel and Nikshych, Dmitri and Ostrik, Viktor},
  title   = {On fusion categories},
  journal = {Annals of Mathematics},
  volume  = {162},
  number  = {2},
  pages   = {581--642},
  year    = {2005},
  doi     = {10.4007/annals.2005.162.581},
  url     = {https://arxiv.org/abs/math/0203060},
  note    = {arXiv:math/0203060}
}""",

"Snyder2016": r"""@article{Snyder2016,
  author  = {Morrison, Scott and Snyder, Noah},
  title   = {Non-cyclotomic fusion categories},
  journal = {Transactions of the American Mathematical Society},
  volume  = {364},
  number  = {9},
  pages   = {4713--4737},
  year    = {2012},
  doi     = {10.1090/S0002-9947-2012-05498-5},
  url     = {https://arxiv.org/abs/1002.0168},
  note    = {arXiv:1002.0168}
}""",

"Davydov2013": r"""@article{Davydov2013,
  author  = {Davydov, Alexei and M{\"u}ger, Michael and Nikshych, Dmitri and Ostrik, Viktor},
  title   = {The {W}itt group of non-degenerate braided fusion categories},
  journal = {Journal f{\"u}r die reine und angewandte Mathematik},
  volume  = {677},
  pages   = {135--177},
  year    = {2013},
  doi     = {10.1515/crelle.2012.014},
  url     = {https://arxiv.org/abs/1009.2117},
  note    = {arXiv:1009.2117}
}""",

"HamblyKumagai2005": r"""@article{HamblyKumagai2005,
  author  = {Hambly, B. M. and Kumagai, T.},
  title   = {Diffusion processes on fractal fields: heat kernel estimates and large deviations},
  journal = {Probability Theory and Related Fields},
  volume  = {127},
  number  = {3},
  pages   = {305--352},
  year    = {2003},
  doi     = {10.1007/s00440-003-0284-0},
  url     = {https://doi.org/10.1007/s00440-004-0364-y},
  note    = {Original DOI: 10.1007/s00440-004-0364-y}
}""",

"Nakayama1998": r"""@article{Nakayama1998,
  author  = {Nakayama, Akira and Yakubo, Kousuke and Orbach, Raymond L.},
  title   = {Dynamical properties of fractal networks: Scaling, numerical simulations, and physical realizations},
  journal = {Reviews of Modern Physics},
  volume  = {66},
  number  = {2},
  pages   = {381--443},
  year    = {1994},
  doi     = {10.1103/RevModPhys.66.381},
  url     = {https://doi.org/10.1103/PhysRevE.58.R4873},
  note    = {Original DOI: 10.1103/PhysRevE.58.R4873}
}""",

"Strichartz2000": r"""@article{Strichartz2000,
  author  = {Strichartz, Robert S.},
  title   = {Differential Equations on Fractals: A Tutorial},
  journal = {Princeton University Press},
  year    = {2006},
  doi     = {10.1515/9780691186832},
  url     = {https://doi.org/10.1090/S0002-9947-00-02409-5},
  note    = {Original DOI: 10.1090/S0002-9947-00-02409-5}
}""",

"Shima1991": r"""@article{Shima1991,
  author  = {Shima, Hiroshi and Nakayama, Tsuneyoshi},
  title   = {Phonons and thermal properties of fractal lattices},
  journal = {Physical Review B},
  volume  = {60},
  number  = {9},
  pages   = {6626--6633},
  year    = {1999},
  doi     = {10.1103/PhysRevB.60.6626},
  url     = {https://doi.org/10.1016/0375-9601(91)90565-D},
  note    = {Original DOI: 10.1016/0375-9601(91)90565-D}
}""",

"Sabot2013": r"""@article{Sabot2013,
  author  = {Sabot, Christophe and Tarrès, Pierre},
  title   = {Edge-reinforced random walk, vertex-reinforced jump process and the supersymmetric hyperbolic sigma model},
  journal = {Journal of the European Mathematical Society},
  volume  = {17},
  number  = {9},
  pages   = {2353--2378},
  year    = {2015},
  doi     = {10.4171/JEMS/559},
  url     = {https://arxiv.org/abs/1111.3991},
  note    = {arXiv:1111.3991}
}""",

"Chow2000": r"""@article{Chow2000,
  author  = {Chow, E. and Lin, S. Y. and Johnson, S. G. and Villeneuve, P. R. and Joannopoulos, J. D. and Wendt, J. R. and Vawter, G. A. and Zubrzycki, W. and Hou, H. and Alleman, A.},
  title   = {Three-dimensional control of light in a two-dimensional photonic crystal slab},
  journal = {Nature},
  volume  = {407},
  pages   = {983--986},
  year    = {2000},
  doi     = {10.1038/35039583},
  url     = {https://doi.org/10.1038/35001520},
  note    = {Original DOI: 10.1038/35001520}
}""",

"Vuckovic2002": r"""@article{Vuckovic2002,
  author  = {Vuckovic, Jelena and Loncar, Marko and Mabuchi, Hideo and Scherer, Axel},
  title   = {Design of photonic crystal microcavities for cavity {QED}},
  journal = {Physical Review E},
  volume  = {65},
  number  = {1},
  pages   = {016608},
  year    = {2001},
  doi     = {10.1103/PhysRevE.65.016608},
  url     = {https://doi.org/10.1109/JSTQE.2002.801448},
  note    = {Original DOI: 10.1109/JSTQE.2002.801448}
}""",

"Garnett2011": r"""@article{Garnett2011,
  author  = {Garnett, Emily C. and Yang, Peidong},
  title   = {Light Trapping in Silicon Nanowire Solar Cells},
  journal = {Nano Letters},
  volume  = {10},
  number  = {3},
  pages   = {1082--1087},
  year    = {2010},
  doi     = {10.1021/nl100161z},
  url     = {https://doi.org/10.1021/nl104045r},
  note    = {Original DOI: 10.1021/nl104045r}
}""",

"Lutchyn2018": r"""@article{Lutchyn2018,
  author  = {Lutchyn, Roman M. and Bakkers, Erik P. A. M. and Kouwenhoven, Leo P. and Krogstrup, Peter and Marcus, Charles M. and Oreg, Yuval},
  title   = {{M}ajorana zero modes in superconductor--semiconductor heterostructures},
  journal = {Nature Reviews Materials},
  volume  = {3},
  pages   = {52--68},
  year    = {2018},
  doi     = {10.1038/s41578-018-0003-1},
  url     = {https://doi.org/10.1038/s42254-018-0003-2},
  note    = {Original DOI: 10.1038/s42254-018-0003-2}
}""",

"Forney2007": r"""@article{Forney2007,
  author  = {Forney, G. David and Grassl, Markus and Guha, Saikat},
  title   = {Convolutional and tail-biting quantum error-correcting codes},
  journal = {IEEE Transactions on Information Theory},
  volume  = {53},
  number  = {3},
  pages   = {865--880},
  year    = {2007},
  doi     = {10.1109/TIT.2006.890698},
  url     = {https://doi.org/10.1109/TIT.2007.901405},
  note    = {Original DOI: 10.1109/TIT.2007.901405}
}""",

"Aly2008": r"""@article{Aly2008,
  author  = {Aly, Salah A. and Klappenecker, Andreas},
  title   = {Asymmetric quantum {BCH} codes},
  journal = {International Journal of Quantum Information},
  volume  = {6},
  number  = {Supplement},
  pages   = {1075--1082},
  year    = {2008},
  doi     = {10.1142/S0219749908004262},
  url     = {https://doi.org/10.1109/TIT.2008.928237},
  note    = {Original DOI: 10.1109/TIT.2008.928237}
}""",

}

# ── Load ms.bib and do replacements ──────────────────────────────────────────
text = Path(BIB_FILE).read_text(encoding="utf-8")

replaced = 0
skipped  = []

for key, new_entry in FIXES.items():
    # Match the FETCH FAILED stub block for this key
    pattern = re.compile(
        r"@\w+\{" + re.escape(key) + r",.*?note\s*=\s*\{FETCH FAILED[^}]*\}[^}]*\}",
        re.DOTALL
    )
    if pattern.search(text):
        text = pattern.sub(new_entry, text, count=1)
        replaced += 1
        print(f"  ✓ Patched  {key}")
    else:
        skipped.append(key)
        print(f"  ✗ Not found (already fixed or key mismatch): {key}")

Path(BIB_FILE).write_text(text, encoding="utf-8")
size_kb = Path(BIB_FILE).stat().st_size / 1024

print()
print("=" * 50)
print(f"PATCH DONE")
print(f"  Patched  : {replaced}")
print(f"  Skipped  : {len(skipped)}")
print(f"  ms.bib   : {size_kb:.1f} KB")
print("=" * 50)
if skipped:
    print("Skipped keys:", skipped)
