# Swarmhost Core

Distributed multiplayer game networking with blockchain-inspired consensus mechanisms.

## Overview

Swarmhost Core is a Rust library that provides decentralized, peer-to-peer networking for multiplayer games. Instead of relying on a single authoritative server or simple P2P connections, Swarmhost distributes the networking load across all players and uses consensus algorithms to prevent cheating.

**Key Features:**
- 🔐 **Cryptographic Security** - Ed25519 signatures for action verification
- 🤝 **Byzantine Fault Tolerance** - Handles malicious players trying to cheat
- 🌐 **Distributed Consensus** - No single point of failure
- ⚡ **Optimistic Execution** - Low latency for responsive gameplay
- 🎮 **Engine Agnostic** - C FFI for integration with any game engine

## Current Status

**Phase 1: Foundation** ✅ Complete

- [x] Cryptography (Ed25519 signing, Blake2s hashing)
- [x] Node configuration system
- [x] Basic node lifecycle
- [x] Error handling
- [x] 14 passing unit tests

**Phase 2: Networking** 🚧 In Progress

- [ ] QUIC transport layer
- [ ] Message protocol (Protocol Buffers)
- [ ] Peer discovery
- [ ] Connection management

**Phase 3: Consensus** 📋 Planned

- [ ] Simplified PBFT implementation
- [ ] Action validation framework
- [ ] Vote collection and tallying
- [ ] Byzantine fault detection

**Phase 4: State Management** 📋 Planned

- [ ] Event sourcing
- [ ] State snapshots
- [ ] State synchronization

## Quick Start

### Prerequisites

- Rust 1.70+ (`rustup` recommended)
- Cargo

### Building

```bash
# Clone the repository
git clone <your-repo-url>
cd swarmhost/swarmhost-core

# Build the library
cargo build

# Run tests
cargo test

# Build optimized release version
cargo build --release
```

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_keypair_generation

# Run with logging
RUST_LOG=debug cargo test -- --nocapture
```

## Project Structure

```
swarmhost-core/
├── src/
│   ├── lib.rs              # Library entry point
│   ├── error.rs            # Error types
│   ├── crypto/             # Cryptography
│   │   └── mod.rs          # ✅ Ed25519, Blake2s
│   ├── node/               # Node implementation
│   │   ├── mod.rs          # ✅ Node lifecycle
│   │   └── config.rs       # ✅ Configuration
│   ├── network/            # Networking (🚧 in progress)
│   │   └── mod.rs
│   ├── consensus/          # Consensus (📋 planned)
│   │   └── mod.rs
│   └── state/              # State management (📋 planned)
│       └── mod.rs
├── Cargo.toml
└── README.md
```

## Usage Example

```rust
use swarmhost_core::{NodeConfig, SwarmhostNode};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a node with default configuration
    let config = NodeConfig::new()
        .with_bootstrap("bootstrap.example.com:8080")
        .with_port(9000);
    
    let node = SwarmhostNode::new(config)?;
    
    // Start the node
    node.start().await?;
    
    // Join a game session
    node.join_game("game-123").await?;
    
    // Submit an action (e.g., player movement)
    node.submit_action(ACTION_MOVE, &action_data).await?;
    
    Ok(())
}
```

## Architecture

```
┌─────────────────────────────────────────┐
│         SwarmhostNode (API)             │
├─────────────────────────────────────────┤
│  Consensus │  State  │  Network         │
│   Manager  │ Manager │  Manager         │
├─────────────────────────────────────────┤
│           Cryptography                  │
│    (Signing, Hashing, Verification)     │
└─────────────────────────────────────────┘
```

### Design Principles

1. **No Single Authority** - Every player's node validates actions
2. **Byzantine Fault Tolerant** - System works even with malicious nodes
3. **Optimistic Execution** - Actions applied immediately, rolled back if rejected
4. **Event Sourcing** - Complete action history for auditing and replay
5. **Engine Agnostic** - C FFI allows integration with any game engine

## Configuration

```rust
use swarmhost_core::{NodeConfig, ConsensusConfig};
use std::time::Duration;

let config = NodeConfig {
    keypair: Some(KeyPair::generate()),
    bootstrap_server: Some("bootstrap.example.com:8080".to_string()),
    listen_port: 9000,
    consensus: ConsensusConfig {
        quorum_numerator: 2,           // 2/3 majority
        quorum_denominator: 3,
        optimistic_execution: true,
        consensus_timeout: Duration::from_secs(5),
        max_concurrent_validations: 100,
    },
    ..Default::default()
};
```

## Performance Goals

| Metric | Target | Current | Status |
|--------|--------|---------|--------|
| Consensus latency | < 500ms | TBD | 🔴 |
| Throughput | > 100 actions/sec | TBD | 🔴 |
| Peer connections | 50+ simultaneous | TBD | 🔴 |
| State sync time | < 2s for 1000 actions | TBD | 🔴 |
| Memory per node | < 100MB | ~10MB | ✅ |

## Development

### Code Quality Tools

```bash
# Format code
cargo fmt

# Lint code
cargo clippy

# Check without building (fast)
cargo check

# Generate documentation
cargo doc --open

# Clean build artifacts
cargo clean
```

### Testing Philosophy

- **Unit tests** for individual functions and modules
- **Integration tests** for multi-node scenarios (coming soon)
- **Property-based tests** for cryptographic operations (coming soon)
- **Benchmarks** for performance-critical code (coming soon)

## Roadmap

### Phase 2: Networking (Current)
- Implement QUIC transport layer
- Define Protocol Buffer message schemas
- Build peer discovery via bootstrap server
- Create connection pool management

### Phase 3: Consensus
- Implement simplified PBFT algorithm
- Build action validation framework
- Create vote collection system
- Add Byzantine fault detection

### Phase 4: State Management
- Implement event log with snapshots
- Build state synchronization protocol
- Add conflict resolution
- Create state recovery mechanisms

### Phase 5: Engine Integration
- C FFI bindings
- C++ wrapper library
- C# wrapper for Unity
- Unreal Engine plugin
- Unity plugin

## Contributing

This is currently a solo development project. Contribution guidelines will be added when the project reaches a more mature state.

### Before Committing
- [ ] Run `cargo fmt`
- [ ] Run `cargo clippy`
- [ ] Run `cargo test`
- [ ] Update this README if needed

### Commit Message Format
```
[module] Brief description

- Detailed change 1
- Detailed change 2
```

## Dependencies

- **tokio** - Async runtime
- **quinn** - QUIC protocol implementation
- **ed25519-dalek** - Ed25519 signatures
- **blake2** - Blake2 hashing
- **prost** - Protocol Buffers
- **thiserror** - Error handling
- **tracing** - Logging

## License

MIT OR Apache-2.0 (to be decided)

## Acknowledgments

Inspired by blockchain consensus mechanisms adapted for real-time gaming.

---

**Status**: Early Development (v0.1.0)  
**Rust Version**: 1.70+  
**Last Updated**: January 2026
