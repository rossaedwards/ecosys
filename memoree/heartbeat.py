"""
Aurphyx_Memoree — Heartbeat Loop
Runs every N seconds: ingest → embed → summarize → state update → invariant check
f0rg3d in l0v3 by Ross Edwards
"""

import asyncio
import logging
from datetime import datetime

log = logging.getLogger("memoree.heartbeat")

INTERVAL_SEC = 10


class HeartbeatLoop:
    def __init__(self):
        from core.memory_engine import MemoryEngine
        self.engine = MemoryEngine()

    async def run(self):
        log.info("Heartbeat loop started (interval={INTERVAL_SEC}s)")
        while True:
            try:
                await self._tick()
            except Exception as e:
                log.error(f"Heartbeat error: {e}")
            await asyncio.sleep(INTERVAL_SEC)

    async def _tick(self):
        ts = datetime.utcnow().isoformat()
        log.debug(f"[{ts}] Heartbeat tick")
        # 1. Enforce invariants: flag deprecated meta
        metas = self.engine.aurafs.read_meta(limit=200)
        for m in metas:
            if m.get("deprecated"):
                log.debug("Deprecated invariant: {m.get('fact','')[:60]}")
        # 2. State machine: read active projects and log
        state = self.engine.aurafs.read_state("projects")
        if state:
            log.debug("Active projects: {list(state.keys())}")
        log.debug("Heartbeat tick complete")
