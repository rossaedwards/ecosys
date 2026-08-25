# devops/ — orientation

You asked for CI/CD, an n8n + Cloudflare setup, and a `devops/` folder to
hold most of the root-level tooling, with actual runnable scripts and
plain guidance rather than another spec to maintain by hand. This is that
folder. Everything in here was built from what's actually in your repo,
not a generic template — see "What I found" below before you do anything
else, because it changes how you should think about the file set you
described.

## What I found (read this first)

You described a list of ~29 files (`AGENTS.md`, `AI-SYNC.md`, `CLAUDE.md`,
`ecosystem.toml`, `HANDOFF.md`, `INTERFACES.md`, `INVARIANTS.md`,
`MASTER_INDEX.md`, `MASTER.yaml`, `PHYSICS.md/.yaml`, `task-claim.md`,
`welcome2tribe.md`, `TRIBE.AGENT.md`, `.agent-lock.md`, and more) as living
"at `C:\rossaedwards\ecosys\memoree\`". **They don't — they live at the
ecosys root**, `C:\rossaedwards\ecosys\`. The `memoree\` folder itself
holds a completely different, much more technical set of files: the actual
FastAPI service (`memoree_service.py`, `routes.py`, `schemas.py`,
`memory_engine.py`), the Hermes Agent docs (`hermes_acp.md`,
`hermes_creating_skills.md`, `hermes_memory_provider.md`,
`hermes_platform_adapter.md`), `config.yaml`, and its own small
`README.md` / `PROJECT_CONTEXT.md` / `SUMZ-SUGGZ.md` / `QUICKSTART.md`.

Both halves are real and both matter — they're just two different layers:

- **The ecosys root** = the shared "canon": meta-documentation that talks
  *about* the whole ecosystem (all your projects, terminology, physics,
  mythology, governance).
- **`memoree/`** (and `aurafs/`, `fuxyez/`, and every other subfolder) =
  the actual working project, with its own small local doc set.

`devops/config/ecosystem_manifest.yaml` encodes exactly this split, by
name, so the tooling checks the real thing instead of the described thing.
I'd also bet `memoree_README.md` sitting at the ecosys root (marked "temp
file until we polish it up for deployment") is meant to eventually become
content inside `memoree/README.md` — that's flagged in the manifest as a
staging file, not auto-moved, since moving it is a judgment call about
what's actually ready.

## Layout

```
devops/
├── README.md                      <- you are here
├── requirements.txt                <- pip install -r devops/requirements.txt
├── config/
│   └── ecosystem_manifest.yaml     <- the declared contract: what "in sync" means, as data
├── scripts/
│   ├── ecosystem_check.py          <- the main tool (validate / check-locks / regen-tree)
│   └── tunnel_guard.py             <- keeps Memoree + its Cloudflare tunnel alive
├── n8n/
│   ├── README.md                   <- n8n vs Hermes Agent vs Cursor vs Claude, and current status
│   └── memoree-sync-loop.starter.json
├── cloudflare/
│   └── README.md                   <- current tunnel state + Pages/Access options
└── generated/                      <- created on first `regen-tree` run; safe to .gitignore or commit, your call
```

Two GitHub Actions workflows were also added (they have to live under
`.github/workflows/`, not `devops/` — GitHub requires that exact path):

- `.github/workflows/ecosystem-ci.yml` (ecosys root) — runs
  `ecosystem_check.py validate` and `check-locks` on every push/PR, plus a
  Python syntax check on `memoree/*.py`.
- `fuxyez/.github/workflows/ci.yml` — fuxyez had **no CI at all** before
  this. It builds/tests each of the four real Rust crates
  (`compiler`, `fute`, `fuxrt`, `governance`) individually, since
  `fuxyez.toml` is the *language's* manifest, not a Cargo workspace file
  (Cargo only reads `[workspace]` from a file literally named `Cargo.toml`
  — that's a real, separate thing worth knowing, not a bug I fixed
  silently). `aurafs/` already had good CI (`fmt`/`clippy`/`test`/
  `build`/`docker`) — untouched.

## The core loop: `ecosystem_check.py`

```
pip install -r devops/requirements.txt

python devops/scripts/ecosystem_check.py validate       # are all required files present? did paired files move together?
python devops/scripts/ecosystem_check.py check-locks    # any stale .agent-lock.md locks?
python devops/scripts/ecosystem_check.py regen-tree     # write a fresh, single-file tree snapshot per project
python devops/scripts/ecosystem_check.py all            # validate + check-locks (what CI runs)
```

It reads `devops/config/ecosystem_manifest.yaml` — **that YAML file is
where you make changes**, not the Python. Want a new project tracked?
Want a new "these files must change together" rule (like the
`MASTER_INDEX.md` + `MASTER.yaml` pairing, or `schemas.py` + `routes.py`
in memoree)? Add it to the manifest. This is the direct answer to "every
project is a compiler, you can't edit one file without updating the rest"
— the manifest is what used to only live in your head.

**It runs in warning mode by default, not strict.** Missing required files
always fail (there are none right now — everything you listed actually
exists on disk). Softer issues — like `MASTER_INDEX.md` referencing files
that don't exist (`AURAFS_SHARD_TAXONOMY.md`,
`AURAFS_CROSS_VOLUME_SUBSTRATE_SPEC.md`, etc. — it's also still flagged
`!!--NEEDS-TO-BE-UPDATED-AND-ALIGNED--!!` at the top) — print as warnings
so CI doesn't go permanently red on day one. Add `--strict` once you've
cleaned those up and want CI to actually enforce it.

## n8n and Cloudflare

Full detail in `devops/n8n/README.md` and `devops/cloudflare/README.md`.
Short version: your Cloudflare tunnel (`memoree.aurphyx.org` → your
laptop) is real and running, so it got a real hardening script
(`tunnel_guard.py`) plus a one-time setup you run once on Windows. n8n has
nothing built in it yet and only 8 days left on trial, so it got a
starter workflow you can import in a minute *if* you keep it, and honest
guidance on when to reach for it vs. Hermes Agent (which is already
wired into Memoree) vs. Cursor vs. Claude — rather than a pile of
automation tied to a trial that might lapse.

## What I did not do

- Did not touch `MASTER_INDEX.md`'s actual content, or any of the other
  drifted docs — fixing *what they say* is a judgment call about your
  canon, not a mechanical sync problem. The tooling will now tell you
  exactly where they're wrong; fixing the prose is still yours (or ask me
  to draft the fix once you've reviewed what `validate` reports).
- Did not commit anything to git. Review the new files, then commit
  when you're ready — or ask and I will.
- Did not build Cloudflare Pages or Access/Zero Trust — documented as
  options in `devops/cloudflare/README.md`, ready to build the moment you
  pick a first site or decide you want SSO in front of Memoree.
