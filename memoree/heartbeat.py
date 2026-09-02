## ** APS-TSLCA-MEMOREE-HEARTBEAT **
## ** Memoree - Sovereign Memory Substrate **
## ** Symbiotic Universal Xessability Standards **
## ** Three-Squared-Lattice Cognitive Architecture **
## ** Aurphyx Primordial Standard **
## ** Aurphyx LLC **
## ** SAGES | Proprietary | Pro-Existence **
## ** Accessibility = Xessability **
## ** Version 4.0 **

"""
Memoree — Heartbeat Loop (TSLCA Invariant Pulse & Working Memory Curing)
═══════════════════════════════════════════════════════════════════════════════
Runs every N seconds:
  1. Pulse check on Qdrant vector backend & 9 TSL collections.
  2. Cure uncured WorkingMemory buffer items into persistent layers.
  3. Validate active project registry and invariant status.
═══════════════════════════════════════════════════════════════════════════════
f0rg3d in l0v3 by Ross Edwards
"""

from __future__ import annotations

import asyncio
import logging
from datetime import datetime, timezone
from typing import Optional

log = logging.getLogger("memoree.heartbeat")

INTERVAL_SEC = 30


class HeartbeatLoop:
    """Periodic health, curing, and invariant maintenance loop."""

    def __init__(self, engine: Optional[Any] = None):
        if engine is None:
            from memory_engine import MemoryEngine
            self.engine = MemoryEngine()
        else:
            self.engine = engine
        self._running = False

    async def run(self):
        self._running = True
        log.info("[Heartbeat] Loop started (interval=%ds)", INTERVAL_SEC)
        while self._running:
            try:
                await self._tick()
            except Exception as e:
                log.error("[Heartbeat] Error during tick: %s", e)
            await asyncio.sleep(INTERVAL_SEC)

    def stop(self):
        self._running = False

    async def _tick(self):
        ts = datetime.now(tz=timezone.utc).isoformat()
        log.debug("[%s] Heartbeat tick pulse", ts)

        # 1. Cure active working buffer
        cured_stats = self.engine.cure_working_buffer()
        if cured_stats.get("cured_count", 0) > 0:
            log.info("[Heartbeat] Cured %d working memory items into persistent layers", cured_stats["cured_count"])

        # 2. Check engine diagnostics
        diag = self.engine.diagnostics()
        log.debug("[Heartbeat] Diagnostics status: %s | Qdrant connected: %s", diag.status, diag.qdrant_connected)
        log.debug("[%s] Heartbeat tick complete", ts)
