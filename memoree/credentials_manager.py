## ** APS-TSLCA-MEMOREE-CREDENTIALS **
## ** Memoree - Sovereign Memory Substrate **
## ** Symbiotic Universal Xessability Standards **
## ** Three-Squared-Lattice Cognitive Architecture **
## ** Aurphyx Primordial Standard **
## ** Aurphyx LLC **
## ** SAGES | Proprietary | Pro-Existence **
## ** Accessibility = Xessability **
## ** Version 4.0 **

"""
Memoree — Credentials Manager
═══════════════════════════════════════════════════════════════════════════════
Manages API keys and provider configurations across the Aurphyx ecosystem.
Stored securely in ~/.memoree/credentials.json with environment variable fallback.
═══════════════════════════════════════════════════════════════════════════════
f0rg3d in l0v3 by Ross Edwards
"""

from __future__ import annotations

import json
import logging
import os
import time
from pathlib import Path
from typing import Any, Dict, Optional, Tuple

log = logging.getLogger("memoree.credentials")

CREDENTIALS_DIR = Path(os.path.expanduser("~")) / ".memoree"
CREDENTIALS_FILE = CREDENTIALS_DIR / "credentials.json"

PROVIDER_ENV_VARS = {
    "gemini": ["GEMINI_API_KEY", "GOOGLE_API_KEY"],
    "claude": ["ANTHROPIC_API_KEY", "CLAUDE_API_KEY"],
    "grok": ["GROK_API_KEY", "XAI_API_KEY"],
    "supergrok": ["GROK_API_KEY", "XAI_API_KEY"],
    "perplexity": ["PERPLEXITY_API_KEY", "PPLX_API_KEY"],
    "hermes": ["HERMES_API_KEY", "NOUS_API_KEY"],
    "lmstudio": ["LMSTUDIO_BASE_URL"],
    "ollama": ["OLLAMA_BASE_URL"],
    "openrouter": ["OPENROUTER_API_KEY"],
}

DEFAULT_MODELS = {
    "gemini": "gemini-2.0-flash",
    "claude": "claude-3-7-sonnet-20250219",
    "grok": "grok-3-beta",
    "supergrok": "grok-3-beta",
    "perplexity": "sonar-pro",
    "hermes": "nous-hermes-3-llama-3.1-70b",
    "lmstudio": "liquid/lfm2.5-1.2b",
    "ollama": "llama3.2:latest",
}

DEFAULT_BASE_URLS = {
    "gemini": "https://generativelanguage.googleapis.com/v1beta",
    "claude": "https://api.anthropic.com/v1",
    "grok": "https://api.x.ai/v1",
    "supergrok": "https://api.x.ai/v1",
    "perplexity": "https://api.perplexity.ai",
    "hermes": "https://api.nousresearch.com/v1",
    "lmstudio": "http://localhost:1234/v1",
    "ollama": "http://localhost:11434",
}


def _mask_key(key: Optional[str]) -> str:
    """Mask key safely: e.g. sk-ant-api03...89Ab"""
    if not key:
        return "Not Configured"
    if len(key) <= 8:
        return "******"
    return f"{key[:6]}...{key[-4:]}"


class CredentialsManager:
    """Singleton credentials manager for model and platform API keys."""

    def __init__(self, credentials_path: Path = CREDENTIALS_FILE):
        self.credentials_path = credentials_path
        self._ensure_dir()

    def _ensure_dir(self):
        self.credentials_path.parent.mkdir(parents=True, exist_ok=True)
        if not self.credentials_path.exists():
            self._save_raw({"providers": {}, "settings": {}})

    def _load_raw(self) -> Dict[str, Any]:
        if not self.credentials_path.exists():
            return {"providers": {}, "settings": {}}
        try:
            with open(self.credentials_path, "r", encoding="utf-8") as f:
                return json.load(f)
        except Exception as e:
            log.warning("Failed to load credentials file %s: %s", self.credentials_path, e)
            return {"providers": {}, "settings": {}}

    def _save_raw(self, data: Dict[str, Any]):
        try:
            with open(self.credentials_path, "w", encoding="utf-8") as f:
                json.dump(data, f, indent=2)
        except Exception as e:
            log.error("Failed to save credentials file %s: %s", self.credentials_path, e)

    def get_key(self, provider: str) -> Optional[str]:
        """Resolve API key: File -> Env Vars -> None."""
        p = provider.lower()
        data = self._load_raw()
        file_key = data.get("providers", {}).get(p, {}).get("api_key")
        if file_key:
            return file_key

        # Check env vars
        for env_name in PROVIDER_ENV_VARS.get(p, []):
            val = os.getenv(env_name)
            if val:
                return val

        # LM Studio / Ollama defaults
        if p == "lmstudio":
            return "lm-studio-local"
        if p == "ollama":
            return "ollama-local"

        return None

    def get_base_url(self, provider: str) -> str:
        """Resolve base URL for provider."""
        p = provider.lower()
        data = self._load_raw()
        custom_url = data.get("providers", {}).get(p, {}).get("base_url")
        if custom_url:
            return custom_url
        return DEFAULT_BASE_URLS.get(p, "")

    def get_default_model(self, provider: str) -> str:
        """Resolve default model name."""
        p = provider.lower()
        data = self._load_raw()
        custom_model = data.get("providers", {}).get(p, {}).get("default_model")
        if custom_model:
            return custom_model
        return DEFAULT_MODELS.get(p, "default")

    def set_key(
        self,
        provider: str,
        api_key: str,
        base_url: Optional[str] = None,
        default_model: Optional[str] = None,
    ):
        """Set credentials for a provider."""
        p = provider.lower()
        data = self._load_raw()
        if "providers" not in data:
            data["providers"] = {}
        if p not in data["providers"]:
            data["providers"][p] = {}

        data["providers"][p]["api_key"] = api_key
        if base_url:
            data["providers"][p]["base_url"] = base_url
        if default_model:
            data["providers"][p]["default_model"] = default_model
        data["providers"][p]["updated_at"] = time.time()

        self._save_raw(data)
        log.info("Credentials updated for provider: %s", p)

    def delete_key(self, provider: str):
        """Delete credentials for a provider."""
        p = provider.lower()
        data = self._load_raw()
        if "providers" in data and p in data["providers"]:
            del data["providers"][p]
            self._save_raw(data)

    def list_status(self) -> Dict[str, Dict[str, Any]]:
        """Return status table of all known providers."""
        data = self._load_raw()
        result = {}
        all_providers = set(DEFAULT_MODELS.keys()) | set(data.get("providers", {}).keys())

        for p in sorted(all_providers):
            p_data = data.get("providers", {}).get(p, {})
            key = p_data.get("api_key")
            source = "credentials.json" if key else "env"
            if not key:
                for env_name in PROVIDER_ENV_VARS.get(p, []):
                    if os.getenv(env_name):
                        key = os.getenv(env_name)
                        source = f"env:{env_name}"
                        break

            is_configured = bool(key) or p in ("lmstudio", "ollama")
            result[p] = {
                "provider": p,
                "configured": is_configured,
                "source": source if is_configured else "none",
                "masked_key": _mask_key(key) if is_configured else "Not Set",
                "base_url": self.get_base_url(p),
                "default_model": self.get_default_model(p),
            }
        return result


# Singleton instance
credentials = CredentialsManager()
