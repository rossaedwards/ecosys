# Aurphyx Protocol & Symbol Naming Codex

Version: 0.1.0  
Scope: Memoree, AuraFS, SoulShot, SAGES, Chakra DataCore, Duality Kernel, APS

---

## 1. Core Concepts

- **Memoree** – Sovereign memory substrate and routing daemon.
- **APC (Agent / Protocol Controller)** – Routing layer that unifies MCP, ACP, and direct API calls across all LLM providers.
- **MCP (Model Context Protocol)** – Tool + context bridge used by LM Studio, Cursor, Claude Desktop, etc.
- **ACP (Agent Communication / Client Protocol)** – Inter-agent / client protocol used for multi-agent workflows.

Each of these must appear in code and docs with the exact casing shown above.

---

## 2. Project Keys and Canonical Names

Derived from `projects.json`. The `key` is the stable identifier used in APIs,
while the display name is free to be more poetic.

| Key              | Display Name                      | Owner       | Primary Domain     |
|------------------|-----------------------------------|-------------|--------------------|
| `memoree`        | Memoree                          | rossaedwards| MEMORY             |
| `aurafs`         | AuraFS                            | aurphyx     | FILE_SYSTEM        |
| `soulshot`       | SoulShot Genesis Engine           | aurphyx     | IDENTITY           |
| `blissid`        | BlissID                           | aurphyx     | IDENTITY           |
| `soulcrypt`      | SoulCrypt                         | rossaedwards| CRYPTOGRAPHY       |
| `sages`          | SAGES                             | aurphyx     | GOVERNANCE         |
| `chakra_datacore`| Chakra DataCore                   | aurphyx     | INFRASTRUCTURE     |
| …                |                                   |             |                    |

Rules:

- Keys are **snake-safe** (`auraorb`, `duality_kernel`), no spaces.
- Display names may include stylization (`g0dm0d3`, `Fuxyez`).
- Once shipped, project keys are immutable; new meaning → new key.

---

## 3. Duality Vocabulary

Dualities are first-class routing hints and must use the `left/right` pattern.

Examples in use (from `projects.json`):

- `coherence/decoherence`
- `discrete/continuous`
- `order/chaos`
- `vacuum/flux`
- `identity/sovereignty`
- `chaos/bliss`
- …

Rules:

- Always lowercase, `/` separator, no spaces.
- The **left** side is the “anchor” (what we’re currently biased toward),
  the right side is the complementary pole.
- When mapping to APC tags, use the full string as the identifier.

---

## 4. Memory Types and Collections

Mirrors `MemoryType` in `schemas.py` and the collections in `MemoryEngine`.

| MemoryType    | Collection   | Description                            |
|---------------|-------------|----------------------------------------|
| EPISODIC      | `episodic`  | Session-bound conversation turns       |
| SEMANTIC      | `semantic`  | Project knowledge, facts, relationships|
| PROCEDURAL    | `procedural`| Workflows, sequences, recipes          |
| META          | `meta`      | Verified facts, axioms, invariants     |
| QUANTUM       | `quantum`   | Physics / simulation state snapshots   |
| CREATIVE      | `creative`  | Art, narrative, divination entries     |
| GOVERNANCE    | `governance`| Votes, policies, mandates              |

These names must not change without a migration.

---

## 5. Provider and Agent Names

Matches `LLMProvider` in `schemas.py`.

- Internal identifiers: `supergrok`, `perplexity`, `gemini`, `claude`,
  `lmstudio`, `openai`, `hermes_agent`.
- External branding can vary, but API payloads and APC envelopes must use
  these exact tokens.

---

## 6. APS Terms

Define APS (Aurphyx Protocol Stack, or Agent/Protocol System – whichever
spelling you settle on) and list top-level entities:

- **APS** – umbrella for APC, MCP, ACP, plus all Memoree routing metadata.
- **APC Envelope** – JSON object with `action`, `payload`, `project`,
  `provider`, `protocol`, etc.
- **Trace** – persisted record in `apc_traces.json`.
