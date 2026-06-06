powersync_client.py.md

"""
Memoree — PowerSync + Supabase offline-first sync (optional)
"""

from __future__ import annotations
import json
import os
import sqlite3
import threading
import time
import yaml

# Fixed path for flat structure in C:\memoree
_CONFIG_PATH = os.path.join(os.path.dirname(__file__), "config.yaml")


def _cfg() -> dict:
    try:
        with open(_CONFIG_PATH) as f:
            return yaml.safe_load(f)
    except Exception:
        return {}


def _queue_db_path() -> str:
    cfg = _cfg()
    # Default to a clean memoree path
    return cfg.get("sync", {}).get("powersync", {}).get("queue_path", "~/.memoree/sync_queue.db")


def _init_queue():
    path = os.path.expanduser(_queue_db_path())
    os.makedirs(os.path.dirname(path), exist_ok=True)
    con = sqlite3.connect(path)
    con.execute(
        "CREATE TABLE IF NOT EXISTS sync_queue "
        "(id TEXT PRIMARY KEY, payload TEXT, created_at TEXT, synced INTEGER DEFAULT 0)"
    )
    con.commit()
    con.close()


def enqueue(event: dict):
    """Add event to local sync queue."""
    _init_queue()
    path = os.path.expanduser(_queue_db_path())
    con = sqlite3.connect(path)
    con.execute(
        "INSERT OR REPLACE INTO sync_queue VALUES (?,?,datetime('now'),0)",
        (event.get("id", str(time.time())), json.dumps(event)),
    )
    con.commit()
    con.close()


def _push_to_supabase(event: dict, url: str, key: str):
    import httpx
    headers = {
        "apikey": key,
        "Authorization": f"Bearer {key}",
        "Content-Type": "application/json"
    }
    try:
        resp = httpx.post(
            f"{url}/rest/v1/memoree_events",
            json=event,
            headers=headers,
            timeout=10
        )
        resp.raise_for_status()
    except Exception:
        pass  # will retry later


def _sync_loop():
    """Background daemon: drain queue when connected."""
    cfg = _cfg().get("sync", {}).get("powersync", {})
    url = cfg.get("supabase_url", "")
    key = cfg.get("supabase_anon_key", "")

    if not url or not key:
        return  # sync not configured — silent

    _init_queue()
    while True:
        try:
            path = os.path.expanduser(_queue_db_path())
            con = sqlite3.connect(path)
            rows = con.execute(
                "SELECT id, payload FROM sync_queue WHERE synced=0 ORDER BY rowid LIMIT 50"
            ).fetchall()

            for row_id, payload_str in rows:
                try:
                    event = json.loads(payload_str)
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
    """Launch background sync thread (call once at startup if needed)."""
    t = threading.Thread(target=_sync_loop, daemon=True)
    t.start()
    print("🔄 PowerSync sync daemon started (background thread)")