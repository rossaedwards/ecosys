"""Memoree v3 — MemOS FastAPI bridge (optional).

MemOS (v2.0.11, March 2026) exposes a FastAPI service at localhost:9090.
This module is a thin client that:
  1. Mirrors write_event → MemOS cube
  2. Optionally reads from MemOS for enriched multi-cube recall

To enable: set memos.enabled: true in config.yaml and run MemOS Docker.
  docker run -p 9090:9090 memos/memos:latest
"""
from __future__ import annotations
import os
import httpx
import yaml

_CONFIG_PATH = os.path.join(os.path.dirname(__file__), "..", "config.yaml")


def _cfg() -> dict:
    with open(_CONFIG_PATH) as f:
        return yaml.safe_load(f)


def _enabled() -> bool:
    return _cfg().get("memos", {}).get("enabled", False)


def _url() -> str:
    return _cfg().get("memos", {}).get("api_url", "http://localhost:9090")


def _cube_id() -> str:
    return _cfg().get("memos", {}).get("cube_id", "aurphyx_main")


def mirror_event(event: dict):
    """Mirror a Memoree event into MemOS cube (non-blocking, best-effort)."""
    if not _enabled():
        return
    try:
        httpx.post(
            f"{_url()}/memory/add",
            json={
                "cube_id": _cube_id(),
                "content": event.get("content", ""),
                "meta": {
                    "llm": event.get("llm"),
                    "type": event.get("type"),
                    "tags": event.get("tags", []),
                    "timestamp": event.get("timestamp"),
                },
            },
            timeout=3,
        )
    except Exception:
        pass


def recall_from_memos(query: str, top_k: int = 5) -> list[dict]:
    """Pull enriched context from MemOS (supplements Memoree native recall)."""
    if not _enabled():
        return []
    try:
        resp = httpx.post(
            f"{_url()}/memory/search",
            json={"cube_id": _cube_id(), "query": query, "top_k": top_k},
            timeout=5,
        )
        resp.raise_for_status()
        return resp.json().get("results", [])
    except Exception:
        return []
