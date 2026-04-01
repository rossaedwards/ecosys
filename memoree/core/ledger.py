"""Memoree v3 — SQLite raw ledger (aiosqlite + FTS5).
Stores every event as a JSON row. FTS5 for keyword search fallback.
"""
from __future__ import annotations
import json
import os
from datetime import datetime
from typing import Optional
import aiosqlite
import yaml

_CONFIG_PATH = os.path.join(os.path.dirname(__file__), "..", "config.yaml")


def _db_path() -> str:
    with open(_CONFIG_PATH) as f:
        cfg = yaml.safe_load(f)
    return cfg["sqlite"]["path"]


async def init_db():
    path = _db_path()
    os.makedirs(os.path.dirname(path), exist_ok=True)
    async with aiosqlite.connect(path) as db:
        await db.execute("PRAGMA journal_mode=WAL;")
        await db.execute("""
            CREATE TABLE IF NOT EXISTS events (
                id TEXT PRIMARY KEY,
                llm TEXT NOT NULL,
                type TEXT NOT NULL,
                content TEXT NOT NULL,
                tags TEXT DEFAULT '[]',
                meta TEXT DEFAULT '{}',
                timestamp TEXT NOT NULL
            );
        """)
        await db.execute("""
            CREATE VIRTUAL TABLE IF NOT EXISTS events_fts
            USING fts5(id UNINDEXED, content, llm, tags);
        """)
        await db.commit()


async def insert_event(event: dict):
    path = _db_path()
    async with aiosqlite.connect(path) as db:
        await db.execute(
            "INSERT OR REPLACE INTO events VALUES (?,?,?,?,?,?,?)",
            (
                event["id"], event["llm"], event["type"],
                event["content"],
                json.dumps(event.get("tags", [])),
                json.dumps(event.get("meta", {})),
                event["timestamp"],
            ),
        )
        await db.execute(
            "INSERT OR REPLACE INTO events_fts(id, content, llm, tags) VALUES (?,?,?,?)",
            (event["id"], event["content"], event["llm"], json.dumps(event.get("tags", []))),
        )
        await db.commit()


async def keyword_search(query: str, llm: Optional[str] = None, limit: int = 10) -> list[dict]:
    path = _db_path()
    async with aiosqlite.connect(path) as db:
        db.row_factory = aiosqlite.Row
        if llm:
            rows = await db.execute_fetchall(
                """SELECT e.* FROM events_fts f
                   JOIN events e ON f.id = e.id
                   WHERE events_fts MATCH ? AND e.llm = ?
                   ORDER BY rank LIMIT ?""",
                (query, llm, limit),
            )
        else:
            rows = await db.execute_fetchall(
                """SELECT e.* FROM events_fts f
                   JOIN events e ON f.id = e.id
                   WHERE events_fts MATCH ?
                   ORDER BY rank LIMIT ?""",
                (query, limit),
            )
        return [dict(r) for r in rows]


async def list_events(
    llm: Optional[str] = None,
    type_: Optional[str] = None,
    since: Optional[datetime] = None,
    limit: int = 50,
) -> list[dict]:
    path = _db_path()
    filters, params = [], []
    if llm:
        filters.append("llm = ?")
        params.append(llm)
    if type_:
        filters.append("type = ?")
        params.append(type_)
    if since:
        filters.append("timestamp >= ?")
        params.append(since.isoformat())
    where = ("WHERE " + " AND ".join(filters)) if filters else ""
    params.append(limit)
    async with aiosqlite.connect(path) as db:
        db.row_factory = aiosqlite.Row
        rows = await db.execute_fetchall(
            f"SELECT * FROM events {where} ORDER BY timestamp DESC LIMIT ?", params
        )
        return [dict(r) for r in rows]
