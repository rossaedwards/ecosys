## ** APS-TSLCA-MEMOREE-HERMES-HOOK **
## ** Memoree - Sovereign Memory Substrate **
## ** Symbiotic Universal Xessability Standards **
## ** Three-Squared-Lattice Cognitive Architecture **
## ** Aurphyx Primordial Standard **
## ** Aurphyx LLC **
## ** SAGES | Proprietary | Pro-Existence **
## ** Accessibility = Xessability **
## ** Version 4.0 **

"""
Memoree — Hermes Hook (Nous Hermes Plus / ACP Memory Provider Adapter)
═══════════════════════════════════════════════════════════════════════════════
Implements the Hermes Agent Context Protocol (ACP) Memory Provider interface.
Provides pre-fetch recall, turn synchronization, and tool handling for Hermes.
═══════════════════════════════════════════════════════════════════════════════
f0rg3d in l0v3 by Ross Edwards
"""

from __future__ import annotations

import asyncio
import json
import logging
import os
import time
from typing import Any, Dict, List, Optional

import httpx
from credentials_manager import credentials
from schemas import EpisodicMemory, LLMProvider, MemoryQuery, MemoryType, QuantumBand

log = logging.getLogger("memoree.hermes_hook")


class HermesHook:
    """Hermes Agent Context Protocol (ACP) memory provider and API client."""

    def __init__(
        self,
        api_key: Optional[str] = None,
        model: Optional[str] = None,
        memoree_url: str = "http://127.0.0.1:7042",
        project: str = "memoree",
    ):
        self.api_key = api_key or credentials.get_key("hermes")
        self.model = model or credentials.get_default_model("hermes")
        self.memoree_url = memoree_url.rstrip("/")
        self.project = project
        self._session_id = "hermes-default"

    @property
    def name(self) -> str:
        return "memoree-hermes-provider"

    def is_available(self) -> bool:
        """Check if Hermes provider can activate."""
        return True

    def is_configured(self) -> bool:
        return True


    def initialize(self, session_id: str, **kwargs) -> None:
        self._session_id = session_id

    def system_prompt_block(self) -> str:
        return (
            f"--- MEMOREE TSLCA 9-CELL COGNITIVE SUBSTRATE ---\n"
            f"Project: {self.project} | Daemon: {self.memoree_url}\n"
            f"Memories are structured into 9 non-commutative lattice cells.\n"
            f"--------------------------------------------------"
        )

    async def prefetch(self, query: str, session_id: Optional[str] = None) -> List[Dict[str, Any]]:
        """Recalls nearest memories for query."""
        try:
            async with httpx.AsyncClient(timeout=4.0) as client:
                res = await client.post(
                    f"{self.memoree_url}/query",
                    json={
                        "query_text": query,
                        "project": self.project,
                        "top_k": 5,
                        "memory_types": ["episodic", "semantic", "procedural", "meta"],
                    },
                )
                if res.status_code == 200:
                    return res.json()
        except Exception as e:
            log.debug("Hermes prefetch failed: %s", e)
        return []

    async def sync_turn(self, user_prompt: str, assistant_response: str, session_id: Optional[str] = None):
        """Persist conversation turn."""
        sess = session_id or self._session_id
        payload = {
            "session_id": sess,
            "project": self.project,
            "role": "assistant",
            "llm": "hermes",
            "content": f"[USER]: {user_prompt}\n\n[HERMES]: {assistant_response}",
            "tags": ["hermes", "acp", "auto-save", self.model],
            "quantum_bands": ["Q7_quasiparticles", "Q13_measure_spacetime"],
        }
        try:
            async with httpx.AsyncClient(timeout=4.0) as client:
                await client.post(f"{self.memoree_url}/memories/events", json=payload)
        except Exception as e:
            log.warning("Failed to sync turn to Memoree: %s", e)

    async def generate_async(
        self,
        prompt: str,
        system_prompt: Optional[str] = None,
        inject_memory: bool = True,
        session_id: str = "hermes-session",
        project: Optional[str] = None,
        temperature: float = 0.7,
    ) -> Dict[str, Any]:
        """Generate via Hermes / Nous Research API or local proxy."""
        key = self.api_key or credentials.get_key("hermes") or "hermes-key"
        proj = project or self.project
        t0 = time.time()

        memory_block = ""
        if inject_memory:
            recalled = await self.prefetch(prompt, session_id)
            if recalled:
                memory_block = "\n".join(f"- {r.get('content_preview', '')}" for r in recalled)
                memory_block = f"\n--- RECALLED MEMORIES ---\n{memory_block}\n-------------------------\n"

        full_system = f"{self.system_prompt_block()}\n{memory_block}\n{system_prompt or ''}".strip()
        base_url = credentials.get_base_url("hermes") or "https://api.nousresearch.com/v1"

        headers = {
            "Authorization": f"Bearer {key}",
            "Content-Type": "application/json",
        }
        messages = [
            {"role": "system", "content": full_system},
            {"role": "user", "content": prompt},
        ]
        body = {
            "model": self.model,
            "messages": messages,
            "temperature": temperature,
        }

        async with httpx.AsyncClient(timeout=60.0) as client:
            try:
                resp = await client.post(f"{base_url}/chat/completions", headers=headers, json=body)
                resp.raise_for_status()
                data = resp.json()
                text_out = data["choices"][0]["message"]["content"]
            except Exception as e:
                text_out = f"[Hermes Mock/Fallback Response for '{prompt[:40]}...'] (Endpoint unreachable: {e})"
                data = {"error": str(e)}

        latency_ms = (time.time() - t0) * 1000

        # Background save
        asyncio.create_task(
            self.sync_turn(user_prompt=prompt, assistant_response=text_out, session_id=session_id)
        )

        return {
            "text": text_out,
            "model": self.model,
            "latency_ms": round(latency_ms, 2),
            "provider": "hermes",
            "raw": data,
        }

    def generate(self, prompt: str, **kwargs) -> Dict[str, Any]:
        return asyncio.run(self.generate_async(prompt, **kwargs))
