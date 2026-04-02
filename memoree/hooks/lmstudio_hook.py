"""Memoree v3 — LM Studio headless post-response callback.
Works with any model loaded in LM Studio (Nemotron 3 Nano 4B, Qwen 3.5 4B, etc.)

LM Studio exposes an OpenAI-compatible API on localhost:1234.
This hook wraps it with auto-save to Memoree.

Usage:
    from memoree.hooks.lmstudio_hook import chat
    reply = chat("nemotron-3-nano-4b", "Explain neglecton topology")
"""
from __future__ import annotations
import httpx

_MEMOREE_URL = "http://127.0.0.1:8765"
_LMS_URL = "http://localhost:1234/v1/chat/completions"


def _save(llm: str, content: str, meta: dict = {}):
    try:
        httpx.post(
            f"{_MEMOREE_URL}/write_event",
            json={"llm": llm, "type": "episodic", "content": content, "meta": meta},
            timeout=5,
        )
    except Exception:
        pass


def chat(model: str, prompt: str, system: str = "", **kwargs) -> str:
    """Chat with a local LM Studio model and auto-save to Memoree."""
    messages = []
    if system:
        messages.append({"role": "system", "content": system})
    messages.append({"role": "user", "content": prompt})
    body = {"model": model, "messages": messages, "stream": False, **kwargs}
    resp = httpx.post(_LMS_URL, json=body, timeout=120)
    resp.raise_for_status()
    text = resp.json()["choices"][0]["message"]["content"]
    _save(model, f"[PROMPT] {prompt}\n[RESPONSE] {text}", {"model": model, "prompt": prompt})
    return text


def nemotron(prompt: str, **kwargs) -> str:
    return chat("nemotron-3-nano-4b", prompt, **kwargs)


def qwen(prompt: str, **kwargs) -> str:
    return chat("qwen-3.5-4b", prompt, **kwargs)
