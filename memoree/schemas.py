"""
Aurphyx_Memoree — Core Memory Schemas
Four memory types: Episodic, Semantic, Procedural, Meta
f0rg3d in l0v3 by Ross Edwards
"""

from __future__ import annotations
from datetime import datetime
from enum import Enum
from typing import Dict, List, Optional, Any
from pydantic import BaseModel, Field
import uuid


class MemoryType(str, Enum):
    EPISODIC   = "episodic"
    SEMANTIC   = "semantic"
    PROCEDURAL = "procedural"
    META       = "meta"


class EpisodicMemory(BaseModel):
    id: str = Field(default_factory=lambda: str(uuid.uuid4()))
    session_id: str
    role: str                          # user | assistant | system
    content: str
    timestamp: datetime = Field(default_factory=datetime.utcnow)
    llm: str = "perplexity"            # perplexity | claude | cursor | gemini | grok | copilot
    user_preferences: Dict[str, Any] = Field(default_factory=dict)
    tags: List[str] = Field(default_factory=list)
    memory_type: MemoryType = MemoryType.EPISODIC


class SemanticMemory(BaseModel):
    id: str = Field(default_factory=lambda: str(uuid.uuid4()))
    project: str
    category: str                      # spec | architecture | axioms | canon | etc.
    content: str
    tags: List[str] = Field(default_factory=list)
    relationships: List[str] = Field(default_factory=list)
    timestamp: datetime = Field(default_factory=datetime.utcnow)
    memory_type: MemoryType = MemoryType.SEMANTIC


class ProceduralMemory(BaseModel):
    id: str = Field(default_factory=lambda: str(uuid.uuid4()))
    task: str
    steps: List[str]
    frequency: int = 1
    success_rate: float = 1.0
    last_executed: Optional[datetime] = None
    timestamp: datetime = Field(default_factory=datetime.utcnow)
    memory_type: MemoryType = MemoryType.PROCEDURAL


class MetaMemory(BaseModel):
    id: str = Field(default_factory=lambda: str(uuid.uuid4()))
    fact: str
    confidence: float = 1.0
    sources: List[str] = Field(default_factory=list)
    verified: bool = False
    deprecated: bool = False
    last_accessed: datetime = Field(default_factory=datetime.utcnow)
    timestamp: datetime = Field(default_factory=datetime.utcnow)
    memory_type: MemoryType = MemoryType.META


class ContextResponse(BaseModel):
    project: str
    llm: str
    session_id: Optional[str]
    last_summary: Optional[str]
    episodic: List[Dict[str, Any]] = Field(default_factory=list)
    semantic: List[Dict[str, Any]] = Field(default_factory=list)
    active_volumes: List[str] = Field(default_factory=list)
    active_axioms: List[str] = Field(default_factory=list)
    active_dualities: List[str] = Field(default_factory=list)
    invariants: List[str] = Field(default_factory=list)


class ThreadSummary(BaseModel):
    session_id: str
    short_summary: str
    long_summary: str
    key_topics: List[str] = Field(default_factory=list)
    aps_refs: List[str] = Field(default_factory=list)
    embedding_cluster_ids: List[str] = Field(default_factory=list)
    timestamp: datetime = Field(default_factory=datetime.utcnow)
