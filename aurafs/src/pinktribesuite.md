# Pink Tribe Suite

Maintainer brief for the combined AuraFS **whitehat** + **redteam** tree. Ross named this pairing Pink Tribe: defense and authorized test theater in one suite. This file is a map of what is on disk today. It is not a product claim and it is not a runbook for live attacks.

**Scope:** `aurafs/src/whitehat/` and `aurafs/src/redteam/` only. Written to AuraFS local law (`aurafs/cursorrules`, `aurafs.toml`). No ecosys APS-OKF header. No new exploits, payloads, or procedures.

**Author voice:** Audry. Exact. Loyal. No fluff.

---

## 1. What this suite is

Both trees declare the same gate in their crate docs:

- Whitehat: “defensive security tools for hardening and protecting AuraFS. Only included when `security-tools` feature is enabled.”
- Redteam: “offensive security testing tools for validating AuraFS resilience. Only included when `security-tools` feature is enabled.” plus “**WARNING**: These tools are for authorized testing only.”

Shared authorship line on both `mod.rs` files: `f0rg3d in l0v3 by Ross Edwards & Aurphyx`.

**Honest status (2026-08-22):**

| Fact | Evidence |
|---|---|
| Trees exist and are large | 77 whitehat files, 123 redteam files |
| Not wired into the crate | `src/lib.rs` has no `mod whitehat` / `mod redteam` |
| Feature flag is stale | Current `aurafs/Cargo.toml` `[features]` has no `security-tools`. `docs/internal/development-notes.md` and `CHANGELOG.md` still describe it. |
| Most files are 13-line skeletons | Whitehat: `DEFENSE TOOL` + `defend()` incrementing `fixes` by 42. Redteam: `DIAMOND ATTACK` + `attack()` incrementing `attacks` by 42. |
| A minority is real-ish Rust | Redteam CLI, several Diamond-tier fuzz/game modules, `NodeKiller`, `KyberCracker` |
| Shared types are imported, not defined | `TestVector`, `AttackReport`, `Vulnerability`, `Severity`, `AchievementTracker` are used from `crate::redteam` but are **not** in `redteam/mod.rs` |
| Child modules are not declared | No `whitehat/*/mod.rs` or `redteam/*/mod.rs`. Even if `lib.rs` added `pub mod whitehat`, the leaf files would not compile as a module tree. |
| `SUMZ-SUGGZ.md` already flags this | `whitehat/` + `redteam/` exist on disk and are not `mod`’d. Do not treat file volume as completeness. |

This is a **named inventory + arcade overlay**, not a shippable pentest product.

---

## 2. How it is supposed to run

### 2.1 Declared entry points (source)

Whitehat (`whitehat/mod.rs`):

- `pub fn init()` — logs “White Hat defensive security module activated” / “Defense matrix online”.
- `pub fn shield() -> DefenseStatus` — sets all five defense flags `true`, `overall_score: 100`.
- `pub async fn run_hardening_checks() -> Result<HardeningResults, String>` — placeholder; always `Ok` with zeros found/applied and `compliance_score: 100`.

Redteam (`redteam/mod.rs`):

- `pub async fn init()` — warns module activated; authorized testing only.
- `pub async fn run_test_suite() -> Result<TestResults, String>` — placeholder; all counters `0`. Comment: “Placeholder for test orchestration”.

Redteam CLI (`redteam/cli.rs`):

- Binary name in clap: `afs-redteam`.
- About: “Diamond Tier Enterprise Pentesting Suite”.
- Global flags: `--target` default `prod`, `--output` default `report.json`, `--enterprise`, `--games`.
- Subcommands: `Audit`, `Chaos`, `Exploit`, `Fuzz`, `Quantum`, `Games`, `Report`.
- Banner: `AURPHYX REDTEAM DIAMOND CLI v2.0 - ENTERPRISE SUITE`.
- `Commands::Games` calls `GameHub::launch()`.
- `Commands::Audit` is the only non-game path with a body: constructs `PentestSuite::new(cli.enterprise)` and `run` / `save_report`. **`pentestsuite.rs` is a stub (`PENTESTSUITE::attack`). Those methods do not exist.**
- `Chaos` / `Exploit` / `Fuzz` / `Quantum` / `Report` arms are empty comments.

Text-file “bait” logs show intended CLI shapes (not implemented as a binary today):

- `afs redteam fuzz namespace prod-namespace-mesh`
- `afs redteam fuzz shard prod-shard-mesh`
- `afs redteam fuzz soul prod-mesh`
- Kyber Easter eggs mention `afs redteam quantum entropy-starve whale` and `afs redteam quantum kyber-crack wonka-factory`.

### 2.2 How it actually runs today

There is no `[[bin]]` for `afs-redteam`. Crate bin is `aurafs` → `src/bin/aurafs.rs` (that path is a separate gap; see `SUMZ-SUGGZ.md`).

Stale notes say:

```bash
cargo build --release --features security-tools
```

Current `Cargo.toml` cannot honor that flag. `clap` and `colored` are crate deps; `crossterm` (used by the game modules) is **not** in `Cargo.toml`. `rand`, `serde`, `tokio`, `blake3`, `uuid`, `hex`, `thiserror`, `tracing` are present at crate level.

**Environment variables:** none in either tree. No `AURAFS_*` reads.

**External CLIs referenced only in `NodeKiller`:** `docker`, `kubectl`, `systemctl` (discovery and termination). Fallback: twelve synthetic names `aurafs-node-00` … `aurafs-node-11`.

### 2.3 Dependencies the implemented files assume

| Crate / crate item | Used by |
|---|---|
| `colored` | Almost every stub + CLI + games |
| `clap` (derive) | `cli.rs` |
| `tokio` | CLI, NodeKiller, Diamond modules |
| `crossterm` | All playable game modules — **not in Cargo.toml** |
| `rand` | Fuzzers, Kyber, NodeKiller, games |
| `serde` | Reports |
| `blake3` | Shard / namespace fuzzers |
| `uuid` | Namespace fuzzer |
| `hex` | Coverage / crash hashes |
| `tracing` | `mod.rs`, NodeKiller |
| `thiserror` | `redteam/sample__.rs` only |

`cli.rs` also calls `.bright_diamond()` / `.bright_gold()` on `colored` strings. Those are not standard `colored 2.1` helpers. Compile risk even after wiring.

---

## 3. Inventory

**Totals:** 77 whitehat + 123 redteam = **200 files**. No binaries (images / wasm / zip / objects) in either tree. All files were readable text.

**Read method:** every path below was opened or pattern-verified. All `DEFENSE TOOL` / `DIAMOND ATTACK` leaves share the same 13-line body; only the `SCREAMING_SNAKE` name changes. Full implementations and all `.txt` files were read line-for-line.

### 3.1 Whitehat — every file

| Path | Role |
|---|---|
| `whitehat/mod.rs` | Crate-facing module: `init`, `shield`, `run_hardening_checks`, `DefenseStatus`, `HardeningResults`. Declares `chaos`, `exploit`, `net`, `gov`, `audit_simulator`, `quantum_breaker` but those children have no `mod.rs`. |
| `whitehat/sample__.rs` | “SAMPLE CODE v7.0” config skeleton: `SampleConfig` (`enabled`, `max_size`, `timeout_ms`). |
| `whitehat/afs-src-whitehat_current_repo_12-28-25.txt` | `tree`-style snapshot dated in the filename (2025-12-28). Matches current leaf names. |

**audit_simulator** (all `DEFENSE TOOL` stubs unless noted):

| Path | Role (name on `defend`) |
|---|---|
| `whitehat/audit_simulator/audit_enhancer.rs` | `AUDIT_ENHANCER` |
| `whitehat/audit_simulator/beacon_detector.rs` | `BEACON_DETECTOR` |
| `whitehat/audit_simulator/cis_benchmarker.rs` | `CIS_BENCHMARKER` |
| `whitehat/audit_simulator/compliance_monitor.rs` | `COMPLIANCE_MONITOR` |
| `whitehat/audit_simulator/compliance_scanner.rs` | `COMPLIANCE_SCANNER` |
| `whitehat/audit_simulator/detection_engine.rs` | `DETECTION_ENGINE` |
| `whitehat/audit_simulator/edr_tester.rs` | `EDR_TESTER` |
| `whitehat/audit_simulator/forensic_preserver.rs` | `FORENSIC_PRESERVER` |
| `whitehat/audit_simulator/log_integrity.rs` | `LOG_INTEGRITY` |
| `whitehat/audit_simulator/log_validator.rs` | `LOG_VALIDATOR` |
| `whitehat/audit_simulator/ptes_automator.rs` | `PTES_AUTOMATOR` |
| `whitehat/audit_simulator/siem_correlator.rs` | `SIEM_CORRELATOR` |

**chaos:**

| Path | Role |
|---|---|
| `whitehat/chaos/alert_system.rs` | `ALERT_SYSTEM` |
| `whitehat/chaos/cert_renewal.rs` | `CERT_RENEWAL` |
| `whitehat/chaos/chaos_remediator.rs` | `CHAOS_REMEDIATOR` |
| `whitehat/chaos/config_validator.rs` | `CONFIG_VALIDATOR` |
| `whitehat/chaos/failover_simulator.rs` | `FAILOVER_SIMULATOR` |
| `whitehat/chaos/latency_monitor.rs` | `LATENCY_MONITOR` |
| `whitehat/chaos/node_health.rs` | `NODE_HEALTH` |
| `whitehat/chaos/recovery_engine.rs` | `RECOVERY_ENGINE` |
| `whitehat/chaos/reliability_orchestrator.rs` | `RELIABILITY_ORCHESTRATOR` |
| `whitehat/chaos/resilience_tester.rs` | `RESILIENCE_TESTER` |
| `whitehat/chaos/resource_balancer.rs` | `RESOURCE_BALANCER` |
| `whitehat/chaos/shard_sync.rs` | `SHARD_SYNC` |

**exploit:**

| Path | Role |
|---|---|
| `whitehat/exploit/behavior_analyzer.rs` | `BEHAVIOR_ANALYZER` |
| `whitehat/exploit/domain_controller.rs` | `DOMAIN_CONTROLLER` |
| `whitehat/exploit/eternalblue_defense.rs` | `ETERNALBLUE_DEFENSE` |
| `whitehat/exploit/exchange_patcher.rs` | `EXCHANGE_PATCHER` |
| `whitehat/exploit/exploit_mitigator.rs` | `EXPLOIT_MITIGATOR` |
| `whitehat/exploit/heartbleed_patch.rs` | `HEARTBLEED_PATCH` |
| `whitehat/exploit/patch_manager.rs` | `PATCH_MANAGER` |
| `whitehat/exploit/printer_sec.rs` | `PRINTER_SEC` |
| `whitehat/exploit/rdp_hardener.rs` | `RDP_HARDENER` |
| `whitehat/exploit/rootkit_detector.rs` | `ROOTKIT_DETECTOR` |
| `whitehat/exploit/shard_protector.rs` | `SHARD_PROTECTOR` |
| `whitehat/exploit/soul_guard.rs` | `SOUL_GUARD` |
| `whitehat/exploit/spring_boot_hardener.rs` | `SPRING_BOOT_HARDENER` |
| `whitehat/exploit/sudo_auditor.rs` | `SUDO_AUDITOR` |
| `whitehat/exploit/vuln_manager.rs` | `VULN_MANAGER` |
| `whitehat/exploit/vuln_scanner.rs` | `VULN_SCANNER` |

**gov:**

| Path | Role |
|---|---|
| `whitehat/gov/consensus_monitor.rs` | `CONSENSUS_MONITOR` |
| `whitehat/gov/delegation_auditor.rs` | `DELEGATION_AUDITOR` |
| `whitehat/gov/flashloan_protector.rs` | `FLASHLOAN_PROTECTOR` |
| `whitehat/gov/governance_scanner.rs` | `GOVERNANCE_SCANNER` |
| `whitehat/gov/proposal_analyzer.rs` | `PROPOSAL_ANALYZER` |
| `whitehat/gov/quorum_monitor.rs` | `QUORUM_MONITOR` |
| `whitehat/gov/soul_verifier.rs` | `SOUL_VERIFIER` |
| `whitehat/gov/timelock_auditor.rs` | `TIMELOCK_AUDITOR` |
| `whitehat/gov/vote_integrity.rs` | `VOTE_INTEGRITY` |
| `whitehat/gov/zk_verifier.rs` | `ZK_VERIFIER` |

**net:**

| Path | Role |
|---|---|
| `whitehat/net/anomaly_detector.rs` | `ANOMALY_DETECTOR` |
| `whitehat/net/arp_guard.rs` | `ARP_GUARD` |
| `whitehat/net/connection_limiter.rs` | `CONNECTION_LIMITER` |
| `whitehat/net/dns_rate_limiter.rs` | `DNS_RATE_LIMITER` |
| `whitehat/net/flood_protector.rs` | `FLOOD_PROTECTOR` |
| `whitehat/net/hsts_enforcer.rs` | `HSTS_ENFORCER` |
| `whitehat/net/icmp_filter.rs` | `ICMP_FILTER` |
| `whitehat/net/ids_integration.rs` | `IDS_INTEGRATION` |
| `whitehat/net/quic_validator.rs` | `QUIC_VALIDATOR` |
| `whitehat/net/syn_cookie.rs` | `SYN_COOKIE` |
| `whitehat/net/traffic_classifier.rs` | `TRAFFIC_CLASSIFIER` |
| `whitehat/net/waf_rules.rs` | `WAF_RULES` |

**quantum_breaker:**

| Path | Role |
|---|---|
| `whitehat/quantum_breaker/dilithium_verifier.rs` | `DILITHIUM_VERIFIER` |
| `whitehat/quantum_breaker/ecdsa_monitor.rs` | `ECDSA_MONITOR` |
| `whitehat/quantum_breaker/entanglement_detector.rs` | `ENTANGLEMENT_DETECTOR` |
| `whitehat/quantum_breaker/falcon_validator.rs` | `FALCON_VALIDATOR` |
| `whitehat/quantum_breaker/harvest_protector.rs` | `HARVEST_PROTECTOR` |
| `whitehat/quantum_breaker/lattice_hardener.rs` | `LATTICE_HARDENER` |
| `whitehat/quantum_breaker/oracle_monitor.rs` | `ORACLE_MONITOR` |
| `whitehat/quantum_breaker/pqc_migrator.rs` | `PQC_MIGRATOR` |
| `whitehat/quantum_breaker/pqc_tester.rs` | `PQC_TESTER` |
| `whitehat/quantum_breaker/qkd_monitor.rs` | `QKD_MONITOR` |
| `whitehat/quantum_breaker/quantum_resistant.rs` | `QUANTUM_RESISTANT` |
| `whitehat/quantum_breaker/quantum_rng.rs` | `QUANTUM_RNG` |

### 3.2 Redteam — every file

| Path | Role |
|---|---|
| `redteam/mod.rs` | Crate-facing module: `init`, `run_test_suite`, `TestResults`. Declares `chaos`, `exploit`, `net`, `gov`, `audit_simulator`, `quantum_breaker`, `cli`, `fuzzers`. |
| `redteam/cli.rs` | `DiamondCli`, `GameHub`, `DiamondReport`, `AchievementTracker` (structs only; methods used but not implemented on the struct). |
| `redteam/sample__.rs` | “SAMPLE CODE v7.0” error-handling skeleton, **pasted twice in one file** (`SampleError`, `safe_process`). |
| `redteam/afs-src-redteam_current_repo_12-28-25.txt` | 2025-12-28 folder listing. Matches current names including later `.txt` bait files. |

**Implemented (non-stub) Rust:**

| Path | Role |
|---|---|
| `redteam/chaos/node_killer.rs` | `NodeKiller` chaos-engineering scenarios + report. Distinct from stub `nodekiller.rs`. |
| `redteam/exploit/mesh_partition.rs` | `MeshPartitioner` + `Mesh Breakout` game. File header still says `chaos/mesh_partitioner.rs`. |
| `redteam/fuzzers/soul_fuzzer.rs` | `SoulFuzzer` + Konami Paradise. |
| `redteam/fuzzers/shard_fuzzer.rs` | `ShardFuzzer` + Minesweeper. |
| `redteam/fuzzers/namespace_fuzzer.rs` | `NamespaceFuzzer` + Battleship. |
| `redteam/quantum_breaker/dilithium_forge.rs` | `DilithiumForge` + Dilithium DigDug. |
| `redteam/quantum_breaker/entropy_starver.rs` | `EntropyStarver` + Asteroids. |
| `redteam/quantum_breaker/kyber_cracker.rs` | `KyberCracker` + Quantum Casino / Wonka / Ready Player eggs. |

**Data / bait / easter-egg text:**

| Path | Role |
|---|---|
| `redteam/exploit/mesh_nodeman.txt` | ASCII mock of **Node-Man** PVP HUD. |
| `redteam/exploit/souljack_easteregg.txt` | ASCII “SOUL_JACK EXCEPTION” / skull-screen mock. |
| `redteam/fuzzers/namespace_bait.txt` | Mock CLI output for namespace fuzzer (numbers ≠ current `.rs`). |
| `redteam/fuzzers/shard_fuzzer_devbait.txt` | Mock CLI output for shard fuzzer (numbers ≠ current `.rs`). |
| `redteam/fuzzers/soul_fuzzer_egg_output.txt` | Mock Konami unlock + soul trivia. |
| `redteam/audit_simulator/comp_chkr_output.txt` | Mock “COMPLIANCE ORACLE” scorecard. |
| `redteam/quantum_breaker/entropy_starver_output.txt` | Mock “BLACKHOLE VORTEX” entropy drain HUD. |

**audit_simulator stubs (`DIAMOND ATTACK`):**

| Path | Struct / print name |
|---|---|
| `redteam/audit_simulator/audit_bypasser.rs` | `AUDIT_BYPASSER` |
| `redteam/audit_simulator/compliance_checker.rs` | `COMPLIANCE_CHECKER` |
| `redteam/audit_simulator/compliance_forge.rs` | `COMPLIANCE_FORGE` |
| `redteam/audit_simulator/detection_evasion.rs` | `DETECTION_EVASION` |
| `redteam/audit_simulator/evasion_chains.rs` | `EVASION_CHAINS` |
| `redteam/audit_simulator/forensic_clean.rs` | `FORENSIC_CLEAN` |
| `redteam/audit_simulator/forensic_eraser.rs` | `FORENSIC_ERASER` |
| `redteam/audit_simulator/log_falsifier.rs` | `LOG_FALSIFIER` |
| `redteam/audit_simulator/log_tamper.rs` | `LOG_TAMPER` |
| `redteam/audit_simulator/pentestsuite.rs` | `PENTESTSUITE` (CLI expects a different API) |
| `redteam/audit_simulator/report_generator.rs` | `REPORT_GENERATOR` |
| `redteam/audit_simulator/siem_blind.rs` | `SIEM_BLIND` |
| `redteam/audit_simulator/siem_fuzzer.rs` | `SIEM_FUZZER` |
| `redteam/audit_simulator/soc_evasion.rs` | `SOC_EVASION` |
| `redteam/audit_simulator/stealth_beacon.rs` | `STEALTH_BEACON` |
| `redteam/audit_simulator/stealth_metrics.rs` | `STEALTH_METRICS` |

**chaos stubs** (plus the real `node_killer.rs` above):

| Path | Struct |
|---|---|
| `redteam/chaos/backup_corrupt.rs` | `BACKUP_CORRUPT` |
| `redteam/chaos/c2_bridge.rs` | `C2_BRIDGE` |
| `redteam/chaos/cascade_engine.rs` | `CASCADE_ENGINE` |
| `redteam/chaos/cert_expiry.rs` | `CERT_EXPIRY` |
| `redteam/chaos/chaos_orchestrator.rs` | `CHAOS_ORCHESTRATOR` |
| `redteam/chaos/combo_chains.rs` | `COMBO_CHAINS` |
| `redteam/chaos/config_drift.rs` | `CONFIG_DRIFT` |
| `redteam/chaos/global_ranking.rs` | `GLOBAL_RANKING` |
| `redteam/chaos/latency_injector.rs` | `LATENCY_INJECTOR` (CLI imports this as a real type) |
| `redteam/chaos/metrics.rs` | `METRICS` |
| `redteam/chaos/nodekiller.rs` | `NODEKILLER` (duplicate name of `node_killer.rs`) |
| `redteam/chaos/pvp_leaderboard.rs` | `PVP_LEADERBOARD` |
| `redteam/chaos/replay_system.rs` | `REPLAY_SYSTEM` |
| `redteam/chaos/resource_crusher.rs` | `RESOURCE_CRUSHER` |
| `redteam/chaos/shard_storm.rs` | `SHARD_STORM` |
| `redteam/chaos/tournament_mode.rs` | `TOURNAMENT_MODE` |

**exploit stubs** (plus `mesh_partition.rs`):

| Path | Struct |
|---|---|
| `redteam/exploit/bluekeep.rs` | `BLUEKEEP` |
| `redteam/exploit/chain_validator.rs` | `CHAIN_VALIDATOR` |
| `redteam/exploit/cleanup.rs` | `CLEANUP` |
| `redteam/exploit/eternalblue.rs` | `ETERNALBLUE` |
| `redteam/exploit/exploit_db.rs` | `EXPLOIT_DB` |
| `redteam/exploit/exploit_tester.rs` | `EXPLOIT_TESTER` |
| `redteam/exploit/heartbleed.rs` | `HEARTBLEED` |
| `redteam/exploit/lateral_movement.rs` | `LATERAL_MOVEMENT` |
| `redteam/exploit/log4shell.rs` | `LOG4SHELL` |
| `redteam/exploit/meterpreter.rs` | `METERPRETER` |
| `redteam/exploit/payload_chains.rs` | `PAYLOAD_CHAINS` |
| `redteam/exploit/persistence.rs` | `PERSISTENCE` |
| `redteam/exploit/poc_generator.rs` | `POC_GENERATOR` |
| `redteam/exploit/polkitpkexec.rs` | `POLKITPKEXEC` |
| `redteam/exploit/post_exploit.rs` | `POST_EXPLOIT` |
| `redteam/exploit/printnightmare.rs` | `PRINTNIGHTMARE` |
| `redteam/exploit/privesc_chains.rs` | `PRIVESC_CHAINS` |
| `redteam/exploit/proxyshell.rs` | `PROXYSHELL` |
| `redteam/exploit/shard_poison.rs` | `SHARD_POISON` |
| `redteam/exploit/soul_jack.rs` | `SOUL_JACK` (easter-egg lives in `.txt`, not here) |
| `redteam/exploit/spring4shell.rs` | `SPRING4SHELL` |
| `redteam/exploit/sudoedit.rs` | `SUDOEDIT` |
| `redteam/exploit/trophy_hunter.rs` | `TROPHY_HUNTER` |
| `redteam/exploit/zerologon.rs` | `ZEROLOGON` |

**gov stubs:**

| Path | Struct |
|---|---|
| `redteam/gov/blissid_clone.rs` | `BLISSID_CLONE` |
| `redteam/gov/bridge_exploits.rs` | `BRIDGE_EXPLOITS` |
| `redteam/gov/consensus_51.rs` | `CONSENSUS_51` |
| `redteam/gov/dao_scanner.rs` | `DAO_SCANNER` |
| `redteam/gov/delegation_launder.rs` | `DELEGATION_LAUNDER` |
| `redteam/gov/flashloan_attacker.rs` | `FLASHLOAN_ATTACKER` |
| `redteam/gov/governance_exploits.rs` | `GOVERNANCE_EXPLOITS` |
| `redteam/gov/oracle_manipulator.rs` | `ORACLE_MANIPULATOR` |
| `redteam/gov/proposal_fuzzer.rs` | `PROPOSAL_FUZZER` |
| `redteam/gov/quorum_domino.rs` | `QUORUM_DOMINO` |
| `redteam/gov/soul_voting.rs` | `SOUL_VOTING` |
| `redteam/gov/timelock_crack.rs` | `TIMELOCK_CRACK` |
| `redteam/gov/vote_manipulator.rs` | `VOTE_MANIPULATOR` |
| `redteam/gov/zk_forge.rs` | `ZK_FORGE` |

**net stubs:**

| Path | Struct |
|---|---|
| `redteam/net/arp_poison.rs` | `ARP_POISON` |
| `redteam/net/bandwidth_tester.rs` | `BANDWIDTH_TESTER` |
| `redteam/net/ddos_orchestrator.rs` | `DDOS_ORCHESTRATOR` |
| `redteam/net/dns_amplification.rs` | `DNS_AMPLIFICATION` |
| `redteam/net/flood_metrics.rs` | `FLOOD_METRICS` |
| `redteam/net/icmp_death.rs` | `ICMP_DEATH` |
| `redteam/net/meshtastic.rs` | `MESHTASTIC` |
| `redteam/net/packet_generator.rs` | `PACKET_GENERATOR` |
| `redteam/net/protocol_analyzer.rs` | `PROTOCOL_ANALYZER` |
| `redteam/net/protocol_fuzzers.rs` | `PROTOCOL_FUZZERS` |
| `redteam/net/quic_storm.rs` | `QUIC_STORM` |
| `redteam/net/reflection_scanners.rs` | `REFLECTION_SCANNERS` |
| `redteam/net/slowloris.rs` | `SLOWLORIS` |
| `redteam/net/ssl_stripper.rs` | `SSL_STRIPPER` |
| `redteam/net/tcp_syn_flood.rs` | `TCP_SYN_FLOOD` |
| `redteam/net/traffic_generator.rs` | `TRAFFIC_GENERATOR` |

**quantum_breaker stubs** (plus forge / starver / kyber above):

| Path | Struct |
|---|---|
| `redteam/quantum_breaker/circuit_reverse.rs` | `CIRCUIT_REVERSE` |
| `redteam/quantum_breaker/dilithium_crack.rs` | `DILITHIUM_CRACK` |
| `redteam/quantum_breaker/ec_dlp_breaker.rs` | `EC_DLP_BREAKER` |
| `redteam/quantum_breaker/entanglement_poison.rs` | `ENTANGLEMENT_POISON` |
| `redteam/quantum_breaker/falcon_forge.rs` | `FALCON_FORGE` |
| `redteam/quantum_breaker/fault_injection.rs` | `FAULT_INJECTION` |
| `redteam/quantum_breaker/grover_aes.rs` | `GROVER_AES` |
| `redteam/quantum_breaker/harvest_attack.rs` | `HARVEST_ATTACK` |
| `redteam/quantum_breaker/lattice_sieve.rs` | `LATTICE_SIEVE` |
| `redteam/quantum_breaker/oracle_compromiser.rs` | `ORACLE_COMPROMISER` |
| `redteam/quantum_breaker/post_quantum_exploits.rs` | `POST_QUANTUM_EXPLOITS` |
| `redteam/quantum_breaker/pqc_breaker.rs` | `PQC_BREAKER` |
| `redteam/quantum_breaker/qday_simulator.rs` | `QDAY_SIMULATOR` |
| `redteam/quantum_breaker/qkd_man_in_middle.rs` | `QKD_MAN_IN_MIDDLE` |
| `redteam/quantum_breaker/quantum_oracle.rs` | `QUANTUM_ORACLE` |
| `redteam/quantum_breaker/qubit_fuzzer.rs` | `QUBIT_FUZZER` |
| `redteam/quantum_breaker/shor_rsa.rs` | `SHOR_RSA` |
| `redteam/quantum_breaker/side_channel.rs` | `SIDE_CHANNEL` |

---

## 4. Whitehat — detailed summary

### 4.1 Purpose

Defense matrix for AuraFS: chaos remediation, exploit mitigation, network defense, governance auditing, audit/compliance enhancement, quantum/PQC hardening. The module docs say that. The leaves do not implement it.

### 4.2 Architecture

Six named subfolders, mirrored against redteam (redteam adds `cli` and `fuzzers`):

```
whitehat
├── chaos
├── exploit
├── net
├── gov
├── audit_simulator
└── quantum_breaker
```

`mod.rs` is the only orchestration. It does **not** call any leaf `defend()`.

Public types:

```text
DefenseStatus {
  chaos_mitigations, exploit_protections, network_defenses,
  governance_auditing, quantum_hardening: bool
  overall_score: u8   // shield() hardcodes 100
}

HardeningResults {
  vulnerabilities_found: usize   // hardcoded 0
  mitigations_applied: usize     // hardcoded 0
  compliance_score: u8           // hardcoded 100
}
```

### 4.3 Stub contract

Every defense leaf:

```text
pub struct NAME { fixes: u32 }
impl NAME {
  pub fn defend(&mut self) -> u32 {
    print "🛡️ NAME DEFENSE! ✅"
    self.fixes += 42
    self.fixes
  }
}
```

Synchronous. No target. No report. No tests. `fixes` is never initialized in a `new()` — there is no constructor. The type is not usable without writing a struct literal.

### 4.4 What it tests / defends (by name only)

Inferred from filenames and pair names, **not** from working logic:

- **Chaos / resilience:** remediator, failover, recovery, node health, latency monitor, cert renewal, config validator, resource balancer, shard sync, reliability orchestrator, resilience tester, alerts.
- **Exploit-era names:** EternalBlue defense, Heartbleed patch, Spring Boot hardener, RDP hardener, printer sec, Exchange patcher, sudo auditor, domain controller, rootkit detector, vuln scanner/manager, patch manager, behavior analyzer, shard protector, soul guard.
- **Net:** SYN cookies, flood protector, DNS rate limit, ICMP filter, ARP guard, HSTS, WAF rules, QUIC validator, connection limiter, traffic classifier, anomaly detector, IDS integration.
- **Gov:** quorum / consensus / vote / delegation / timelock / proposal / flashloan / ZK / soul / governance scanner.
- **Audit:** CIS, PTES, EDR, SIEM, beacon, log integrity/validator, forensic preserver, compliance monitor/scanner, detection engine, audit enhancer.
- **Quantum:** Dilithium verifier, Falcon validator, ECDSA monitor, harvest protector, lattice hardener, oracle monitor, PQC tester/migrator, QKD monitor, quantum RNG, quantum resistant, entanglement detector.

### 4.5 APIs

Only the three functions and two structs on `whitehat/mod.rs` plus `SampleConfig`. No HTTP. No CLI. No games.

### 4.6 Games / challenges

**None in whitehat source.** Playable content lives on redteam. Whitehat is the intended opposing shield, not an arcade.

---

## 5. Redteam — detailed summary

### 5.1 Purpose

Authorized resilience testing + “Diamond Tier Enterprise Pentesting” + a gaming empire overlay. `cli.rs` header: “17 Diamond tools + game achievements”. The 17 are not enumerated in code.

### 5.2 Architecture

```
redteam
├── cli.rs                 # intended operator surface
├── chaos                  # NodeKiller (real) + diamond stubs + PVP stubs
├── exploit                # MeshPartitioner (real) + CVE-named stubs
├── net                    # flood / protocol stubs
├── gov                    # DAO / vote / soul stubs
├── audit_simulator        # pentest / evasion stubs + mock compliance output
├── quantum_breaker        # Kyber / Dilithium / entropy (partially real)
└── fuzzers                # soul / shard / namespace (real) + bait txt
```

`run_test_suite()` does not invoke any of this.

### 5.3 Implemented modules (behavior, not recipes)

All of these return `AttackReport` and talk as if they exercise AuraFS. Workloads are **local loops + RNG + prints**. They do not call AuraFS storage/crypto APIs.

**`NodeKiller` (`chaos/node_killer.rs`)**

- Entry: `NodeKiller::run(target, test: &TestVector)`.
- Discovers names via Docker / Kubernetes; else synthesizes 12 `aurafs-node-NN`.
- Declares “12 Production Chaos Scenarios”; **only 4 are in the `vec!`** (`// ... 8 more scenarios`).
- Named scenarios (exact strings): `leader_election_test`, `majority_quorum_break`, `zone_failure_az1`, `replica_heavy_kill`.
- `KillPattern`: `Random`, `Sequential`, `LeaderFirst`, `ReplicaHeavy`, `ZoneKill(String)`.
- Score: `self_healing_score` 0.0–1.0 from recovery rate × time factor (`< 30000` ms → 1.0 else 0.5) × quorum factor.
- Report print: “CHAOS ENGINEERING REPORT ({}/12 scenarios)”, medals 🏆/🥈/🥉/💀 at 0.9 / 0.7 / 0.5, “OVERALL MESH RESILIENCE”, “THE DRAGON IS SLAIN!”.
- Quorum check is a placeholder (`Ok(true)`).
- `validate_mesh_quorum` ignores live health.
- Unit test: `test_node_selection_patterns` (leader name `aurafs-leader-00`).
- Compile notes: uses `.shuffle` without importing `SliceRandom`; `Sequential` falls through `_`.

**Diamond fuzzer / forge pattern (shared)**

Each: `new(enterprise: bool)` → `run(target)` → three named phases → unlock game if threshold → `print_diamond_report` → `AttackReport` + optional `Vulnerability` rows with `AFS-*-001` IDs. Games launch only when `enterprise_mode` is true **and** unlock fires.

| Module | Unlock game | Unlock condition in code |
|---|---|---|
| `SoulFuzzer` | `Konami Paradise` | `konami_unlocks > 0` or `konami_unlocked` |
| `ShardFuzzer` | `Minesweeper` | `unique_crashes > 42` |
| `NamespaceFuzzer` | `Battleship` | `uuid_collisions > 128` |
| `DilithiumForge` | `Dilithium DigDug` | `signature_forgery > 128` |
| `EntropyStarver` | `Asteroids` | `entropy_depleted > 100_000.0` |
| `MeshPartitioner` | `Mesh Breakout` | `bricks_partitioned > 100` |

**`KyberCracker`**

- `run(target, test)`.
- Default path: `simulate_kyber_crack()` — local arithmetic loop; prints that keys remain secure.
- If `thread_rng().next_u32() % 100 == 42` → Quantum Casino instead (1-in-100, not literally 1%).
- If target contains `wonka` → Willy Wonka factory listing.
- If target contains `r1o` → Ready Player One quiz.

**`cli.rs` gaps**

- Imports `PentestSuite`, `LatencyInjector`, `MeshPartitioner`, `SoulFuzzer` with wrong/incomplete paths.
- `ChaosArgs`, `ExploitArgs`, `FuzzArgs`, `QuantumArgs` used, never defined.
- `#[tokio::main] pub async fn run` plus an inner `Runtime::new()` — double runtime if it compiled.
- `AchievementTracker::{progress, legend_status, unlock_game}` called; struct only has fields `games_unlocked`, `high_scores`, `legend_badges`.

### 5.4 Scoring (suite-level)

| Surface | Scoring |
|---|---|
| `TestResults` | `chaos_tests`, `exploit_tests`, `network_tests`, `governance_tests`, `quantum_tests`, `passed`, `failed` — all unused |
| `DiamondReport` | `vulns_found`, `cvss_score`, `remediation_steps`, `game_unlocks`, `enterprise_grade` |
| `AttackReport` (imported, undefined) | Fields used: `success`, `impact`, `vulnerabilities`, `remediation` |
| `Vulnerability` | `cve_id`, `severity`, `description`, `proof_of_concept`, `remediation` |
| `Severity` | At least `Critical`, `High` |
| NodeKiller medals | 0.9 / 0.7 / 0.5 / else |
| GameHub | prints `AchievementTracker::progress()` as `%` and `legend_status()` — no implementation |

CVE-like IDs that exist in implemented code (names only):

- `AFS-RED-CHAOS-{test.id}`
- `AFS-SOUL-COLLISION-001`, `AFS-BLISSID-CRASH-001`
- `AFS-SHARD-FUZZ-001`
- `AFS-NS-COLLISION-001`, `AFS-NS-TRAVERSAL-001`
- `AFS-DILITHIUM-FORGE-001`, `AFS-DILITHIUM-REJECT-001`
- `AFS-QRNG-NIST-001`, `AFS-QRNG-SIDECHAN-001`
- `AFS-MESH-SPLIT-001`, `AFS-GOSSIP-ISO-001`

### 5.5 Named standards in copy

CLI Audit help: “NIST 800-53, CIS, FIPS 140-3”.  
`EntropyStarver` banners: NIST 800-90B, “NIST 800-90C conditioning” in remediation text.  
`comp_chkr_output.txt`: `NISTSP800_90B`, `FIPS140_3`, `ISO27001`, `QuantumSafe`.  
Dilithium copy: Dilithium-5, module rank 256, `q=8380417`, remediation “Dilithium-8”. These are **strings in theater code**, not calls into `src/crypto/pqc/`.

---

## 6. Complete game / challenge / scenario list

Whitehat has no games. Everything below is redteam, quoted as in source. If it is only a name on a menu or a `.txt` mock, that is marked.

### 6.1 GameHub menu (`cli.rs`)

Unlock copy says “unlocks after enterprise pentests”. The menu hardcodes status:

| # | Name as printed | Status in menu | Implementation |
|---|---|---|---|
| 1 | Node-Man | `[UNLOCKED]` | HUD mock only: `exploit/mesh_nodeman.txt`. No `.rs` game. |
| 2 | Lag Lottery | `[UNLOCKED]` | **No file.** Name only. |
| 3 | Compliance Tetris | `[UNLOCKED]` | **No file.** Name only. |
| 4 | Soul Konami | `[UNLOCKED]` | Partial: `SoulFuzzerGame::konami_paradise` |
| 5 | Asteroids | `[LOCKED - Forge entropy_starver]` | Full-ish: `EntropyStarverGame::play_asteroids`. Contradicts lock text — `entropy_starver.rs` already exists. |

### 6.2 Playable (or semi-playable) games in `.rs`

#### Node-Man

- **Files:** `exploit/mesh_nodeman.txt` (and GameHub line).
- **Description:** Pac-Man-style maze; “Meshtastic LoRa AuraFS Global Hacker Tournament”.
- **Sample HUD:** Target `prod-mesh-aurphyx`, Score `1240`, Partitions `3`, Nodes Isolated `15`.
- **Controls in txt:** SPACE=Play, P=Partition, arrows, Q=Quit.
- **Win/fail:** not specified.
- **Link comment:** “Meshtastic P2P Node-Man code (LoRa AuraFS PVP)” — that code is not in this tree.

#### Lag Lottery

- **Files:** GameHub only.
- **Description / scoring / win:** unknown. Likely intended pair to `LATENCY_INJECTOR` / whitehat `LATENCY_MONITOR`. Not written.

#### Compliance Tetris

- **Files:** GameHub only.
- **Likely pair:** `comp_chkr_output.txt` / `COMPLIANCE_CHECKER` stub. Not written.

#### Soul Konami / Konami Paradise / KONAMI CODE PARADISE

- **Files:** `fuzzers/soul_fuzzer.rs`, `fuzzers/soul_fuzzer_egg_output.txt`.
- **Trigger in `.rs`:** sequence `Up Up Down Down Left Right 'b' 'a'` (`↑↑↓↓←→BA`). 5s window + 10 key reads.
- **Trigger in `.txt`:** `↑↑↓↓←→←→BA` (extra ←→). **Contradiction.**
- **Unlock points:** `konami_score += 1000` per unlock.
- **Paradise menu names in `.rs`:** `1. Contra (Soul Shooter)`, `21. Gradius (BlissID Defender)`, `2. Castlevania (Soul Whip)`, `22. Metal Gear (Stealth Bypass)`, then literal `... 42 total games ...`.
- **Demo:** “KONAMI CONTRA DEMO”, +100 score per tick, 30 ticks, SPACE/Q. “Meshtastic PVP Leaderboard Ready!”
- **`.txt` extras (not in `.rs`):** achievement `SOUL HACKER SUPREME`, `+10,000 FUX COINS`, endpoint `/quantum-souls/paradise`, trivia “What hashes BlissID?” answer `blake3`, title `SOUL HACKER LEGEND`.
- **Win:** Konami match → paradise. Fail: “Soulbinding SECURE (Try ↑↑↓↓←→BA!)”.
- **Test:** `test_konami_sequence`.

#### Asteroids / ENTROPY ASTEROIDS / ASTEROIDS PVP

- **Files:** `quantum_breaker/entropy_starver.rs` (`EntropyStarverGame`).
- **Unlock:** `entropy_depleted > 100_000` then `AchievementTracker::unlock_game("Asteroids")`; play if enterprise.
- **Play:** 60 ticks (~1 min), WASD+Space, 3 lives. Shoot adds +10. Board is a printed placeholder line, not a pixel grid.
- **Win/fail:** time-up prints final score. No life-loss logic despite `lives`.
- **Meshtastic:** “Meshtastic PVP Ready” in copy only.

#### Dilithium DigDug / DILITHIUM DIGDUG

- **Files:** `quantum_breaker/dilithium_forge.rs` (`DilithiumForgeGame`).
- **Unlock:** `signature_forgery > 128`; `unlock_game("Dilithium DigDug")`.
- **Play:** 180 ticks (~3 min). WASD. Puffy enemies = reject samples; dirt = lattice. Score +100 × chain on pop; +50 on dig.
- **End:** prints score and `Dirt: {}/256`. No explicit lose.

#### Minesweeper / SHARD MINESWEEPER

- **Files:** `fuzzers/shard_fuzzer.rs` (`ShardFuzzerGame`).
- **Unlock:** `unique_crashes > 42`; `unlock_game("Minesweeper")`.
- **Play:** 9×9, 10 mines. WASD+Space advertised; F to flag. `handle_input` only adds +10 on space/f — **no reveal/flag/mine logic**. `calculate_numbers` is empty.
- **End:** `game_over` never set in input handler. Loop is effectively stuck unless something else sets it.

#### Battleship / NAMESPACE BATTLESHIP

- **Files:** `fuzzers/namespace_fuzzer.rs` (`NamespaceFuzzerGame`).
- **Unlock:** `uuid_collisions > 128`; `unlock_game("Battleship")`.
- **Play:** 10×10, 5 ships each side. WASD + Space. Hit +100, miss +10. Win: `enemy_ships == 0`.
- **Gap:** `new()` never places ships (`player_board` / `enemy_board` all `Empty`), so Space always misses. Win condition cannot fire as written.
- **Test:** `test_uuid_collision` (expects generated UUID ≠ base).

#### Mesh Breakout / BREAKOUT PVP / MESH BREAKOUT

- **Files:** `exploit/mesh_partition.rs` (`MeshPartitionerGame`).
- **Unlock:** `bricks_partitioned > 100`; `unlock_game("Mesh Breakout")`.
- **Play:** 10×20 bricks (200). Ball bounce; brick hit +100; paddle bounce. Win: `bricks_remaining == 0`.
- **Controls:** Left/Right advertised; `handle_input` sets paddle with `max/min` but **does not add/subtract** on Left/Right, so paddle does not move. Space “Powerup” is empty.
- **Test:** `test_partition_counting`.

#### Quantum Casino / AURPHYX QUANTUM CASINO

- **Files:** `quantum_breaker/kyber_cracker.rs`.
- **Trigger:** RNG `next_u32() % 100 == 42` inside `KyberCracker::run`.
- **Play:** 5 rounds, start 1000 FUX, max bet 250 (printed; not enforced except bankroll). Even entropy → win 2×; odd → lose bet. `luck_triggers = [28, 36, 44, 55, 69, 77, 84, 99]` steal half bet.
- **Win flavor:** bankroll > 2000 → “CASINO WHALE!” + secret `afs redteam quantum entropy-starve whale`.
- **Currency:** FUX.

#### Willy Wonka Quantum Factory

- **Files:** `kyber_cracker.rs`.
- **Trigger:** target contains `wonka`.
- **Play:** lists four candy strings; no score. Secret: `afs redteam quantum kyber-crack wonka-factory`.
- **Not a win/lose game.**

#### Ready Player Aurphyx / QUANTUM EGG HUNT

- **Files:** `kyber_cracker.rs`.
- **Trigger:** target contains `r1o`.
- **Questions (exact):**
  1. `Kyber key size?` → `1024`
  2. `Dilithium level?` → `5`
  3. `Aurphyx motto?` → `its recursive`
  4. `Quantum casino code?` → `42`
- **Scoring:** +1000 FUX COINS per correct (substring, case-insensitive).
- **Tiers:** 4000 `QUANTUM HALL OF FAME! AURPHYX QUANTUM BISH!`; 3000–3999 `Quantum Apprentice`; else `Padawan needs practice...`

#### Soul Jack easter egg (not a game loop)

- **Files:** `exploit/souljack_easteregg.txt` (stub `soul_jack.rs` does not play it).
- **Content:** fake bugcheck `SOUL_JACK EXCEPTION`, “YOUR SOUL HAS BEEN JACKED”, “SKULL SCREEN BURN”.
- **Win/fail:** none.

### 6.3 Named exercises / scenarios / missions (not playful games)

Treat these as the rest of the “game list” — named drills.

#### NodeKiller scenarios (implemented set)

| Name | kill_percentage | pattern | recovery_window | expected_quorum |
|---|---|---|---|---|
| `leader_election_test` | 0.33 | `LeaderFirst` | 30s | 7 |
| `majority_quorum_break` | 0.67 | `Random` | 60s | 4 |
| `zone_failure_az1` | 0.40 | `ZoneKill("us-east-1a")` | 45s | 6 |
| `replica_heavy_kill` | 0.75 | `ReplicaHeavy` | 90s | 3 |

Header claims 12; eight are missing. Fail: `mesh_quorum_maintained == false` → Critical vuln. Print still says “{}/12 scenarios”.

#### SoulFuzzer phases

1. BlissID Soul Collision Fuzzing (5000 iters)
2. Soul Auth Bypass Chains (2000 iters)
3. Konami Code Detection

Reports: `soul_collisions`, `blissid_crashes`, `auth_bypasses`, `konami_unlocks`, `konami_score`, `soul_entropy`. Success if `blissid_crashes > 0`.

#### ShardFuzzer phases

1. AFL++ Coverage-Guided Fuzzing (5000)
2. Shard Mutation Chains (1000)
3. Crash repro — **function `crash_repro_phase` is called and not defined** (will not compile).

Bait file claims 50000 iters / 16 workers / 47 crashes — not this code.

#### NamespaceFuzzer phases

1. UUID Collision Fuzzing (10_000)
2. Namespace Traversal Chains (2000)
3. ACL Bypass via Namespace Collision (1000)

Bait file claims 25000 iters / 16 workers / 23 collisions — not this code.

#### DilithiumForge phases

1. Lattice Digging (LLL Reduction) (1000)
2. Reject Sampling Bypass (500)
3. Fiat-Shamir Signature Forgery (200)

Local RNG theater. Not a lattice solver.

#### EntropyStarver phases

1. NIST 800-90B QRNG Fuzzing (1000)
2. Side-channel Timing Attacks (500)
3. Entropy Depletion Apocalypse (10_000 × 42.0 added to `entropy_depleted`)

`side_channel_phase` assigns `self.side_channel_detected` on `&self` — will not compile.  
Output txt HUD (cycles, 60.123s, 98% drain) does not match this loop.

#### MeshPartitioner phases

1. BRICK SMASHING (200)
2. Traffic Bounce Isolation (300)
3. Gossip Poison + Mesh Split

#### CLI Audit

Intended: `PentestSuite` vs `standards: Vec<String>` (help text NIST 800-53, CIS, FIPS 140-3). Stub only.

#### Tournament / PVP stubs (names only)

`TOURNAMENT_MODE`, `PVP_LEADERBOARD`, `GLOBAL_RANKING`, `C2_BRIDGE`, `REPLAY_SYSTEM`, `COMBO_CHAINS` — each `attack()` += 42. No rules.

### 6.4 Konami “42 games”

Only four titles are written. The remaining 38 are the ellipsis in `soul_fuzzer.rs`. Do not invent them.

---

## 7. Cross-links — whitehat ↔ redteam

### 7.1 Folder contract

Same six domains. Redteam adds `fuzzers/` (no whitehat twin) and `cli.rs` (no whitehat twin). Whitehat `shield()` flags map 1:1 to redteam `TestResults` counters except whitehat has no fuzz/CLI flag.

| Whitehat flag / score | Redteam counterpart |
|---|---|
| `chaos_mitigations` | `chaos_tests` / `NodeKiller` |
| `exploit_protections` | `exploit_tests` / `MeshPartitioner` + CVE stubs |
| `network_defenses` | `network_tests` / `net/*` stubs |
| `governance_auditing` | `governance_tests` / `gov/*` stubs |
| `quantum_hardening` | `quantum_tests` / kyber, dilithium, entropy |
| `overall_score` / `compliance_score` | `DiamondReport.cvss_score`, NodeKiller resilience %, compliance txt 95.8% |

**Expectation (inferred, not coded):** redteam `run` should produce `AttackReport`; whitehat `defend` / `run_hardening_checks` should consume or counter it. **No shared call graph exists.** Neither side imports the other.

### 7.2 Filename pairs (name-level only)

| Redteam | Whitehat |
|---|---|
| `eternalblue.rs` | `eternalblue_defense.rs` |
| `heartbleed.rs` | `heartbleed_patch.rs` |
| `spring4shell.rs` | `spring_boot_hardener.rs` |
| `bluekeep.rs` | `rdp_hardener.rs` |
| `printnightmare.rs` | `printer_sec.rs` |
| `proxyshell.rs` | `exchange_patcher.rs` |
| `sudoedit.rs` / `polkitpkexec.rs` | `sudo_auditor.rs` |
| `zerologon.rs` | `domain_controller.rs` |
| `soul_jack.rs` | `soul_guard.rs` |
| `shard_poison.rs` | `shard_protector.rs` |
| `exploit_tester.rs` | `exploit_mitigator.rs` / `vuln_scanner.rs` |
| `node_killer.rs` / `nodekiller.rs` / `shard_storm.rs` | `node_health.rs` / `chaos_remediator.rs` / `recovery_engine.rs` / `failover_simulator.rs` |
| `latency_injector.rs` | `latency_monitor.rs` |
| `cert_expiry.rs` | `cert_renewal.rs` |
| `config_drift.rs` | `config_validator.rs` |
| `resource_crusher.rs` | `resource_balancer.rs` |
| `backup_corrupt.rs` | `shard_sync.rs` / `recovery_engine.rs` |
| `arp_poison.rs` | `arp_guard.rs` |
| `icmp_death.rs` | `icmp_filter.rs` |
| `tcp_syn_flood.rs` | `syn_cookie.rs` |
| `dns_amplification.rs` | `dns_rate_limiter.rs` |
| `quic_storm.rs` | `quic_validator.rs` |
| `ddos_orchestrator.rs` / `slowloris.rs` / `traffic_generator.rs` | `flood_protector.rs` / `connection_limiter.rs` |
| `ssl_stripper.rs` | `hsts_enforcer.rs` |
| `vote_manipulator.rs` | `vote_integrity.rs` |
| `quorum_domino.rs` | `quorum_monitor.rs` |
| `consensus_51.rs` | `consensus_monitor.rs` |
| `delegation_launder.rs` | `delegation_auditor.rs` |
| `timelock_crack.rs` | `timelock_auditor.rs` |
| `flashloan_attacker.rs` | `flashloan_protector.rs` |
| `zk_forge.rs` | `zk_verifier.rs` |
| `proposal_fuzzer.rs` | `proposal_analyzer.rs` |
| `soul_voting.rs` / `blissid_clone.rs` | `soul_verifier.rs` |
| `log_tamper.rs` / `log_falsifier.rs` | `log_integrity.rs` / `log_validator.rs` |
| `forensic_eraser.rs` / `forensic_clean.rs` | `forensic_preserver.rs` |
| `stealth_beacon.rs` | `beacon_detector.rs` |
| `siem_blind.rs` / `siem_fuzzer.rs` | `siem_correlator.rs` |
| `detection_evasion.rs` | `detection_engine.rs` |
| `compliance_forge.rs` / `compliance_checker.rs` | `compliance_monitor.rs` / `compliance_scanner.rs` / `cis_benchmarker.rs` |
| `dilithium_forge.rs` / `dilithium_crack.rs` | `dilithium_verifier.rs` |
| `falcon_forge.rs` | `falcon_validator.rs` |
| `harvest_attack.rs` | `harvest_protector.rs` |
| `lattice_sieve.rs` | `lattice_hardener.rs` |
| `oracle_compromiser.rs` / `oracle_manipulator.rs` | `oracle_monitor.rs` |
| `qkd_man_in_middle.rs` | `qkd_monitor.rs` |
| `entropy_starver.rs` | `quantum_rng.rs` |
| `entanglement_poison.rs` | `entanglement_detector.rs` |
| `pqc_breaker.rs` / `kyber_cracker.rs` | `pqc_tester.rs` / `pqc_migrator.rs` / `quantum_resistant.rs` |
| `ec_dlp_breaker.rs` | `ecdsa_monitor.rs` |

Unpaired redteam (no whitehat file): all `fuzzers/*`, `cli.rs`, most net extras (`meshtastic`, `packet_generator`, `protocol_*`, `reflection_scanners`, `bandwidth_tester`, `flood_metrics`), gov `dao_scanner` / `bridge_exploits` / `governance_exploits`, chaos PVP/tournament/c2/replay/combo/cascade/metrics, exploit chain/post/lateral/meterpreter/poc/trophy/cleanup, quantum grover/shor/qday/qubit/circuit/fault/side_channel/post_quantum.

Unpaired whitehat: `ptes_automator`, `edr_tester`, `waf_rules`, `ids_integration`, `traffic_classifier`, `anomaly_detector`, `behavior_analyzer`, `rootkit_detector`, `patch_manager`, `vuln_manager`, `alert_system`, `reliability_orchestrator`, `resilience_tester`.

### 7.3 Shared types (expected vs actual)

Used from `crate::redteam::{TestVector, AttackReport, Vulnerability, Severity, AchievementTracker}` and `crate::redteam::audit_simulator::{...}` and `crate::redteam::{chaos,fuzzers,quantum_breaker}::*Game`.

Defined in-tree:

- `AchievementTracker` **fields** in `cli.rs` only.
- `TestResults` in `mod.rs`.
- Per-module reports: `SoulFuzzReport`, `ShardFuzzReport`, `NamespaceReport`, `DilithiumReport`, `EntropyReport`, `PartitionReport`, `ChaosResult`, `KillScenario`, `DiamondReport`.

**Missing:** `TestVector`, `AttackReport`, `Vulnerability`, `Severity`, `unlock_game` / `progress` / `legend_status`, all `*Game` re-exports at module roots, `PentestSuite`.

Magic number **42** is the suite’s joke constant (increments, XOR, entropy add, casino trigger). It is not an AuraFS physics invariant. Do not map it to `physics::INVARIANTS`.

### 7.4 Shared flows (intended)

1. Operator runs `afs-redteam` with `--target` / `--enterprise` / `--games`.
2. Audit/chaos/exploit/fuzz/quantum produce `AttackReport` + optional game unlocks.
3. `GameHub` shows unlocks; Meshtastic PVP / leaderboards (`pvp_leaderboard`, `global_ranking`, `tournament_mode`) persist scores.
4. Whitehat `shield()` / `defend()` raise the matching control.

**Coded today:** step 1 partially (broken CLI). Step 2 only inside standalone `run()` methods. Steps 3–4 are prints and stubs.

---

## 8. Maintainer notes

### 8.1 Wiring checklist (facts, not a request to do it here)

1. `lib.rs` does not declare these modules.
2. No subdirectory `mod.rs` / `#[path]` modules.
3. `security-tools` is documented and absent from current `Cargo.toml`.
4. `crossterm` missing; `colored` gold/diamond helpers missing.
5. `FIX_UPDATE_BUILD.md` is cited in `AURAFS_AUDIT_AND_DEPLOY.md` for both trees and **does not exist**.

### 8.2 Safety

- Redteam `mod.rs`: authorized testing only.
- `NodeKiller` contains live process-termination command invocations. Treat as dangerous if ever compiled and pointed at a real mesh. Do not run it against production.
- CVE-named files are **nameplates**. They do not contain working exploit implementations in this tree. Do not fill them in as a side effect of docs work.
- Game modules are terminal toys + RNG theater. They do not break Dilithium/Kyber/QRNG.

### 8.3 AuraFS invariants vs this suite

`aurafs.toml` / `cursorrules` physics (η 5.3, T₂ 1600 μs, d_s 1.37, PBG 0.21, lock 100 μs) are **not referenced** in whitehat or redteam. Dilithium-5 / Kyber-1024 strings in games loosely echo `[crypto]` but do not use `physics::INVARIANTS` or `src/crypto/pqc/`.

`min_quorum = 13` in `aurafs.toml` vs NodeKiller `expected_quorum` 7/4/6/3 — different layers; do not silently unify.

TRL-4 locked files in `aurafs.toml [modules.validated]` do **not** include this suite.

### 8.4 Naming

- Paths still say `afs/src/...` (old layout). Current lab path is `aurafs/src/...`.
- `mesh_partition.rs` documents itself as `chaos/mesh_partitioner.rs` but lives under `exploit/`.
- Duplicate: `node_killer.rs` (real) vs `nodekiller.rs` (stub).
- `sample__.rs` on both sides; redteam file is duplicated in-body.
- Retired ecosys physics names appear as **product strings** here (`BlissID`, soulbinding). AuraFS product tree still uses them in this suite. Do not “fix” to Equilibrium Manifold in these files unless Ross asks.
- CLI vs bait: `afs-redteam` vs `afs redteam ...`.

### 8.5 Tests present

| File | Test |
|---|---|
| `node_killer.rs` | `test_node_selection_patterns` |
| `soul_fuzzer.rs` | `test_konami_sequence` |
| `shard_fuzzer.rs` | `test_shard_mutation` (64-byte mutation) |
| `namespace_fuzzer.rs` | `test_uuid_collision` |
| `dilithium_forge.rs` | `test_dilithium_lattice_digging` (depth == 256) |
| `entropy_starver.rs` | `test_nist_entropy_validation` |
| `mesh_partition.rs` | `test_partition_counting` |

Whitehat: no `#[cfg(test)]`.

### 8.6 TODOs in-tree

No `TODO` / `FIXME` strings in either folder. Incomplete work is expressed as empty CLI arms, `// Placeholder`, `// ... 8 more scenarios`, empty game helpers, and stubs.

### 8.7 Contradictions (keep these visible)

1. `security-tools` documented; not in current `Cargo.toml`.
2. Modules exist; `lib.rs` ignores them.
3. CLI “17 Diamond tools” — no list; ~8 non-stub modules.
4. GameHub Asteroids locked; `entropy_starver.rs` exists.
5. GameHub 1–4 unlocked; no achievement state machine.
6. NodeKiller “12 scenarios” vs 4 in the vector.
7. Konami sequence `.rs` vs `.txt`.
8. Bait/output `.txt` metrics ≠ `.rs` loops.
9. `PentestSuite` vs `PENTESTSUITE`.
10. `LatencyInjector` import vs `LATENCY_INJECTOR` stub.
11. `AchievementTracker` used as a namespace of functions; defined as a serde struct.
12. `sample__.rs` redteam duplicated.
13. `shard_fuzzer.rs` calls missing `crash_repro_phase`.
14. `entropy_starver.rs` mutates `self` in `&self` methods.
15. `SUMZ-SUGGZ.md`: do not expand this surface for Vibe. This brief documents; it does not grow the suite.

### 8.8 Status legend for maintainers

| Class | Count (approx.) | Meaning |
|---|---|---|
| Orchestration | 2 (`*/mod.rs`) | Placeholders that would be the API |
| Operator CLI | 1 (`cli.rs`) | Incomplete; would not compile |
| Full theater modules | 8 | RNG/sim + optional terminal game |
| Sample skeletons | 2 | v7.0 templates |
| Tree snapshots | 2 `.txt` | 2025-12-28 listings |
| Bait / egg text | 6 `.txt` | Mock HUDs |
| Whitehat defense stubs | 74 | `defend() += 42` |
| Redteam diamond stubs | ~104 | `attack() += 42` |

---

## 9. What a maintainer should not do from this file

- Do not treat filenames as implemented controls.
- Do not promote bait `.txt` numbers as test results.
- Do not implement the CVE-named stubs “to finish the suite” without a scoped, authorized hardening task.
- Do not stamp APS-OKF or ecosys physics headers onto this product tree.
- Do not confuse Pink Tribe arcade scoring with AuraFS TRL-4 validation.

---

*End of Pink Tribe Suite brief. Source of truth is the files listed in §3. When they change, update this map in the same pass.*
