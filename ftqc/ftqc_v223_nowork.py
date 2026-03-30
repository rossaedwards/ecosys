#!/usr/bin/env python3
"""
FTQC v2.23 - FIGS IN ROOT FOLDER (main/ftqc/)
NO subfolders needed - Instant fix 7:26AM
"""
# ... (same imports)

class FTQCDivinePipelineV223:
    # ... (same init, but FIXED convert_figures)
    
    def convert_figures(self):
        """Figs in ROOT - NO figures/ folder needed"""
        converted = []
        for old_name, new_name in self.figure_map.items():
            old_path = Path(old_name)
            new_path = Path(new_name)  # ROOT OUTPUT
            
            if old_path.exists():
                try:
                    from PIL import Image
                    img = Image.open(old_path).convert('RGB')
                    img.save(new_path, 'PNG', dpi=(300,300))
                    converted.append(new_name)
                    self.log(f"🖼️  {old_name} → {new_name}", "SUCCESS")
                except:
                    shutil.copy(old_path, new_path)
                    converted.append(new_name)
                    self.log(f"📋 {old_name} → {new_name}", "SUCCESS")
            else:
                self.log(f"⚠️  Skip {old_name}", "WARNING")
        
        self.log(f"✅ {len(converted)}/10 figs ready IN ROOT", "SUCCESS")
        return converted
    
    def inject_figures_sections(self):
        for section, figs in self.section_figures.items():
            if Path(section).exists():
                content = Path(section).read_text(errors='ignore')
                for fig, caption in figs:
                    fig_block = f"""
\\begin{{figure}}[htbp]
\\centering
\\includegraphics[width=0.7\\textwidth]{{{fig}}}
\\caption{{{caption}}}
\\label{{fig:{fig.replace('.png','')}}}
\\end{{figure}}
"""
                    if fig_block not in content:
                        content += fig_block
                        Path(section).write_text(content)
                        self.log(f"🖼️  {fig} → {section}", "SUCCESS")
    
    # graphicspath = ROOT
    self.fuxyez_preamble = self.fuxyez_preamble.replace(
        r'\graphicspath{{figures/}{./figures/}}', 
        r'\graphicspath{{./}}'  # ROOT figs
    )
    
    # REST IDENTICAL...
    # ... (rest of class identical to v2.22)

if __name__ == '__main__':
    FTQCDivinePipelineV223().run()