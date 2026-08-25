# Cloudflare — current state and options

## What's actually live (as of 2026-08-25)

- Billing is set up; you're on the **Free plan**.
- Domains: `aurphyx.com` and `aurphyx.org`.
- One confirmed tunnel replica, from `cloudflared -v`/dashboard:
  - Hostname: `Aura` (your Windows machine, `windows_amd64`, cloudflared
    `2026.3.0`)
  - Routes `https://memoree.aurphyx.org` → `127.0.0.1:7042` (Memoree's
    FastAPI daemon)
  - Edge locations: iad10, iad20, iad08 (Ashburn, VA-area PoPs)

That tunnel is the one real, running piece of infrastructure. Everything
below is either hardening that tunnel, or documenting other Cloudflare
features you *could* turn on later — none of it is built in this pass
unless you ask.

## 1. Hardening the Memoree tunnel (built in this pass)

`devops/scripts/tunnel_guard.py` checks, on an interval:

1. Is `memoree_service.py` actually running?
2. Does `http://127.0.0.1:7042/health` respond?
3. Does `https://memoree.aurphyx.org/health` respond (the public hop)?

If the local service is down, it starts it. If the local service is up but
the public tunnel isn't, that isolates the problem to `cloudflared` itself,
and the script logs it (and optionally restarts `cloudflared.exe`, if you
pass `--restart-tunnel` — off by default, since restarting someone's tunnel
process automatically is the kind of thing that should be opt-in).

### One-time setup on your Windows machine

1. Make sure `cloudflared` is running as a Windows **service**, not a
   terminal you have to keep open:
   ```
   cloudflared service install
   ```
   (Run once, elevated. If you're currently running `cloudflared tunnel
   run <name>` in a terminal window instead, that's the thing that goes
   away when you close the laptop lid or the terminal crashes — `service
   install` fixes that permanently.)

2. Register the guard script to run at logon:
   ```
   schtasks /create /tn "Memoree Tunnel Guard" ^
     /tr "python C:\rossaedwards\ecosys\devops\scripts\tunnel_guard.py --loop" ^
     /sc onlogon /rl highest
   ```

3. Check the log any time at `devops/logs/tunnel_guard.log`.

## 2. Cloudflare Pages — for the public site folders (documented, not built)

You have several static-site-shaped folders at the ecosys root already:
`aurphyxcom/`, `aurphyxstore/`, `aurphyxonline/`, `aurphyxorg/`,
`aurphyxnet/`, `fuxyezcom/`, `fuxyezstore/`, `fuxyezinfo/`. Cloudflare Pages
(free on your current plan) can deploy any of these straight from GitHub —
push to a branch, Pages builds and publishes it, no server to babysit.

If/when you want this, the shape is:
- One Pages project per site folder (Pages doesn't like multiple sites
  sharing one repo root well — point each project at its own subfolder as
  the "root directory" in the Pages project settings).
- Add `custom domain` in the Pages project → point `aurphyx.com` /
  `aurphyx.store` / etc at it.
- A GitHub Actions workflow per site (or Cloudflare's own GitHub
  integration, which needs zero YAML) triggers the deploy on push.

Say the word and this pass can add the GitHub Actions side for a specific
site once you tell me which one you want live first — trying to stand up
all eight at once isn't a good use of a first pass.

## 3. Cloudflare Access / Zero Trust tied to Google Workspace (documented, not built)

Zero Trust has a free tier (up to 50 users), and since you're on GWS
Business Standard, Access can gate anything behind Google SSO — e.g. put
`https://memoree.aurphyx.org` behind a login instead of leaving it open to
anyone who finds the URL. Rough shape, when you want it:

1. Cloudflare dashboard → Zero Trust → Settings → Authentication → add
   Google as an identity provider (uses your Workspace OAuth client).
2. Zero Trust → Access → Applications → add `memoree.aurphyx.org`, restrict
   to your Workspace domain/email.
3. No code changes needed in Memoree itself — Access sits in front of the
   tunnel, not inside your FastAPI app.

Worth doing once Memoree is something other than you hitting it from your
own devices — right now, low urgency.
