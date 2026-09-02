## ** APS-TSLCA-MEMOREE-OLLAMA-HOOK **
## ** Memoree - Sovereign Memory Substrate **
## ** Symbiotic Universal Xessability Standards **
## ** Three-Squared-Lattice Cognitive Architecture **
## ** Aurphyx Primordial Standard **
## ** Aurphyx LLC **
## ** SAGES | Proprietary | Pro-Existence **
## ** Accessibility = Xessability **
## ** Version 4.0 **

"""
Memoree — Ollama Hook (Local Offline Model Execution)
═══════════════════════════════════════════════════════════════════════════════
Auto-saves conversations from local Ollama models to Memoree v4.0.
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

log = logging.getLogger("memoree.ollama_hook")


class OllamaHook:
    """Real client & memory hook for Ollama instances."""

    def __init__(
        self,
        base_url: Optional[str] = None,
        model: Optional[str] = None,
        memoree_url: str = "http://127.0.0.1:7042",
        project: str = "memoree",
    ):
        self.base_url = (base_url or credentials.get_base_url("ollama") or "http://localhost:11434").rstrip("/")
        self.model = model or credentials.get_default_model("ollama") or "llama3.2:latest"
        self.memoree_url = memoree_url.rstrip("/")
        self.project = project

    def is_configured(self) -> bool:
        return True

    async def generate_async(
        self,
        prompt: str,
        system_prompt: Optional[str] = None,
        inject_memory: bool = True,
        session_id: str = "ollama-session",
        project: Optional[str] = None,
    ) -> Dict[str, Any]:
        proj = project or self.project
        t0 = time.time()

        memory_system_block = ""
        if inject_memory:
            try:
                async with httpx.AsyncClient(timeout=3.0) as client:
                    res = await client.get(f"{self.memoree_url}/context/active?project={proj}&llm=ollama")
                    if res.status_code == 200:
                        ctx = res.json()
                        memory_system_block = (
                            f"--- MEMOREE CONTEXT (Project: {proj}) ---\n"
                            f"Axioms: {ctx.get('active_axioms', [])}\n"
                            f"-----------------------------------------\n\n"
                        )
            except Exception:
                pass

        full_system = f"{memory_system_block}{system_prompt or ''}".strip()
        messages = []
        if full_system:
            messages.append({"role": "system", "content": full_system})
        messages.append({"role": "user", "content": prompt})

        body = {
            "model": self.model,
            "messages": messages,
            "stream": False,
        }

        async with httpx.AsyncClient(timeout=120.0) as client:
            try:
                resp = await client.post(f"{self.base_url}/api/chat", json=body)
                resp.raise_for_status()
                data = resp.json()
                text_out = data["message"]["content"]
            except Exception as e:
                text_out = f"[Ollama Offline: {e}]"
                data = {"error": str(e)}

        latency_ms = (time.time() - t0) * 1000

        # Background save
        async def _save():
            try:
                payload = {
                    "session_id": session_id,
                    "project": proj,
                    "role": "assistant",
                    "llm": "ollama",
                    "content": f"[USER]: {prompt}\n\n[OLLAMA]: {text_out}",
                    "tags": ["ollama", "local", "auto-save", self.model],
                    "quantum_bands": ["Q7_quasiparticles", "Q13_measure_spacetime"],
                }
                async with httpx.AsyncClient(timeout=3.0) as client:
                    await client.post(f"{self.memoree_url}/memories/events", json=payload)
            except Exception:
                pass

        asyncio.create_task(_save())

        return {
            "text": text_out,
            "model": self.model,
            "latency_ms": round(latency_ms, 2),
            "provider": "ollama",
            "raw": data,
        }

    def generate(self, prompt: str, **kwargs) -> Dict[str, Any]:
        return asyncio.run(self.generate_async(prompt, **kwargs))
