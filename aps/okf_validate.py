#!/usr/bin/env python3
"""Validate APS Open Knowledge Format YAML examples against okf.schema.json.

Uses only PyYAML for load, plus a small Draft-07 subset checker so the
contract can be verified without installing jsonschema.
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


def fail(path: Path, msg: str) -> None:
    print(f"FAIL  {path.name}: {msg}", file=sys.stderr)
    raise SystemExit(1)


def require_keys(data: dict[str, Any], keys: list[str], path: Path) -> None:
    missing = [k for k in keys if k not in data]
    if missing:
        fail(path, f"missing required keys: {missing}")


def is_slug(value: str) -> bool:
    if not value or value[0] in "-":
        return False
    return all(c.isalnum() or c == "-" for c in value) and value == value.lower()


def check_named_entity(item: Any, path: Path, field: str) -> None:
    if not isinstance(item, dict):
        fail(path, f"{field} entries must be mappings with id and name")
    if "id" not in item or "name" not in item:
        fail(path, f"{field} entry missing id/name: {item!r}")
    if not isinstance(item["id"], str) or not isinstance(item["name"], str):
        fail(path, f"{field} id and name must be strings")


def check_string_list(data: dict[str, Any], key: str, path: Path) -> None:
    if key not in data:
        return
    value = data[key]
    if not isinstance(value, list) or not all(isinstance(x, str) for x in value):
        fail(path, f"{key} must be a YAML sequence of strings, not {type(value).__name__}")


def validate_shared(data: dict[str, Any], path: Path) -> None:
    require_keys(data, ["okf", "type", "id", "title", "description"], path)
    if data["okf"] != "1.0":
        fail(path, f"okf must be '1.0', got {data['okf']!r}")
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
    check_string_list(data, "admin-modules", path)
    if "status" in data and data["status"] not in {
        "draft",
        "active",
        "locked",
        "deprecated",
    }:
        fail(path, f"invalid status {data['status']!r}")


def validate_ecosys(data: dict[str, Any], path: Path) -> None:
    require_keys(
        data,
        ["workspaces", "theories-or-standards", "operating-systems", "cores"],
        path,
    )
    for field in ("theories-or-standards", "operating-systems", "cores"):
        items = data[field]
        if not isinstance(items, list) or not items:
            fail(path, f"{field} must be a non-empty YAML sequence")
        for item in items:
            check_named_entity(item, path, field)
    if not isinstance(data["workspaces"], dict):
        fail(path, "workspaces must be a mapping of github/local paths")
    github = data["workspaces"].get("github", {})
    if not isinstance(github, dict) or "personal" not in github or "org" not in github:
        fail(path, "workspaces.github must include personal and org")


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
    raw = path.read_text(encoding="utf-8")
    data = yaml.safe_load(raw)
    if not isinstance(data, dict):
        fail(path, "document did not parse to a mapping")
    validate_shared(data, path)
    if data["type"] == "ecosys-manifest":
        validate_ecosys(data, path)
    else:
        validate_theory(data, path)
    print(f"OK    {path.name}  type={data['type']}  id={data['id']}")


def main() -> None:
    if not SCHEMA_PATH.is_file():
        fail(SCHEMA_PATH, "schema file missing")
    for example in EXAMPLES:
        if not example.is_file():
            fail(example, "example file missing")
        validate_file(example)


if __name__ == "__main__":
    main()
