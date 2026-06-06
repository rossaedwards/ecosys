gemini_hook.py.md

"""
Memoree — Gemini Hook
Auto-saves Gemini responses to Memoree
"""

import asyncio
import logging
from datetime import datetime
from typing import Any, AsyncGenerator, List, Optional

import httpx

# Configure logging for production monitoring
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger("AudryHook")


class AudryGeminiHook:
    """
    Production wrapper for Gemini GenerativeModel.
    Integrates real-time episodic memory injection and auto-archiving to Memoree.
    """

    def __init__(
        self,
        model: Any,
        memoree_url: str = "http://127.0.0.1:7042",
        project_id: str = "audry-core-v1",
    ):
        self._model = model
        self.memoree_url = memoree_url.rstrip("/")
        self.project_id = project_id

    async def _archive_to_memoree(self, prompt: str, content: str):
        """Internal helper to push data to the Memoree MCP Server."""
        payload = {
            "role": "assistant",
            "content": f"[USER_PROMPT]: {prompt}\n\n[AUDRY_RESPONSE]: {content}",
            "metadata": {
                "llm_provider": "google-gemini",
                "project": self.project_id,
                "timestamp": datetime.utcnow().isoformat(),
                "streaming": True,
            },
            "tags": ["audry-ai", "sovereign-network", self.project_id],
        }

        try:
            async with httpx.AsyncClient(timeout=10.0) as client:
                response = await client.post(
                    f"{self.memoree_url}/memories/events", json=payload
                )
                response.raise_for_status()
                logger.info(f"Successfully archived event to Memoree.")
        except Exception as e:
            logger.error(f"Failed to archive to Memoree: {str(e)}")

    async def generate_content_async(self, prompt: str, **kwargs) -> Any:
        """Standard async generation with background archiving."""
        response = await self._model.generate_content_async(prompt, **kwargs)

        try:
            # Only archive if the response contains valid text
            if response.candidates and hasattr(response, "text"):
                asyncio.create_task(self._archive_to_memoree(prompt, response.text))
        except Exception as e:
            logger.warning(f"Metadata extraction failed: {e}")

        return response

    async def stream_content_async(self, prompt: str, **kwargs) -> AsyncGenerator:
        """
        HTTPS Streaming implementation.
        Yields chunks to the UI immediately, archives the full transcript at the end.
        """
        full_transcript: List[str] = []

        # Ensure streaming is enabled in kwargs
        kwargs["stream"] = True

        async for chunk in await self._model.generate_content_async(prompt, **kwargs):
            if chunk.text:
                full_transcript.append(chunk.text)
                yield chunk

        # Post-stream processing: Archive the complete conversation
        if full_transcript:
            complete_text = "".join(full_transcript)
            asyncio.create_task(self._archive_to_memoree(prompt, complete_text))

    def __getattr__(self, name: str):
        """Forward all native GenerativeModel methods (start_chat, etc) to the core model."""
        return getattr(self._model, name)


def wrap_audry(model: Any, tunnel_url: Optional[str] = None) -> AudryGeminiHook:
    """Entry point to wrap a Gemini model instance with Audry logic."""
    url = tunnel_url or "http://127.0.0.1:7042"
    return AudryGeminiHook(model, memoree_url=url)
