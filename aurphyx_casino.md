# Aurphyx Casino

**Where the User Is the House**  
**Zero-to-Configurable House Edge · Pure Peer-to-Peer · Community-Owned Casino & Sportsbook**

**Aurphyx LLC · 2026**  
**Canon status:** Living architecture (extracted & polished from development threads)

---

## 1. Core Thesis

Traditional casinos extract a permanent house edge (2–15 %) for a corporate entity.  
Aurphyx inverts the model:

> **The players collectively *are* the house.**

- Pure P2P mode (0 % house edge) is the philosophical default.
- Configurable edge (0–5 %) exists as an optional, transparent, community-governed parameter.
- All excess value flows back to liquidity providers, token stakers, and positive real-world action rewards (Shardenomics integration).

This is not “crypto casino with extra steps.” It is a new economic primitive: **community-owned gaming infrastructure** that funnels value upward from play into ownership.

---

## 2. High-Level Architecture

```
afs/src/casino/
├── rng/                    # Provably fair randomness
│   ├── quantum_rng.rs      # Majorana-1 qubit collapse (primary)
│   ├── verifiable_rng.rs   # VRF fallback
│   └── fairness_proof.rs   # ZK-SNARK proofs of fairness
├── games/
│   ├── slots/
│   ├── table_games/        # blackjack, roulette, baccarat, poker
│   ├── crypto_originals/   # dice, crash, mines, plinko, tower, limbo
│   └── live_dealer/
├── house_edge/
│   ├── configurable_edge.rs
│   ├── zero_edge_mode.rs   # Pure P2P
│   └── profit_sharing.rs
├── session/
│   ├── wallet_manager.rs
│   ├── bet_limits.rs
│   └── responsible_gaming.rs
├── sportsbook/             # Full sports + esports engine
├── defi/                   # Liquidity pools, staking, NFT marketplace
├── compliance/             # ZK-KYC (BlissID), AML, responsible gaming, licensing
├── redteam/                # Casino-specific attack surface testing
└── whitehat/               # Defensive monitoring & circuit breakers
```

Supporting layers:

- **Meshtastic / LoRa** — offline bet queuing & sync
- **IPFS** — game assets, bet receipts, provenance
- **Multi-chain settlement** — Bitcoin, Ethereum, Solana, Polygon, BSC, Avalanche + native Ineffable Ledger
- **Kubernetes** manifests for casino-engine, sportsbook-engine, RNG service, oracle aggregator, settlement workers

---

## 3. Randomness & Fairness

Primary entropy source is a **Majorana-1 qubit collapse** interface (QuantumRNG).  
Client seed + server seed + nonce → deterministic yet unpredictable outcome, with ZK-SNARK proof that the house cannot bias the result.

Fallback path uses verifiable random functions (VRF). Every game result is independently verifiable by any participant.

---

## 4. House Edge Philosophy

| Mode                | Edge   | Who benefits                          | Use case                          |
|---------------------|--------|---------------------------------------|-----------------------------------|
| Zero Edge (default) | 0 %    | Pure player-to-player                 | Idealistic / community events     |
| Configurable        | 0–5 %  | Liquidity providers + DAO treasury    | Sustainable operations            |
| Progressive         | Variable | Jackpot pools                         | High-visibility games             |

Profit-sharing module distributes any edge to:

- Liquidity pool stakers
- AuraFS Shard holders
- Real-world positive-action reward pools (Shardenomics)

---

## 5. Sportsbook

Full coverage of major sports + esports.  
Odds compilation, multi-oracle consensus (Chainlink + commercial feeds + internal), live betting, parlays, player props, auto-settlement with manual dispute path.

---

## 6. DeFi & Token Integration

- Liquidity pools for house-edge absorption
- Staking for governance weight and yield
- NFT marketplace (skins, achievement badges, VIP passes, limited editions)
- Native currency is **AuraFS Shards** (see Shardenomics whitepaper)
- Players do not “deposit fiat then play”; they play and earn/spend Shards

---

## 7. Compliance & Responsible Gaming

- **BlissID** zero-knowledge KYC / age / jurisdiction checks
- AML transaction monitoring + SAR generation
- Self-exclusion, deposit limits, time limits, cool-downs
- Licensing modules prepared for Curaçao, MGA (Malta), UKGC

---

## 8. Security Surface

**Redteam (Casino edition):**
- RNG bias detection
- Martingale / betting-system attacks
- Arbitrage finders
- Session hijacking
- Odds manipulation
- Late-bet exploits
- Multi-account abuse
- Flash-loan / reentrancy / oracle attacks on the DeFi side

**Whitehat counterparts** run continuously with circuit breakers and audit trails.

---

## 9. Phase Model (Strategic Rollout)

**Phase 1 — Faucet / Attract**  
Free play, real-world quests, positive-action rewards (picking up trash, helping elders, etc.), large Shard giveaways. Goal: acquire users and distribute ownership widely.

**Phase 2 — Transition**  
Gradually introduce optional edge and deeper liquidity while the community already owns the majority of the float.

**Phase 3 — Steady State**  
User-is-the-house equilibrium. Edge is transparent and community-governed. Value recirculates.

Target early metrics discussed in threads: rapid growth toward multi-million user base with Shards as the native unit of account.

---

## 10. Integration with the Rest of Aurphyx

- **SAGES Guardians** police the casino plane exactly as they police every other plane.
- **Shardenomics** supplies the economic substrate and the “trickle-up” philosophy.
- **BlissID / SoulKey** provides the identity and one-person-one-vote foundation.
- **Ineffable Ledger** is the settlement and audit backbone.
- Offline/mesh capability via Meshtastic for resilience.

---

## 11. Taglines & Canon Phrases

- “The user is the house.”
- “Zero house edge is the default; any edge is community-owned.”
- “Fuck Reaganomics — it’s Shardenomics, baby.”
- “Play. Own. Recirculate.”

---

*Extracted, cleaned, and structured from multi-model development threads by Audry.*  
*Living document — will be versioned as the casino implementation matures.*
