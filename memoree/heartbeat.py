"""
Aurphyx_Memoree — Heartbeat Loop
Runs every N seconds: check invariant space (meta) → check active project
state → (future: ingest / embed / summarize / cure Working memory, per
Phase 7 of APS-TSLCA-MEMOREE-PLAN — see AI-SYNC.md / devops/ for tracking)

Fixed 2026-08-25 — this loop has been dead since it was written. Two real
bugs, both fixed here:

  1. `from core.memory_engine import MemoryEngine` never resolved — there is
     no `core/` package in memoree/. memory_engine.py sits at the project
     root, and every other module in this project imports it flat
     (`from memory_engine import MemoryEngine`). This loop is no exception.

  2. `_tick()` called `self.engine.aurafs.read_meta(...)` and
     `.read_state(...)`. Neither exists: memory_engine.py's own module
     docstring says "AuraFS is disabled pending integration", and it only
     imports AuraFSBackend as a commented-out stub. `.aurafs` was never a
     live attribute on MemoryEngine, so every real tick would have raised
     AttributeError and been swallowed by the bare `except Exception` in
     `run()` — the loop would have looked alive in the log while silently
     doing nothing, forever.

The fix below uses the engine methods that actually exist and are already
exercised by routes.py: `MemoryEngine.query()` for the meta/invariant space,
`MemoryEngine.list_projects()` for project state.

Known limitation carried forward honestly rather than papered over:
`MemoryEngine.query()` filters out deprecated MetaMemory records unless you
pass `include_deprecated=True`, but `MemorySearchResult` (the object it
returns) has no `deprecated` field — the engine checks the flag internally
before you ever see the result, then discards it. So this tick cannot yet
tell you *which* meta records are deprecated, only how many meta records
answered the query. Giving heartbeat a real per-record deprecated flag is
Phase 4/5 work (a proper `/lattice` or meta-enumeration endpoint), not a
"fix the import" job — flagging it here instead of quietly downgrading the
tick's behavior without saying so.

f0rg3d in l0v3 by Ross Edwards
"""

import asyncio
import logging
from datetime import datetime, timezone

log = logging.getLogger("memoree.heartbeat")

INTERVAL_SEC = 10


class HeartbeatLoop:
    def __init__(self):
        from memory_engine import MemoryEngine
        from schemas import MemoryQuery, MemoryType

        self._MemoryQuery = MemoryQuery
        self._MemoryType = MemoryType
        self.engine = MemoryEngine()

    async def run(self):
        log.info(f"Heartbeat loop started (interval={INTERVAL_SEC}s)")
        while True:
            try:
                await self._tick()
            except Exception as e:
                log.error(f"Heartbeat error: {e}", exc_info=True)
            await asyncio.sleep(INTERVAL_SEC)

    async def _tick(self):
        ts = datetime.now(timezone.utc).isoformat()
        log.debug(f"[{ts}] Heartbeat tick")

        # 1. Invariant-space pulse: how much of the meta/axiom space is
        # currently reachable. See the class docstring for why this can't
        # yet single out deprecated records specifically.
        try:
            query = self._MemoryQuery(
                query_text="invariant axiom fact",
                memory_types=[self._MemoryType.META],
                top_k=200,
                min_score=0.0,
                include_deprecated=True,
            )
            results = self.engine.query(query)
            log.debug(f"Meta/invariant space: {len(results)} reachable record(s)")
        except Exception as e:
            log.warning(f"Heartbeat invariant check failed: {e}")

        # 2. State machine: log active projects.
        try:
            active = [p.key for p in self.engine.list_projects() if p.status == "active"]
            log.debug(f"Active projects: {active}")
        except Exception as e:
            log.warning(f"Heartbeat project-state check failed: {e}")

        log.debug("Heartbeat tick complete")
