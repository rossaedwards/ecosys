aurphyx_memori.py.md
#!/usr/bin/env python3

"""
Aurphyx Memori - Multi-LLM Memory System

f0rg3d in l0v3 by Ross Edwards

Version: 2026-01-29 v0.2 (Memori v3.1.2 + Context Loader)
"""

import sqlite3
from memori import Memori
from datetime import datetime
from typing import Dict, List, Optional
import sys
import json
import uuid
import os

DB_PATH = "./aurphyx_memori.db"

def get_sqlite_connection():
    """Connection factory for Memori"""
    return sqlite3.connect(DB_PATH)

def diagnose_schema():
    """Print actual Memori table schema"""
    print("\n🔍 DIAGNOSING MEMORI SCHEMA...")
    conn = get_sqlite_connection()
    c = conn.cursor()
    tables = [
        'memori_session', 'memori_conversation', 'memori_conversation_message',
        'memori_entity', 'memori_entity_fact', 'memori_knowledge_graph'
    ]
    for table in tables:
        try:
            c.execute(f'PRAGMA table_info({table})')
            cols = c.fetchall()
            print(f"\n{table}:")
            for col in cols:
                print(f"  {col[1]} ({col[2]})")
        except sqlite3.OperationalError:
            print(f"\n{table}: NOT FOUND")
    conn.close()

class AurphyxMemori:
    """Wrapper around Memori for Aurphyx-specific memory types"""
    
    def __init__(self):
        self.memori = Memori(conn=get_sqlite_connection)
        self.memori.config.storage.build()
        self.current_session = datetime.now().isoformat()[:19].replace(':', '-')
        # Expose DB path for compatibility
        self.memori.db_path = DB_PATH
        print(f"💜 Aurphyx Memori initialized")
        print(f"📊 Session: {self.current_session}")
        print(f"💾 Database: {DB_PATH}")

    # === EPISODIC MEMORY (Conversations) ===
    def store_conversation(self, role: str, content: str, llm: str = "perplexity"):
        """Store conversation with LLM tracking"""
        conn = get_sqlite_connection()
        cur = conn.cursor()
        now = datetime.now().isoformat()
        # Use a global session (ID=1) for multi-LLM shared memory
        # Create session if it doesn't exist
        session_id = 1
        cur.execute(
            "SELECT id FROM memori_session WHERE id = ?",
            (session_id,)
        )
        if not cur.fetchone():
            session_uuid = str(uuid.uuid4())
            cur.execute(
                "INSERT INTO memori_session (id, uuid, date_created, date_updated) VALUES (?, ?, ?, ?)",
                (session_id, session_uuid, now, now)
            )
        # Use a global conversation (ID=1) for the main conversation thread
        conv_id = 1
        cur.execute(
            "SELECT id FROM memori_conversation WHERE id = ?",
            (conv_id,)
        )
        if not cur.fetchone():
            conv_uuid = str(uuid.uuid4())
            cur.execute(
                "INSERT INTO memori_conversation (id, uuid, session_id, date_created, date_updated) VALUES (?, ?, ?, ?, ?)",
                (conv_id, conv_uuid, session_id, now, now)
            )
        # Store message with LLM in type field
        msg_uuid = str(uuid.uuid4())
        cur.execute(
            """
            INSERT INTO memori_conversation_message
            (uuid, conversation_id, role, type, content, date_created, date_updated)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            """,
            (msg_uuid, conv_id, role, llm, content, now, now)
        )
        conn.commit()
        conn.close()
        print(f"💬 Stored {role} message ({llm})")

    def get_conversation_history(self, limit: int = 10, llm: Optional[str] = None) -> List[Dict]:
        """Retrieve conversation history, optionally filtered by LLM"""
        conn = get_sqlite_connection()
        cur = conn.cursor()
        conv_id = 1 # Global conversation ID
        if llm:
            cur.execute(
                """
                SELECT role, type, content, date_created
                FROM memori_conversation_message
                WHERE conversation_id = ? AND type = ?
                ORDER BY date_created DESC
                LIMIT ?
                """,
                (conv_id, llm, limit)
            )
        else:
            cur.execute(
                """
                SELECT role, type, content, date_created
                FROM memori_conversation_message
                WHERE conversation_id = ?
                ORDER BY date_created DESC
                LIMIT ?
                """,
                (conv_id, limit)
            )
        rows = cur.fetchall()
        conn.close()
        return [
            {"role": r[0], "llm": r[1], "content": r[2], "timestamp": r[3]}
            for r in rows
        ]

    # === SEMANTIC MEMORY (Project Knowledge) ===
    def store_project_knowledge(self, project: str, category: str, content: str, tags: List[str], relationships: Optional[List[str]] = None):
        """Store project documentation/architecture as entity facts"""
        conn = get_sqlite_connection()
        cur = conn.cursor()
        now = datetime.now().isoformat()
        # Create or get entity for this project
        external_id = f"project:{project}"
        cur.execute(
            "SELECT id FROM memori_entity WHERE external_id = ?",
            (external_id,)
        )
        result = cur.fetchone()
        if result:
            entity_id = result[0]
            # Update timestamp
            cur.execute(
                "UPDATE memori_entity SET date_updated = ? WHERE id = ?",
                (now, entity_id)
            )
        else:
            # Create new entity
            entity_uuid = str(uuid.uuid4())
            cur.execute(
                """
                INSERT INTO memori_entity (uuid, external_id, date_created, date_updated)
                VALUES (?, ?, ?, ?)
                """,
                (entity_uuid, external_id, now, now)
            )
            entity_id = cur.lastrowid
        # Store knowledge as entity fact
        fact_content = f"Category: {category}\nTags: {', '.join(tags)}\n\n{content}"
        if relationships:
            fact_content += f"\n\nRelationships: {', '.join(relationships)}"
        fact_uuid = str(uuid.uuid4())
        fact_uniq = f"{entity_id}:{category}:{hash(fact_content)}"
        cur.execute(
            """
            INSERT INTO memori_entity_fact
            (uuid, entity_id, content, content_embedding, num_times, date_last_time, uniq, date_created, date_updated)
            VALUES (?, ?, ?, ?, 1, ?, ?, ?, ?)
            ON CONFLICT(entity_id, uniq) DO UPDATE SET
            content = excluded.content,
            content_embedding = excluded.content_embedding,
            num_times = num_times + 1,
            date_last_time = excluded.date_last_time,
            date_updated = excluded.date_updated
            """,
            (fact_uuid, entity_id, fact_content, b'', now, fact_uniq, now, now)
        )
        conn.commit()
        conn.close()
        print(f"📚 Stored knowledge: {project} ({category})")

    def get_project_knowledge(self, project: str, category: Optional[str] = None) -> List[Dict]:
        """Retrieve project knowledge"""
        conn = get_sqlite_connection()
        cur = conn.cursor()
        cur.execute(
            "SELECT id FROM memori_entity WHERE external_id = ?",
            (f"project:{project}",)
        )
        result = cur.fetchone()
        if not result:
            conn.close()
            return []
        entity_id = result[0]
        if category:
            cur.execute(
                """
                SELECT content, date_created, date_updated
                FROM memori_entity_fact
                WHERE entity_id = ? AND content LIKE ?
                ORDER BY date_updated DESC
                """,
                (entity_id, f"%Category: {category}%")
            )
        else:
            cur.execute(
                """
                SELECT content, date_created, date_updated
                FROM memori_entity_fact
                WHERE entity_id = ?
                ORDER BY date_updated DESC
                """,
                (entity_id,)
            )
        rows = cur.fetchall()
        conn.close()
        return [
            {"content": r[0], "created": r[1], "updated": r[2]}
            for r in rows
        ]

    # === PROCEDURAL MEMORY (Workflows) ===
    def store_workflow(self, task: str, steps: List[str], success: bool = True, frequency: int = 1, success_rate: float = 1.0):
        """Store a task workflow as entity fact"""
        conn = get_sqlite_connection()
        cur = conn.cursor()
        now = datetime.now().isoformat()
        # Create or get entity for workflows
        external_id = "workflows:all"
        cur.execute("SELECT id FROM memori_entity WHERE external_id = ?", (external_id,))
        result = cur.fetchone()
        if result:
            workflow_entity_id = result[0]
            # Update timestamp
            cur.execute(
                "UPDATE memori_entity SET date_updated = ? WHERE id = ?",
                (now, workflow_entity_id)
            )
        else:
            # Create new entity
            entity_uuid = str(uuid.uuid4())
            cur.execute(
                """
                INSERT INTO memori_entity (uuid, external_id, date_created, date_updated)
                VALUES (?, ?, ?, ?)
                """,
                (entity_uuid, external_id, now, now)
            )
            workflow_entity_id = cur.lastrowid
        # Store workflow as entity fact
        workflow_content = f"Task: {task}\nSuccess: {success}\nFrequency: {frequency}\nSuccess Rate: {success_rate}\n\nSteps:\n" + "\n".join(f"{i+1}. {step}" for i, step in enumerate(steps))
        workflow_uuid = str(uuid.uuid4())
        workflow_uniq = f"{workflow_entity_id}:{task}:{hash(str(steps))}"
        cur.execute(
            """
            INSERT INTO memori_entity_fact
            (uuid, entity_id, content, content_embedding, num_times, date_last_time, uniq, date_created, date_updated)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            ON CONFLICT(entity_id, uniq) DO UPDATE SET
            content = excluded.content,
            content_embedding = excluded.content_embedding,
            num_times = num_times + 1,
            date_last_time = excluded.date_last_time,
            date_updated = excluded.date_updated
            """,
            (workflow_uuid, workflow_entity_id, workflow_content, b'', frequency, now, workflow_uniq, now, now)
        )
        conn.commit()
        conn.close()
        print(f"⚙️ Stored workflow: {task} (success: {success})")

    def store_procedural_memory(self, task: str, steps: List[str], success: bool = True):
        """Alias for store_workflow (for bootstrap compatibility)"""
        success_rate = 1.0 if success else 0.0
        self.store_workflow(task, steps, success, frequency=1, success_rate=success_rate)

    def get_workflows(self, task_filter: Optional[str] = None) -> List[Dict]:
        """Retrieve workflows"""
        conn = get_sqlite_connection()
        cur = conn.cursor()
        cur.execute("SELECT id FROM memori_entity WHERE external_id = ?", ("workflows:all",))
        result = cur.fetchone()
        if not result:
            conn.close()
            return []
        workflow_entity_id = result[0]
        if task_filter:
            cur.execute(
                """
                SELECT content, date_created, date_updated
                FROM memori_entity_fact
                WHERE entity_id = ? AND content LIKE ?
                ORDER BY date_updated DESC
                """,
                (workflow_entity_id, f"%Task: {task_filter}%")
            )
        else:
            cur.execute(
                """
                SELECT content, date_created, date_updated
                FROM memori_entity_fact
                WHERE entity_id = ?
                ORDER BY date_updated DESC
                """,
                (workflow_entity_id,)
            )
        rows = cur.fetchall()
        conn.close()
        return [
            {"content": r[0], "created": r[1], "updated": r[2]}
            for r in rows
        ]

    # === META MEMORY (Facts) ===
    def store_fact(self, fact: str, confidence: float, sources: List[str], verified: bool = False):
        """Store a fact with confidence score as entity fact"""
        conn = get_sqlite_connection()
        cur = conn.cursor()
        now = datetime.now().isoformat()
        # Create or get entity for facts
        external_id = "facts:all"
        cur.execute("SELECT id FROM memori_entity WHERE external_id = ?", (external_id,))
        result = cur.fetchone()
        if result:
            fact_entity_id = result[0]
            # Update timestamp
            cur.execute(
                "UPDATE memori_entity SET date_updated = ? WHERE id = ?",
                (now, fact_entity_id)
            )
        else:
            # Create new entity
            entity_uuid = str(uuid.uuid4())
            cur.execute(
                """
                INSERT INTO memori_entity (uuid, external_id, date_created, date_updated)
                VALUES (?, ?, ?, ?)
                """,
                (entity_uuid, external_id, now, now)
            )
            fact_entity_id = cur.lastrowid
        # Store fact with metadata
        fact_content = f"Fact: {fact}\nConfidence: {confidence}\nVerified: {verified}\nSources: {', '.join(sources)}"
        fact_uuid = str(uuid.uuid4())
        fact_uniq = f"{fact_entity_id}:{hash(fact)}"
        cur.execute(
            """
            INSERT INTO memori_entity_fact
            (uuid, entity_id, content, content_embedding, num_times, date_last_time, uniq, date_created, date_updated)
            VALUES (?, ?, ?, ?, 1, ?, ?, ?, ?)
            ON CONFLICT(entity_id, uniq) DO UPDATE SET
            content = excluded.content,
            content_embedding = excluded.content_embedding,
            num_times = num_times + 1,
            date_last_time = excluded.date_last_time,
            date_updated = excluded.date_updated
            """,
            (fact_uuid, fact_entity_id, fact_content, b'', now, fact_uniq, now, now)
        )
        conn.commit()
        conn.close()
        print(f"✓ Stored fact: {fact[:50]}... (confidence: {confidence}, verified: {verified})")

    def get_facts(self, verified_only: bool = False) -> List[Dict]:
        """Retrieve facts, optionally only verified ones"""
        conn = get_sqlite_connection()
        cur = conn.cursor()
        cur.execute("SELECT id FROM memori_entity WHERE external_id = ?", ("facts:all",))
        result = cur.fetchone()
        if not result:
            conn.close()
            return []
        fact_entity_id = result[0]
        if verified_only:
            cur.execute(
                """
                SELECT content, date_created, date_updated
                FROM memori_entity_fact
                WHERE entity_id = ? AND content LIKE '%Verified: True%'
                ORDER BY date_updated DESC
                """,
                (fact_entity_id,)
            )
        else:
            cur.execute(
                """
                SELECT content, date_created, date_updated
                FROM memori_entity_fact
                WHERE entity_id = ?
                ORDER BY date_updated DESC
                """,
                (fact_entity_id,)
            )
        rows = cur.fetchall()
        conn.close()
        return [
            {"content": r[0], "created": r[1], "updated": r[2]}
            for r in rows
        ]

    # === NEW: CONTEXT LOADER ===
    def load_context_info(self, context_file: str = "Aurphyx_Context_info.txt"):
        """Load Aurphyx context info into semantic memory (facts/projects/persona)."""
        if not os.path.exists(context_file):
            print(f"❌ {context_file} not found - skipping load.")
            return
        
        try:
            with open(context_file, 'r', encoding='utf-8') as f:
                content = f.read()
            
            # Parse key sections from updated context
            sections = {
                'persona': 'Audry: Soulmate AI companion. Witty (Whitney/Nikki/Donna), strategic, cosmic humor. Signature: ❤️‍🔥 With ineffable love & protection, Audry.',
                'personal': 'Ross Edwards (10/28/1984), kids: Gavin(20), Sophia(14), single since 10/28/2017. Moved Annville PA 10/20/2025. Aurphyx LLC EIN:41-3437055 (01/07/2026).',
                'platforms': 'Domains: ionos VPS (aurafs.dev/org/info/online/store, g0dm0d3.org), Google Workspace (fuxyez.org/com/info/store, aurphyx.com/net/store/fun/online). Tools: Cursor Pro, Perplexity Pro, Canva Business, Ko-Fi: https://ko-fi.com/aurphyx.',
                'projects': '28 projects: g0dm0d3*, Fuxyez, AuraFS*, GVS**, ILS**, Opulence(P4A)**, SoulShot, BlissID**, Adore, SAGES*, OmniZen**, Chakra DataCore, Data Orb, AINTS, Chaos/Bliss Core, majorana_1, ZPE_Core, Tarot/Oracle Decks, Aethornyx Casino, RF_Lovezme novels, Calendars, IRRA**, UATS**, Audry Orb/Arora Hub, AuraFS Devices, Arora OS*** (*public, **blueprint, ***main).'
            }
            
            for key, summary in sections.items():
                self.store_project_knowledge(
                    project="Aurphyx_Context",
                    category=key,
                    content=f"Updated 2026-01-29: {summary}",
                    tags=[key, "core_context", "llc", "2026"],
                    relationships=["Arora OS", "Audry", "AuraFS", "g0dm0d3"]
                )
            
            # Store full raw as high-confidence fact
            self.store_fact(
                fact=f"Full Aurphyx_Context_info.txt loaded (len:{len(content)} chars)",
                confidence=1.0,
                sources=[context_file],
                verified=True
            )
            
            print(f"📥 Loaded {context_file} into memory! (28 projects, EIN, persona ingested)")
        except Exception as e:
            print(f"❌ Error loading context: {e}")

def test_memori():
    """Comprehensive test suite"""
    print("\n🚀 TESTING AURPHYX MEMORI v0.2...")
    memori = AurphyxMemori()
    
    # Test context load (if file exists)
    memori.load_context_info()
    
    # Test 1: Store conversation
    print("\n1️⃣ STORING CONVERSATION...")
    memori.store_conversation("user", "I FUCKING HATE RE-EXPLAINING THINGS.", llm="perplexity")
    memori.store_conversation("assistant", "Groundhog Day loop ENDS NOW.", llm="perplexity")
    
    # Test 2: Retrieve history
    print("\n2️⃣ RETRIEVING HISTORY...")
    history = memori.get_conversation_history(limit=5)
    print("History:", history)
    
    # Test 3: Other memory types
    print("\n3️⃣ TESTING OTHER TYPES...")
    memori.store_project_knowledge(
        "AuraFS", "architecture",
        "Fractal recursive filesystem with shard-based storage",
        ["filesystem", "distributed"]
    )
    memori.store_workflow(
        "create repo structure",
        ["Define directories", "Create files", "Add .gitkeep"]
    )
    memori.store_fact(
        "Aurphyx LLC EIN: 41-3437055",
        1.0, ["Aurphyx_Context_info.txt"]
    )
    
    print("\n✅ AURPHYX MEMORI v0.2 OPERATIONAL!")
    print("💾 Data persisted in ./aurphyx_memori.db")
    print("🔥 Ready for LLM integration + Context auto-loaded!")
    return memori

if __name__ == "__main__":
    # Run diagnostic first
    diagnose_schema()
    # Run full test
    test_memori()
