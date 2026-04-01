"""Memoree v3 — PowerSync + Supabase offline-first sync.

Strategy:
  1. Every write_event also appends to a local sync_queue SQLite DB.
  2. A background thread checks connectivity; on WiFi push queue → Supabase REST.
  3. On reconnect, drain queue in FIFO order.

Requires in config.yaml:
  sync.powersync.supabase_url
  sync.powersync.supabase_anon_key
"""
from __future__ import annotations
import asyncio
import json
import os
import sqlite3
import threading
import time
from typing import Optional
import yaml

_CONFIG_PATH = os.path.join(os.path.dirname(__file__), "..", "config.yaml")


def _cfg() -> dict:
    with open(_CONFIG_PATH) as f:
        return yaml.safe_load(f)


def _queue_db_path() -> str:
    return _cfg()["sync"]["powersync"]["queue_path"]


def _init_queue():
    path = _queue_db_path()
    os.makedirs(os.path.dirname(path), exist_ok=True)
    con = sqlite3.connect(path)
    con.execute(
        "CREATE TABLE IF NOT EXISTS sync_queue (id TEXT, payload TEXT, created_at TEXT, synced INTEGER DEFAULT 0)"
    )
    con.commit()
    con.close()


def enqueue(event: dict):
    """Add event to local sync queue."""
    _init_queue()
    con = sqlite3.connect(_queue_db_path())
    con.execute(
        "INSERT INTO sync_queue VALUES (?,?,datetime('now'),0)",
        (event["id"], json.dumps(event)),
    )
    con.commit()
    con.close()


def _push_to_supabase(event: dict, url: str, key: str):
    import httpx
    headers = {"apikey": key, "Authorization": f"Bearer {key}", "Content-Type": "application/json"}
    resp = httpx.post(f"{url}/rest/v1/memoree_events", json=event, headers=headers, timeout=10)
    resp.raise_for_status()


def _sync_loop():
    """Background daemon: drain queue when connected."""
    cfg = _cfg()["sync"]["powersync"]
    url = cfg.get("supabase_url", "")
    key = cfg.get("supabase_anon_key", "")
    if not url or not key:
        return  # sync not configured — skip silently
    _init_queue()
    while True:
        try:
            con = sqlite3.connect(_queue_db_path())
            rows = con.execute(
                "SELECT id, payload FROM sync_queue WHERE synced=0 ORDER BY rowid LIMIT 50"
            ).fetchall()
            for row_id, payload_str in rows:
                event = json.loads(payload_str)
                try:
                    _push_to_supabase(event, url, key)
                    con.execute("UPDATE sync_queue SET synced=1 WHERE id=?", (row_id,))
                    con.commit()
                except Exception:
                    pass  # retry next cycle
            con.close()
        except Exception:
            pass
        time.sleep(30)  # poll every 30s


def start_sync_daemon():
    """Launch background sync thread (call once at startup)."""
    t = threading.Thread(target=_sync_loop, daemon=True)
    t.start()
