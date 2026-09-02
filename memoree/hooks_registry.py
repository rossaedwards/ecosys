## ** APS-TSLCA-MEMOREE-HOOKS-REGISTRY **
## ** Memoree - Sovereign Memory Substrate **
## ** Symbiotic Universal Xessability Standards **
## ** Three-Squared-Lattice Cognitive Architecture **
## ** Aurphyx Primordial Standard **
## ** Aurphyx LLC **
## ** SAGES | Proprietary | Pro-Existence **
## ** Accessibility = Xessability **
## ** Version 4.0 **

"""
Memoree — Model & Platform Hooks Registry
═══════════════════════════════════════════════════════════════════════════════
Central catalog and unified dispatcher across all AI models:
  • Google Gemini Pro / Flash (gemini_hook)
  • Anthropic Claude 3.7 / 3.5 Sonnet (claude_hook)
  • xAI SuperGrok-3 / Grok-2 (supergrok_hook)
  • Perplexity Sonar Pro (perplexity_hook)
  • Nous Hermes Plus ACP (hermes_hook)
  • LM Studio Local (lmstudio_hook)
  • Ollama Local (ollama_hook)
═══════════════════════════════════════════════════════════════════════════════
f0rg3d in l0v3 by Ross Edwards
"""

from __future__ import annotations

import asyncio
import logging
from typing import Any, Dict, List, Optional

from claude_hook import ClaudeHook
from credentials_manager import credentials
from gemini_hook import GeminiHook
from hermes_hook import HermesHook
from lmstudio_hook import LMStudioHook
from ollama_hook import OllamaHook
from perplexity_hook import PerplexityHook
from supergrok_hook import SuperGrokHook

log = logging.getLogger("memoree.hooks_registry")

HOOK_FACTORIES = {
    "gemini": GeminiHook,
    "claude": ClaudeHook,
    "grok": SuperGrokHook,
    "supergrok": SuperGrokHook,
    "perplexity": PerplexityHook,
    "hermes": HermesHook,
    "lmstudio": LMStudioHook,
    "ollama": OllamaHook,
}


def get_hook(
    provider: str,
    project: str = "memoree",
    model: Optional[str] = None,
    api_key: Optional[str] = None,
    memoree_url: str = "http://127.0.0.1:7042",
) -> Any:
    """Instantiate a hook for a given provider."""
    p = provider.lower()
    factory = HOOK_FACTORIES.get(p)
    if not factory:
        raise ValueError(
            f"Unknown model provider '{provider}'. Available: {list(HOOK_FACTORIES.keys())}"
        )
    return factory(api_key=api_key, model=model, memoree_url=memoree_url, project=project)


def list_available_hooks() -> List[Dict[str, Any]]:
    """Return status and metadata for all registered hooks."""
    status_map = credentials.list_status()
    out = []
    for p, factory in HOOK_FACTORIES.items():
        if p == "grok":
            continue  # alias for supergrok
        info = status_map.get(p, {})
        hook = factory()
        out.append(
            {
                "provider": p,
                "configured": hook.is_configured(),
                "default_model": info.get("default_model", ""),
                "base_url": info.get("base_url", ""),
                "masked_key": info.get("masked_key", "Not Set"),
            }
        )
    return out


async def chat_with_memory_async(
    provider: str,
    prompt: str,
    project: str = "memoree",
    system_prompt: Optional[str] = None,
    session_id: Optional[str] = None,
    inject_memory: bool = True,
) -> Dict[str, Any]:
    """Execute a prompt with memory context pre-fetching and auto-archiving."""
    hook = get_hook(provider, project=project)
    sess = session_id or f"{provider}-chat"
    return await hook.generate_async(
        prompt=prompt,
        system_prompt=system_prompt,
        inject_memory=inject_memory,
        session_id=sess,
        project=project,
    )


def chat_with_memory(provider: str, prompt: str, **kwargs) -> Dict[str, Any]:
    """Synchronous chat wrapper."""
    return asyncio.run(chat_with_memory_async(provider, prompt, **kwargs))
