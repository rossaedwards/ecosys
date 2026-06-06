"""
memoree_fs_server.py
Windows-native MCP filesystem tools for Memoree
PowerShell-backed, path-scoped, JSON-returning
f0rg3d in l0v3 by Ross Edwards
"""

from __future__ import annotations

import json
import subprocess
from pathlib import Path
from typing import Any, Dict, List, Optional

ALLOWED_DIRS = [
    Path(r"d:\rossaedwards\main").resolve(),
    Path(r"d:\aurphyx\main").resolve(),
]

OS_CONTEXT = {
    "platform": "win11home",
    "backend": "powershell",
    "version": "7.4"
}


def _norm_path(raw: str) -> Path:
    p = Path(raw).expanduser().resolve()
    return p


def _is_allowed(path: Path) -> bool:
    path_str = str(path).lower()
    for allowed in ALLOWED_DIRS:
        allowed_str = str(allowed).lower()
        if path_str == allowed_str or path_str.startswith(allowed_str + "\\"):
            return True
    return False


def _guard_path(raw: str) -> Path:
    path = _norm_path(raw)
    if not _is_allowed(path):
        raise ValueError(f"Path not allowed: {path}")
    return path


def _ps(script: str) -> Any:
    cmd = [
        "powershell",
        "-NoProfile",
        "-ExecutionPolicy", "Bypass",
        "-Command",
        script,
    ]
    proc = subprocess.run(cmd, capture_output=True, text=True, encoding="utf-8")
    if proc.returncode != 0:
        raise RuntimeError(proc.stderr.strip() or "PowerShell command failed")
    stdout = proc.stdout.strip()
    if not stdout:
        return None
    try:
        return json.loads(stdout)
    except json.JSONDecodeError:
        return stdout


def _json_result(tool: str, data: Any, ok: bool = True) -> Dict[str, Any]:
    return {
        "ok": ok,
        "tool": tool,
        "os_context": OS_CONTEXT,
        "data": data,
    }


def list_allowed_directories() -> Dict[str, Any]:
    return _json_result(
        "memoree_list_allowed_directories",
        [str(p) for p in ALLOWED_DIRS]
    )


def os_ping() -> Dict[str, Any]:
    script = r"""
$os = Get-CimInstance Win32_OperatingSystem | Select-Object Caption, Version, OSArchitecture
$ps = $PSVersionTable | Select-Object PSVersion, PSEdition, Platform
[pscustomobject]@{
  os = $os
  powershell = $ps
} | ConvertTo-Json -Depth 6
"""
    return _json_result("memoree_os_ping", _ps(script))


def list_directory(path: str) -> Dict[str, Any]:
    safe = _guard_path(path)
    script = rf"""
Get-ChildItem -LiteralPath '{safe}' -Force |
Select-Object Name, FullName, Exists, Extension, Length, Mode, CreationTime, LastWriteTime, PSIsContainer |
ConvertTo-Json -Depth 4
"""
    return _json_result("memoree_list_directory", _ps(script))


def directory_tree(path: str, depth: int = 3) -> Dict[str, Any]:
    safe = _guard_path(path)
    depth = max(1, min(depth, 8))
    script = rf"""
Get-ChildItem -LiteralPath '{safe}' -Recurse -Depth {depth} -Force |
Select-Object FullName, Name, PSIsContainer, Extension, Length, LastWriteTime |
ConvertTo-Json -Depth 4
"""
    return _json_result("memoree_directory_tree", _ps(script))


def read_file(path: str, encoding: str = "utf8") -> Dict[str, Any]:
    safe = _guard_path(path)
    script = rf"""
Get-Content -LiteralPath '{safe}' -Raw -Encoding {encoding} |
ConvertTo-Json -Depth 2
"""
    return _json_result("memoree_read_file", _ps(script))


def read_multiple_files(paths: List[str], encoding: str = "utf8") -> Dict[str, Any]:
    results = []
    for raw in paths:
        safe = _guard_path(raw)
        script = rf"""
[pscustomobject]@{{
  path = '{safe}'
  content = Get-Content -LiteralPath '{safe}' -Raw -Encoding {encoding}
}} | ConvertTo-Json -Depth 3
"""
        results.append(_ps(script))
    return _json_result("memoree_read_multiple_files", results)


def write_file(path: str, content: str, encoding: str = "utf8") -> Dict[str, Any]:
    safe = _guard_path(path)
    parent = safe.parent
    if not _is_allowed(parent):
        raise ValueError(f"Parent path not allowed: {parent}")
    script = rf"""
$parent = Split-Path -Parent '{safe}'
if (-not (Test-Path -LiteralPath $parent)) {{
  New-Item -ItemType Directory -Path $parent -Force | Out-Null
}}
Set-Content -LiteralPath '{safe}' -Value @'
{content}
'@ -Encoding {encoding}
[pscustomobject]@{{
  path = '{safe}'
  written = $true
}} | ConvertTo-Json -Depth 3
"""
    return _json_result("memoree_write_file", _ps(script))


def edit_file(path: str, find_text: str, replace_text: str, encoding: str = "utf8") -> Dict[str, Any]:
    safe = _guard_path(path)
    script = rf"""
$content = Get-Content -LiteralPath '{safe}' -Raw -Encoding {encoding}
$updated = $content.Replace(@'
{find_text}
'@, @'
{replace_text}
'@)
Set-Content -LiteralPath '{safe}' -Value $updated -Encoding {encoding}
[pscustomobject]@{{
  path = '{safe}'
  edited = $true
}} | ConvertTo-Json -Depth 3
"""
    return _json_result("memoree_edit_file", _ps(script))


def create_directory(path: str) -> Dict[str, Any]:
    safe = _guard_path(path)
    script = rf"""
New-Item -ItemType Directory -Path '{safe}' -Force |
Select-Object FullName, Exists |
ConvertTo-Json -Depth 3
"""
    return _json_result("memoree_create_directory", _ps(script))


def move_file(src: str, dst: str) -> Dict[str, Any]:
    safe_src = _guard_path(src)
    safe_dst = _guard_path(dst)
    script = rf"""
$parent = Split-Path -Parent '{safe_dst}'
if (-not (Test-Path -LiteralPath $parent)) {{
  New-Item -ItemType Directory -Path $parent -Force | Out-Null
}}
Move-Item -LiteralPath '{safe_src}' -Destination '{safe_dst}' -Force
[pscustomobject]@{{
  from = '{safe_src}'
  to = '{safe_dst}'
  moved = $true
}} | ConvertTo-Json -Depth 3
"""
    return _json_result("memoree_move_file", _ps(script))


def get_file_info(path: str) -> Dict[str, Any]:
    safe = _guard_path(path)
    script = rf"""
Get-Item -LiteralPath '{safe}' -Force |
Select-Object FullName, Name, Exists, Extension, Length, Mode, CreationTime, LastAccessTime, LastWriteTime, Attributes |
ConvertTo-Json -Depth 4
"""
    return _json_result("memoree_get_file_info", _ps(script))


def search_files(path: str, pattern: str, recurse: bool = True) -> Dict[str, Any]:
    safe = _guard_path(path)
    recurse_flag = "-Recurse" if recurse else ""
    script = rf"""
Get-ChildItem -LiteralPath '{safe}' -Force {recurse_flag} -File -Filter '{pattern}' |
Select-Object Name, FullName, Length, LastWriteTime |
ConvertTo-Json -Depth 4
"""
    return _json_result("memoree_search_files", _ps(script))


if __name__ == "__main__":
    print(json.dumps({
        "name": "memoree-filesystem",
        "tools": [
            "memoree_os_ping",
            "memoree_list_allowed_directories",
            "memoree_list_directory",
            "memoree_directory_tree",
            "memoree_read_file",
            "memoree_read_multiple_files",
            "memoree_write_file",
            "memoree_edit_file",
            "memoree_create_directory",
            "memoree_move_file",
            "memoree_get_file_info",
            "memoree_search_files",
        ],
        "os_context": OS_CONTEXT,
        "allowed_dirs": [str(p) for p in ALLOWED_DIRS],
    }, indent=2))
