# 🎰 Aurphyx Casino

**Provably Fair Multi-Chain Casino & Sportsbook Platform**

Aurphyx Casino is a next-generation, provably fair casino and sportsbook platform built with Rust, featuring quantum-certified randomness, multi-chain blockchain integration, and advanced ML-powered odds calculation.

## 🔥 The Manifesto

Aurphyx Casino represents the future of online gambling:

- **🔬 Quantum-Certified Randomness**: 256-qubit quantum computer integration + Chainlink VRF
- **⛓️ Multi-Chain**: Ethereum, Solana, Polygon, and Ineffable Ledger support
- **📡 Off-Grid Betting**: Meshtastic LoRa mesh network for offline betting
- **🤖 ML-Powered Odds**: Advanced machine learning models for sportsbook odds
- **🛡️ Secure Win Lockdown**: 24-72hr cooldown system for large wins
- **🔮 13 Sentinel Integration**: S.A.G.E.S security framework
- **💎 Shardenomics**: AURPHYX token economy with 555T supply
- **🌉 AINTS Bridge**: Full integration with Aura ecosystem

## 🚀 Quick Start

### Prerequisites

- Rust 1.70+
- Node.js 18+
- Python 3.10+
- PostgreSQL 15+
- Redis 7+

### Development Setup

```bash
# Clone the repository
git clone <repository-url>
cd aurphyx-casino

# Start infrastructure
docker-compose up -d postgres redis

# Setup Rust backend
cd src
cargo build
cargo run

# Setup frontend
cd web
npm install
npm run dev

# Setup Python ML
cd python
pip install -e .
```

## 📁 Project Structure

- `src/` - Rust backend core
- `web/` - Next.js frontend
- `mobile/` - React Native mobile app
- `python/` - ML models and analytics
- `contracts/` - Solidity smart contracts
- `sdk/` - Client SDKs (Rust, JS, Python)
- `cli/` - Command-line tools
- `docs/` - Documentation
- `config/` - Configuration files
- `deployment/` - Docker, K8s, Terraform

## 🎮 Features

### Casino Games
- Video Slots with Progressive Jackpots
- Blackjack (Classic + Variants)
- Roulette (American + European)
- Poker (Texas Hold'em + Variants)
- Baccarat, Craps, Dice
- Crash, Plinko, Mines, Wheel, Keno
- Live Dealer Games

### Sportsbook
- NFL, NBA, MLB, NHL
- Soccer (EPL, La Liga, etc.)
- MMA, Boxing, Esports
- F1, NASCAR, Horse Racing
- Tennis, Special Events
- Live In-Play Betting
- Parlay Builder

## 🔐 Security

- Provably fair game verification
- KYC/AML compliance
- Fraud detection ML models
- DDoS protection
- Quantum-safe encryption
- Secure win lockdown system

## 📚 Documentation

See the `docs/` directory for comprehensive documentation:
- [Getting Started](docs/GETTING_STARTED.md)
- [Architecture](ARCHITECTURE.md)
- [Provably Fair System](docs/PROVABLY_FAIR.md)
- [API Reference](docs/api/REST_API.md)

## 🤝 Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## 📄 License

MIT OR Apache-2.0 OR SAGES License

See [LICENSE](LICENSE) for details.

## 🎯 Roadmap

- **June 6th, 2026 Launch**: Full production deployment
- See [docs/06-06-2026_LAUNCH.md](docs/NYE_2026_LAUNCH.md) for details

---

**Built with ❤️ by the Aurphyx Team**
