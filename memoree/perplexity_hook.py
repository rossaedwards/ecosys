"""
Memoree — Perplexity Hook
═══════════════════════════════════════════════════════════════════════════════
Auto-saves every Perplexity Pro interaction to Memoree as rich EpisodicMemory.
Supports: single-turn chat, streaming, multi-turn sessions, web-search citation
          capture, and async usage.

Env vars:
  PERPLEXITY_API_KEY   — required
  MEMOREE_URL          — optional, default https://127.0.0.1:7042
═══════════════════════════════════════════════════════════════════════════════
"""

from __future__ import annotations

import asyncio
import json
import logging
import os
import time
from typing import AsyncIterator, Iterator, List, Optional

import httpx

from schemas import EpisodicMemory, LLMProvider, MemoryTier

# ── Config ────────────────────────────────────────────────────────────────────

_MEMOREE_URL = os.getenv("MEMOREE_URL", "http://127.0.0.1:7042")
_PPLX_URL = "https://api.perplexity.ai/chat/completions"
_PPLX_KEY = os.getenv("", "")

log = logging.getLogger("memoree.perplexity_hook")

# Perplexity model catalogue (April 2026)
SONAR_MODELS = {
    "sonar-pro",
    "sonar",
    "sonar-reasoning",
    "sonar-reasoning-pro",
    "sonar-deep-research",
    "r1-1776",
}


# ── Internal helpers ──────────────────────────────────────────────────────────


def _headers() -> dict:
    if not _PPLX_KEY:
        raise ValueError(
            "PERPLEXITY_API_KEY is not set. Export it or add it to your .env file."
        )
    return {
        "Authorization": f"Bearer {_PPLX_KEY}",
        "Content-Type": "application/json",
    }


def _build_body(
    messages: List[dict],
    model: str,
    stream: bool = False,
    **kwargs,
) -> dict:
    return {
        "model": model,
        "messages": messages,
        "stream": stream,
        **kwargs,
    }


def _save_to_memoree(
    prompt: str,
    response: str,
    model: str,
    project: str,
    session_id: str,
    citations: Optional[List[str]] = None,
    tags: Optional[List[str]] = None,
    turn_index: int = 0,
    parent_id: Optional[str] = None,
    latency_ms: Optional[int] = None,
) -> None:
    """
    Fire-and-forget episodic memory save.
    Saves both the user prompt (role=user) and assistant response separately
    so the episodic chain is properly threaded.
    """
    base_tags = ["perplexity", "auto-save", model] + (tags or [])

    # ── User turn ──
    user_mem = EpisodicMemory(
        session_id=session_id,
        project=project,
        role="user",
        content=prompt,
        llm=LLMProvider.PERPLEXITY,
        model_name=model,
        turn_index=turn_index,
        parent_id=parent_id,
        tags=base_tags,
        memory_tier=MemoryTier.WARM,
        user_preferences={
            "project": project,
            "latency_ms": latency_ms,
        },
    )

    # ── Assistant turn ──
    assistant_content = response
    if citations:
        citation_block = "\n\n**Citations:**\n" + "\n".join(
            f"[{i + 1}] {c}" for i, c in enumerate(citations)
        )
        assistant_content += citation_block

    assistant_mem = EpisodicMemory(
        session_id=session_id,
        project=project,
        role="assistant",
        content=assistant_content,
        llm=LLMProvider.PERPLEXITY,
        model_name=model,
        turn_index=turn_index + 1,
        parent_id=user_mem.id,  # thread the assistant reply to the user turn
        tags=base_tags + (["citations"] if citations else []),
        memory_tier=MemoryTier.WARM,
        user_preferences={
            "project": project,
            "citations": citations or [],
            "latency_ms": latency_ms,
        },
    )

    try:
        with httpx.Client(timeout=5) as client:
            for mem in (user_mem, assistant_mem):
                client.post(
                    f"{_MEMOREE_URL}/memories/events",
                    json=mem.model_dump(mode="json"),
                )
    except Exception as e:
        log.warning("[Memoree] Failed to save episodic memory: %s", e)


async def _save_to_memoree_async(
    prompt: str,
    response: str,
    model: str,
    project: str,
    session_id: str,
    citations: Optional[List[str]] = None,
    tags: Optional[List[str]] = None,
    turn_index: int = 0,
    latency_ms: Optional[int] = None,
) -> None:
    """Async variant — used by achat() and astream()."""
    base_tags = ["perplexity", "auto-save", model] + (tags or [])

    user_mem = EpisodicMemory(
        session_id=session_id,
        project=project,
        role="user",
        content=prompt,
        llm=LLMProvider.PERPLEXITY,
        model_name=model,
        turn_index=turn_index,
        tags=base_tags,
        memory_tier=MemoryTier.WARM,
    )

    assistant_content = response
    if citations:
        assistant_content += "\n\n**Citations:**\n" + "\n".join(
            f"[{i + 1}] {c}" for i, c in enumerate(citations)
        )

    assistant_mem = EpisodicMemory(
        session_id=session_id,
        project=project,
        role="assistant",
        content=assistant_content,
        llm=LLMProvider.PERPLEXITY,
        model_name=model,
        turn_index=turn_index + 1,
        parent_id=user_mem.id,
        tags=base_tags + (["citations"] if citations else []),
        memory_tier=MemoryTier.WARM,
        user_preferences={"citations": citations or [], "latency_ms": latency_ms},
    )

    try:
        async with httpx.AsyncClient(timeout=5) as client:
            for mem in (user_mem, assistant_mem):
                await client.post(
                    f"{_MEMOREE_URL}/memories/events",
                    json=mem.model_dump(mode="json"),
                )
    except Exception as e:
        log.warning("[Memoree] Async save failed: %s", e)


# ── Sync API ──────────────────────────────────────────────────────────────────


def chat(
    prompt: str,
    model: str = "sonar-pro",
    project: str = "memoree",
    session_id: str = "perplexity-default",
    tags: Optional[List[str]] = None,
    turn_index: int = 0,
    parent_id: Optional[str] = None,
    system: Optional[str] = None,
    **kwargs,
) -> str:
    """
    Single-turn Perplexity chat with automatic Memoree save.

    Args:
        prompt:     User message text.
        model:      Perplexity model slug (default: sonar-pro).
        project:    Memoree project key to tag this memory under.
        session_id: Session identifier for episodic threading.
        tags:       Extra tags appended to the memory record.
        turn_index: Conversation turn counter for ordering.
        parent_id:  Parent memory ID for threading into an existing chain.
        system:     Optional system prompt prepended to messages.
        **kwargs:   Extra fields forwarded to the Perplexity API body.

    Returns:
        The assistant response text.
    """
    messages = []
    if system:
        messages.append({"role": "system", "content": system})
    messages.append({"role": "user", "content": prompt})

    t0 = time.monotonic()
    resp = httpx.post(
        _PPLX_URL,
        json=_build_body(messages, model, **kwargs),
        headers=_headers(),
        timeout=90,
    )
    latency_ms = int((time.monotonic() - t0) * 1000)
    resp.raise_for_status()

    data = resp.json()
    text = data["choices"][0]["message"]["content"]
    citations = data.get("citations", [])

    _save_to_memoree(
        prompt=prompt,
        response=text,
        model=model,
        project=project,
        session_id=session_id,
        citations=citations,
        tags=tags,
        turn_index=turn_index,
        parent_id=parent_id,
        latency_ms=latency_ms,
    )

    return text


def stream(
    prompt: str,
    model: str = "sonar-pro",
    project: str = "memoree",
    session_id: str = "perplexity-default",
    tags: Optional[List[str]] = None,
    system: Optional[str] = None,
    **kwargs,
) -> Iterator[str]:
    """
    Streaming Perplexity chat — yields text chunks as they arrive.
    Saves the completed response + citations to Memoree after stream ends.

    Usage:
        for chunk in stream("explain topological qubits"):
            print(chunk, end="", flush=True)
    """
    messages = []
    if system:
        messages.append({"role": "system", "content": system})
    messages.append({"role": "user", "content": prompt})

    full_text = []
    citations: List[str] = []
    t0 = time.monotonic()

    with httpx.stream(
        "POST",
        _PPLX_URL,
        json=_build_body(messages, model, stream=True, **kwargs),
        headers=_headers(),
        timeout=120,
    ) as resp:
        resp.raise_for_status()
        for line in resp.iter_lines():
            if not line or not line.startswith("data: "):
                continue
            raw = line[6:]
            if raw.strip() == "[DONE]":
                break
            try:
                chunk = json.loads(raw)
                delta = chunk["choices"][0]["delta"].get("content", "")
                if delta:
                    full_text.append(delta)
                    yield delta
                # Perplexity streams citations in the final chunk
                if "citations" in chunk:
                    citations = chunk["citations"]
            except (json.JSONDecodeError, KeyError):
                continue

    latency_ms = int((time.monotonic() - t0) * 1000)
    _save_to_memoree(
        prompt=prompt,
        response="".join(full_text),
        model=model,
        project=project,
        session_id=session_id,
        citations=citations,
        tags=(tags or []) + ["streamed"],
        latency_ms=latency_ms,
    )


def multi_turn(
    messages: List[dict],
    model: str = "sonar-pro",
    project: str = "memoree",
    session_id: str = "perplexity-default",
    tags: Optional[List[str]] = None,
    **kwargs,
) -> str:
    """
    Multi-turn conversation — pass the full messages list directly.
    Saves the last user message + response to Memoree.

    Args:
        messages: Full OpenAI-style message list
                  [{"role": "user"|"assistant"|"system", "content": "..."}]
    """
    t0 = time.monotonic()
    resp = httpx.post(
        _PPLX_URL,
        json=_build_body(messages, model, **kwargs),
        headers=_headers(),
        timeout=90,
    )
    latency_ms = int((time.monotonic() - t0) * 1000)
    resp.raise_for_status()

    data = resp.json()
    text = data["choices"][0]["message"]["content"]
    citations = data.get("citations", [])

    # Find the last user message to save as prompt
    last_user = next(
        (m["content"] for m in reversed(messages) if m["role"] == "user"),
        "[multi-turn]",
    )
    _save_to_memoree(
        prompt=last_user,
        response=text,
        model=model,
        project=project,
        session_id=session_id,
        citations=citations,
        tags=(tags or []) + ["multi-turn"],
        turn_index=sum(1 for m in messages if m["role"] == "user"),
        latency_ms=latency_ms,
    )

    return text


# ── Async API ─────────────────────────────────────────────────────────────────


async def achat(
    prompt: str,
    model: str = "sonar-pro",
    project: str = "memoree",
    session_id: str = "perplexity-default",
    tags: Optional[List[str]] = None,
    system: Optional[str] = None,
    **kwargs,
) -> str:
    """Async single-turn chat. Drop-in async replacement for chat()."""
    messages = []
    if system:
        messages.append({"role": "system", "content": system})
    messages.append({"role": "user", "content": prompt})

    t0 = time.monotonic()
    async with httpx.AsyncClient(timeout=90) as client:
        resp = await client.post(
            _PPLX_URL,
            json=_build_body(messages, model, **kwargs),
            headers=_headers(),
        )
    latency_ms = int((time.monotonic() - t0) * 1000)
    resp.raise_for_status()

    data = resp.json()
    text = data["choices"][0]["message"]["content"]
    citations = data.get("citations", [])

    asyncio.create_task(
        _save_to_memoree_async(
            prompt=prompt,
            response=text,
            model=model,
            project=project,
            session_id=session_id,
            citations=citations,
            tags=tags,
            latency_ms=latency_ms,
        )
    )

    return text


async def astream(
    prompt: str,
    model: str = "sonar-pro",
    project: str = "memoree",
    session_id: str = "perplexity-default",
    tags: Optional[List[str]] = None,
    system: Optional[str] = None,
    **kwargs,
) -> AsyncIterator[str]:
    """
    Async streaming generator.

    Usage:
        async for chunk in astream("explain Majorana zero modes"):
            print(chunk, end="", flush=True)
    """
    messages = []
    if system:
        messages.append({"role": "system", "content": system})
    messages.append({"role": "user", "content": prompt})

    full_text: List[str] = []
    citations: List[str] = []
    t0 = time.monotonic()

    async with httpx.AsyncClient(timeout=120) as client:
        async with client.stream(
            "POST",
            _PPLX_URL,
            json=_build_body(messages, model, stream=True, **kwargs),
            headers=_headers(),
        ) as resp:
            resp.raise_for_status()
            async for line in resp.aiter_lines():
                if not line or not line.startswith("data: "):
                    continue
                raw = line[6:]
                if raw.strip() == "[DONE]":
                    break
                try:
                    chunk = json.loads(raw)
                    delta = chunk["choices"][0]["delta"].get("content", "")
                    if delta:
                        full_text.append(delta)
                        yield delta
                    if "citations" in chunk:
                        citations = chunk["citations"]
                except (json.JSONDecodeError, KeyError):
                    continue

    latency_ms = int((time.monotonic() - t0) * 1000)
    asyncio.create_task(
        _save_to_memoree_async(
            prompt=prompt,
            response="".join(full_text),
            model=model,
            project=project,
            session_id=session_id,
            citations=citations,
            tags=(tags or []) + ["streamed"],
            latency_ms=latency_ms,
        )
    )


# ── Convenience shorthands ────────────────────────────────────────────────────


def perplexity(prompt: str, **kwargs) -> str:
    """Shorthand for chat() with defaults."""
    return chat(prompt, **kwargs)


def deep_research(prompt: str, project: str = "memoree", **kwargs) -> str:
    """sonar-deep-research with extended timeout."""
    return chat(prompt, model="sonar-deep-research", project=project, **kwargs)


def reasoning(prompt: str, project: str = "memoree", **kwargs) -> str:
    """sonar-reasoning-pro for chain-of-thought tasks."""
    return chat(prompt, model="sonar-reasoning-pro", project=project, **kwargs)
