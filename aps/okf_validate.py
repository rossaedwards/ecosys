#!/usr/bin/env python3
"""Validate APS Open Knowledge Format YAML examples (OKF 1.1 / TSLCA lattice).

Uses only PyYAML. Checks the nine-cell contract without jsonschema.
"""

from __future__ import annotations

import sys
from pathlib import Path
from typing import Any

import yaml

ROOT = Path(__file__).resolve().parent
SCHEMA_PATH = ROOT / "okf.schema.json"
EXAMPLES = [
    ROOT / "okf.ecosys-manifest.yaml",
    ROOT / "okf.theory-standard-framework.yaml",
]

CHANNELS = ("sic", "scc", "icc")
CELLS = tuple(f"{row}-{col}" for row in CHANNELS for col in CHANNELS)
MEMOREE_NODES = {
    "temporal",
    "relational",
    "operational",
    "axiomatic",
    "state",
    "generative",
    "mandate",
    "entity",
    "lattice",
}
CELL_MEMOREE = {
    "sic-sic": "temporal",
    "sic-scc": "relational",
    "sic-icc": "entity",
    "scc-sic": "operational",
    "scc-scc": "axiomatic",
    "scc-icc": "mandate",
    "icc-sic": "state",
    "icc-scc": "generative",
    "icc-icc": "lattice",
}


def fail(path: Path, msg: str) -> None:
    print(f"FAIL  {path.name}: {msg}", file=sys.stderr)
    raise SystemExit(1)


def require_keys(data: dict[str, Any], keys: list[str], path: Path) -> None:
    missing = [k for k in keys if k not in data]
    if missing:
        fail(path, f"missing required keys: {missing}")


def is_slug(value: str) -> bool:
    if not value or value[0] == "-":
        return False
    return all(c.isalnum() or c == "-" for c in value) and value == value.lower()


def check_named_entity(item: Any, path: Path, field: str) -> None:
    if not isinstance(item, dict):
        fail(path, f"{field} entries must be mappings with id and name")
    if "id" not in item or "name" not in item:
        fail(path, f"{field} entry missing id/name: {item!r}")


def check_string_list(data: dict[str, Any], key: str, path: Path) -> None:
    if key not in data:
        return
    value = data[key]
    if not isinstance(value, list) or not all(isinstance(x, str) for x in value):
        fail(path, f"{key} must be a YAML sequence of strings")


def validate_tsl(tsl: Any, path: Path, *, ecosys: bool) -> None:
    if not isinstance(tsl, dict):
        fail(path, "tsl must be a mapping")
    require_keys(tsl, ["lattice", "cell"], path)
    if tsl["lattice"] != "tslca":
        fail(path, f"tsl.lattice must be tslca, got {tsl['lattice']!r}")
    cell = tsl["cell"]
    if ecosys:
        if cell != "usaic":
            fail(path, "ecosys-manifest tsl.cell must be usaic")
    elif cell not in CELLS:
        fail(path, f"tsl.cell must be one of the nine cells, got {cell!r}")
    if "also" in tsl:
        extra = tsl["also"]
        if not isinstance(extra, list) or not all(x in CELLS for x in extra):
            fail(path, "tsl.also must be a sequence of nine-cell ids")
    node = tsl.get("memoree-node")
    if node is not None and node not in MEMOREE_NODES:
        fail(path, f"unknown memoree-node {node!r}")
    grammar = tsl.get("grammar")
    if grammar is not None and grammar != [3, 6, 9, 13]:
        fail(path, f"tsl.grammar must be [3, 6, 9, 13], got {grammar!r}")


def validate_shared(data: dict[str, Any], path: Path) -> None:
    require_keys(data, ["okf", "type", "id", "title", "description", "tsl"], path)
    if data["okf"] not in {"1.0", "1.1"}:
        fail(path, f"okf must be '1.0' or '1.1', got {data['okf']!r}")
    if data["type"] not in {"ecosys-manifest", "theory-standard-framework"}:
        fail(path, f"unknown type {data['type']!r}")
    if not is_slug(str(data["id"])):
        fail(path, f"id is not a lowercase slug: {data['id']!r}")
    if "resource" in data:
        resource = data["resource"]
        if not isinstance(resource, str) or " | " in resource:
            fail(path, "resource must be a single URL with no prose suffix")
        if not resource.startswith(("http://", "https://")):
            fail(path, f"resource is not an http(s) URL: {resource!r}")
    check_string_list(data, "tags", path)
    check_string_list(data, "aints", path)
    check_string_list(data, "license", path)
    if "status" in data and data["status"] not in {
        "draft",
        "active",
        "locked",
        "deprecated",
    }:
        fail(path, f"invalid status {data['status']!r}")
    validate_tsl(data["tsl"], path, ecosys=data["type"] == "ecosys-manifest")


def validate_cell(cell_id: str, cell: Any, path: Path) -> None:
    if not isinstance(cell, dict):
        fail(path, f"cells.{cell_id} must be a mapping")
    require_keys(cell, ["tensor", "name", "role"], path)
    tensor = cell["tensor"]
    expected = cell_id.split("-")
    if tensor != expected:
        fail(path, f"cells.{cell_id}.tensor must be {expected}, got {tensor!r}")
    node = cell.get("memoree-node")
    if node is not None and node != CELL_MEMOREE[cell_id]:
        fail(
            path,
            f"cells.{cell_id}.memoree-node must be {CELL_MEMOREE[cell_id]}, got {node!r}",
        )
    holds = cell.get("holds")
    if holds is not None and not isinstance(holds, dict):
        fail(path, f"cells.{cell_id}.holds must be a mapping")


def validate_ecosys(data: dict[str, Any], path: Path) -> None:
    require_keys(data, ["channels", "cells"], path)
    channels = data["channels"]
    if set(channels) != set(CHANNELS):
        fail(path, f"channels must be exactly {list(CHANNELS)}, got {list(channels)}")
    cells = data["cells"]
    if set(cells) != set(CELLS):
        missing = [c for c in CELLS if c not in cells]
        extra = [c for c in cells if c not in CELLS]
        fail(path, f"cells must be the nine TSLCA cells; missing={missing} extra={extra}")
    for cell_id in CELLS:
        validate_cell(cell_id, cells[cell_id], path)
    sages = data.get("sages")
    if sages is not None and sages.get("guardians") != 13:
        fail(path, "sages.guardians must be 13")


def validate_theory(data: dict[str, Any], path: Path) -> None:
    tos = data.get("theories-or-standards")
    if tos is None:
        return
    if not isinstance(tos, list):
        fail(path, "theories-or-standards must be a sequence")
    for item in tos:
        if isinstance(item, str):
            continue
        check_named_entity(item, path, "theories-or-standards")


def validate_file(path: Path) -> None:
    data = yaml.safe_load(path.read_text(encoding="utf-8"))
    if not isinstance(data, dict):
        fail(path, "document did not parse to a mapping")
    validate_shared(data, path)
    if data["type"] == "ecosys-manifest":
        validate_ecosys(data, path)
    else:
        validate_theory(data, path)
    print(
        f"OK    {path.name}  type={data['type']}  id={data['id']}  "
        f"tsl.cell={data['tsl']['cell']}"
    )


def main() -> None:
    if not SCHEMA_PATH.is_file():
        fail(SCHEMA_PATH, "schema file missing")
    for example in EXAMPLES:
        if not example.is_file():
            fail(example, "example file missing")
        validate_file(example)


if __name__ == "__main__":
    main()
