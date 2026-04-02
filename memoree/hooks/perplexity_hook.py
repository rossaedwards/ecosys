"""Memoree v3 — Perplexity Pro API post-response hook.

Usage:
    from memoree.hooks.perplexity_hook import chat
    reply = chat("What is the latest on MemOS v2?")
    # → auto-saved to Memoree
"""
from __future__ import annotations
import os
import httpx

_MEMOREE_URL = "http://127.0.0.1:8765"
_PPLX_URL = "https://api.perplexity.ai/chat/completions"
_PPLX_KEY = os.getenv("PERPLEXITY_API_KEY", "")


def _save(content: str, meta: dict = {}):
    try:
        httpx.post(
            f"{_MEMOREE_URL}/write_event",
            json={"llm": "perplexity", "type": "episodic", "content": content, "meta": meta},
            timeout=5,
        )
    except Exception:
        pass


def chat(prompt: str, model: str = "sonar-pro", **kwargs) -> str:
    """Send a prompt to Perplexity and auto-save the response."""
    headers = {"Authorization": f"Bearer {_PPLX_KEY}", "Content-Type": "application/json"}
    body = {"model": model, "messages": [{"role": "user", "content": prompt}], **kwargs}
    resp = httpx.post(_PPLX_URL, json=body, headers=headers, timeout=30)
    resp.raise_for_status()
    text = resp.json()["choices"][0]["message"]["content"]
    _save(f"[PROMPT] {prompt}\n[RESPONSE] {text}", {"model": model, "prompt": prompt})
    return text
