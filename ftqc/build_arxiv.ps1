##################################################################
## ARXIV BUILD SCRIPT FOR FTQC THESIS
## Author: Ross A. Edwards
## Purpose: Compile rae-ftqc_arxiv_complete.tex with full validation
## Platform: Windows 11 + TeX Live 2025
##################################################################

param(
    [ValidateSet("arxiv", "prx")]
    [string]$Style = "arxiv",

    [ValidateSet("FINAL", "UPDATED")]
    [string]$Variant = "FINAL"
)

# Set error handling
$ErrorActionPreference = "Continue"

# Configuration
$ProjectRoot = "C:\rossaedwards\main\ftqc"
$FiguresDir = "figures"

switch ($Style) {
    "arxiv" {
        $MainTexBase = "rae-ftqc_arxiv_complete"
        $BibFile = "arxiv_ftqc"
        $SubmissionLabel = "arXiv"
        $UploadUrl = "https://arxiv.org/submit"
    }
    "prx" {
        $MainTexBase = "rae-ftqc_prx_submission"
        $BibFile = "prx_ftqc"
        $SubmissionLabel = "PRX"
        $UploadUrl = "https://journals.aps.org/authors/revtex"
    }
}

$MainTex = "${MainTexBase}_${Variant}"
$OutputDir = "${Style}_submission_ready_${Variant}".ToLower()

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "$SubmissionLabel FTQC Thesis Compilation Starting" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# Change to project directory
Set-Location $ProjectRoot
Write-Host "[INFO] Working directory: $ProjectRoot" -ForegroundColor Green
Write-Host "[INFO] Build style: $SubmissionLabel ($Style)" -ForegroundColor Green
Write-Host "[INFO] Variant: $Variant" -ForegroundColor Green
Write-Host "[INFO] Main source: $MainTex.tex" -ForegroundColor Green
Write-Host "[INFO] Bibliography base: $BibFile.bib" -ForegroundColor Green

# Verify essential files exist
Write-Host "`n[STEP 1] Verifying required files..." -ForegroundColor Yellow

$RequiredFiles = @(
    "$MainTex.tex",
    "compile_citations.py"
)

$MissingFiles = @()
foreach ($file in $RequiredFiles) {
    if (-not (Test-Path $file)) {
        $MissingFiles += $file
        Write-Host "  [ERROR] MISSING: $file" -ForegroundColor Red
    } else {
        Write-Host "  [OK] Found: $file" -ForegroundColor Green
    }
}

if ($MissingFiles.Count -gt 0) {
    Write-Host "`n[ERROR] Cannot proceed. Missing required files." -ForegroundColor Red
    exit 1
}

if (-not (Test-Path "citations")) {
    Write-Host "  [ERROR] MISSING: citations directory" -ForegroundColor Red
    exit 1
} else {
    Write-Host "  [OK] Found: citations directory" -ForegroundColor Green
}

# Verify figures directory
if (-not (Test-Path $FiguresDir)) {
    Write-Host "  [ERROR] MISSING: $FiguresDir directory" -ForegroundColor Red
    exit 1
} else {
    $FigCount = (Get-ChildItem "$FiguresDir\*.png" -File).Count
    Write-Host "  [OK] Found: $FiguresDir directory with $FigCount PNG files" -ForegroundColor Green
}

# Clean previous build artifacts
Write-Host "`n[STEP 2] Cleaning previous build artifacts..." -ForegroundColor Yellow

$CleanExtensions = @("*.aux", "*.bbl", "*.blg", "*.log", "*.out", "*.toc", "*.lof", "*.lot", "*.fls", "*.fdb_latexmk", "*.synctex.gz")
foreach ($pattern in $CleanExtensions) {
    $files = Get-ChildItem $pattern -File -ErrorAction SilentlyContinue
    if ($files) {
        Remove-Item $files -Force
        Write-Host "  [OK] Removed: $pattern files" -ForegroundColor Gray
    }
}

Write-Host "  [OK] Build environment cleaned" -ForegroundColor Green

# Citation pre-build: regenerate merged bibliography from citations sources.
Write-Host "`n[CITATIONS] Compiling merged bibliography from citations/..." -ForegroundColor Yellow
$CitationCmd = Get-Command py -ErrorAction SilentlyContinue
if ($CitationCmd) {
    & py "compile_citations.py"
} else {
    & python "compile_citations.py"
}
$CitationExitCode = $LASTEXITCODE

if ($CitationExitCode -ne 0) {
    Write-Host "  [ERROR] Citation compile failed with exit code $CitationExitCode" -ForegroundColor Red
    exit 1
}

if (-not (Test-Path "$BibFile.bib")) {
    if ($Style -eq "prx" -and (Test-Path "arxiv_ftqc.bib")) {
        Copy-Item "arxiv_ftqc.bib" -Destination "$BibFile.bib" -Force
        Write-Host "  [OK] Created $BibFile.bib from merged arXiv bibliography output" -ForegroundColor Green
    } else {
        Write-Host "  [ERROR] Citation compile did not generate $BibFile.bib" -ForegroundColor Red
        exit 1
    }
}

Write-Host "  [OK] Citation merge complete: $BibFile.bib refreshed from citations/" -ForegroundColor Green

# PASS 1: Initial LaTeX compilation
Write-Host "`n[STEP 3] Pass 1/4 - Initial pdflatex compilation..." -ForegroundColor Yellow
Write-Host "  This generates auxiliary files (.aux) for bibliography processing." -ForegroundColor Gray

$Pass1 = pdflatex -interaction=nonstopmode "$MainTex.tex" 2>&1
$Pass1ExitCode = $LASTEXITCODE

if ($Pass1ExitCode -ne 0) {
    Write-Host "  [ERROR] Pass 1 failed with exit code $Pass1ExitCode" -ForegroundColor Red
    Write-Host "`n[ERROR LOG EXCERPT]" -ForegroundColor Red
    $Pass1 | Select-String -Pattern "^!" | Select-Object -First 10 | ForEach-Object { Write-Host $_ -ForegroundColor Red }
    exit 1
}

# Check for critical errors in log
$LogContent = Get-Content "$MainTex.log" -Raw
if ($LogContent -match "^!") {
    Write-Host "  [WARN] Critical errors detected in log file" -ForegroundColor Yellow
    $LogContent | Select-String -Pattern "^!" | Select-Object -First 5 | ForEach-Object { Write-Host "    $_" -ForegroundColor Red }
} else {
    Write-Host "  [OK] Pass 1 completed successfully" -ForegroundColor Green
}

# PASS 2: BibTeX processing
Write-Host "`n[STEP 4] Pass 2/4 - BibTeX bibliography processing..." -ForegroundColor Yellow
Write-Host "  This resolves all citation references from $BibFile.bib" -ForegroundColor Gray

$Pass2 = bibtex "$MainTex" 2>&1
$Pass2ExitCode = $LASTEXITCODE

if ($Pass2ExitCode -ne 0) {
    Write-Host "  [ERROR] BibTeX failed with exit code $Pass2ExitCode" -ForegroundColor Red
    Write-Host "`n[BIBTEX ERROR LOG]" -ForegroundColor Red
    Get-Content "$MainTex.blg" | Select-String -Pattern "(error|warning)" -Context 1,1 | ForEach-Object { Write-Host $_ -ForegroundColor Red }
    exit 1
}

# Verify .bbl file was created
if (-not (Test-Path "$MainTex.bbl")) {
    Write-Host "  [ERROR] BibTeX did not generate .bbl file" -ForegroundColor Red
    exit 1
}

# Check for BibTeX warnings
$BlgContent = Get-Content "$MainTex.blg" -Raw
if ($BlgContent -match "Warning") {
    Write-Host "  [WARN] BibTeX warnings detected:" -ForegroundColor Yellow
    $BlgContent | Select-String -Pattern "Warning--" | Select-Object -First 5 | ForEach-Object { Write-Host "    $_" -ForegroundColor Yellow }
} else {
    Write-Host "  [OK] BibTeX completed without warnings" -ForegroundColor Green
}

# Count bibliography entries
$BblContent = Get-Content "$MainTex.bbl" -Raw
$BibItemCount = ([regex]::Matches($BblContent, "\\bibitem")).Count
Write-Host "  [OK] Bibliography contains $BibItemCount entries" -ForegroundColor Green

# PASS 3: Second LaTeX compilation
Write-Host "`n[STEP 5] Pass 3/4 - Second pdflatex (integrate bibliography)..." -ForegroundColor Yellow
Write-Host "  This incorporates bibliography into the document." -ForegroundColor Gray

$Pass3 = pdflatex -interaction=nonstopmode "$MainTex.tex" 2>&1
$Pass3ExitCode = $LASTEXITCODE

if ($Pass3ExitCode -ne 0) {
    Write-Host "  [ERROR] Pass 3 failed with exit code $Pass3ExitCode" -ForegroundColor Red
    exit 1
}

Write-Host "  [OK] Pass 3 completed" -ForegroundColor Green

# PASS 4: Final LaTeX compilation
Write-Host "`n[STEP 6] Pass 4/4 - Final pdflatex (resolve cross-references)..." -ForegroundColor Yellow
Write-Host "  This resolves all internal cross-references and finalizes the PDF." -ForegroundColor Gray

$Pass4 = pdflatex -interaction=nonstopmode "$MainTex.tex" 2>&1
$Pass4ExitCode = $LASTEXITCODE

if ($Pass4ExitCode -ne 0) {
    Write-Host "  [ERROR] Pass 4 failed with exit code $Pass4ExitCode" -ForegroundColor Red
    exit 1
}

Write-Host "  [OK] Pass 4 completed" -ForegroundColor Green

# Verify PDF was created
if (-not (Test-Path "$MainTex.pdf")) {
    Write-Host "`n[ERROR] PDF was not generated despite successful compilation." -ForegroundColor Red
    exit 1
}

# PDF validation
Write-Host "`n[STEP 7] Validating output PDF..." -ForegroundColor Yellow

$PdfSize = (Get-Item "$MainTex.pdf").Length / 1MB
Write-Host "  [OK] PDF generated: $MainTex.pdf ($([math]::Round($PdfSize, 2)) MB)" -ForegroundColor Green

# Check for undefined references in log
$UndefRefs = $LogContent | Select-String -Pattern "Reference.*undefined" | Measure-Object
if ($UndefRefs.Count -gt 0) {
    Write-Host "  [WARN] $($UndefRefs.Count) undefined references detected" -ForegroundColor Yellow
    Write-Host "    Check the .log file for details: $MainTex.log" -ForegroundColor Yellow
} else {
    Write-Host "  [OK] All references resolved" -ForegroundColor Green
}

# Check for undefined citations
$UndefCites = $LogContent | Select-String -Pattern "Citation.*undefined" | Measure-Object
if ($UndefCites.Count -gt 0) {
    Write-Host "  [WARN] $($UndefCites.Count) undefined citations detected" -ForegroundColor Yellow
} else {
    Write-Host "  [OK] All citations resolved" -ForegroundColor Green
}

# Check page count
$PageMatches = $LogContent | Select-String -Pattern "Output written.*\((\d+) pages" | Select-Object -First 1
if ($PageMatches) {
    $PageCount = $PageMatches.Matches.Groups[1].Value
    Write-Host "  [OK] Document is $PageCount pages" -ForegroundColor Green
    
    if ([int]$PageCount -lt 35 -or [int]$PageCount -gt 45) {
        Write-Host "    [WARN] Expected ~40 pages, got $PageCount - verify content completeness" -ForegroundColor Yellow
    }
}

# Create submission package
Write-Host "`n[STEP 8] Creating $SubmissionLabel submission-ready package..." -ForegroundColor Yellow

if (Test-Path $OutputDir) {
    Remove-Item $OutputDir -Recurse -Force
    Write-Host "  [OK] Cleaned existing submission directory" -ForegroundColor Gray
}

New-Item -ItemType Directory -Path $OutputDir -Force | Out-Null

# Copy required files
Copy-Item "$MainTex.tex" -Destination $OutputDir
Copy-Item "$MainTex.bbl" -Destination $OutputDir
Copy-Item "$BibFile.bib" -Destination $OutputDir
Write-Host "  [OK] Copied LaTeX and bibliography files" -ForegroundColor Green

# Copy all figures
Copy-Item "$FiguresDir\*.png" -Destination $OutputDir
$CopiedFigs = (Get-ChildItem "$OutputDir\*.png").Count
Write-Host "  [OK] Copied $CopiedFigs figure files" -ForegroundColor Green

# Copy section files if they exist
$SectionFiles = Get-ChildItem "section_*.tex" -File -ErrorAction SilentlyContinue
if ($SectionFiles) {
    Copy-Item $SectionFiles -Destination $OutputDir
    Write-Host "  [OK] Copied section files" -ForegroundColor Green
}

# Create tarball (requires tar.exe on Windows 10/11)
Set-Location $OutputDir
$TarballName = "rae_ftqc_${Style}_${Variant}_submission_$(Get-Date -Format 'yyyyMMdd').tar.gz".ToLower()

if (Get-Command tar -ErrorAction SilentlyContinue) {
    tar -czf "..\$TarballName" *
    Write-Host "  [OK] Created submission tarball: $TarballName" -ForegroundColor Green
} else {
    Write-Host "  [WARN] tar.exe not found - tarball not created" -ForegroundColor Yellow
    Write-Host "    You can manually create a .tar.gz archive from the $OutputDir directory" -ForegroundColor Yellow
}

Set-Location $ProjectRoot

# Final summary
Write-Host "`n========================================" -ForegroundColor Cyan
Write-Host "COMPILATION SUMMARY" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "Status:           SUCCESS" -ForegroundColor Green
Write-Host "Submission style: $SubmissionLabel ($Style)" -ForegroundColor White
Write-Host "Variant:          $Variant" -ForegroundColor White
Write-Host "Output PDF:       $MainTex.pdf" -ForegroundColor White
Write-Host "Bibliography:     $BibItemCount entries from $BibFile.bib" -ForegroundColor White
Write-Host "Page count:       $PageCount pages" -ForegroundColor White
Write-Host "Submission pkg:   $OutputDir\" -ForegroundColor White
if (Get-Command tar -ErrorAction SilentlyContinue) {
    Write-Host "Tarball:          $TarballName" -ForegroundColor White
}
Write-Host "`nNext steps:" -ForegroundColor Yellow
Write-Host "  1. Review the generated PDF: $MainTex.pdf" -ForegroundColor White
Write-Host "  2. Verify all figures render correctly" -ForegroundColor White
Write-Host "  3. Check that all citations appear (no [?] marks)" -ForegroundColor White
Write-Host "  4. Upload/package guide: $UploadUrl" -ForegroundColor White
Write-Host "========================================`n" -ForegroundColor Cyan