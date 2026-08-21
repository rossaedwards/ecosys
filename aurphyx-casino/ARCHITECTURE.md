# 🏗️ Aurphyx Casino Architecture

## Overview

Aurphyx Casino is a multi-layered, provably fair casino and sportsbook platform built with a Rust backend, Next.js frontend, and Python ML components.

## System Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Frontend Layer                           │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐       │
│  │   Next.js    │  │ React Native │  │   Web3 SDK   │       │
│  │   Web App    │  │  Mobile App  │  │  Integration │       │
│  └──────────────┘  └──────────────┘  └──────────────┘       │
└─────────────────────────────────────────────────────────────┘
                            │
                            │ HTTP/WebSocket/GraphQL
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                    API Layer (Rust)                         │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐       │
│  │   REST API   │  │  WebSocket   │  │   GraphQL    │       │
│  │   (Axum)     │  │   Server     │  │   Server     │       │
│  └──────────────┘  └──────────────┘  └──────────────┘       │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                  Business Logic Layer                       │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐       │
│  │   Casino     │  │  Sportsbook   │  │  Payment     │      │
│  │   Engine     │  │   Engine      │  │  Processing  │      │
│  └──────────────┘  └──────────────┘  └──────────────┘       │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐       │
│  │  Quantum RNG │  │  Shardenomics │  │  Security  │        │
│  │   System     │  │   Tokenomics  │  │   Layer      │      │
│  └──────────────┘  └──────────────┘  └──────────────┘       │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                  Data & Integration Layer                   │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐       │
│  │  PostgreSQL  │  │    Redis     │  │  Blockchain  │       │
│  │   Database   │  │    Cache     │  │  Networks    │       │
│  └──────────────┘  └──────────────┘  └──────────────┘       │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐       │
│  │   AuraFS     │  │   Meshtastic  │  │   SAGES      │      │
│  │  Immutable   │  │   LoRa Mesh   │  │  Sentinels   │      │
│  └──────────────┘  └──────────────┘  └──────────────┘       │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                  External Services                          │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐       │
│  │  Quantum     │  │  Chainlink    │  │   ML Models  │      │
│  │  Computer    │  │     VRF       │  │   (Python)   │      │
│  └──────────────┘  └──────────────┘  └──────────────┘       │
└─────────────────────────────────────────────────────────────┘
```

## Core Components

### 1. Casino Engine (`src/casino/`)

- **Game Engine**: Core game logic and state management
- **Quantum RNG**: Integration with quantum computers and Chainlink VRF
- **Games**: Slots, Blackjack, Roulette, Poker, etc.
- **Jackpots**: Progressive jackpot system
- **Live Dealer**: WebRTC streaming for live games

### 2. Sportsbook Engine (`src/sportsbook/`)

- **Odds Engine**: ML-powered odds calculation
- **Bet Placement**: Bet processing and validation
- **Settlement**: Automatic bet settlement
- **Live Betting**: In-play betting system
- **Risk Management**: Liability management

### 3. Blockchain Integration (`src/blockchain/`)

- **Multi-Chain Bridge**: Ethereum, Solana, Polygon, Ineffable Ledger System
- **Wallet Integration**: Metamask, Phantom, WalletConnect
- **Smart Contracts**: Solidity contracts for provably fair games
- **Transaction Management**: Cross-chain transaction handling

### 4. Security (`src/security/`)

- **Win Lockdown**: 24-72hr cooldown for large wins
- **KYC/AML**: Identity verification and screening
- **Fraud Detection**: ML-based pattern detection
- **Encryption**: Kyber1024 + Dilithium5

### 5. Shardenomics (`src/shardenomics/`)

- **Token Contract**: AURPHYX token (555T supply)
- **Staking**: Token staking with APY
- **Rewards**: Loyalty and bonus system
- **Treasury**: Casino treasury management

## Data Flow

### Casino Game Flow

1. User places bet via frontend
2. Frontend sends request to REST API
3. API validates bet and user balance
4. Casino Engine generates quantum randomness
5. Game logic executes with randomness
6. House edge applied to outcome
7. Result stored in database
8. User balance updated
9. Response sent to frontend

### Sportsbook Flow

1. ML model calculates odds
2. Odds displayed to user
3. User places bet
4. Bet validated and stored
5. Event outcome monitored
6. Automatic settlement on completion
7. Payout processed

## Technology Stack

### Backend
- **Language**: Rust
- **Framework**: Axum
- **Database**: PostgreSQL + Redis
- **Blockchain**: ethers.rs, solana-client

### Frontend
- **Framework**: Next.js 14 (App Router)
- **Language**: TypeScript
- **Styling**: Tailwind CSS
- **State**: Zustand

### ML/Analytics
- **Language**: Python
- **Libraries**: scikit-learn, XGBoost, PyTorch
- **Deployment**: FastAPI service

### Smart Contracts
- **Language**: Solidity
- **Framework**: Hardhat
- **Networks**: Ethereum, Polygon

## Scalability

- Horizontal scaling via Kubernetes
- Redis caching for high-frequency reads
- Database connection pooling
- WebSocket connection management
- CDN for static assets

## Security Architecture

- Multi-layer security with SAGES integration
- Provably fair game verification
- Encrypted communication (TLS)
- Rate limiting and DDoS protection
- Secure win lockdown system
- Regular security audits

## Deployment

- **Development**: Docker Compose
- **Staging**: Kubernetes on cloud provider
- **Production**: Multi-region Kubernetes with auto-scaling

See [deployment/](deployment/) for detailed deployment configurations.
