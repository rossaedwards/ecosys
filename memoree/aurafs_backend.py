"""
Aurphyx_Memoree — AuraFS JSON Backend
Fractal-sharded sovereign filesystem persistence
f0rg3d in l0v3 by Ross Edwards
"""

import os
import json
import hashlib
from datetime import datetime
from pathlib import Path
from typing import Any, Dict, List, Optional

from core.schemas import EpisodicMemory, SemanticMemory, ProceduralMemory, MetaMemory


class AuraFSBackend:
    """
    Wraps AuraFS shard structure for Memoree.
    Mirrors aurafs_local.py pattern with extended category paths.
    Path: {root}/shards/{memory_type}/{project_or_session}/{id}.json
    """

    def __init__(self, root_path: str = "~/.aurphyx/memoree"):
        self.root = Path(os.path.expanduser(root_path))
        for sub in ["shards/episodic", "shards/semantic", "shards/procedural",
                    "shards/meta", "shards/aps_canon", "index", "state", "summaries"]:
            (self.root / sub).replace if False else (self.root / sub).mkdir(parents=True, exist_ok=True)

    def _shard_id(self, content: str) -> str:
        raw = f"{content}{datetime.utcnow().isoformat()}"
        return hashlib.blake2b(raw.encode(), digest_size=10).hexdigest()

    def _write(self, category: str, sub: str, obj_id: str, data: Dict[str, Any]) -> Path:
        path = self.root / "shards" / category / sub
        path.mkdir(parents=True, exist_ok=True)
        file_path = path / f"{obj_id}.json"
        with open(file_path, "w", encoding="utf-8") as f:
            json.dump(data, f, indent=2, default=str)
        return file_path

    def _read_dir(self, category: str, sub: str, limit: int = 20) -> List[Dict]:
        path = self.root / "shards" / category / sub
        if not path.exists():
            return []
        files = sorted(path.glob("*.json"), key=os.path.getmtime, reverse=True)
        results = []
        for fp in files[:limit]:
            with open(fp, "r", encoding="utf-8") as f:
                results.append(json.load(f))
        return results

    # ── Writers ──────────────────────────────────────────────────────────────

    def write_episodic(self, mem: EpisodicMemory) -> str:
        self._write("episodic", mem.session_id, mem.id, mem.model_dump())
        return mem.id

    def write_semantic(self, mem: SemanticMemory) -> str:
        self._write("semantic", f"{mem.project}__{mem.category}", mem.id, mem.model_dump())
        return mem.id

    def write_procedural(self, mem: ProceduralMemory) -> str:
        self._write("procedural", "all", mem.id, mem.model_dump())
        return mem.id

    def write_meta(self, mem: MetaMemory) -> str:
        self._write("meta", "all", mem.id, mem.model_dump())
        return mem.id

    # ── Readers ──────────────────────────────────────────────────────────────

    def read_episodic(self, session_id: str, limit: int = 20) -> List[Dict]:
        return self._read_dir("episodic", session_id, limit)

    def read_semantic(self, project: str, category: Optional[str] = None, limit: int = 20) -> List[Dict]:
        sub = f"{project}__{category}" if category else project
        results = self._read_dir("semantic", sub, limit)
        if not results and not category:
            # glob all categories for project
            base = self.root / "shards" / "semantic"
            all_results = []
            for d in base.glob(f"{project}__*"):
                all_results.extend(self._read_dir("semantic", d.name, limit))
            return all_results[:limit]
        return results

    def read_meta(self, limit: int = 50) -> List[Dict]:
        return self._read_dir("meta", "all", limit)

    # ── State helpers ────────────────────────────────────────────────────────

    def read_state(self, key: str) -> Dict:
        fp = self.root / "state" / f"{key}.json"
        if fp.exists():
            with open(fp, "r", encoding="utf-8") as f:
                return json.load(f)
        return {}

    def write_state(self, key: str, data: Dict) -> None:
        fp = self.root / "state" / f"{key}.json"
        with open(fp, "w", encoding="utf-8") as f:
            json.dump(data, f, indent=2, default=str)

    # ── Summary helpers ──────────────────────────────────────────────────────

    def write_summary(self, session_id: str, summary: Dict) -> None:
        path = self.root / "summaries"
        path.mkdir(exist_ok=True)
        fp = path / f"{session_id}.json"
        with open(fp, "w", encoding="utf-8") as f:
            json.dump(summary, f, indent=2, default=str)

    def read_summary(self, session_id: str) -> Optional[Dict]:
        fp = self.root / "summaries" / f"{session_id}.json"
        if fp.exists():
            with open(fp, "r", encoding="utf-8") as f:
                return json.load(f)
        return None
