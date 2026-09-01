# Vibe Audio Ecosystem — Sibling Products Brainstorm

Every "what if" extension of the VASP/Vibe Audio brand, across apps, plugins, VSTs, DAW packs, and hardware. Pure ideation — pull anything worth building into its own spec later.

**Legend:** `[Already seeded]` your own docs already name this · `[New]` fresh idea

---

## 1. Already Seeded (hiding in your own docs)

- **Vibe Media Player (VMP)** — `[Already seeded]` Named in `VibeTribe_Pillarz.md` as VAP's sibling player. Worth deciding now: is this a desktop/cross-platform build, or a broader "video + audio" scope that VAP (audio-only, mobile) deliberately isn't?
- **SoulSync** — `[Already seeded]` Named in `VASP_ExecSum_ProjContext.md` as the thing VAP outputs standardized metadata *to*. Reads like it wants to be the account/cloud-sync layer — one Aurphyx identity, VASP-tagged library and Tribe votes synced across every app below.
- **AuraOrb** — `[Already seeded]` Named in `VASP_TechSpec_Manual.md`, certified for Pillar 7 (Photometric) and Pillar 8 (Kinetic) streams. Currently a spec line — worth a real companion-app pairing/config flow.

## 2. Companion Apps Across the Stack

- **Vibe Radio** — the one that got you excited. Live "stations" auto-DJ'd purely on VASP vibe-match (Affective + Contextual), not genre — "Night Drive FM," "Gym Peak FM." Community version: Tribe members with high authenticity scores curate their own stations.
- **Vibe Tribe Voting System (gvs-vasp)** — the voting layer spun out as its own standalone app. Short-form, swipe-through, 13-question-max quizzes on songs — more TikTok than "buried inside a music player's settings," which is honestly where the viral potential of your Tribe system actually lives.
- **Vibe Cast** — Android TV / Google TV app. Big-screen Photometric visualizer doubles as ambient room lighting for a party, phone becomes the remote/queue controller.
- **Vibe Wear** — Wear OS companion, Kinetic-pillar-forward: live HR-synced track selection, haptic bass on the wrist, standalone playback for runs without the phone.
- **Vibe Sleep / Vibe Focus** — stripped-to-nothing single-purpose apps. Only surface low-arousal Affective + Sleep/Focus-tagged Contextual matches. No Tribe layer, no browsing rabbit holes — the opposite of engagement-optimized, on purpose.
- **Vibe Kids** — parental-control spinoff leaning hard on `EXPLICIT_FILTER`. Bright simplified UI, no social layer, no Tribe voting.
- **Vibe Karaoke** — spins the pitch-tracked sing-along idea out of VAP entirely into its own app, built around the Linguistic pillar's `DELIVERY_STYLE`/lyric-timing data.
- **Vibe Match** — playful, semi-dating-adjacent: compare two people's aggregate VASP listening fingerprints (average valence/arousal, dominant archetype) for a "compatibility score." Low effort, high shareability.
- **Vibe Fest** — festival companion. Syncs venue lighting rigs (AuraOrb at venue scale) to a live set's real-time Photometric stream; in-app lineup browsing tagged by vibe instead of just set times.
- **Vibe Wrapped** — seasonal recap microsite/feature: "your year was 62% The Heart, 24% The Body." Cheap to build off data you're already collecting, easy organic-share hook.

## 3. DAW Plugins & VSTs (production-side)

This is the category with the most actual leverage — instead of only ever *extracting* VASP data from a finished file, let producers generate it at the source. Solves the DSP-accuracy problem from the other direction.

- **VASP Tagger** (VST3/AU/AAX) — sits on the master bus in Ableton/FL/Logic, runs the same analysis pipeline as the on-device DSP pass in real time during mixdown, writes the VASP profile directly into the bounced file. Tracks are born tagged — nothing downstream has to guess.
- **The Skeleton** — standalone rhythm-analyzer utility plugin. Live meters for kick transient ms, syncopation index, groove quantization while you're mixing drums.
- **The Heart** — a mood-automation plugin: a 2D Valence/Arousal XY pad (Kaoss-Pad-style) labeled in VASP language, mapped to whatever the producer assigns — filter cutoff, reverb send, anything.
- **The Eye** — exports a lighting-cue file (DMX/Art-Net or a simple JSON script) alongside the audio bounce, generated from the same chromatic-mapping logic as the app. Free starting cue sheet for the lighting designer/VJ.
- **Genealogy Finder** — fingerprints stems/samples against a rights database at project-save time, flags likely-uncleared samples before release. Legal-safety utility riding on the `DNA_SAMPLING` concept.
- **Reverse VASP / VASP Composer** — the standout idea here: instead of analyzing a finished track, you *set target pillar values* — Arousal 0.8, Valence −0.6, BPM 140 — and it generates a MIDI sketch or surfaces matching samples/loops to hit that brief. This is exactly the tool a trailer or sync-licensing composer wants when a brief says "give me something dark and aggressive, 90 seconds."

## 4. DAW Packs & Sample Libraries

- **Vibe Pillar Packs** — sample packs organized by archetype/vibe instead of instrument type: "Pillar 5: The Heart — Euphoric Major Chord Stabs," "Pillar 8: The Body — 808s by MET Score." Producers browse the same emotional language listeners do.
- **Archetype Drum Racks** (Ableton Rack / Kontakt) — prebuilt kits per archetype: Skeleton kit = quantized/mechanical percussion, Flesh kit = warm analog leads, etc.
- **Camelot Chord Packs** — MIDI progression packs organized by Camelot wheel position, for producers prepping harmonically-mixable material.

## 5. Browser, Web & Developer Platform

- **Vibe Web Player** — lightweight browser preview for a SoulSync-synced library: drag a track in, see its 9 orbs instantly, no install.
- **Vibe Extension** — browser extension overlaying VASP orb data on YouTube/SoundCloud/Bandcamp pages as you browse, using client-side analysis or crowd data.
- **VASP Developer SDK / API** — the actual platform play: license the scoring engine + schema so third-party apps (fitness apps, other players, smart-lighting software) can pull VASP data without building their own DSP pipeline.
- **Vibe Label Dashboard** — B2B web tool for labels/artists: aggregate Tribe-vote data and VASP distribution across a catalog, as an audience-insight product.

## 6. Hardware Companions

- **AuraOrb** — `[Already seeded]` smart light orb, Pillar 7/8 certified.
- **Vibe Puck** — small Bluetooth haptic disc, pocket or clip-on, that just does the haptic-bass kick-pulse feature standalone — for people who want the physical feel without a smartwatch.

---

## If you only chase one thing first

**Reverse VASP / VASP Composer** and **VASP Tagger** are the two with real product-market pull beyond your own app — they turn VASP from "a cool metadata layer under one player" into "a taxonomy other producers and tools build against," which is the actual moat. Everything else here is a genuinely fun extension, but those two are the ones that make Aurphyx a platform instead of an app.
