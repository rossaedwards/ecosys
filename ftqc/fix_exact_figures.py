# ORCID: 0009-0008-0539-1289
import os, re
import glob

# The exact files available are Fig1.png through Fig11.png.
# We map any weird name to one of these 11.
mapping = {
    'Fig4_TRCA_Band_Structure': 'Fig5_Band_Structure',
    'Fig7_AuraFS_Sierpinski': 'Fig7_Device_Cross_Section',
    'Fig5_Majorana_T_Shape': 'Fig8_Majorana_T_Junction',
    'Fig6_ZPE_Majorana_Stability': 'Fig6_Neglecton_Braiding',
    'Fig8_Fractal_Lattice_Sim': 'Fig11_Fractal_Lattice_Sim',
    'Fig1_Hilbert_Scaling_080646': 'Fig1_Hilbert_Scaling',
    'Fig3_Fractal_Localization_080647': 'Fig3_Anderson_Localization',
    'Fig2_Coherence_Dynamics_080647': 'Fig2_Coherence_Dynamics',
    'Fig3_Fractal_Localization': 'Fig3_Anderson_Localization'
}

for tex_file in glob.glob('*.tex') + glob.glob('arxiv_submission/*.tex'):
    with open(tex_file, 'r', encoding='utf-8') as f:
        content = f.read()
    
    dirty = False
    for old, new in mapping.items():
        if old in content:
            content = content.replace(old, new)
            dirty = True
            
    if dirty:
        with open(tex_file, 'w', encoding='utf-8') as f:
            f.write(content)

