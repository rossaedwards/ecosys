"""
Aurphyx_Memoree — ChromaDB Vector Backend
Local, sovereign, AuraFS-aligned. No API key. No server.
Collections: episodic | semantic | procedural | meta | aps_canon
f0rg3d in l0v3 by Ross Edwards
"""

import os
from pathlib import Path
from typing import Any, Dict, List, Optional

try:
    import chromadb
    from chromadb.config import Settings
    from sentence_transformers import SentenceTransformer
    CHROMA_AVAILABLE = True
except ImportError:
    CHROMA_AVAILABLE = False
    print("⚠️  ChromaDB not installed. Run: pip install chromadb sentence-transformers")


COLLECTIONS = ["episodic", "semantic", "procedural", "meta", "aps_canon"]
DEFAULT_MODEL = "all-MiniLM-L6-v2"


class VectorBackend:
    """
    ChromaDB local persistent vector store.
    Embedding model: all-MiniLM-L6-v2 (local, fast, sovereign).
    """

    def __init__(self, persist_dir: str = "~/.aurphyx/memoree/embeddings",
                 model_name: str = DEFAULT_MODEL):
        self.persist_dir = Path(os.path.expanduser(persist_dir))
        self.persist_dir.mkdir(parents=True, exist_ok=True)
        self.available = CHROMA_AVAILABLE

        if self.available:
            self.client = chromadb.PersistentClient(
                path=str(self.persist_dir),
                settings=Settings(anonymized_telemetry=False)
            )
            self.model = SentenceTransformer(model_name)
            self._collections = {
                name: self.client.get_or_create_collection(name)
                for name in COLLECTIONS
            }
            print(f"✨ [VectorDB] ChromaDB ready at {self.persist_dir}")
        else:
            self._collections = {}

    def _embed(self, text: str) -> List[float]:
        return self.model.encode(text).tolist()

    def upsert(self, collection: str, doc_id: str, text: str, metadata: Dict[str, Any]) -> bool:
        if not self.available:
            return False
        col = self._collections.get(collection)
        if col is None:
            return False
        embedding = self._embed(text)
        col.upsert(
            ids=[doc_id],
            embeddings=[embedding],
            documents=[text],
            metadatas=[{k: str(v) for k, v in metadata.items()}]
        )
        return True

    def query(self, collection: str, query_text: str, n_results: int = 5,
              where: Optional[Dict] = None) -> List[Dict]:
        if not self.available:
            return []
        col = self._collections.get(collection)
        if col is None:
            return []
        embedding = self._embed(query_text)
        kwargs: Dict[str, Any] = {"query_embeddings": [embedding], "n_results": n_results}
        if where:
            kwargs["where"] = where
        results = col.query(**kwargs)
        out = []
        for i, doc_id in enumerate(results["ids"][0]):
            out.append({
                "id": doc_id,
                "document": results["documents"][0][i],
                "metadata": results["metadatas"][0][i],
                "distance": results["distances"][0][i],
            })
        return out

    def delete(self, collection: str, doc_id: str) -> bool:
        if not self.available:
            return False
        col = self._collections.get(collection)
        if col:
            col.delete(ids=[doc_id])
            return True
        return False
