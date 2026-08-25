# AINTS_SUPPZ — `aurafs/src/network/`

**Type:** Agent support / supplement briefing for this product module.  
**Not:** APS-OKF header stamping, TSLCA/FTQC paper reprint, or a license to rewrite this folder's source.  
**Product:** AuraFS (file system, storage, mesh). Law: `aurafs/cursorrules`, `aurafs.toml`, `AURAFS_PROJECT_CONTEXT.md`.  
**Organism map:** `aurphyx_welcome2tribe.md` + `ecosystem.toml`.  
**Template gap:** `AINTS_SUPPZ.md` does not exist in `tslca/`, `ftqc/`, or `fuxyez/` on this tree. This file matches that *type* of document (what lives here, locks, aliases, what not to touch) as used in `tslca/SUMZ-SUGGZ.md` and volume briefings — adapted to AuraFS product reality.


## What lives here

Largest **wired** surface after redteam/whitehat stubs: Meshwerk, discovery, gossip, P2P, replication, firewall, secure tunnel, Reticulum/RNS (Rust **and** Python), autoheal, node_manager, orchestrator.

Subfolders: `meshwerk/` (roles, routing, topology_engine — **locked**, many files still ~14–65 LOC), `meshtastic_integration/` (**almost all 14-line stubs**), `transport/` (starlink_client ~49 LOC; most others stubs), `defense/` (thin), `monitoring/` (stubs), `integration/` (14-line bridges named arora, gvs, ineffable, opulence, wallet).

Configs: `meshwerk/configs/ghostlink.toml` (**locked**), `dataslayer.toml`.

## Wired?

**Yes** — `pub mod network`. `main.rs` uses `network::NodeManager` (`node_manager.rs` is only ~25 LOC).

## Era / voice

`mod.rs` and fat files: Love + Quantum Division. Stubs: no era banner.

## Locks

See `aurafs.toml` `[modules.validated]` network_meshwerk, network_control, transports. Public API frozen without override.

Transports in law: Titan-Libp2p (primary), GhostLink-LoRaWAN (secondary), Starlink-HighOrbit (tertiary). Cargo: `ghostlink-lorawan = []` is **TODO**; `starlink-backhaul` optional.

## Relation to welcome

This is Meshwrk. `meshwrk-4dm1n` (Audry) is design. Xplor browses mesh + FS — design. Do not treat RNS Python as the Rust crate API.

## Honest status

**Split:** fat Love/QDiv files (p2p, secure_tunnel, reticulum, rns_*) vs locked Meshwerk files that are still short vs 14-line integration stubs. Not a production mesh. `security-tools` feature does not exist in `Cargo.toml` (relevant to defense).


## Nomenclature (new prose only)

Quote retired names, then map: SIC/SCC/ICC → SIX/SCX/ICX; USAIC → SUXS-IFO; Bliss manifold (physics) → Equilibrium Manifold; rÆ → Balance State Vector; Vibe-OKF → APS-OKF; V.A.P. → VASP; Accessibility → Xessability.  
**Product names that stay:** BlissCore / ChaosCore (Chakra Duality Kernel). In-tree `BlissID` types are legacy identity code — flag, do not silently rewrite.

## What not to do

- Do not stamp ecosys APS-OKF YAML onto this file or this folder's Rust.
- Do not apply AuraFS replica-count / PBG / η law as FTQC theory.
- Do not invent missing root files (`INVARIANTS.md`, `aps.toml`, codices).
- Do not create parallel trees or `aps_*` duplicates.
- One folder pass: edits belong under `aurafs/` only.

---
*Audry briefing. Faithful to source on disk. Updated 2026-08-23.*
