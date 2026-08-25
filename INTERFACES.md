# INTERFACES.md

Explicit contracts between components that different agents own or touch.
The rule: if you change something listed here, you MUST update this file in
the same commit/task, and flag it in HANDOFF.md — a silent break here is the
most expensive kind of mistake in a multi-agent codebase.

---

## Template (copy per interface)

```
## <Component/Module name>

**Owner(s):** <agent or human most familiar with it — not exclusive, just first contact>
**Location:** <path or paths>

### Public contract
- Function/endpoint/schema signature(s), with types.
- What it guarantees (invariants, side effects, error behavior).
- What it explicitly does NOT guarantee.

### Consumers
- <who/what calls or depends on this> — <path or component>

### Change protocol
- Backward-incompatible changes require: <e.g. version bump, migration note,
  ping in Slack #workstream, update all consumers listed above in same task>
```

---

## Example

## Memoree sync client

**Owner(s):** cursor
**Location:** `ecosys/memoree/sync/client.*`

### Public contract
- `push(records: Record[]) -> SyncResult` — batches, retries 3x with backoff,
  never throws; failures surface in `SyncResult.errors[]`.
- `pull(since: timestamp) -> Record[]` — returns records modified after
  `since`, sorted ascending by `updated_at`.
- Assumes Qdrant collection schema matches `ecosys/memoree/schema.json`.

### Consumers
- n8n workflow `memoree-sync-loop`
- Obsidian frontend (reads via `pull`)

### Change protocol
- Changing `Record` shape requires updating `schema.json` and the n8n workflow
  in the same task, plus a CHANGELOG.md entry and HANDOFF.md note.
