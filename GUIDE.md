# VajraChain Development Guide

This guide provides a detailed overview of the VajraChain architecture and instructions for developers contributing to or building upon the protocol.

## 🏗️ Architecture Overview

VajraChain is organized into several modular crates, each responsible for a specific aspect of the blockchain protocol.

### 1. Core Ledger (`ledger`, `crypto`)

- Defines the fundamental data structures: blocks, transactions, and headers.
- Implements cryptographic primitives using Ed25519 signatures.
- Manages the Merkle Tree for state verification.

### 2. Consensus (`consensus`)

- Handles the agreement protocol among validators.
- Implements a weighted voting system based on square-root of stake.

### 3. Networking (`network`)

- Built on `libp2p`.
- Uses Gossipsub for broadcasting blocks and transactions.
- Handles peer discovery and connection management.

### 4. Storage (`storage`)

- Persistent Key-Value storage using `sled`.
- Implements state rent to prevent state bloat.

### 5. Virtual Machine (`vm`, `contracts`)

- A custom stack-based VM for executing smart contracts.
- Includes gas metering to prevent infinite loops and resource exhaustion.
- Supports native system contracts for core functionality.

## 💻 Developer Setup

### Prerequisites

- Rust (latest stable)
- Clang (for RocksDB/sled dependencies if applicable)
- Protobuf compiler (optional, depending on network upgrades)

### Building the Project

```bash
cargo build
```

### Running Tests

To ensure everything is working correctly:

```bash
cargo test
```

## 🚀 Running a Local Testnet

Currently, the `node` binary starts a single standalone node.

```bash
cargo run -p node
```

To run a light node (reduced cache):

```bash
cargo run -p node -- --light
```

## 📦 Project Structure

```text
vajrachain/
├── cli/            # Command-line interface tools
├── consensus/      # Consensus logic
├── contracts/      # Smart contract execution
├── crypto/         # Cryptography utilities
├── economics/      # Tokenomics and inflation logic
├── governance/     # On-chain governance
├── identity/       # DID and Verifiable Credentials
├── interop/        # Cross-chain bridge support
├── ledger/         # Core blockchain data structures
├── network/        # P2P networking
├── node/           # Main node binary
├── security/       # Slashing and security mechanisms
├── storage/        # Database layer
└── vm/             # Virtual Machine
```
