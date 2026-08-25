# CHANGELOG.md

Human-readable, agent-attributed history of changes. Unlike HANDOFF.md (which
is process/reasoning-focused and per-task), this is outcome-focused and
per-release-or-milestone — what actually shipped.

Format loosely follows [Keep a Changelog](https://keepachangelog.com), with an
added `agent` tag per entry so you can audit which model did what over time.

---

## [Unreleased]

### Added
- <!-- - Short description of new capability. (agent: cursor, task-0042) -->

### Changed
- <!-- - Short description of a change to existing behavior. (agent: claude) -->

### Fixed
- <!-- - Short description of a bug fix. (agent: grok) -->

### Removed
- <!-- - Short description of removed capability. (agent: gemini) -->

---

## [Example] 2026-08-23

### Added
- Retry logic with exponential backoff for Memoree sync client. (agent: cursor, task-0042)

### Fixed
- Fixed race condition in n8n webhook handler causing duplicate Slack posts. (agent: claude)

---

## Conventions
- One bullet per user/system-visible change, not per commit.
- Always tag `(agent: <name>[, task-id])` at the end of the line.
- Move `[Unreleased]` entries into a dated section on release/milestone cut,
  newest section on top.
