## ** APS-TSLCA-MEMOREE-GEMINI-HOOK **
## ** Memoree - Sovereign Memory Substrate **
## ** Symbiotic Universal Xessability Standards **
## ** Three-Squared-Lattice Cognitive Architecture **
## ** Aurphyx Primordial Standard **
## ** Aurphyx LLC **
## ** SAGES | Proprietary | Pro-Existence **
## ** Accessibility = Xessability **
## ** Version 4.0 **

"""
Memoree — Gemini Hook (Google DeepMind / GenAI Real API Integration)
═══════════════════════════════════════════════════════════════════════════════
Provides real Google Gemini API calls with automatic Memoree v4.0 9-cell
context pre-fetching and post-generation episodic archiving.
═══════════════════════════════════════════════════════════════════════════════
f0rg3d in l0v3 by Ross Edwards
"""

from __future__ import annotations

import asyncio
import json
import logging
import os
import time
from datetime import datetime, timezone
from typing import Any, AsyncGenerator, Dict, List, Optional

import httpx
from credentials_manager import credentials
from schemas import ContextResponse, EpisodicMemory, LLMProvider, MemoryTier, QuantumBand

log = logging.getLogger("memoree.gemini_hook")


class GeminiHook:
    """Real API client & context hook for Google Gemini."""

    def __init__(
        self,
        api_key: Optional[str] = None,
        model: Optional[str] = None,
        memoree_url: str = "http://127.0.0.1:7042",
        project: str = "memoree",
    ):
        self.api_key = api_key or credentials.get_key("gemini")
        self.model = model or credentials.get_default_model("gemini")
        self.memoree_url = memoree_url.rstrip("/")
        self.project = project

    def is_configured(self) -> bool:
        return bool(self.api_key or credentials.get_key("gemini"))

    def _get_active_key(self) -> str:
        key = self.api_key or credentials.get_key("gemini")
        if not key:
            raise ValueError(
                "Gemini API key is not configured. Run 'memoree auth gemini <key>' or set GEMINI_API_KEY."
            )
        return key

    async def fetch_context(self, project: Optional[str] = None) -> Optional[Dict[str, Any]]:
        """Fetch contracted 9-cell memory context from Memoree."""
        proj = project or self.project
        try:
            async with httpx.AsyncClient(timeout=4.0) as client:
                res = await client.get(f"{self.memoree_url}/context/active?project={proj}&llm=gemini")
                if res.status_code == 200:
                    return res.json()
        except Exception as e:
            log.debug("Context fetch failed (daemon offline?): %s", e)
        return None

    async def archive_turn(
        self,
        user_prompt: str,
        assistant_response: str,
        session_id: str = "gemini-default",
        project: Optional[str] = None,
        latency_ms: float = 0.0,
        tags: Optional[List[str]] = None,
    ):
        """Archive turn into Memoree /memories/events."""
        proj = project or self.project
        payload = {
            "session_id": session_id,
            "project": proj,
            "role": "assistant",
            "llm": "gemini",
            "content": f"[USER]: {user_prompt}\n\n[GEMINI]: {assistant_response}",
            "tags": ["gemini", "auto-save", self.model] + (tags or []),
            "quantum_bands": ["Q7_quasiparticles", "Q13_measure_spacetime"],
        }
        try:
            async with httpx.AsyncClient(timeout=4.0) as client:
                await client.post(f"{self.memoree_url}/memories/events", json=payload)
        except Exception as e:
            log.warning("Failed to archive turn to Memoree: %s", e)

    async def generate_async(
        self,
        prompt: str,
        system_instruction: Optional[str] = None,
        inject_memory: bool = True,
        session_id: str = "gemini-session",
        project: Optional[str] = None,
        temperature: float = 0.7,
    ) -> Dict[str, Any]:
        """Execute a real Gemini API generation with memory injection."""
        key = self._get_active_key()
        proj = project or self.project
        t0 = time.time()

        memory_prompt_prefix = ""
        if inject_memory:
            ctx = await self.fetch_context(proj)
            if ctx:
                memory_prompt_prefix = (
                    f"--- ACTIVE MEMOREE CONTEXT (Project: {proj}) ---\n"
                    f"Axioms: {ctx.get('active_axioms', [])}\n"
                    f"Dualities: {ctx.get('active_dualities', [])}\n"
                    f"Invariants: {len(ctx.get('invariants', []))} active\n"
                    f"Total Memories: {ctx.get('total_memories', 0)}\n"
                    f"--------------------------------------------------\n\n"
                )

        full_prompt = f"{memory_prompt_prefix}{prompt}" if memory_prompt_prefix else prompt
        url = f"https://generativelanguage.googleapis.com/v1beta/models/{self.model}:generateContent?key={key}"

        body: Dict[str, Any] = {
            "contents": [{"parts": [{"text": full_prompt}]}],
            "generationConfig": {"temperature": temperature},
        }
        if system_instruction:
            body["systemInstruction"] = {"parts": [{"text": system_instruction}]}

        async with httpx.AsyncClient(timeout=60.0) as client:
            resp = await client.post(url, json=body)
            resp.raise_for_status()
            data = resp.json()

        latency_ms = (time.time() - t0) * 1000
        text_out = ""
        try:
            text_out = data["candidates"][0]["content"]["parts"][0]["text"]
        except (KeyError, IndexError):
            text_out = json.dumps(data)

        # Background save
        asyncio.create_task(
            self.archive_turn(
                user_prompt=prompt,
                assistant_response=text_out,
                session_id=session_id,
                project=proj,
                latency_ms=latency_ms,
            )
        )

        return {
            "text": text_out,
            "model": self.model,
            "latency_ms": round(latency_ms, 2),
            "provider": "gemini",
            "raw": data,
        }

    def generate(self, prompt: str, **kwargs) -> Dict[str, Any]:
        """Synchronous wrapper."""
        return asyncio.run(self.generate_async(prompt, **kwargs))
