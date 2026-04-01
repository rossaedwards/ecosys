"""Memoree v3 — Rich Pydantic v2 schemas.
Every memory entry carries full provenance metadata.
"""
from __future__ import annotations
from datetime import datetime, timezone
from typing import Any, Literal, Optional
from uuid import uuid4
from pydantic import BaseModel, Field

MemoryType = Literal["episodic", "semantic", "procedural", "meta"]
MoodType   = Literal["neutral", "focused", "inspired", "debug", "urgent"]


class MemoryEvent(BaseModel):
    """A single sovereign memory unit."""
    # Identity
    id:          str = Field(default_factory=lambda: str(uuid4()))
    # Source provenance
    llm:         str  = Field(...,  description="grok | gemini | perplexity | nemotron | qwen | etc.")
    model:       str  = Field("",   description="Exact model name e.g. gemini-2.5-pro")
    # Classification
    type:        MemoryType = "episodic"
    project:     str  = Field("memoree-v3", description="Top-level project slug")
    subproject:  str  = Field("",            description="Sub-feature or experiment")
    session_id:  str  = Field("",            description="Conversation session UUID")
    # Content
    content:     str
    tags:        list[str]       = []
    meta:        dict[str, Any]  = {}
    # Numeric quality signals
    priority:    int   = Field(default=5,   ge=1, le=10)
    confidence:  float = Field(default=0.8, ge=0.0, le=1.0)
    mood:        MoodType = "neutral"
    source:      str  = Field("api", description="api | hook | manual | ws")
    # Internal (excluded from serialization to clients)
    embedding:   Optional[list[float]] = Field(default=None, exclude=True)
    # Timestamp
    timestamp:   datetime = Field(default_factory=lambda: datetime.now(timezone.utc))

    model_config = {"populate_by_name": True}


class ContextQuery(BaseModel):
    """Semantic recall request."""
    query:      str
    llm:        Optional[str]         = None
    type:       Optional[MemoryType]  = None
    project:    Optional[str]         = None
    tags:       list[str]             = []
    top_k:      int                   = Field(default=8, ge=1, le=100)
    since:      Optional[datetime]    = None
    min_confidence: float             = Field(default=0.0, ge=0.0, le=1.0)
    min_priority:   int               = Field(default=1,   ge=1, le=10)
    hybrid:     bool                  = True   # use hybrid search when backend supports it


class ContextResult(BaseModel):
    event:   MemoryEvent
    score:   float
    source:  Literal["vector", "keyword", "hybrid"] = "vector"


class WriteResponse(BaseModel):
    id:      str
    status:  Literal["ok", "error"]
    message: str = ""


class ReadResponse(BaseModel):
    results: list[ContextResult]
    total:   int
    query:   str
    backend: str   # which vector backend served this query


# WebSocket message types
class WSIncoming(BaseModel):
    action:  Literal["write", "read", "ping"]
    payload: dict[str, Any]


class WSOutgoing(BaseModel):
    action:  str
    data:    Any
    ok:      bool = True
    error:   str  = ""
