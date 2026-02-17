# VajraChain

**VajraChain** is a high-performance, modular blockchain framework written in Rust. It is designed to be scalable, secure, and extensible, providing a solid foundation for decentralized applications.

![Build Status](https://img.shields.io/badge/build-passing-brightgreen)
![License](https://img.shields.io/badge/license-MIT-blue)
![Rust Version](https://img.shields.io/badge/rust-1.70%2B-orange)

## 🚀 Key Features

VajraChain is built with a modular architecture, consisting of 10 core phases that have been fully implemented and verified:

- **Core Ledger Protocol**: Secure block and transaction structures with Merkle Tree integration and Ed25519 signatures.
- **Consensus Mechanism**: robust validator set management with square-root voting power.
- **Networking Layer**: P2P networking using `libp2p` with Gossipsub messaging.
- **State Storage Engine**: Persistent state management using `sled` with State Rent logic.
- **Execution Environment (VM)**: Stack-based VM with gas metering and custom OpCodes.
- **Smart Contract System**: Native contract execution and system contract registry.
- **Security Layer**: Slashing mechanisms and Fork Choice rules for network security.
- **Identity & Privacy**: DID (Decentralized Identity) support and Verifiable Credentials.
- **Governance System**: On-chain proposals and voting engines.
- **Interoperability**: Cross-chain bridge messaging and state outbox.

## 🛠️ Installation

Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed.

```bash
git clone https://github.com/pronzzz/vajrachain.git
cd vajrachain
cargo build --release
```

## 🏃 Usage

### Running the Node

To start a VajraChain node, use the `node` binary:

```bash
cargo run --release -p node
```

This will initialize the node, connecting the networking, storage, and consensus layers.

### using the CLI

The `cli` tool provides utilities for key generation and interaction:

```bash
cargo run --release -p cli -- keygen
```

## 📖 Documentation

For a detailed walkthrough of the architecture and development guide, please refer to [GUIDE.md](GUIDE.md).

## 🤝 Contributing

Contributions are welcome! Please read our [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct and the process for submitting pull requests.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
