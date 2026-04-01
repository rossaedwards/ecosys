"""Memoree v3 — Pydantic v2 schemas."""
from __future__ import annotations
from datetime import datetime, timezone
from typing import Any, Literal, Optional
from uuid import uuid4
from pydantic import BaseModel, Field

MemoryType = Literal["episodic", "semantic", "procedural", "meta"]


class MemoryEvent(BaseModel):
    """A single memory unit written to the ledger."""
    id: str = Field(default_factory=lambda: str(uuid4()))
    llm: str = Field(..., description="Source LLM: gemini | perplexity | supergrok | nemotron | qwen | etc.")
    type: MemoryType = "episodic"
    content: str
    tags: list[str] = []
    meta: dict[str, Any] = {}
    embedding: Optional[list[float]] = Field(default=None, exclude=True)
    timestamp: datetime = Field(default_factory=lambda: datetime.now(timezone.utc))


class ContextQuery(BaseModel):
    """Query parameters for read_context / query_context."""
    query: str
    llm: Optional[str] = None           # filter by source LLM
    type: Optional[MemoryType] = None   # filter by memory type
    top_k: int = Field(default=5, ge=1, le=50)
    since: Optional[datetime] = None    # timestamp floor
    tags: list[str] = []


class ContextResult(BaseModel):
    """A single ranked result from read_context."""
    event: MemoryEvent
    score: float


class WriteResponse(BaseModel):
    id: str
    status: Literal["ok", "error"]
    message: str = ""


class ReadResponse(BaseModel):
    results: list[ContextResult]
    total: int
    query: str
