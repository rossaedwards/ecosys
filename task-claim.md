# task-claim convention

Purpose: a lightweight task board in the filesystem so any agent can see what's
claimed, what's open, and what's done — without needing Notion access.
(Notion can still be the human-facing source of truth; this is the
agent-facing mirror that works even when an agent can't hit the Notion API.)

## Folder structure
```
tasks/
  open/       <- unclaimed tasks, any agent may pick one up
  claimed/    <- in-progress, one file per task
  done/       <- completed, moved here on finish (keep for history)
```

## Claiming a task
1. Pick a file from `tasks/open/<task-id>.json`.
2. Move it to `tasks/claimed/<task-id>.json` and add `agent` + `claimed_at`.
3. If the task touches specific files, also create a matching `.agent-lock`.
4. On completion, move it to `tasks/done/<task-id>.json`, fill in `completed_at`
   and `result`, and add a HANDOFF.md entry.

## Task file format

```json
{
  "task_id": "task-0042",
  "title": "Add retry logic to Memoree sync client",
  "description": "n8n workflow retries failed pushes 3x with backoff before alerting Slack.",
  "created_by": "ross",
  "created_at": "2026-08-23T13:40:00Z",
  "priority": "medium",
  "depends_on": [],
  "agent": null,
  "claimed_at": null,
  "completed_at": null,
  "result": null
}
```

## Rules of thumb
- One task = one file. Don't batch unrelated work into a single claim.
- If a task is too big to finish in one session, split it into subtasks rather
  than holding a claim open indefinitely.
- `depends_on` lists other `task_id`s — don't claim a task whose dependencies
  aren't in `done/` yet.
- Stale claims (no HANDOFF.md activity in 24h+) are fair game for another
  agent to pick up — but check in (Slack/HANDOFF.md) before taking over.
