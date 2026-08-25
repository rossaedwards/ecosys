# n8n — where it fits, and what's actually here

## Where things stand (as of 2026-08-25)

- You created an n8n account: 8 days left on trial, nothing configured yet.
- `INTERFACES.md` at the repo root already documents an n8n workflow called
  `memoree-sync-loop` as a "consumer" of Memoree's interface, with a formal
  rule: *"changing the Record shape requires updating schema.json and the
  n8n workflow in the same task."* That workflow doesn't exist as a file
  anywhere in the repo yet — it's a plan, not a thing.
- `AI-SYNC.md` separately says n8n was **dropped as a runtime dependency
  this phase** — i.e. Memoree itself shouldn't require n8n to be up in
  order to work.

Those two things aren't actually in conflict: n8n can still be the tool you
use to *orchestrate around* Memoree (health checks, sync loops, alerts)
without Memoree depending on it to function. That's the design this starter
assumes.

## n8n vs. Hermes Agent vs. Cursor vs. Claude — pick per task, not once

You don't have to choose one automation tool for everything. They're good
at different layers:

- **Hermes Agent** (already deeply documented in `memoree/hermes_*.md`) —
  use this for anything that needs to *reason about Memoree's own memory*,
  route between your other AI providers/tools, or act as the in-ecosystem
  agent that already understands your memory layers, ACP, and skills.
  Since it's already wired into Memoree's architecture, prefer it over n8n
  for anything memory-aware.
- **n8n** — use this for glue between *external* services that don't care
  about Memoree's internals: "ping a URL on a schedule," "post to Slack,"
  "watch an inbox," "call a webhook when GitHub Actions finishes." It's a
  visual workflow tool, good for things a non-coder wants to see and adjust
  without reading Python.
- **Cursor** — use this when you're in an editor actively shaping code with
  an AI pair, one file/change at a time.
- **Claude (this tool, or Claude Code/Cowork)** — use this for exactly what
  just happened: repo-wide, cross-file, "make this whole ecosystem
  consistent" work, plus writing the scripts the other three end up running.

Given you only have 8 days left on the n8n trial and haven't built anything
in it yet, the low-regret move is: **don't commit to n8n yet.** Everything
in `devops/` works without it. If the trial lapses, you've lost nothing —
the starter workflow below imports into any n8n instance (trial, paid, or
self-hosted free-tier) in under a minute whenever you're ready.

## What's in this folder

- `memoree-sync-loop.starter.json` — an importable n8n workflow: a
  schedule trigger every 5 minutes → HTTP GET to
  `https://memoree.aurphyx.org/health` → an IF node that branches on
  failure → a **No-Op** placeholder node on the failure branch (deliberately
  inert — it does not email or Slack anyone until you wire real credentials
  into that branch). Import via n8n's UI: **Workflows → Import from File**.

That's the whole starter. It intentionally does not try to implement the
full `memoree-sync-loop` "Record sync" contract described in
`INTERFACES.md`, because that contract references a `Record` shape that
should be pinned down in `schemas.py` first — build the real sync workflow
once you're actually shipping schema changes that need it, not before.
