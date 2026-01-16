# Swarmhost

**Distributed multiplayer game networking with blockchain-inspired consensus**

Swarmhost is a next-generation networking solution for multiplayer games that eliminates the need for dedicated servers while providing strong cheat prevention. By distributing the networking load across all players and using consensus mechanisms inspired by blockchain technology, Swarmhost creates resilient, decentralized multiplayer experiences.

[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![Status](https://img.shields.io/badge/status-early%20development-yellow.svg)](https://github.com/yourusername/swarmhost)

## 🎯 The Problem

Traditional multiplayer game hosting has two main approaches, each with significant drawbacks:

**Dedicated Servers:**
- ✅ Good performance and cheat prevention
- ❌ Expensive to maintain
- ❌ Single point of failure
- ❌ High latency for distant players
- ❌ Shut down when company stops supporting the game

**Peer-to-Peer (P2P):**
- ✅ No server costs
- ✅ Players can host indefinitely
- ❌ Host has authority (easily exploited)
- ❌ Vulnerable to cheating
- ❌ Poor experience if host has bad connection

## 💡 The Swarmhost Solution

Swarmhost combines the best of both approaches:

- **Distributed hosting** - No single server or host; load spread across all players
- **Consensus validation** - Multiple nodes verify each action before accepting it
- **Byzantine fault tolerance** - System continues working even with malicious players
- **Optimistic execution** - Actions applied immediately for low latency, rolled back if rejected
- **No infrastructure costs** - Runs entirely on player machines

### How It Works

```
Traditional P2P:
Player 1 (Host) → [Action] → Player 2 ✓ (trusts host)
                           → Player 3 ✓ (trusts host)
Problem: Host can cheat, others must accept it

Swarmhost:
Player 1 → [Action] → Player 2 → Validates → Votes ✓
                   → Player 3 → Validates → Votes ✓
                   → Player 4 → Validates → Votes ✓
Result: Action accepted only if 2/3 players agree it's valid
Problem: Cheating detected and rejected by consensus
```

## 🎮 Use Cases

Swarmhost is ideal for:

- **Turn-based strategy games** - Perfect fit with forgiving latency requirements
- **Card games** (poker, trading card games) - Private information + public validation
- **Board games** (chess, checkers, Catan) - Clear rules, easy to validate
- **Co-op games** - Players work together, less incentive to cheat
- **Indie multiplayer games** - No server costs, community-hosted forever

**Future support:**
- Real-time strategy games
- Fighting games
- First-person shooters (with latency optimizations)

## 🏗️ Architecture

Swarmhost is designed as a monorepo with multiple components:

```
swarmhost/
├── swarmhost-core/        # Rust core networking library
├── bindings/              # Language bindings (C++, C#)
├── plugins/               # Engine-specific plugins
│   ├── unreal/           # Unreal Engine plugin
│   └── unity/            # Unity plugin
├── examples/             # Reference implementations
│   └── poker/            # Poker game for testing
├── tests/                # Integration tests
└── docs/                 # Documentation
```

### Core Library (`swarmhost-core/`)

The heart of Swarmhost, written in Rust for performance and safety:

- **Cryptography** - Ed25519 signatures for action verification
- **Networking** - QUIC protocol for fast, reliable connections
- **Consensus** - Simplified PBFT for Byzantine fault tolerance
- **State Management** - Event sourcing with snapshots
- **FFI Layer** - C bindings for engine integration

[View swarmhost-core documentation →](./swarmhost-core/README.md)

### Engine Plugins

Thin wrappers that make Swarmhost easy to use in popular game engines:

- **Unreal Engine** - C++ component-based integration
- **Unity** - C# manager with event system
- **Custom Engines** - Use the C FFI directly

## 🚀 Current Status

**Phase 1: Foundation** ✅ Complete (v0.1.0)
- Cryptography (signing, hashing, verification)
- Node configuration system
- Basic lifecycle management
- Error handling
- 14 passing unit tests

**Phase 2: Networking** 🚧 In Progress
- QUIC transport layer
- Message protocol (Protocol Buffers)
- Peer discovery
- Connection management

**Phase 3: Consensus** 📋 Planned
- Simplified PBFT implementation
- Action validation framework
- Byzantine fault detection

**Phase 4: State Management** 📋 Planned
- Event sourcing
- State snapshots
- State synchronization

**Phase 5: Engine Integration** 📋 Planned
- Unreal Engine plugin
- Unity plugin
- Example games

## 📦 Installation

### Prerequisites

- **Rust** 1.70+ ([install via rustup](https://rustup.rs/))
- **Git**
- **C++ compiler** (for FFI bindings)
- Optional: Unreal Engine or Unity for plugin development

### Quick Start

```bash
# Clone the repository
git clone https://github.com/yourusername/swarmhost.git
cd swarmhost

# Build the core library
cd swarmhost-core
cargo build --release
cargo test

# Run the poker example (coming soon)
cd ../examples/poker
docker-compose up
```

## 📖 Documentation

- [Architecture Overview](./docs/architecture/overview.md) *(coming soon)*
- [Core Library API](./swarmhost-core/README.md)
- [Unreal Integration Guide](./docs/guides/unreal-integration.md) *(coming soon)*
- [Unity Integration Guide](./docs/guides/unity-integration.md) *(coming soon)*
- [Protocol Specification](./docs/architecture/protocol.md) *(coming soon)*

## 🎯 Example: Simple Usage

```rust
use swarmhost_core::{NodeConfig, SwarmhostNode};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create and configure a node
    let config = NodeConfig::new()
        .with_bootstrap("bootstrap.example.com:8080")
        .with_port(9000);
    
    let node = SwarmhostNode::new(config)?;
    
    // Start networking
    node.start().await?;
    
    // Join a game
    node.join_game("poker-game-123").await?;
    
    // Submit an action (e.g., betting in poker)
    let bet_data = serialize_bet(100);
    node.submit_action(ACTION_BET, &bet_data).await?;
    
    // Action is validated by consensus before being accepted
    
    Ok(())
}
```

## 🎲 Example Project: Poker

We're building a fully-functional poker game to validate Swarmhost:

- **6-player Texas Hold'em**
- **Distributed deck shuffling** (commit-reveal scheme)
- **Private hole cards** (encrypted per player)
- **Cheat detection** (invalid bets, card manipulation)
- **Docker-based testing** (6 nodes, automated scenarios)

This poker implementation serves as both a test bed and a reference for building your own Swarmhost-powered games.

## 🤝 Contributing

Swarmhost is currently in early development as a solo project. Once the core functionality is stable, contribution guidelines will be added.

### Development Workflow

```bash
# Format code
cargo fmt

# Lint code
cargo clippy

# Run tests
cargo test

# Run integration tests (once available)
cd tests/integration
docker-compose up
```

## 🗺️ Roadmap

### 2026 Q1-Q2: Core Functionality
- ✅ Foundation (cryptography, config, errors)
- 🚧 Networking (QUIC, messages, peers)
- 📋 Consensus (PBFT, validation)
- 📋 State management (event log, snapshots)

### 2026 Q3: Validation & Testing
- 📋 Poker example complete
- 📋 Integration tests
- 📋 Performance benchmarks
- 📋 Security audit

### 2026 Q4: Engine Integration
- 📋 C FFI layer
- 📋 C++ bindings
- 📋 C# bindings
- 📋 Unreal Engine plugin
- 📋 Unity plugin

### 2027+: Expansion
- 📋 Additional game examples
- 📋 FPS optimizations
- 📋 Community features
- 📋 Production hardening

## 📊 Project Structure

| Directory | Purpose | Status |
|-----------|---------|--------|
| `swarmhost-core/` | Rust networking library | 🚧 In Progress |
| `bindings/` | C++/C# wrappers | 📋 Planned |
| `plugins/` | Unreal/Unity plugins | 📋 Planned |
| `examples/` | Reference implementations | 📋 Planned |
| `tests/` | Integration/performance tests | 📋 Planned |
| `docs/` | Documentation | 📋 Planned |

## 🔬 Technology Stack

- **Language**: Rust (core library)
- **Async Runtime**: Tokio
- **Transport**: QUIC (via Quinn)
- **Serialization**: Protocol Buffers
- **Cryptography**: Ed25519 (signing), Blake2s (hashing)
- **Testing**: Docker (multi-node scenarios)
- **FFI**: C bindings (cbindgen)

## 📄 License

This project is dual-licensed under:
- MIT License
- Apache License 2.0

You may choose either license at your option.

## ⚠️ Status Notice

**Swarmhost is in early development (v0.1.0)**

- Core functionality is still being built
- APIs will change
- Not production-ready
- Use at your own risk

We welcome feedback, but please understand this is experimental software.

## 🙏 Acknowledgments

- Inspired by blockchain consensus mechanisms (PBFT, Raft)
- Influenced by peer-to-peer protocols (BitTorrent, IPFS)
- Built with the amazing Rust ecosystem

## 📬 Contact

- **Issues**: [GitHub Issues](https://github.com/yourusername/swarmhost/issues)
- **Discussions**: [GitHub Discussions](https://github.com/yourusername/swarmhost/discussions)
- **Email**: your.email@example.com *(coming soon)*

---

**Built with 🦀 Rust** • **Early Development** • **v0.1.0** • **2026**
