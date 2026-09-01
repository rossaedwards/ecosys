# Aurphyx LLC — Trademark / IP Strategy Briefing

Not legal advice — directional information to make an informed decision, and a map of what to bring to a real trademark attorney. Verify all figures at uspto.gov and copyright.gov before acting.

## The core tension, and how to resolve it structurally

Publishing VASP as an open standard (web app, Android, VLC plugin — all free) is in direct tension with wanting to monopolize it. The standard way this gets resolved, used by every standards body that's pulled it off:

| Layer | Openness | Protected by |
| --- | --- | --- |
| The spec/schema itself (taxonomy, JSON schema) | Open — freely implementable | Copyright, published under a permissive license |
| Reference implementations (VAP, VMP, VLC plugin) | Open or closed, doesn't matter much | N/A — this is distribution, not the moat |
| The name and certification badge ("VASP," "VASP Certified") | Tightly controlled | Trademark / certification mark |
| The Tribe vote corpus | Inherently uncopyable | Nothing legal — just years of community nobody else has |

This is the Bluetooth/USB-IF/PDF playbook: give the format away, license the badge.

## VASP

- Register **"VASP"** as a standard trademark on your own software/spec (Class 9 downloadable software, Class 42 SaaS).
- **"VASP Certified" must be a separate certification mark**, not a variant of the same trademark. Confirmed current USPTO rule (TMEP 1306, 15 U.S.C. § 1064): the owner of a certification mark cannot use it on their own goods — not even a little — and the same mark cannot be registered as both a standard trademark and a certification mark for the same goods/services. Practical result:
  - **AuraOrb** (your own hardware) legally cannot carry "VASP Certified" — market it as "VASP-native" or "Built for VASP" instead.
  - "VASP Certified" is licensed exclusively to third-party makers (Govee, LIFX, whoever), which is what you actually wanted for the licensing revenue play — just be precise about the language split.
  - The certification mark application requires you to submit written certification standards to the USPTO up front (what a device has to do to earn the badge) — worth drafting that conformance spec early since it's a filing requirement, not just marketing copy.
- Publish the schema/taxonomy itself under an explicit open license (something CC-BY-style or a custom open-spec license) — decide this now, before more implementers build against it informally.

## Aurphyx

- Coined/invented word — the strongest category of mark there is (a "fanciful" mark, in trademark terms), meaning it's both easy to register and easy to defend later. Good news, no real complications here.
- Standard trademark, likely Class 42 (SaaS) at minimum, plus Class 9 (downloadable software) and possibly 41 (entertainment services) depending how the full product family gets described in the filing.

## Adoré

- Also a coined/stylized use in this context — good in theory.
- **Real conflict risk found in a first-pass search:** a live USPTO application for plain "ADORE" (serial 98311941, filed Dec 2023) covers Class 009 — "musical sound recordings, downloadable musical sound recordings... featuring music." Same class family you'd file a DAW under. Not necessarily fatal (DAW software vs. music recordings are different goods within the same class), but close enough that a real clearance search matters before more time goes into this name specifically.
- Dior's "J'ADORE" also exists (Class 003, cosmetics/perfume) — different goods, lower collision risk, but famous enough that it's worth a lawyer's eyes given how dilution law treats very famous marks even across unrelated categories.
- **Don't skip a paid clearance search** (TESS plus a real search service, low hundreds of dollars) before sinking more design/dev time into this specific name. Far cheaper than rebranding a shipped DAW.

## What this actually costs right now

- **USPTO federal filing:** $350/class base fee — but only if you use the USPTO's pre-approved goods/services language from their ID Manual. Write your own description instead (most first-timers do) and it's +$100–200/class in surcharges. Budget closer to $500–550/class to be safe.
- Since nothing's shipped yet, file **intent-to-use (ITU)** applications — this reserves the name now without needing to already be selling something. Comes with a Statement of Use filing later once you actually launch (or a $125/class extension fee if you're not ready by then).
- **Copyright registration** on the VASP spec text/schema itself: roughly $45–65/work through copyright.gov (verify current fee). Worth doing regardless of the open license you publish under — registration is what gives you real teeth (statutory damages, ability to actually sue) if someone rips off the spec wholesale rather than implementing against the open version honestly.
- A **real clearance search** beyond what a quick search turns up: low hundreds of dollars if you pay a service for it.

## Sequencing, given the budget reality

1. **Now, cheap:** proper clearance searches on all three names — especially Adoré, given the conflict above — before more design/dev time goes into any of them.
2. **Now, cheap:** file ITU applications on whichever names clear the search. The whole point of intent-to-use is locking the name down early and cheaply, before someone else files it out from under you.
3. **Defer:** international filings, a full-service IP firm, active enforcement — none of it matters until there's something with real usage worth defending.
4. **Worth paying for once there's a dollar to spend on it:** a real trademark attorney specifically on the VASP / "VASP Certified" split. That mechanism is structurally load-bearing for the whole B2B monetization plan — a mishandled certification mark can get cancelled outright under the rule above, and that's the one mark this entire plan depends on getting right.
