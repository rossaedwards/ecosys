"""Memoree v3 — SQLite raw ledger (aiosqlite + FTS5).
Full provenance: stores project, subproject, session_id, priority, confidence, mood.
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
                id          TEXT PRIMARY KEY,
                llm         TEXT NOT NULL,
                model       TEXT DEFAULT '',
                type        TEXT NOT NULL DEFAULT 'episodic',
                project     TEXT DEFAULT 'memoree-v3',
                subproject  TEXT DEFAULT '',
                session_id  TEXT DEFAULT '',
                content     TEXT NOT NULL,
                tags        TEXT DEFAULT '[]',
                meta        TEXT DEFAULT '{}',
                priority    INTEGER DEFAULT 5,
                confidence  REAL DEFAULT 0.8,
                mood        TEXT DEFAULT 'neutral',
                source      TEXT DEFAULT 'api',
                timestamp   TEXT NOT NULL
            );
        """)
        await db.execute("""
            CREATE VIRTUAL TABLE IF NOT EXISTS events_fts
            USING fts5(id UNINDEXED, content, llm, project, tags);
        """)
        # Indexes for common filter patterns
        await db.execute("CREATE INDEX IF NOT EXISTS idx_llm       ON events(llm);")
        await db.execute("CREATE INDEX IF NOT EXISTS idx_project   ON events(project);")
        await db.execute("CREATE INDEX IF NOT EXISTS idx_type      ON events(type);")
        await db.execute("CREATE INDEX IF NOT EXISTS idx_timestamp ON events(timestamp DESC);")
        await db.commit()


async def insert_event(event: dict):
    path = _db_path()
    async with aiosqlite.connect(path) as db:
        await db.execute(
            """
            INSERT OR REPLACE INTO events
            (id, llm, model, type, project, subproject, session_id,
             content, tags, meta, priority, confidence, mood, source, timestamp)
            VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)
            """,
            (
                event["id"],
                event["llm"],
                event.get("model", ""),
                event.get("type", "episodic"),
                event.get("project", "memoree-v3"),
                event.get("subproject", ""),
                event.get("session_id", ""),
                event["content"],
                json.dumps(event.get("tags", [])),
                json.dumps(event.get("meta", {})),
                int(event.get("priority", 5)),
                float(event.get("confidence", 0.8)),
                event.get("mood", "neutral"),
                event.get("source", "api"),
                event["timestamp"],
            ),
        )
        await db.execute(
            "INSERT OR REPLACE INTO events_fts(id, content, llm, project, tags) VALUES (?,?,?,?,?)",
            (
                event["id"],
                event["content"],
                event["llm"],
                event.get("project", ""),
                json.dumps(event.get("tags", [])),
            ),
        )
        await db.commit()


async def keyword_search(
    query: str,
    llm:     Optional[str] = None,
    project: Optional[str] = None,
    limit:   int           = 10,
) -> list[dict]:
    path = _db_path()
    async with aiosqlite.connect(path) as db:
        db.row_factory = aiosqlite.Row
        base = """
            SELECT e.* FROM events_fts f
            JOIN events e ON f.id = e.id
            WHERE events_fts MATCH ?
        """
        params: list = [query]
        if llm:
            base += " AND e.llm = ?"
            params.append(llm)
        if project:
            base += " AND e.project = ?"
            params.append(project)
        base += " ORDER BY rank LIMIT ?"
        params.append(limit)
        rows = await db.execute_fetchall(base, params)
        return [dict(r) for r in rows]


async def list_events(
    llm:     Optional[str]      = None,
    type_:   Optional[str]      = None,
    project: Optional[str]      = None,
    since:   Optional[datetime] = None,
    limit:   int                = 50,
) -> list[dict]:
    path = _db_path()
    filters, params = [], []
    if llm:     filters.append("llm = ?");     params.append(llm)
    if type_:   filters.append("type = ?");    params.append(type_)
    if project: filters.append("project = ?"); params.append(project)
    if since:   filters.append("timestamp >= ?"); params.append(since.isoformat())
    where  = ("WHERE " + " AND ".join(filters)) if filters else ""
    params.append(limit)
    async with aiosqlite.connect(path) as db:
        db.row_factory = aiosqlite.Row
        rows = await db.execute_fetchall(
            f"SELECT * FROM events {where} ORDER BY timestamp DESC LIMIT ?", params
        )
        return [dict(r) for r in rows]
