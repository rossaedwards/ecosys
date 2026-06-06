memori_bridge.py.md
"""
Memoree — Memori Bridge
"""

from typing import Any, Dict, List, Optional

try:
    from aurphyx_memori import AurphyxMemori
    MEMORI_AVAILABLE = True
except ImportError:
    MEMORI_AVAILABLE = False
    print("aurphyx_memori.py not found. Memori bridge disabled.")


class MemoriBridge:
    def __init__(self):
        self.available = MEMORI_AVAILABLE
        if self.available:
            self.am = AurphyxMemori()
            print("[MemoriBridge] AurphyxMemori connected")
        else:
            self.am = None

    def mirror_episodic(self, role: str, content: str, llm: str = "perplexity") -> bool:
        if not self.available:
            return False
        self.am.store_conversation(role=role, content=content, llm=llm)
        return True

    def mirror_semantic(self, project: str, category: str, content: str,
                        tags: List[str], relationships: Optional[List[str]] = None) -> bool:
        if not self.available:
            return False
        self.am.store_project_knowledge(
            project=project, category=category,
            content=content, tags=tags, relationships=relationships
        )
        return True

    def mirror_procedural(self, task: str, steps: List[str],
                          success: bool = True, frequency: int = 1,
                          success_rate: float = 1.0) -> bool:
        if not self.available:
            return False
        self.am.store_workflow(
            task=task, steps=steps,
            success=success, frequency=frequency, success_rate=success_rate
        )
        return True

    def mirror_meta(self, fact: str, confidence: float = 1.0,
                    sources: Optional[List[str]] = None, verified: bool = False) -> bool:
        if not self.available:
            return False
        self.am.store_fact(
            fact=fact, confidence=confidence,
            sources=sources or [], verified=verified
        )
        return True