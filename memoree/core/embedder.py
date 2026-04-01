"""Memoree v3 — Pluggable embedder.
Backend: sentence_transformers (CPU local) or LM Studio REST endpoint.
"""
from __future__ import annotations
import os
from typing import Optional
import yaml

_CONFIG_PATH = os.path.join(os.path.dirname(__file__), "..", "config.yaml")
_cfg: dict = {}
_st_model = None  # lazy-loaded


def _load_cfg() -> dict:
    global _cfg
    if not _cfg:
        with open(_CONFIG_PATH) as f:
            _cfg = yaml.safe_load(f)
    return _cfg


def embed(texts: list[str]) -> list[list[float]]:
    """Embed a list of strings. Returns list of float vectors."""
    cfg = _load_cfg().get("embedder", {})
    backend = cfg.get("backend", "sentence_transformers")
    if backend == "lmstudio":
        return _embed_lmstudio(texts, cfg)
    return _embed_st(texts, cfg)


def _embed_st(texts: list[str], cfg: dict) -> list[list[float]]:
    global _st_model
    from sentence_transformers import SentenceTransformer  # type: ignore
    if _st_model is None:
        model_name = cfg.get("model", "all-MiniLM-L6-v2")
        _st_model = SentenceTransformer(model_name)
    vecs = _st_model.encode(texts, show_progress_bar=False)
    return [v.tolist() for v in vecs]


def _embed_lmstudio(texts: list[str], cfg: dict) -> list[list[float]]:
    import httpx
    url = cfg.get("lmstudio_url", "http://localhost:1234/v1/embeddings")
    resp = httpx.post(url, json={"input": texts, "model": cfg.get("model", "text-embedding-ada-002")})
    resp.raise_for_status()
    data = resp.json()["data"]
    return [d["embedding"] for d in data]
