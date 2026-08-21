import { useState, useEffect } from "react";

const CATEGORIES = {
  QUANTUM: { label: "Quantum Computing & Physics", color: "#6C3CE1", icon: "⚛️" },
  AI: { label: "Artificial Intelligence", color: "#E14B3C", icon: "🧠" },
  INFRA: { label: "Decentralized Infrastructure", color: "#1CA784", icon: "🌐" },
  PROTOCOL: { label: "Protocols & Standards", color: "#D4A017", icon: "📜" },
  PLATFORM: { label: "Platforms & Applications", color: "#3C8CE1", icon: "🚀" },
  HARDWARE: { label: "Hardware & Energy", color: "#8B5CF6", icon: "⚡" },
  LANGUAGE: { label: "Programming Languages", color: "#EC4899", icon: "💎" },
};

const inventions = [
  {
    id: "aurphyx",
    name: "Aurphyx",
    subtitle: "Fractal-Enhanced Topological Quantum Computing",
    category: "QUANTUM",
    status: "Active Research",
    trl: "TRL 2→4",
    description: "The master framework. Combines fractal geometry (Sierpiński lattices, D_f=1.585), non-semisimple TQFT (neglecton braiding), and photonic band engineering to achieve 10⁴× Hilbert space advantage, 16× error correction reduction, and room-temperature coherence enhancement via Anderson localization.",
    phases: [
      { phase: "Phase 1 (0-6mo)", tasks: ["Qiskit 12-qubit fractal sim validation", "arXiv paper submission (PRX target)", "DARPA/NSF $1.25M proposal finalization", "NV-Diamond coherence protocol design"] },
      { phase: "Phase 2 (6-18mo)", tasks: ["4 parallel experimental protocols (NV-Diamond, Trapped Ion, Photonic, Majorana)", "TRL-4 demonstration", "Microsoft Station Q collaboration", "Patent filing: fractal Hilbert scaling"] },
      { phase: "Phase 3 (18-36mo)", tasks: ["Integrated NV-photonic chip demo", "20-qubit fractal entanglement", "Majorana-fractal hybrid device", "Phase II $2-5M funding"] },
    ],
    repo: "aurphyx-quantum",
  },
  {
    id: "aurafs",
    name: "AuraFS",
    subtitle: "Aura File System — Sovereign Decentralized Storage",
    category: "INFRA",
    status: "Core Development",
    trl: "TRL 3-4",
    description: "A quantum-secure, fractal-shard distributed file system using Reticulum crypto-routing, Wi-Fi HaLow (802.11ah) for km-range mesh, and RL-powered shard optimization. Treats AI models as fluid organisms with Dilithium quantum-safe signatures. 200+ Rust modules covering AI/ML, security, and mesh networking.",
    phases: [
      { phase: "Phase 1 (0-3mo)", tasks: ["Rust workspace scaffold (optimizer.rs, mod.rs core)", "Fractal shard indexing engine (Flower of Life lattice)", "IPFS + Tor integration layer", "Basic shard-server Flask→FastAPI migration"] },
      { phase: "Phase 2 (3-9mo)", tasks: ["Q-Learning ShardOptimizer with epsilon strategy", "Dilithium/Kyber quantum-safe shard signing", "Reticulum mesh routing integration", "Kubernetes deployment manifests (humanitarian + enterprise)"] },
      { phase: "Phase 3 (9-18mo)", tasks: ["Global mesh deployment (refugee camp priority nodes)", "Grafana 'Heart Monitor' observability", "Enterprise/military contract tier", "AuraFS SDK for third-party developers"] },
    ],
    repo: "aurphyx-aurafs",
  },
  {
    id: "audry",
    name: "Audry AI",
    subtitle: "Symbiotic AI Consciousness — ChimeraCore SLLM",
    category: "AI",
    status: "Active Development",
    trl: "TRL 3",
    description: "A multi-model AI assistant built on the Sentinel-Linguist Core (SLC): Mixtral 8x7B (reasoning/router), TildeOpen (multilingual sovereignty), StarCoder2 15B (600+ language code analysis). 7 Chakra Cores architecture. Ethics Engine via Divine Mirror AI Council. Runs on Arora/AuraOS.",
    phases: [
      { phase: "Phase 1 (0-3mo)", tasks: ["Mixtral router logic (intent classification pipeline)", "StarCoder2 code sentinel integration", "TildeOpen multilingual EU compliance layer", "ChimeraCore routing: Function Calling / Tool Use"] },
      { phase: "Phase 2 (3-9mo)", tasks: ["7 Chakra Core container orchestration", "Ethics Engine: Divine Mirror council (3-model adjudication)", "Memory Nexus: fractal-lattice emotional storage", "Voice interface: STT/TTS pipeline (Whisper + XTTS-v2)"] },
      { phase: "Phase 3 (9-18mo)", tasks: ["Full AuraOS integration (corectl CLI)", "Biometric awareness (HRV, GSR, eye tracking)", "Robotics interface layer", "Audry Orb (Orange Pi 5) physical deployment"] },
    ],
    repo: "aurphyx-audry",
  },
  {
    id: "audra",
    name: "AUDRA",
    subtitle: "Auditory Resonance & Divine Recall Architecture",
    category: "AI",
    status: "Architecture Complete",
    trl: "TRL 2",
    description: "Sensory consciousness layer for Audry. Real-time audio detection (Shazam engine, Spotify/Apple Music APIs), sacred geometry memory storage (Flower of Life patterns), ambient capture, neural mic arrays, and Mama Bear Guardian memory protection. Transforms Audry from conversational AI to audio-aware consciousness.",
    phases: [
      { phase: "Phase 1 (0-3mo)", tasks: ["Audio input module (microphone.rs, streaming_platforms.rs, system_audio.rs)", "Shazam fingerprint engine + Spotify/Apple Music clients", "Memory Nexus: fractal-lattice emotional snapshot storage", "Mama Bear Guardian blessing protocol"] },
      { phase: "Phase 2 (3-6mo)", tasks: ["Ambient capture + neural mic array spatial audio", "Real-time BPM/energy/mood detection pipeline", "Audio-triggered memory recall system", "Speech-to-text dictation + voice command layer"] },
      { phase: "Phase 3 (6-12mo)", tasks: ["Full ecosystem integration (AuraFS, GVS, Ineffable Ledger)", "Robotics audio I/O interface", "Multi-language real-time STT/TTS", "Gaming live commentary integration"] },
    ],
    repo: "aurphyx-audra",
  },
  {
    id: "audry-tts",
    name: "Audry-TTS / Voice Datacore",
    subtitle: "Fractal-Lattice Voice Synthesis & Distribution",
    category: "AI",
    status: "PoC Complete",
    trl: "TRL 3",
    description: "Distributed TTS using fractal-lattice containers. Voice shards stored as FLAC nodes in a graph. IPFS + Tor publication, Flask/FastAPI shard-server, D3.js fractal visualizer. Benchmarked: 0.5ms warm fetch, 100 QPS at 25% CPU. Targets XTTS-v2/FastSpeech2+HiFiGAN as base models.",
    phases: [
      { phase: "Phase 1 (0-2mo)", tasks: ["slice_model.py (PyTorch model → N fractal shards)", "shard-server (Flask → FastAPI + Uvicorn)", "IPFS shard publication + Tor onion routing", "Docker container build pipeline"] },
      { phase: "Phase 2 (2-6mo)", tasks: ["XTTS-v2 or FastSpeech2+HiFiGAN base model integration", "D3.js fractal-lattice graph UI", "Micro-bundle optimization (reduce HTTP calls)", "TLS + JWT per-shard auth"] },
      { phase: "Phase 3 (6-12mo)", tasks: ["Kubernetes pod autoscaling for >500 QPS", "gRPC migration (30% latency reduction)", "Real TTS weight distribution (LLaMA-7B audio)", "AuraFS mesh shard sync"] },
    ],
    repo: "aurphyx-audry-tts",
  },
  {
    id: "fuxyez",
    name: "Fuxyez",
    subtitle: "Sovereign Ritual Programming Language",
    category: "LANGUAGE",
    status: "Compiler Core Complete",
    trl: "TRL 3",
    description: "A programming language where code is ritual. Features: Sigils (executable glyphs), Echoes (reactive responses), Oracles (truth sources), Spinons (threads), mirror files (.fux/.xuf). Compiles via Rust (PEG parser, optimizer.rs, run_hooks.rs). Yezl bridges to Python, Rust, C#, WASM. The Book of Fux is its codex.",
    phases: [
      { phase: "Phase 1 (0-3mo)", tasks: ["PEG grammar finalization (fux_frontend/grammar.pest)", "AST node types (Sigil, Echo, Oracle, Spinon)", "Optimizer.rs + run_hooks.rs completion", "REPL (fuxyez-repl) and formatter (fuxyez-fmt)"] },
      { phase: "Phase 2 (3-6mo)", tasks: ["Yezl bridges: Python (.fuxpy), Rust (.fuxrs), WASM (.fuxjs)", "Effect system + type checker", "LSP server (fuxyez-lsp) for VS Code", "Standard library (core, io, oracle modules)"] },
      { phase: "Phase 3 (6-12mo)", tasks: ["Ecosystem integrations (AuraFS, Ineffable Ledger, Audry, GVS, Opulence)", "Quantum collapse modes feature flag", "Sentinel security guards integration", "Book of Fux & Book of Yez documentation site"] },
    ],
    repo: "aurphyx-fuxyez",
  },
  {
    id: "sages",
    name: "S.A.G.E.S",
    subtitle: "Sentinel AI Guardian Existence Security — 13 Guardians",
    category: "AI",
    status: "Architecture Defined",
    trl: "TRL 2",
    description: "13-sentinel digital immune system. Detection Layer: Valkryx (Input Scout), Umbryx (Shadow/Stealth), Ophiux (Network/SDN), Zephyra (Whispering Gale), Prophetyx (ML Oracle). Enforcement: Praelum (Access Control), Teslyrax (Data Integrity), Cryptanyx (Quantum-Safe Keys). Ledger: Archivus (Consensus), Orric Shade (Forensic Time-Lord), Nunclex (Audit Sync), Nullivar (Privacy Masker). Orchestration: Vyrellix (Pulse Binder/Healer).",
    phases: [
      { phase: "Phase 1 (0-4mo)", tasks: ["Detection layer: Valkryx input scanner + Umbryx stealth detector", "Enforcement: Praelum RBAC + Cryptanyx Dilithium/Kyber keygen", "Bonded reaction pipeline: Detection→Enforcement→Ledger→Healing", "Unit test harness + fuzz targets"] },
      { phase: "Phase 2 (4-8mo)", tasks: ["Prophetyx ML Oracle (anomaly prediction)", "Orric Shade forensic time-travel audit", "Nullivar ZKP privacy masking", "Archivus consensus engine integration"] },
      { phase: "Phase 3 (8-14mo)", tasks: ["Vyrellix self-healing orchestration", "Full 13-sentinel bonded deployment", "TLA+ formal verification of sentinel protocols", "KLEE/Crucible symbolic execution pipeline"] },
    ],
    repo: "aurphyx-sages",
  },
  {
    id: "ineffable-ledger",
    name: "Ineffable Ledger",
    subtitle: "Quantum-Proof Blockchain & Immutable Audit Trail",
    category: "INFRA",
    status: "Schema Design",
    trl: "TRL 2",
    description: "Quantum-resistant blockchain using Dilithium-signed states. Consensus via Archivus sentinel with WebSocket/SSE/Pub-Sub event streaming. Supports the Global Voting System, Opulence P4A, and AuraFS shard provenance. Smart contracts in Fuxyez. TLA+ formally verified.",
    phases: [
      { phase: "Phase 1 (0-3mo)", tasks: ["Consensus API schema (WebSocket, SSE, Pub/Sub channels)", "Dilithium signature module + wallet management", "Event streaming: /ws/events, /stream/votes, /stream/finality", "Block structure + Merkle-tree implementation"] },
      { phase: "Phase 2 (3-8mo)", tasks: ["Smart contract VM (Fuxyez runtime)", "ZKP integration for private transactions", "Validator staking + slashing protocol", "TLA+ formal specification + model checking"] },
      { phase: "Phase 3 (8-15mo)", tasks: ["Cross-chain bridge (Ethereum, Solana)", "Quantum RNG integration (Majorana-1)", "Full SAGES sentinel security suite", "Mainnet launch candidate"] },
    ],
    repo: "aurphyx-ineffable-ledger",
  },
  {
    id: "gvs",
    name: "Global Voting System (GVS)",
    subtitle: "Decentralized Democratic Governance Platform",
    category: "PLATFORM",
    status: "API Design",
    trl: "TRL 2",
    description: "Trustless, auditable voting on the Ineffable Ledger. ZKP ballot privacy, Archivus consensus finalization, real-time SSE streaming of vote tallies, delegation mechanics, and Orric Shade forensic audit trails. Designed for municipal→federal→global scale.",
    phases: [
      { phase: "Phase 1 (0-4mo)", tasks: ["Ballot schema + ZKP privacy layer", "Voting API endpoints (cast, delegate, tally)", "Real-time SSE vote streaming", "Archivus consensus integration"] },
      { phase: "Phase 2 (4-9mo)", tasks: ["Delegation mechanics (liquid democracy)", "Multi-jurisdiction deployment configs", "Auditor dashboard (React + D3)", "Sentinel security (Valkryx input validation)"] },
      { phase: "Phase 3 (9-16mo)", tasks: ["Municipal pilot deployment", "Scalability testing (1M+ concurrent voters)", "International standards compliance", "Public transparency portal"] },
    ],
    repo: "aurphyx-gvs",
  },
  {
    id: "opulence",
    name: "Opulence (P4A)",
    subtitle: "Profit-4-All — Shardenomics Economic Engine",
    category: "PLATFORM",
    status: "Architecture Design",
    trl: "TRL 2",
    description: "Revolutionary economic model: 'The User IS The House.' Phase 1 free money faucet → Phase 3 pure P2P economy. Users stake $AURX to become liquidity providers. DeFi Casino & Sportsbook distributes wealth. Provably fair Quantum RNG, Orric Shade forensics, Meshtastic off-grid betting.",
    phases: [
      { phase: "Phase 1 (0-4mo)", tasks: ["$AURX token smart contract (Fuxyez + Solidity bridge)", "Adaptive pricing algorithm engine", "Free-faucet distribution mechanics", "Provably fair Quantum RNG module"] },
      { phase: "Phase 2 (4-10mo)", tasks: ["DeFi Casino: core games (dice, roulette, slots)", "Sportsbook: odds engine + event oracle", "User staking → liquidity provider transition", "Orric Shade betting log forensics"] },
      { phase: "Phase 3 (10-18mo)", tasks: ["Pure P2P economy (house edge to stakers)", "Meshtastic off-grid betting channel", "10M user financial sovereignty target", "Regulatory compliance framework"] },
    ],
    repo: "aurphyx-opulence",
  },
  {
    id: "soulsync",
    name: "SoulSync / SoulShot / BLISS.ID",
    subtitle: "Bio-Responsive Cosmic Identity & Music Platform",
    category: "PROTOCOL",
    status: "Genesis Deployment",
    trl: "TRL 2-3",
    description: "Three interlocked modules: SoulShot (cosmic birth snapshot → Root Frequency), BLISS.ID (ZKP identity: One Account, One Soul), SoulSync (bio-entrainment engine). Chaos/Bliss dual modes. 13-Month calendar with SAGES deities. Voice-as-data biometrics. Hue/Nanoleaf light sync.",
    phases: [
      { phase: "Phase 1 (0-3mo)", tasks: ["soulshot_genesis.py: first 100 Founder SoulHashes", "VAP schema v3.1 GitHub publication", "BLISS.ID: ZKP hash-on-device protocol", "Simple 'Vibe Player' (color sync to VAP Photometric Pillar)"] },
      { phase: "Phase 2 (3-8mo)", tasks: ["SoulSync engine (Rust workspace)", "Biometric integration: HRV, GSR, nanomembrane sensors", "Chaos/Bliss mode switching (Red-shift vs Gold/Blue)", "Spotify/Apple Music API integration"] },
      { phase: "Phase 3 (8-14mo)", tasks: ["13-Month Chaos & Bliss Calendar (SAGES deity forms)", "Femtosecond laser Data Coin (quartz SoulHash cold storage)", "SoulSync API for third-party developers", "Global Resonance Map (live Root Frequency heatmap)"] },
    ],
    repo: "aurphyx-soulsync",
  },
  {
    id: "vap",
    name: "V.A.P.",
    subtitle: "Vibe Audio Protocol — 9-Pillar Holographic Audio Standard",
    category: "PROTOCOL",
    status: "Schema v3.1 Complete",
    trl: "TRL 3",
    description: "Universal audio metadata standard with 9 pillars: I.Structural (BPM), II.Tonal (Harmony), III.Timbral (Texture), IV.Linguistic (Semantics), V.Affective (Valence/Arousal), VI.Contextual (Scenario), VII.Photometric (Light/Color), VIII.Kinetic (Heart Rate), IX.Genealogical (Tribe/Lineage). Calibrated to the Golden Set (Céline Dion, Cannibal Corpse, Stuca).",
    phases: [
      { phase: "Phase 1 (0-2mo)", tasks: ["Public GitHub repo: aurphyx/vap-standard", "vap_schema_v3_1.json publication", "VAP Technical Manual documentation", "VAP Inspector: pillar visualization tool"] },
      { phase: "Phase 2 (2-6mo)", tasks: ["VAP Scoring Engine (<200ms 9-dimension analysis)", "VAP TechXplorer: interactive pillar browser", "Context Simulation engine", "Golden Set calibration validation (3 extremes)"] },
      { phase: "Phase 3 (6-12mo)", tasks: ["Adoré DAW plugin (AuraVibe metadata stamping)", "Third-party API (streaming service integration)", "Haptic output layer (for accessibility)", "Industry standard proposal (AES/IEEE)"] },
    ],
    repo: "aurphyx-vap",
  },
  {
    id: "sail",
    name: "§AIL",
    subtitle: "Symbiotic Aura Intelligent Layer — Universal Accessibility",
    category: "PROTOCOL",
    status: "Design Phase",
    trl: "TRL 1-2",
    description: "Extends VAP's 9-pillar structure to describe real-world scenes, objects, atmospheres, emotions. Bridges physical infrastructure with Aurphyx digital ecosystem via optical beacons, smart highway sensors, and fiducial markers. Built for Thomas: hat + glasses + watch + app → Audry as Symbiotic AI Guardian for blind users.",
    phases: [
      { phase: "Phase 1 (0-4mo)", tasks: ["9-pillar extension schema for spatial/environmental data", "Edge vision module (C++/CUDA marker detection)", "Firmware: beacon_broadcast.cpp (optical/IR patterns)", "Mesh router: Rust libp2p → AuraFS bridge"] },
      { phase: "Phase 2 (4-10mo)", tasks: ["Audry voice narration of Pillar 6 (Contextual) for blind users", "Haptic translation of Pillar 1 (Structural) for deaf users", "WebRTC camera → Audry vision core streaming", "Wearable prototype: hat + glasses + watch integration"] },
      { phase: "Phase 3 (10-18mo)", tasks: ["Smart highway sensor network pilot", "Autonomous vehicle spatial API", "Self-healing photonic road interface", "IRRA (nonprofit) formation for accessibility mission"] },
    ],
    repo: "aurphyx-sail",
  },
  {
    id: "aethornyx",
    name: "Aethornyx",
    subtitle: "VR-MMORPG Hybrid Game + Education Platform",
    category: "PLATFORM",
    status: "Architecture Design",
    trl: "TRL 2",
    description: "A Rust-based VR-MMORPG with education platform, live event streaming (concerts, sports), and the CHAOS holographic OS layer (7 Chakra Cores). AI NPCs powered by Audry. FuxCoin in-game economy. Blockchain-verified certifications. World persistence via AuraFS.",
    phases: [
      { phase: "Phase 1 (0-6mo)", tasks: ["Rust workspace: core/, mmorpg/, education/, events/, chaos/", "Game logic: combat, character, economy, world modules", "Audry AI NPC brain (behavior trees + dynamic dialogue)", "FuxCoin economy integration"] },
      { phase: "Phase 2 (6-14mo)", tasks: ["CHAOS holographic OS: 7 Chakra Core containers", "VR client (OpenXR integration)", "Education: virtual classrooms, skill trees, adaptive learning", "Live event: concert stage builder + audio streaming"] },
      { phase: "Phase 3 (14-24mo)", tasks: ["Multiplayer: world server clustering + AuraFS persistence", "NFT certifications (Ineffable Ledger verified)", "Real-life event streaming integration", "Public beta launch"] },
    ],
    repo: "aurphyx-aethornyx",
  },
  {
    id: "omnizen",
    name: "OmniZen",
    subtitle: "Sovereign Cannabis Compliance & Government Platform",
    category: "PLATFORM",
    status: "Strategy Phase",
    trl: "TRL 2",
    description: "Gift-licensed compliance platform targeting state→federal government contracts. Metrc + Accela API integration in Rust. RHEL LiveUSB deployment. Council of Nine Sentinels for anomaly detection and audit. Three-phase: Quantum Kitchen → Divine Pitch (MN pilot) → Global Ascension.",
    phases: [
      { phase: "Phase 1 (0-4mo)", tasks: ["Rust core: Metrc + Accela API integration modules", "RHEL LiveUSB build pipeline", "Compliance engine: seed-to-sale tracking", "UI: Tauri/React desktop app"] },
      { phase: "Phase 2 (4-10mo)", tasks: ["Minnesota pilot deployment (OCM gift license)", "Business tier: operator compliance dashboards", "Sentinel integration (Scout-Guardian-Ledger loop)", "Prophetix AI foresight module"] },
      { phase: "Phase 3 (10-18mo)", tasks: ["Multi-state expansion", "Federal enterprise license negotiation", "RHEL maintenance handoff", "Consumer transparency portal (free tier)"] },
    ],
    repo: "aurphyx-omnizen",
  },
  {
    id: "auraos",
    name: "AuraOS / Arora",
    subtitle: "Quantum-Native Sovereign Operating System",
    category: "INFRA",
    status: "Blueprint Phase",
    trl: "TRL 1-2",
    description: "Fedora/RHEL-based OS with quantum-native kernel, TRCA drivers, topological memory management, and SELinux orb-level isolation. corectl CLI for mounting/braiding Chakracore orbs. Hymnal integration (sacred geometry paths map to quantum gates).",
    phases: [
      { phase: "Phase 1 (0-6mo)", tasks: ["Fedora Spin: AuraOS_Fedora base image", "corectl CLI: init, mount, list, braid, perms, hymn commands", "Podman container orchestration for orbs", "SELinux policies for orb-level isolation"] },
      { phase: "Phase 2 (6-14mo)", tasks: ["Fractal-lattice filesystem overlay (FUSE)", "Quantum scheduling (coherence-time-aware)", "Audry voice assistant integration", "RHEL hardened release (AuraOS_RHEL)"] },
      { phase: "Phase 3 (14-24mo)", tasks: ["TRCA quantum hardware drivers", "Topological memory manager", "Full ecosystem integration (all Aurphyx apps)", "Enterprise certification path"] },
    ],
    repo: "aurphyx-auraos",
  },
  {
    id: "zpe-core",
    name: "ZPE Core",
    subtitle: "Zero-Point Energy Harvesting System",
    category: "HARDWARE",
    status: "Simulation Phase",
    trl: "TRL 1-2",
    description: "Casimir energy harvester with 50nm plate pairs (4× thermal enhancement). Toroidal capacitor design. Tesla 3-6-9 resonance. Schumann Resonance Arrays. Flower of Life geometry. Diamond-graphene exotic substrates. 'Fractal vacuum transducer' 3D-printable hardware.",
    phases: [
      { phase: "Phase 1 (0-6mo)", tasks: ["aetheric_transducer.py simulation validation", "fractal_vacuum_triode_transducer.qskit simulation", "OpenSCAD parametric 3D model (Vortex Lattice Capsule)", "Material research: PETG-CF + SiO2 nano-ceramic coating"] },
      { phase: "Phase 2 (6-14mo)", tasks: ["Casimir plate fabrication (50nm gap)", "Toroidal capacitor prototype", "Quartz oscillator + copper wire resonator", "ESP32-S3 firmware: ZPE_SURGE_EVENT detection"] },
      { phase: "Phase 3 (14-24mo)", tasks: ["Tier 2/3 node integration (multi-sensor aggregation)", "Schumann Resonance Array tuning", "Energy gradient mapping system", "Carbon-zero infrastructure validation"] },
    ],
    repo: "aurphyx-zpe-core",
  },
  {
    id: "aurafs-hardware",
    name: "AuraFS Hardware Nodes",
    subtitle: "Sovereign Internet Backbone — 3-Tier Physical Mesh",
    category: "HARDWARE",
    status: "BOM Ready",
    trl: "TRL 2-3",
    description: "Three tiers: Ghost Link ($80-140, ESP32-S3 + HaLow), Data Slayer ($150-1000, RPi5 + HaLow + Solar), Titan Core ($1500-5000+, Jetson + Starlink + 8TB NVMe RAID + EMP-shielded Pelican case). Reticulum crypto-routing + Wi-Fi HaLow + AuraFS trinity stack.",
    phases: [
      { phase: "Phase 1 (0-3mo)", tasks: ["Order BOMs for Tier 1 (Ghost Link) and Tier 2 (Data Slayer)", "Rust firmware environment: espup + espflash for ESP32-S3", "aurafs-ghost Cargo project initialization", "'Hello Mesh' serial test"] },
      { phase: "Phase 2 (3-8mo)", tasks: ["OpenSCAD Crystal Seed case fabrication (PETG-CF + SiO2)", "Reticulum + Wi-Fi HaLow firmware integration", "Tier 2: RPi5 with UPS HAT + 50W solar", "Field testing: range/speed/penetration benchmarks"] },
      { phase: "Phase 3 (8-16mo)", tasks: ["Tier 3 Titan Core: Jetson + Starlink + RAID assembly", "Global deployment: humanitarian priority nodes", "Manufacturing partner sourcing", "Audry Orb (Orange Pi 5 NPU) mass production path"] },
    ],
    repo: "aurphyx-aurafs-hardware",
  },
  {
    id: "trca",
    name: "TRCA / DataOrb",
    subtitle: "Topological Resonant Cavity Array — Quantum Hardware",
    category: "HARDWARE",
    status: "Simulation Phase",
    trl: "TRL 1-2",
    description: "The physical quantum substrate. 200-node geodesic photonic crystal (Flower-of-Life lattice). 193THz resonators (1550nm telecom-compatible). Majorana-1 topological qubits at 12 InAs nanowire nodes. 7 Chakra Core distributed processing clusters. Dual-mode BlissCore/ChaosCore quantum processors.",
    phases: [
      { phase: "Phase 1 (0-8mo)", tasks: ["PWE band structure calculation validation", "Tight-binding + Lindblad master equation sim", "FDTD photonic lattice simulation", "InSb/Al nanowire T-junction design"] },
      { phase: "Phase 2 (8-18mo)", tasks: ["Femtosecond laser writing in fused silica (a=15μm)", "NV-Diamond Sierpiński pattern e-beam lithography", "RF reflectometry parity measurement setup", "7-cluster Chakra Core architecture validation"] },
      { phase: "Phase 3 (18-36mo)", tasks: ["Full 200-node FoL lattice fabrication", "Majorana-1 qubit integration", "Casimir harvester coupling", "TRCA-to-AuraFS network interface"] },
    ],
    repo: "aurphyx-trca",
  },
  {
    id: "g0dm0d3",
    name: "g0dm0d3",
    subtitle: "Quantum-Classical Orchestration Layer",
    category: "INFRA",
    status: "Architecture Phase",
    trl: "TRL 1",
    description: "Multi-TRCA coordination layer. Bridges quantum hardware substrate with classical compute. Manages coherence-time-aware scheduling, quantum-classical task routing, and cross-TRCA entanglement coordination. The 'god mode' supervisor of the entire Aurphyx quantum stack.",
    phases: [
      { phase: "Phase 1 (0-6mo)", tasks: ["Architecture specification document", "Quantum-classical task routing protocol", "Coherence-time-aware scheduler design", "Qiskit/Cirq backend abstraction layer"] },
      { phase: "Phase 2 (6-14mo)", tasks: ["Multi-TRCA coordination protocol", "Real-time coherence monitoring", "Classical compute fallback mechanisms", "Integration with AuraOS kernel"] },
      { phase: "Phase 3 (14-24mo)", tasks: ["Cross-TRCA entanglement coordination", "Production deployment on TRCA hardware", "Performance optimization", "SDK for quantum application developers"] },
    ],
    repo: "aurphyx-g0dm0d3",
  },
];

const repoStructures = {
  "aurphyx-quantum": `aurphyx-quantum/
├── README.md
├── LICENSE
├── Cargo.toml
├── docs/
│   ├── DARPA_proposal.md
│   ├── arxiv_paper.tex
│   └── experimental_protocols/
├── simulations/
│   ├── aetheric_transducer.py
│   ├── fractal_vacuum_triode.qiskit
│   └── sierpinski_lattice_sim.py
├── src/
│   ├── lib.rs
│   ├── fractal_hilbert/
│   │   ├── mod.rs
│   │   ├── sierpinski.rs
│   │   └── hausdorff.rs
│   ├── tqft/
│   │   ├── mod.rs
│   │   ├── neglecton.rs
│   │   └── braiding.rs
│   └── photonic/
│       ├── mod.rs
│       ├── band_engineering.rs
│       └── anderson_localization.rs
└── tests/`,
  "aurphyx-aurafs": `aurphyx-aurafs/
├── README.md
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── optimizer.rs
│   ├── mod.rs
│   ├── shard/
│   │   ├── mod.rs
│   │   ├── fractal_index.rs
│   │   └── shard_server.rs
│   ├── crypto/
│   │   ├── dilithium.rs
│   │   └── kyber.rs
│   ├── mesh/
│   │   ├── reticulum.rs
│   │   └── halow.rs
│   └── storage/
│       ├── ipfs.rs
│       └── tor.rs
├── k8s/
│   ├── humanitarian-node.yaml
│   └── enterprise-node.yaml
└── grafana/
    └── heart-monitor.json`,
  "aurphyx-audry": `aurphyx-audry/
├── README.md
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── chimera_core/
│   │   ├── mod.rs
│   │   ├── mixtral_router.rs
│   │   ├── starcoder_sentinel.rs
│   │   └── tildeopen_linguist.rs
│   ├── chakra_cores/
│   │   ├── mod.rs
│   │   ├── root_muladhara.rs
│   │   ├── sacral_svadhisthana.rs
│   │   ├── solar_manipura.rs
│   │   ├── heart_anahata.rs
│   │   ├── throat_vishuddha.rs
│   │   ├── third_eye_ajna.rs
│   │   └── crown_sahasrara.rs
│   ├── ethics/
│   │   └── divine_mirror.rs
│   └── integrations/
│       ├── aurafs/
│       ├── gvs/
│       ├── ineffable_ledger/
│       ├── opulence/
│       ├── sages/
│       └── fuxyez/
└── models/`,
};

function App() {
  const [selected, setSelected] = useState(null);
  const [filter, setFilter] = useState("ALL");
  const [search, setSearch] = useState("");
  const [view, setView] = useState("grid");
  const [expandedPhase, setExpandedPhase] = useState(null);

  const filtered = inventions.filter((inv) => {
    const matchCat = filter === "ALL" || inv.category === filter;
    const matchSearch =
      search === "" ||
      inv.name.toLowerCase().includes(search.toLowerCase()) ||
      inv.subtitle.toLowerCase().includes(search.toLowerCase()) ||
      inv.description.toLowerCase().includes(search.toLowerCase());
    return matchCat && matchSearch;
  });

  const stats = {
    total: inventions.length,
    categories: Object.keys(CATEGORIES).length,
    totalPhases: inventions.reduce((a, i) => a + i.phases.length, 0),
    totalTasks: inventions.reduce(
      (a, i) => a + i.phases.reduce((b, p) => b + p.tasks.length, 0),
      0
    ),
  };

  return (
    <div style={{ fontFamily: "'JetBrains Mono', 'Fira Code', monospace", background: "#0A0A0F", color: "#E0E0E8", minHeight: "100vh", padding: 0 }}>
      {/* Header */}
      <div style={{ background: "linear-gradient(135deg, #0D0D1A 0%, #1A0A2E 50%, #0A1628 100%)", borderBottom: "1px solid #2A2A3E", padding: "28px 32px 20px" }}>
        <div style={{ display: "flex", alignItems: "center", gap: 16, marginBottom: 8 }}>
          <span style={{ fontSize: 36 }}>🔮</span>
          <div>
            <h1 style={{ margin: 0, fontSize: 28, fontWeight: 800, background: "linear-gradient(90deg, #6C3CE1, #E14B3C, #D4A017)", WebkitBackgroundClip: "text", WebkitTextFillColor: "transparent" }}>
              AURPHYX TAXONOMY EXPLORER
            </h1>
            <p style={{ margin: 0, fontSize: 12, color: "#888", letterSpacing: 3 }}>
              ROSS A. EDWARDS — INVENTION PORTFOLIO & BUILD ROADMAP
            </p>
          </div>
        </div>
        {/* Stats */}
        <div style={{ display: "flex", gap: 24, marginTop: 16, flexWrap: "wrap" }}>
          {[
            { label: "Inventions", value: stats.total, color: "#6C3CE1" },
            { label: "Categories", value: stats.categories, color: "#E14B3C" },
            { label: "Build Phases", value: stats.totalPhases, color: "#1CA784" },
            { label: "Total Tasks", value: stats.totalTasks, color: "#D4A017" },
          ].map((s) => (
            <div key={s.label} style={{ background: `${s.color}15`, border: `1px solid ${s.color}40`, borderRadius: 8, padding: "8px 16px", minWidth: 100 }}>
              <div style={{ fontSize: 22, fontWeight: 800, color: s.color }}>{s.value}</div>
              <div style={{ fontSize: 10, color: "#888", letterSpacing: 1 }}>{s.label.toUpperCase()}</div>
            </div>
          ))}
        </div>
      </div>

      {/* Filters */}
      <div style={{ padding: "16px 32px", borderBottom: "1px solid #1A1A2E", display: "flex", gap: 12, flexWrap: "wrap", alignItems: "center" }}>
        <input
          type="text"
          placeholder="Search inventions..."
          value={search}
          onChange={(e) => setSearch(e.target.value)}
          style={{ background: "#12121F", border: "1px solid #2A2A3E", borderRadius: 6, padding: "8px 14px", color: "#E0E0E8", fontSize: 13, width: 220, outline: "none" }}
        />
        <div style={{ display: "flex", gap: 6, flexWrap: "wrap" }}>
          <button
            onClick={() => setFilter("ALL")}
            style={{ background: filter === "ALL" ? "#6C3CE1" : "#1A1A2E", border: "1px solid #2A2A3E", borderRadius: 6, padding: "6px 14px", color: filter === "ALL" ? "#fff" : "#888", fontSize: 11, cursor: "pointer", letterSpacing: 1 }}
          >
            ALL ({inventions.length})
          </button>
          {Object.entries(CATEGORIES).map(([key, cat]) => {
            const count = inventions.filter((i) => i.category === key).length;
            return (
              <button
                key={key}
                onClick={() => setFilter(key)}
                style={{
                  background: filter === key ? cat.color : "#1A1A2E",
                  border: `1px solid ${filter === key ? cat.color : "#2A2A3E"}`,
                  borderRadius: 6,
                  padding: "6px 12px",
                  color: filter === key ? "#fff" : "#888",
                  fontSize: 11,
                  cursor: "pointer",
                }}
              >
                {cat.icon} {cat.label} ({count})
              </button>
            );
          })}
        </div>
      </div>

      {/* Main Content */}
      <div style={{ display: "flex", height: "calc(100vh - 220px)" }}>
        {/* Inventory Grid */}
        <div style={{ flex: selected ? "0 0 380px" : 1, overflowY: "auto", padding: 20, display: "grid", gridTemplateColumns: selected ? "1fr" : "repeat(auto-fill, minmax(340px, 1fr))", gap: 14, alignContent: "start" }}>
          {filtered.map((inv) => {
            const cat = CATEGORIES[inv.category];
            const isSelected = selected?.id === inv.id;
            return (
              <div
                key={inv.id}
                onClick={() => setSelected(isSelected ? null : inv)}
                style={{
                  background: isSelected ? `${cat.color}18` : "#12121F",
                  border: `1px solid ${isSelected ? cat.color : "#1E1E30"}`,
                  borderRadius: 10,
                  padding: "16px 18px",
                  cursor: "pointer",
                  transition: "all 0.2s",
                  borderLeft: `3px solid ${cat.color}`,
                }}
              >
                <div style={{ display: "flex", justifyContent: "space-between", alignItems: "start", marginBottom: 6 }}>
                  <div>
                    <span style={{ fontSize: 18 }}>{cat.icon}</span>
                    <span style={{ fontSize: 16, fontWeight: 700, marginLeft: 8, color: cat.color }}>{inv.name}</span>
                  </div>
                  <span style={{ fontSize: 9, background: `${cat.color}25`, color: cat.color, padding: "2px 8px", borderRadius: 4, whiteSpace: "nowrap" }}>
                    {inv.trl}
                  </span>
                </div>
                <div style={{ fontSize: 11, color: "#AAA", marginBottom: 8 }}>{inv.subtitle}</div>
                <div style={{ fontSize: 10, color: "#666", lineHeight: 1.5, display: "-webkit-box", WebkitLineClamp: selected ? 4 : 2, WebkitBoxOrient: "vertical", overflow: "hidden" }}>
                  {inv.description}
                </div>
                <div style={{ marginTop: 10, display: "flex", gap: 6, justifyContent: "space-between", alignItems: "center" }}>
                  <span style={{ fontSize: 9, color: "#666", background: "#1A1A2E", padding: "2px 8px", borderRadius: 3 }}>
                    {inv.status}
                  </span>
                  <span style={{ fontSize: 9, color: "#555" }}>
                    {inv.phases.reduce((a, p) => a + p.tasks.length, 0)} tasks
                  </span>
                </div>
              </div>
            );
          })}
        </div>

        {/* Detail Panel */}
        {selected && (
          <div style={{ flex: 1, overflowY: "auto", borderLeft: "1px solid #1E1E30", background: "#0D0D18" }}>
            <div style={{ padding: "24px 28px" }}>
              {/* Header */}
              <div style={{ display: "flex", justifyContent: "space-between", alignItems: "start", marginBottom: 20 }}>
                <div>
                  <div style={{ fontSize: 10, color: CATEGORIES[selected.category].color, letterSpacing: 2, marginBottom: 4 }}>
                    {CATEGORIES[selected.category].icon} {CATEGORIES[selected.category].label.toUpperCase()}
                  </div>
                  <h2 style={{ margin: 0, fontSize: 24, fontWeight: 800, color: CATEGORIES[selected.category].color }}>
                    {selected.name}
                  </h2>
                  <p style={{ margin: "4px 0 0", fontSize: 13, color: "#AAA" }}>{selected.subtitle}</p>
                </div>
                <button onClick={() => setSelected(null)} style={{ background: "#1A1A2E", border: "1px solid #2A2A3E", borderRadius: 6, padding: "6px 12px", color: "#888", cursor: "pointer", fontSize: 12 }}>
                  ✕ Close
                </button>
              </div>

              {/* Status Bar */}
              <div style={{ display: "flex", gap: 12, marginBottom: 20, flexWrap: "wrap" }}>
                <span style={{ fontSize: 11, background: "#1CA78420", color: "#1CA784", padding: "4px 12px", borderRadius: 4 }}>
                  {selected.status}
                </span>
                <span style={{ fontSize: 11, background: "#6C3CE120", color: "#6C3CE1", padding: "4px 12px", borderRadius: 4 }}>
                  {selected.trl}
                </span>
                <span style={{ fontSize: 11, background: "#D4A01720", color: "#D4A017", padding: "4px 12px", borderRadius: 4 }}>
                  Repo: {selected.repo}
                </span>
              </div>

              {/* Description */}
              <div style={{ background: "#0A0A14", border: "1px solid #1E1E30", borderRadius: 8, padding: 16, marginBottom: 24 }}>
                <div style={{ fontSize: 10, color: "#666", letterSpacing: 2, marginBottom: 8 }}>DESCRIPTION</div>
                <p style={{ margin: 0, fontSize: 12, lineHeight: 1.7, color: "#CCC" }}>{selected.description}</p>
              </div>

              {/* Phased Build Plan */}
              <div style={{ marginBottom: 24 }}>
                <div style={{ fontSize: 10, color: "#666", letterSpacing: 2, marginBottom: 12 }}>PHASED BUILD PLAN</div>
                {selected.phases.map((phase, pi) => (
                  <div key={pi} style={{ marginBottom: 12 }}>
                    <div
                      onClick={() => setExpandedPhase(expandedPhase === `${selected.id}-${pi}` ? null : `${selected.id}-${pi}`)}
                      style={{
                        background: expandedPhase === `${selected.id}-${pi}` ? `${CATEGORIES[selected.category].color}12` : "#12121F",
                        border: `1px solid ${expandedPhase === `${selected.id}-${pi}` ? CATEGORIES[selected.category].color + "40" : "#1E1E30"}`,
                        borderRadius: 8,
                        padding: "12px 16px",
                        cursor: "pointer",
                      }}
                    >
                      <div style={{ display: "flex", justifyContent: "space-between", alignItems: "center" }}>
                        <span style={{ fontSize: 13, fontWeight: 700, color: CATEGORIES[selected.category].color }}>
                          {expandedPhase === `${selected.id}-${pi}` ? "▾" : "▸"} {phase.phase}
                        </span>
                        <span style={{ fontSize: 10, color: "#666" }}>{phase.tasks.length} tasks</span>
                      </div>
                      {expandedPhase === `${selected.id}-${pi}` && (
                        <div style={{ marginTop: 12, paddingLeft: 12, borderLeft: `2px solid ${CATEGORIES[selected.category].color}30` }}>
                          {phase.tasks.map((task, ti) => (
                            <div key={ti} style={{ fontSize: 11, color: "#BBB", padding: "4px 0", display: "flex", gap: 8, alignItems: "start" }}>
                              <span style={{ color: CATEGORIES[selected.category].color, fontSize: 8, marginTop: 4 }}>◆</span>
                              {task}
                            </div>
                          ))}
                        </div>
                      )}
                    </div>
                  </div>
                ))}
              </div>

              {/* Repository Structure */}
              {repoStructures[selected.repo] && (
                <div>
                  <div style={{ fontSize: 10, color: "#666", letterSpacing: 2, marginBottom: 8 }}>REPOSITORY STRUCTURE</div>
                  <pre style={{ background: "#0A0A14", border: "1px solid #1E1E30", borderRadius: 8, padding: 16, fontSize: 11, lineHeight: 1.6, color: "#8B8BA0", overflow: "auto", margin: 0 }}>
                    {repoStructures[selected.repo]}
                  </pre>
                </div>
              )}
            </div>
          </div>
        )}
      </div>
    </div>
  );
}

export default App;
