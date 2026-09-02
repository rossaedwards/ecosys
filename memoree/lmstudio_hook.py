## ** APS-TSLCA-MEMOREE-LMSTUDIO-HOOK **
## ** Memoree - Sovereign Memory Substrate **
## ** Symbiotic Universal Xessability Standards **
## ** Three-Squared-Lattice Cognitive Architecture **
## ** Aurphyx Primordial Standard **
## ** Aurphyx LLC **
## ** SAGES | Proprietary | Pro-Existence **
## ** Accessibility = Xessability **
## ** Version 4.0 **

"""
Memoree — LM Studio Hook (Local LLM Execution)
═══════════════════════════════════════════════════════════════════════════════
Auto-saves conversations from local LM Studio models to Memoree v4.0.
Supports liquid/lfm2.5-1.2b, nemotron, qwen, and local model switching.
═══════════════════════════════════════════════════════════════════════════════
f0rg3d in l0v3 by Ross Edwards
"""

from __future__ import annotations

import asyncio
import json
import logging
import time
from typing import Any, Dict, List, Optional

import httpx
from credentials_manager import credentials

log = logging.getLogger("memoree.lmstudio_hook")


class LMStudioHook:
    """Real client & memory hook for LM Studio local instances."""

    def __init__(
        self,
        base_url: Optional[str] = None,
        model: Optional[str] = None,
        memoree_url: str = "http://127.0.0.1:7042",
        project: str = "memoree",
    ):
        self.base_url = (base_url or credentials.get_base_url("lmstudio") or "http://localhost:1234/v1").rstrip("/")
        self.model = model or credentials.get_default_model("lmstudio") or "liquid/lfm2.5-1.2b"
        self.memoree_url = memoree_url.rstrip("/")
        self.project = project

    def is_configured(self) -> bool:
        return True

    async def fetch_context(self, project: Optional[str] = None) -> Optional[Dict[str, Any]]:
        proj = project or self.project
        try:
            async with httpx.AsyncClient(timeout=3.0) as client:
                res = await client.get(f"{self.memoree_url}/context/active?project={proj}&llm=lmstudio")
                if res.status_code == 200:
                    return res.json()
        except Exception:
            pass
        return None

    async def archive_turn(
        self,
        user_prompt: str,
        assistant_response: str,
        session_id: str = "lms-default",
        project: Optional[str] = None,
        latency_ms: float = 0.0,
    ):
        proj = project or self.project
        payload = {
            "session_id": session_id,
            "project": proj,
            "role": "assistant",
            "llm": "lmstudio",
            "content": f"[USER]: {user_prompt}\n\n[LMSTUDIO]: {assistant_response}",
            "tags": ["lmstudio", "local", "auto-save", self.model],
            "quantum_bands": ["Q7_quasiparticles", "Q13_measure_spacetime"],
        }
        try:
            async with httpx.AsyncClient(timeout=3.0) as client:
                await client.post(f"{self.memoree_url}/memories/events", json=payload)
        except Exception as e:
            log.warning("Failed to archive turn to Memoree: %s", e)

    async def generate_async(
        self,
        prompt: str,
        system_prompt: Optional[str] = None,
        inject_memory: bool = True,
        session_id: str = "lms-session",
        project: Optional[str] = None,
        temperature: float = 0.7,
    ) -> Dict[str, Any]:
        proj = project or self.project
        t0 = time.time()

        memory_system_block = ""
        if inject_memory:
            ctx = await self.fetch_context(proj)
            if ctx:
                memory_system_block = (
                    f"--- ACTIVE MEMOREE CONTEXT (Project: {proj}) ---\n"
                    f"Axioms: {ctx.get('active_axioms', [])}\n"
                    f"Dualities: {ctx.get('active_dualities', [])}\n"
                    f"--------------------------------------------------\n\n"
                )

        full_system = f"{memory_system_block}{system_prompt or ''}".strip()
        messages = []
        if full_system:
            messages.append({"role": "system", "content": full_system})
        messages.append({"role": "user", "content": prompt})

        body = {
            "model": self.model,
            "messages": messages,
            "temperature": temperature,
            "stream": False,
        }

        async with httpx.AsyncClient(timeout=120.0) as client:
            try:
                resp = await client.post(f"{self.base_url}/chat/completions", json=body)
                resp.raise_for_status()
                data = resp.json()
                text_out = data["choices"][0]["message"]["content"]
            except Exception as e:
                text_out = f"[LM Studio Local Offline: {e}]"
                data = {"error": str(e)}

        latency_ms = (time.time() - t0) * 1000

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
            "provider": "lmstudio",
            "raw": data,
        }

    def generate(self, prompt: str, **kwargs) -> Dict[str, Any]:
        return asyncio.run(self.generate_async(prompt, **kwargs))