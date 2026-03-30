# Book of Fux — LaTeX Compilation Guide
    
    ## Requirements
    ```bash
    sudo apt install texlive-full
    # OR on Debian 13:
    sudo apt install texlive-xetex texlive-fonts-extra texlive-latex-extra
    ```
    
    ## Compile Command
    ```bash
    # Use XeLaTeX (required for fontspec/custom fonts)
    xelatex Book_of_Fux_Alien_Manuscript.tex
    xelatex Book_of_Fux_Alien_Manuscript.tex  # Run twice for TOC
    ```
    
    ## Output
    - `Book_of_Fux_Alien_Manuscript.pdf` — Full alien manuscript PDF
    
    ## Notes
    - Dark background requires XeLaTeX engine
    - Fuxyez syntax highlighting included
    - All 14 sections with alien manuscript styling
    - Custom neon cyan / alien purple color scheme
    