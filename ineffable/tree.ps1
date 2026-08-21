#!/usr/bin/env pwsh
param([string]$Path = ".")

function Show-Tree {
    param([string]$Path = ".")
    
    $root = Get-Item -LiteralPath $Path
    $rootName = Split-Path $root.FullName -Leaf  # Get folder name for filename
    $outputFile = "$rootName_repo.txt"           # e.g., "aurafs_repo.txt"
    
    Write-Host "💎 Scanning $rootName..." -ForegroundColor Cyan
    
    # Print tree to console
    Write-Output $root.FullName
    $treeOutput = @()
    $treeOutput += $root.FullName
    
    Get-ChildItem -LiteralPath $root.FullName -Recurse -Force | 
        Where-Object { 
            -not $_.Attributes.HasFlag([IO.FileAttributes]::Hidden) -and
            -not $_.Attributes.HasFlag([IO.FileAttributes]::System) -and
            $_.Name -ne "target" -and
            $_.Name -ne "*.git*" -and
            $_.Name -ne "node_modules"
        } | ForEach-Object { 
            $relative = $_.FullName.Substring($root.FullName.Length).TrimStart('\')
            $depth = ($relative.Split('\').Count - 1)
            $prefix = if ($depth -gt 0) { 
                ('│   ' * ($depth - 1)) + '├── ' 
            } else { '' }
            $line = "$prefix$($_.Name)"
            Write-Output $line
            $treeOutput += $line
        }
    
    # Save to PERFECTLY-NAMED file in current directory
    $treeOutput | Out-File -FilePath $outputFile -Encoding UTF8
    Write-Host "`n🚀 $rootName repo tree saved to '$outputFile' 🚀" -ForegroundColor Green
    Write-Host "📁 Location: $($PWD.Path)\$outputFile" -ForegroundColor Yellow
}

# Auto-run in current directory
Show-Tree $Path