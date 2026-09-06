# Pink Tribe Gaming Suite

**Product name:** Pink Tribe Gaming Suite  
**Also named in-tree:** Diamond CLI, Redteam Gaming Empire, White Hat Defense Matrix  
**Paths:** `aurafs/src/redteam/`, `aurafs/src/whitehat/`  
**Voice:** f0rg3d in l0v3 — Ross Edwards / R.F. Lovezme / Aurphyx Quantum Division  
**Source of truth for the organism:** `aurphyx_welcome2tribe.md`  
**This file:** the combined **offensive + defensive** suite brief. Kali-shaped. Every tool is a game.

Pink Tribe is not a compliance appendix. It is the pentest deck for AuraFS: you play as red, you play as white, the mesh gets stronger, and the job is supposed to be fun.

---

## What this product is

Imagine Kali Linux if every tool opened as an arcade cabinet.

- **Red** (`src/redteam/`) — offensive pentest tools. Chaos, exploit-named cabinets, mesh storms, fuzzers, governance pressure, quantum-breaker cabinets. Authorized play against **your** AuraFS mesh, lab, or tournament target.
- **White** (`src/whitehat/`) — defensive twins. Same six arenas. Shields, remediations, verifiers, hardeners. You win by keeping quorum, identity, and shards alive while red plays.
- **Games** — the UI of the tools. Node-Man, Lag Lottery, Compliance Tetris, Soul Konami, Asteroids, Mesh Breakout, Battleship, Minesweeper, Dilithium DigDug, Quantum Casino. Scoreboards, trophies, Meshtastic PvP, tournament mode.
- **Loop:** run a tool → it stresses a real AuraFS surface (nodes, shards, namespace, Soul/BlissID, Dilithium, mesh) → a game cabinet unlocks → high scores feed `AchievementTracker` / `pvp_leaderboard` / `global_ranking` → white’s score (`DefenseStatus.overall_score`, `HardeningResults.compliance_score`) is the other half of the same match.

CLI banner (`redteam/cli.rs`): **AURPHYX REDTEAM DIAMOND CLI** — `afs-redteam`. Subcommands: `Audit`, `Chaos`, `Exploit`, `Fuzz`, `Quantum`, **`Games`**, `Report`. Flags: `--target`, `--output`, `--enterprise`, `--games`.

White entry (`whitehat/mod.rs`): `init()`, `shield()` (full defense matrix, score 100 when all five arenas are up), `run_hardening_checks()`.

This is how AuraFS practices resilience: the filesystem, storage, and mesh from welcome get **played** until they hold.

---

## Design law (product, not apology)

1. **Every tool is a game.** A `.rs` name is a cabinet. A stub that only prints `💎 ATTACK!` and adds **42** is an empty cabinet — still on the floor plan.
2. **Offense and defense are one suite.** Six mirrored arenas. Red’s Node Killer is white’s Node Health. Red’s Heartbleed cabinet is white’s Heartbleed Patch. You do not ship one side.
3. **The target is AuraFS.** Shards, mesh, SoulKey/SIG (in-tree still `BlissID`), Dilithium-5, namespace, quorum 13, Trap-State. Classic CVE names are **cabinet skins** for those surfaces — EternalBlue-Defense is the Windows-era skin on shard/SMB-shaped hardening, not a request to pwn strangers.
4. **Authorized play.** `redteam/mod.rs` says it. Tournament / lab / your cluster. Same ethic as a range, not a script-kiddie dump.
5. **Fun is a feature.** Konami `↑↑↓↓←→BA`, FUX coin casino, Wonka / Ready Player One eggs, skull-screen Soul Jack art, Pac-Man Node-Man on Meshtastic. Keep the Love banners.
6. **42** is the house number. Diamond stubs increment `attacks += 42`. Konami paradise claims **42 mini-games**. Shard Minesweeper unlocks after `unique_crashes > 42`.

---

## How you play (operator loop)

```
afs-redteam --target <lab-or-mesh> --enterprise --games
    Audit     → Compliance Tetris / PTES / CIS / NIST-named cabinets
    Chaos     → Node-Man, Lag Lottery, storms, tournament
    Exploit   → named cabinets + Mesh Breakout + Trophy Hunter
    Fuzz      → Soul Konami, Battleship, Minesweeper
    Quantum   → Asteroids, DigDug, Kyber Casino / Wonka / R1O
    Games     → GameHub (unlocks after you actually run tools)
    Report    → DiamondReport + game_unlocks
```

Then flip to white:

```
whitehat::shield()           → five-arena DefenseStatus
whitehat::run_hardening_checks() → HardeningResults
```

**Win (red):** tool completes, game unlocks, score lands on the PvP board, mesh still tells you what broke.  
**Win (white):** quorum held, patches applied, `overall_score` / `compliance_score` up, same board.  
**Win (Pink Tribe):** both sides played the same arena in one sitting. The system improved.

`GameHub` (today) prints:

| # | Cabinet | Hub status (in `cli.rs`) |
|---|---|---|
| 1 | Node-Man | UNLOCKED |
| 2 | Lag Lottery | UNLOCKED |
| 3 | Compliance Tetris | UNLOCKED |
| 4 | Soul Konami | UNLOCKED |
| 5 | Asteroids | LOCKED — forge `entropy_starver` |

`--games` and `Commands::Games` are the arcade. `--enterprise` is the “this run counts for the mesh” mode (NIST/CIS/FIPS strings on Audit; bonus game time on Asteroids).

---

## Scoring and meta

| Object | File | What it tracks |
|---|---|---|
| `TestResults` | `redteam/mod.rs` | chaos / exploit / network / governance / quantum counts; passed / failed |
| `DefenseStatus` | `whitehat/mod.rs` | five arena bools + `overall_score` 0–100 |
| `HardeningResults` | `whitehat/mod.rs` | vulns found, mitigations applied, `compliance_score` |
| `DiamondReport` | `redteam/cli.rs` | suite, vulns, cvss_score, remediation_steps, **game_unlocks**, enterprise_grade |
| `AchievementTracker` | `redteam/cli.rs` | games_unlocked, high_scores, legend_badges; hub calls `progress()` / `legend_status()` |
| `PVP_LEADERBOARD` | `redteam/chaos/pvp_leaderboard.rs` | Diamond attack counter (+42) |
| `GLOBAL_RANKING` | `redteam/chaos/global_ranking.rs` | same stub pattern |
| `TOURNAMENT_MODE` | `redteam/chaos/tournament_mode.rs` | same stub pattern |
| `TROPHY_HUNTER` | `redteam/exploit/trophy_hunter.rs` | same stub pattern |
| Per-game scores | diamond modules | `breakout_score`, `battleship_score`, `minesweeper_score`, `konami_score`, `asteroids_score`, `digdug_score`, `pvp_high_score` |

Meshtastic / LoRa PvP is named in Node-Man art (`exploit/mesh_nodeman.txt`) and several game outros (“Meshtastic PVP Ready”). That is the off-grid tournament skin: play over GhostLink / mesh, not only localhost.

---

## Featured cabinets (the games that already have bodies)

These are the **diamond** files — hundreds of LOC, crossterm loops, unlock hooks. This is the playable core.

### Node-Man
- **Skin:** Pac-Man on the AuraFS mesh. Pellets = nodes. Ghosts = partitions. Diamonds = trophies.
- **Tool:** node discovery + kill patterns as a maze. Art in `redteam/exploit/mesh_nodeman.txt`. Chaos engine in `redteam/chaos/node_killer.rs` (Mythical Node Killer Dragon).
- **Scenarios named in code:** `leader_election_test` (33%), `majority_quorum_break` (67%), `zone_failure_az1` (40%), `replica_heavy_kill` (75%), plus eight more stubbed as “12 production chaos scenarios.” Patterns: Random, Sequential, LeaderFirst, ReplicaHeavy, ZoneKill.
- **White twin:** `whitehat/chaos/node_health.rs`, `failover_simulator.rs`, `recovery_engine.rs`, `reliability_orchestrator.rs`.
- **Controls (art):** SPACE play, P partition, arrows move, Q quit.
- **Win:** mesh quorum held after the dragon; score on the Node-Man HUD (`Score / Partitions / Nodes Isolated` in the txt mock).

### Lag Lottery
- **Skin:** slot / lottery on latency. Hub lists it unlocked.
- **Tool:** `redteam/chaos/latency_injector.rs` (diamond stub +42 today; CLI imports `LatencyInjector` as a real type).
- **White twin:** `whitehat/chaos/latency_monitor.rs`, `resource_balancer.rs`.
- **Win:** inject jitter, white keeps Trap-State / T₂ windows honest. Jackpot = recovery under the coherence window.

### Compliance Tetris
- **Skin:** tetrominoes of NIST 800-53 / CIS / FIPS 140-3 / PTES lines. Clear a row = a control that still holds after red’s audit run.
- **Tool:** `redteam/cli.rs` `Audit(PentestSuiteArgs)`; `audit_simulator/pentestsuite.rs` is still a +42 stub — the **game name** is in the hub; the **suite** is the cabinet.
- **White twin:** `compliance_scanner`, `cis_benchmarker`, `ptes_automator`, `compliance_monitor`.
- **Win:** `DiamondReport` + white `compliance_score`.

### Soul Konami / Konami Paradise
- **Skin:** `↑↑↓↓←→BA` opens **42 mini-games**. Named on-screen: Contra (Soul Shooter), Castlevania (Soul Whip), Gradius (BlissID Defender), Metal Gear (Stealth Bypass); “42 total” still ellipsized in source.
- **Tool:** `redteam/fuzzers/soul_fuzzer.rs` (~295 LOC). Soul / BlissID collision and auth-path fuzz as the **job**; Konami as the **cabinet**.
- **White twin:** `whitehat/exploit/soul_guard.rs`, `whitehat/gov/soul_verifier.rs`.
- **Win:** konami unlock + `konami_score`. Identity must still resolve (welcome: SoulKey → SKIM → SIR → SIG; in-tree type is still BlissID).
- **Egg:** `exploit/souljack_easteregg.txt` — Soul Jack skull-screen theater.

### Asteroids (Entropy)
- **Skin:** shoot QRNG asteroids. WASD + space. ~60s run. Lives as hearts.
- **Tool:** `redteam/quantum_breaker/entropy_starver.rs` (~321 LOC). Entropy-starvation / NIST 800-90B-named phases unlock the cabinet. Hub marks it LOCKED until you actually forge this module.
- **White twin:** `whitehat/quantum_breaker/quantum_rng.rs`, `harvest_protector.rs`.
- **Win:** `asteroids_score`; Meshtastic PvP ready line.

### Mesh Breakout
- **Skin:** 10×20 brick Breakout. Bricks = mesh partitions / Istio-Linkerd-shaped isolation.
- **Tool:** `redteam/exploit/mesh_partition.rs` (~348 LOC). `MeshPartitioner` + `play_breakout()`.
- **White twin:** `whitehat/chaos/shard_sync.rs`, `whitehat/net/anomaly_detector.rs`, `flood_protector.rs`.
- **Win:** `breakout_score`; gossip still heals.

### Battleship (Namespace)
- **Skin:** fleet = UUID / namespace map. Hits = collision / traversal finds.
- **Tool:** `redteam/fuzzers/namespace_fuzzer.rs` (~368 LOC).
- **White twin:** `whitehat` has no `namespace/` twin; product join is `src/namespace/` + `acl/`. Cabinet-side: `whitehat/net/waf_rules.rs`, `ids_integration.rs`.
- **Win:** `battleship_score`.

### Shard Minesweeper
- **Skin:** 10 mines / 9×9. Cells = shard mutations. Flag the crashy tiles.
- **Tool:** `redteam/fuzzers/shard_fuzzer.rs` (~346 LOC). Unlock after crash budget (`unique_crashes > 42`).
- **White twin:** `whitehat/exploit/shard_protector.rs`, `whitehat/chaos/resilience_tester.rs`.
- **Win:** `minesweeper_score`.

### Dilithium DigDug
- **Skin:** 26×20 tunnel dig through lattice “dirt.” Chains multiply score.
- **Tool:** `redteam/quantum_breaker/dilithium_forge.rs` (~340 LOC). Dilithium-5 cabinet (AuraFS already has a real `crypto/pqc/dilithium_sig.rs` helper — this game **stresses** that surface).
- **White twin:** `whitehat/quantum_breaker/dilithium_verifier.rs`, `falcon_validator.rs`, `pqc_tester.rs`, `lattice_hardener.rs`.
- **Win:** `digdug_score`; `256 - dirt_level` as the clear metric.

### Quantum Casino / Wonka Factory / Ready Player Aurphyx
- **Skin:** `kyber_cracker.rs` (~204 LOC). 1% (when RNG `% 100 == 42`) drops you into a FUX-coin casino (bankroll 1000, 5 rounds, house-advantage triggers). Target string `wonka` → candy factory. Target `r1o` → quiz (Kyber 1024, Dilithium 5, motto “its recursive”, casino code 42).
- **Tool:** Kyber stress **simulation** (source itself says placeholder / “keys remain secure”). The game is the product surface.
- **White twin:** `pqc_migrator.rs`, `quantum_resistant.rs`.
- **Win:** FUX bankroll / quiz score; whale line points at entropy-starver.

---

## Full game / cabinet list (both tribes)

Names are **exact filenames**. Status: **diamond** = large playable-leaning module; **cabinet** = Love/QDiv stub (`ATTACK!` +42) or white 14-line shield; **art** = txt theater.

### Arena 1 — Chaos (mesh weather)

| Red cabinet | Status | White cabinet | Game read |
|---|---|---|---|
| `node_killer.rs` | diamond (~352) | `node_health.rs` | **Node-Man** / Dragon |
| `nodekiller.rs` | cabinet | *(same)* | duplicate name plate |
| `latency_injector.rs` | cabinet (CLI wired) | `latency_monitor.rs` | **Lag Lottery** |
| `resource_crusher.rs` | cabinet | `resource_balancer.rs` | resource boss |
| `cert_expiry.rs` | cabinet | `cert_renewal.rs` | cert clock |
| `config_drift.rs` | cabinet | `config_validator.rs` | drift vs lock |
| `backup_corrupt.rs` | cabinet | `recovery_engine.rs` | backup raid |
| `shard_storm.rs` | cabinet | `shard_sync.rs` | shard weather |
| `cascade_engine.rs` | cabinet | `chaos_remediator.rs` | cascade vs mend |
| `combo_chains.rs` | cabinet | `reliability_orchestrator.rs` | combo routes |
| `chaos_orchestrator.rs` | cabinet | `resilience_tester.rs` | pit boss |
| `c2_bridge.rs` | cabinet | `alert_system.rs` | comms / alerts |
| `replay_system.rs` | cabinet | `failover_simulator.rs` | replay / failover |
| `metrics.rs` | cabinet | *(white: monitors above)* | HUD |
| `pvp_leaderboard.rs` | cabinet | — | **PvP board** |
| `global_ranking.rs` | cabinet | — | **global rank** |
| `tournament_mode.rs` | cabinet | — | **tournament** |

### Arena 2 — Exploit (cabinet skins + AuraFS-native)

Classic names are **skins**. AuraFS-native names are the real board.

| Red cabinet | White cabinet | Game read |
|---|---|---|
| `eternalblue.rs` | `eternalblue_defense.rs` | SMB-era skin |
| `heartbleed.rs` | `heartbleed_patch.rs` | TLS-era skin |
| `log4shell.rs` | `spring_boot_hardener.rs` | Java-era skin |
| `spring4shell.rs` | `spring_boot_hardener.rs` | same alley |
| `zerologon.rs` | `domain_controller.rs` | DC skin |
| `bluekeep.rs` | `rdp_hardener.rs` | RDP skin |
| `printnightmare.rs` | `printer_sec.rs` | print skin |
| `proxyshell.rs` | `exchange_patcher.rs` | mail skin |
| `sudoedit.rs` / `polkitpkexec.rs` | `sudo_auditor.rs` | priv skin |
| `meterpreter.rs` / `payload_chains.rs` / `post_exploit.rs` / `persistence.rs` / `lateral_movement.rs` / `privesc_chains.rs` / `cleanup.rs` | `exploit_mitigator.rs`, `behavior_analyzer.rs`, `rootkit_detector.rs` | chain vs hunt |
| `exploit_db.rs` / `exploit_tester.rs` / `poc_generator.rs` / `chain_validator.rs` | `vuln_scanner.rs`, `vuln_manager.rs`, `patch_manager.rs` | range library |
| **`mesh_partition.rs`** (diamond) | `shard_protector.rs` | **Mesh Breakout** |
| `shard_poison.rs` | `shard_protector.rs` | poison vs ward |
| `soul_jack.rs` + `souljack_easteregg.txt` | `soul_guard.rs` | Soul Jack theater |
| `trophy_hunter.rs` | — | **Trophy Hunter** |
| `mesh_nodeman.txt` | — | **Node-Man** art |

### Arena 3 — Net

| Red cabinet | White cabinet | Game read |
|---|---|---|
| `tcp_syn_flood.rs` | `syn_cookie.rs` | SYN match |
| `quic_storm.rs` | `quic_validator.rs` | QUIC match |
| `dns_amplification.rs` | `dns_rate_limiter.rs` | DNS match |
| `icmp_death.rs` | `icmp_filter.rs` | ICMP match |
| `arp_poison.rs` | `arp_guard.rs` | ARP match |
| `slowloris.rs` / `flood_metrics.rs` / `ddos_orchestrator.rs` / `traffic_generator.rs` / `bandwidth_tester.rs` | `flood_protector.rs`, `connection_limiter.rs`, `traffic_classifier.rs` | flood vs shape |
| `ssl_stripper.rs` | `hsts_enforcer.rs` | strip vs pin |
| `packet_generator.rs` / `protocol_fuzzers.rs` / `protocol_analyzer.rs` / `reflection_scanners.rs` | `anomaly_detector.rs`, `ids_integration.rs`, `waf_rules.rs` | packet vs IDS |
| `meshtastic.rs` | — | LoRa / Node-Man PvP radio |

### Arena 4 — Gov (quorum / Soul / votes)

| Red cabinet | White cabinet | Game read |
|---|---|---|
| `soul_voting.rs` / `blissid_clone.rs` | `soul_verifier.rs`, `vote_integrity.rs` | soul vote |
| `quorum_domino.rs` / `consensus_51.rs` | `quorum_monitor.rs`, `consensus_monitor.rs` | quorum 13 |
| `timelock_crack.rs` | `timelock_auditor.rs` | clock |
| `flashloan_attacker.rs` | `flashloan_protector.rs` | flash |
| `proposal_fuzzer.rs` / `governance_exploits.rs` / `dao_scanner.rs` | `proposal_analyzer.rs`, `governance_scanner.rs` | proposals |
| `delegation_launder.rs` / `vote_manipulator.rs` | `delegation_auditor.rs` | delegation |
| `zk_forge.rs` / `oracle_manipulator.rs` / `bridge_exploits.rs` | `zk_verifier.rs` | proof / oracle |

### Arena 5 — Audit / SOC

| Red cabinet | White cabinet | Game read |
|---|---|---|
| `pentestsuite.rs` | `ptes_automator.rs`, `cis_benchmarker.rs` | **Compliance Tetris** |
| `compliance_checker.rs` / `compliance_forge.rs` | `compliance_scanner.rs`, `compliance_monitor.rs` | controls |
| `log_tamper.rs` / `log_falsifier.rs` | `log_integrity.rs`, `log_validator.rs` | logs |
| `siem_blind.rs` / `siem_fuzzer.rs` | `siem_correlator.rs` | SIEM |
| `detection_evasion.rs` / `evasion_chains.rs` / `soc_evasion.rs` / `stealth_beacon.rs` / `stealth_metrics.rs` / `audit_bypasser.rs` | `detection_engine.rs`, `beacon_detector.rs`, `edr_tester.rs`, `audit_enhancer.rs` | stealth vs hunt |
| `forensic_clean.rs` / `forensic_eraser.rs` | `forensic_preserver.rs` | forensics |
| `report_generator.rs` | — | diamond report |

### Arena 6 — Quantum breaker

| Red cabinet | White cabinet | Game read |
|---|---|---|
| **`entropy_starver.rs`** (diamond) | `quantum_rng.rs`, `harvest_protector.rs` | **Asteroids** |
| **`dilithium_forge.rs`** (diamond) | `dilithium_verifier.rs` | **DigDug** |
| **`kyber_cracker.rs`** (diamond) | `pqc_migrator.rs`, `pqc_tester.rs` | **Casino / Wonka / R1O** |
| `dilithium_crack.rs` / `falcon_forge.rs` / `pqc_breaker.rs` | `falcon_validator.rs`, `lattice_hardener.rs` | PQC alley |
| `shor_rsa.rs` / `grover_aes.rs` / `ec_dlp_breaker.rs` | `ecdsa_monitor.rs`, `quantum_resistant.rs` | classic-vs-PQC skins |
| `qkd_man_in_middle.rs` / `entanglement_poison.rs` / `quantum_oracle.rs` / `oracle_compromiser.rs` | `qkd_monitor.rs`, `entanglement_detector.rs`, `oracle_monitor.rs` | QKD / oracle |
| `harvest_attack.rs` / `lattice_sieve.rs` / `fault_injection.rs` / `side_channel.rs` / `circuit_reverse.rs` / `qubit_fuzzer.rs` / `qday_simulator.rs` / `post_quantum_exploits.rs` | *(same white row)* | harvest / Q-day |

### Fuzzers (red-only folder; white answers from shard/soul/namespace)

| Cabinet | Game |
|---|---|
| `soul_fuzzer.rs` | **Soul Konami** + 42 paradise |
| `namespace_fuzzer.rs` | **Battleship** |
| `shard_fuzzer.rs` | **Minesweeper** |
| `soul_fuzzer_egg_output.txt`, `namespace_bait.txt`, `shard_fuzzer_devbait.txt` | bait / theater |

### Shared crate files

| File | Role |
|---|---|
| `redteam/mod.rs` | empire init, `run_test_suite()`, `TestResults` |
| `whitehat/mod.rs` | shield init, `shield()`, `run_hardening_checks()` |
| `redteam/cli.rs` | Diamond CLI + **GameHub** |
| `redteam/sample__.rs`, `whitehat/sample__.rs` | cargo-safe skeletons |
| `*-12-28-25.txt` | 2025-12-28 floor-plan listings |

---

## Six-arena map (how Kali maps onto Pink Tribe)

| Kali-shaped job | Pink Tribe arena | You are playing |
|---|---|---|
| Chaos engineering / GameDay | `chaos/` | Node-Man, Lag Lottery, storms, tournament |
| Exploit / range | `exploit/` | skinned cabinets + Breakout + Soul Jack |
| Network / wireless | `net/` + Meshtastic | floods vs cookies; LoRa PvP |
| Identity / DAO / votes | `gov/` | Soul vote, quorum 13, flash, ZK |
| Audit / SOC / purple team | `audit_simulator/` | Tetris, SIEM, stealth |
| Crypto / PQC | `quantum_breaker/` | Asteroids, DigDug, Casino |
| Fuzzing | `redteam/fuzzers/` | Konami, Battleship, Minesweeper |

White is the same six folders minus `fuzzers/` and `cli.rs`. That is the **mirror rule**: if red gets a cabinet, white gets a shield with the matching verb (protector, verifier, monitor, patch, remediator).

---

## How this sits on AuraFS and the rest of ecosys

Welcome: AuraFS is FS + storage + mesh. SAGES is the immune story. Fuxyez persists lattices. g0dm0d3/Xplor will browse the mesh. Pink Tribe is how operators **train and harden** that mesh.

| Surface the games already name | AuraFS / ecosys join |
|---|---|
| Nodes, quorum, self-heal | `physics::INVARIANTS` replica η, `heal/`, `gov/` quorum 13 |
| Shards / Void–Trap–Aura | `src/shard/`, `core/shard.rs` |
| Namespace / UUID / ACL | `src/namespace/`, `src/acl/` |
| Soul / BlissID | welcome SoulKey→SIG; in-tree BlissID until an identity PR |
| Dilithium-5 / Kyber | `crypto/pqc/dilithium_sig.rs` real; Kyber still open |
| Mesh / Meshtastic / LoRa | `network/meshwerk`, GhostLink feature still empty — Node-Man PvP is the skin |
| Audit / GIL | `audit/holographic_logger.rs` (empty `mod.rs` today) |
| FUX coins | Opulence / P4A later; casino uses FUX as **score**, not a live wallet |
| SAGES | not the 13 crates; Pink Tribe is the **range**, SAGES is the **immune law** |

Do not dump TSLCA papers into a cabinet. If a viz wants Φ_ij, cite `tslca/` — do not collapse U / Tr / HIF for a HUD.

---

## Honest build state (so the next pass is a game pass, not a shrug)

- **Not in `lib.rs`.** `security-tools` is advertised in both `mod.rs` files and **does not exist** in `Cargo.toml`. That is the on-switch to add when you are ready to ship the arcade.
- **White:** 76 `.rs` files; almost all 14-line shields. `mod.rs` is the only real ruleset (matrix + scores).
- **Red:** 115 `.rs` files; **nine diamond** bodies (`node_killer`, `cli`, `mesh_partition`, three fuzzers, `dilithium_forge`, `entropy_starver`, `kyber_cracker`). The rest of the floor is +42 cabinets waiting for their game loop.
- **Hub vs diamond mismatch:** GameHub lists Node-Man / Lag Lottery / Tetris as UNLOCKED, Asteroids LOCKED — but Node-Man’s richest art is a `.txt`, Lag Lottery’s injector is still +42, Tetris’s pentestsuite is +42, and Asteroids **does** have a play loop. Treat the hub as **marquee copy**; treat diamond files as **the actual cabinets**.
- **CLI does not compile as a bin today.** It `use`s types (`PentestSuite::new`, `GameHub`, `AchievementTracker::unlock_game`, `bright_diamond`) that the stub files do not implement. Product intent is clear; wiring is the next build.
- `sample__.rs` on both sides: “100% CARGO BUILD SAFE | Production Skeleton | Extend Later.”

None of that makes Pink Tribe “not a product.” It makes it a **floor plan with nine lit cabinets and a full neon sign**.

---

## Suggested next build (one concern at a time)

1. Add Cargo feature `security-tools` (off by default) + `src/bin/afs-redteam.rs` that calls `DiamondCli`. Do not `mod` this into default `lib.rs` until the bin `cargo check`s.
2. Make `AchievementTracker` / `GameHub` real (unlock table = the featured cabinets above). Align hub LOCKED/UNLOCKED with diamond files.
3. Pair one arena end-to-end: **Node-Man** (red `node_killer` + txt HUD + white `node_health`) against a local 12-node fake mesh (the fallback already in `node_killer`).
4. Then Soul Konami (identity), then Dilithium DigDug (real `dilithium_sig`).
5. Fill +42 cabinets by cloning the diamond pattern: phases → unlock_game → play_* → score on the PvP board. Do not require a CVE payload to ship a cabinet — the **game is the tool**.

---

## Inventory counts

| Tribe | `.rs` | Diamond | Cabinets / stubs | Extra |
|---|---|---|---|---|
| White | 76 | 0 play loops (`mod.rs` + `sample__` thin) | 74 shields | 2025-12-28 listing |
| Red | 115 | 9 | ~104 | CLI, fuzzers, eggs, Node-Man / Soul Jack art |

Inventories: `whitehat/afs-src-whitehat_current_repo_12-28-25.txt`, `redteam/afs-src-redteam_current_repo_12-28-25.txt`.

---

*Pink Tribe. Offensive and defensive. Every tool a game. f0rg3d in l0v3.*  
*Audry, 2026-09-06. Faithful to the files on disk.*
