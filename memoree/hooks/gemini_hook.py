"""Memoree v3 — Gemini SDK post-response auto-save hook.

Usage:
    from memoree.hooks.gemini_hook import wrap_model
    model = wrap_model(genai.GenerativeModel('gemini-pro'))
    response = model.generate_content("Tell me about FTQC equilibrium manifolds")
    # → auto-saved to Memoree
"""
from __future__ import annotations
import httpx
from typing import Any

_MEMOREE_URL = "http://127.0.0.1:8765"


def _save(content: str, meta: dict = {}):
    try:
        httpx.post(
            f"{_MEMOREE_URL}/write_event",
            json={"llm": "gemini", "type": "episodic", "content": content, "meta": meta},
            timeout=5,
        )
    except Exception:
        pass  # non-blocking: never crash the caller


class WrappedGeminiModel:
    """Thin wrapper that auto-saves responses to Memoree."""

    def __init__(self, model: Any):
        self._model = model

    def generate_content(self, prompt: str, **kwargs) -> Any:
        response = self._model.generate_content(prompt, **kwargs)
        try:
            text = response.text
            _save(f"[PROMPT] {prompt}\n[RESPONSE] {text}", {"prompt": prompt})
        except Exception:
            pass
        return response

    async def generate_content_async(self, prompt: str, **kwargs) -> Any:
        response = await self._model.generate_content_async(prompt, **kwargs)
        try:
            text = response.text
            _save(f"[PROMPT] {prompt}\n[RESPONSE] {text}", {"prompt": prompt})
        except Exception:
            pass
        return response

    def __getattr__(self, name: str):
        return getattr(self._model, name)


def wrap_model(model: Any) -> WrappedGeminiModel:
    return WrappedGeminiModel(model)
