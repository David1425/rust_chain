# RustChain (WIP) ⛓️

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Tests](https://img.shields.io/badge/tests-87%20passing-green.svg)](#testing)

> **A production-ready blockchain implementation in Rust with enterprise-grade features**

RustChain is a complete blockchain solution featuring proof-of-work consensus, P2P networking, persistent storage, HD wallets, and comprehensive analytics - designed for real-world deployment.

## 🚀 Quick Start

```bash
# Clone and build
git clone https://github.com/David1425/rust_chain.git
cd rust_chain && cargo build --release

# Initialize blockchain
cargo run -- init-chain

# Start network node and RPC server
cargo run -- start-node 0.0.0.0 8333
cargo run -- start-rpc 8545
```

## ✨ Features

### Core Blockchain
- 🔗 **Genesis Block** - Automated blockchain initialization
- ⛏️ **Proof of Work** - Configurable difficulty mining algorithm
- 🔍 **Block Validation** - Complete integrity checking
- 🌳 **Merkle Trees** - Efficient transaction verification
- 📊 **Chain Analytics** - Comprehensive blockchain statistics

### Storage & Persistence
- 💾 **RocksDB Backend** - High-performance persistent storage
- 🗃️ **Transaction Indexing** - Fast lookup by hash, address, and block
- 💰 **UTXO State** - Unspent transaction output tracking
- 🔄 **Cross-Session Recovery** - Automatic state restoration

### Networking & RPC
- 🌐 **P2P Protocol** - Production-ready peer-to-peer connections
- 🔍 **Peer Discovery** - Automatic network peer discovery
- 🔄 **Block Synchronization** - Real-time blockchain sync
- 🔌 **JSON-RPC API** - Enterprise-grade RESTful API

### Wallet & Security
- 🔐 **HD Wallets** - Hierarchical deterministic wallet support
- 🔑 **Seed Phrases** - BIP39-compatible mnemonic generation
- ✍️ **Digital Signatures** - Production-grade cryptographic validation
- 🛡️ **Enhanced Validation** - Comprehensive transaction verification

## 📋 Table of Contents

- [Installation](#-installation)
- [Usage Examples](#-usage-examples)
- [API Reference](#-api-reference)
- [Production Deployment](#-production-deployment)
- [Development](#-development)
- [Architecture](#-architecture)
- [Troubleshooting](#-troubleshooting)
- [Contributing](#-contributing)

## 🔧 Installation

### Prerequisites
- **Rust** 1.70+ ([Install Rust](https://rustup.rs/))
- **System Dependencies**:

```bash
# Ubuntu/Debian
sudo apt update && sudo apt install -y clang libclang-dev cmake build-essential

# macOS
brew install cmake

# CentOS/RHEL
sudo yum groupinstall -y "Development Tools" && sudo yum install -y clang cmake
```

### Build from Source
```bash
git clone https://github.com/David1425/rust_chain.git
cd rust_chain
cargo build --release
cargo test  # Optional: run tests
```

## 💡 Usage Examples

### Initialize and Start Node
```bash
# Initialize blockchain
cargo run -- init-chain

# View blockchain status
cargo run -- stats

# Start P2P network node
cargo run -- start-node 0.0.0.0 8333

# Start JSON-RPC server
cargo run -- start-rpc 8545
```

### Wallet Operations
```bash
# Generate new wallet address
cargo run -- generate-address

# List all addresses
cargo run -- list-addresses

# Show seed phrase (keep safe!)
cargo run -- show-seed

# Backup wallet
cargo run -- backup-wallet wallet_backup.json
```

### Mining and Transactions
```bash
# Add transaction to mempool
cargo run -- add-transaction alice bob 100

# Mine a block with mempool transactions
cargo run -- mine-mempool

# View mining statistics
cargo run -- mining-stats

# View mempool status
cargo run -- mempool-stats
```

### Network Operations
```bash
# Connect to peer
cargo run -- connect-peer 192.168.1.100 8333

# Discover peers
cargo run -- discover-peers node1.example.com:8333

# Show network statistics
cargo run -- network-stats
```

### Analytics
```bash
# Comprehensive blockchain analysis
cargo run -- analyze-chain

# Block statistics
cargo run -- block-stats 42

# Validate chain integrity
cargo run -- validate-chain
```

## 🔌 API Reference

### JSON-RPC Endpoints

**Base URL**: `http://localhost:8545/rpc`

#### Blockchain Methods
```bash
# Get blockchain information
curl -X POST http://localhost:8545/rpc \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"getblockchaininfo","id":1}'

# Get block count
curl -X POST http://localhost:8545/rpc \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"getblockcount","id":1}'

# Get block by hash
curl -X POST http://localhost:8545/rpc \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"getblock","params":["<block_hash>"],"id":1}'
```

#### Mempool Methods
```bash
# Get mempool information
curl -X POST http://localhost:8545/rpc \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"getmempoolinfo","id":1}'

# Get raw mempool
curl -X POST http://localhost:8545/rpc \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"getrawmempool","id":1}'
```

#### Wallet Methods
```bash
# Generate new address
curl -X POST http://localhost:8545/rpc \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"getnewaddress","id":1}'

# Get balance
curl -X POST http://localhost:8545/rpc \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"getbalance","id":1}'
```

#### Health and Metrics
```bash
# Health check
curl http://localhost:8545/health

# Blockchain metrics
curl http://localhost:8545/metrics
```

### Command Line Interface

| Command | Description |
|---------|-------------|
| `init-chain` | Initialize blockchain with genesis block |
| `stats` | Show blockchain statistics |
| `mine-block` | Mine a block with sample transaction |
| `mine-mempool` | Mine a block using mempool transactions |
| `add-transaction <from> <to> <amount>` | Add transaction to mempool |
| `start-node [addr] [port]` | Start P2P network node |
| `start-rpc [port]` | Start JSON-RPC server |
| `connect-peer <addr> <port>` | Connect to peer |
| `generate-address` | Generate new wallet address |
| `analyze-chain` | Comprehensive blockchain analysis |

## 🏭 Production Deployment

### Quick Production Setup
```bash
# Clone and build
git clone https://github.com/David1425/rust_chain.git
cd rust_chain && cargo build --release

# Create system user and directories
sudo useradd -r -s /bin/false blockchain
sudo mkdir -p /opt/rustchain/{bin,data,logs}
sudo cp target/release/rust_chain /opt/rustchain/bin/
sudo chown -R blockchain:blockchain /opt/rustchain

# Initialize and start
sudo -u blockchain /opt/rustchain/bin/rust_chain init-chain
sudo -u blockchain /opt/rustchain/bin/rust_chain start-node 0.0.0.0 8333 &
sudo -u blockchain /opt/rustchain/bin/rust_chain start-rpc 8545 &
```

### Systemd Service Configuration

**Node Service** (`/etc/systemd/system/rustchain-node.service`):
```ini
[Unit]
Description=RustChain Blockchain Node
After=network.target

[Service]
Type=simple
User=blockchain
ExecStart=/opt/rustchain/bin/rust_chain start-node 0.0.0.0 8333
Restart=always
RestartSec=5

[Install]
WantedBy=multi-user.target
```

**RPC Service** (`/etc/systemd/system/rustchain-rpc.service`):
```ini
[Unit]
Description=RustChain JSON-RPC Server
After=network.target rustchain-node.service

[Service]
Type=simple
User=blockchain
ExecStart=/opt/rustchain/bin/rust_chain start-rpc 8545
Restart=always
RestartSec=5

[Install]
WantedBy=multi-user.target
```

**Enable Services**:
```bash
sudo systemctl daemon-reload
sudo systemctl enable rustchain-node rustchain-rpc
sudo systemctl start rustchain-node rustchain-rpc
```

### Multi-Node Network
```bash
# Bootstrap Node
cargo run -- init-chain
cargo run -- start-node 0.0.0.0 8333
cargo run -- start-rpc 8545

# Additional Nodes
cargo run -- connect-peer <BOOTSTRAP_IP> 8333
cargo run -- start-node 0.0.0.0 8334
cargo run -- start-rpc 8546
```

### Firewall Configuration
```bash
# UFW (Ubuntu/Debian)
sudo ufw allow 8333/tcp comment 'RustChain P2P'
sudo ufw allow 8545/tcp comment 'RustChain RPC'

# Firewalld (CentOS/RHEL)  
sudo firewall-cmd --permanent --add-port=8333/tcp --add-port=8545/tcp
sudo firewall-cmd --reload
```

## 🛠️ Development

### Project Structure
```
src/
├── blockchain/          # Core blockchain logic
│   ├── block.rs        # Block and transaction structures
│   ├── chain.rs        # Blockchain with persistence
│   └── state.rs        # UTXO state management
├── consensus/           # Consensus algorithms
│   ├── pow.rs          # Proof-of-work implementation
│   └── fork_choice.rs  # Fork choice rules
├── storage/             # Data persistence
│   ├── db.rs           # Database abstraction
│   └── block_store.rs  # Persistent block storage
├── network/             # P2P networking
│   ├── server.rs       # Network server
│   ├── protocol.rs     # Network protocol
│   └── discovery.rs    # Peer discovery
├── wallet/              # Wallet functionality
│   └── keychain.rs     # HD wallet implementation
├── mempool/             # Transaction pool
│   ├── pool.rs         # Mempool with persistence
│   └── validator.rs    # Transaction validation
├── rpc/                 # JSON-RPC server
│   ├── server.rs       # RPC server
│   └── handlers.rs     # RPC method handlers
└── cli/                 # Command-line interface
    └── *.rs            # Various command modules
```

### Running Tests
```bash
# Run all tests
cargo test

# Run specific test modules
cargo test --test phase1_2
cargo test --test phase3

# Run with output
cargo test -- --nocapture
```

### Building for Production
```bash
# Optimized release build
cargo build --release

# The binary will be available at:
./target/release/rust_chain
```

## 🏗️ Architecture

### Network Architecture
- **P2P Protocol**: TCP-based custom protocol (default port 8333)
- **JSON-RPC API**: RESTful API with JSON-RPC 2.0 (default port 8545)
- **Data Storage**: RocksDB with optimized configuration
- **Persistence**: Automatic state recovery across restarts

### Data Flow
```
┌─────────────┐    ┌──────────────┐    ┌─────────────┐
│   Wallet    │────│   Mempool    │────│  Blockchain │
└─────────────┘    └──────────────┘    └─────────────┘
       │                   │                   │
       │                   │                   │
   ┌───▼───┐           ┌───▼───┐           ┌───▼───┐
   │  HD   │           │ JSON  │           │ Block │
   │Wallet │           │Files  │           │Store  │
   └───────┘           └───────┘           └───────┘
```

### Security Model
- **Transaction Validation**: Cryptographic signature verification
- **Network Security**: Peer validation and secure connections
- **Data Integrity**: Blockchain validation and corruption detection
- **Access Control**: Role-based CLI and API access

## 🔧 Troubleshooting

### Common Issues

**Database Lock Error**
```
Error: IO error: lock hold by current process
```
**Solution**: Only one instance can access the same database directory. Use separate data directories for multiple nodes.

**Connection Refused**
```
Error connecting to peer: Connection refused
```
**Solution**: Check if the target node is running and firewall settings allow connections.

**Build Errors**
**Solution**: Install required system dependencies:
```bash
sudo apt install clang libclang-dev cmake build-essential  # Ubuntu/Debian
brew install cmake                                          # macOS
```

**Mining Performance**
```
Mining... attempts: 10000
```
**Information**: This shows real proof-of-work in progress. Higher difficulty requires more attempts.

### Debug Mode
```bash
# Enable debug logging
RUST_LOG=debug cargo run -- <command>

# View command help
cargo run -- help
```

## 🤝 Contributing

We welcome contributions! Here's how to get started:

1. **Fork the repository**
2. **Create a feature branch**: `git checkout -b feature/amazing-feature`
3. **Make your changes** and add tests
4. **Run tests**: `cargo test`
5. **Commit changes**: `git commit -m 'Add amazing feature'`
6. **Push to branch**: `git push origin feature/amazing-feature`
7. **Open a Pull Request**

### Development Setup
```bash
git clone https://github.com/your-username/rust_chain.git
cd rust_chain
cargo build
cargo test
```

### Code Style
- Follow Rust best practices and `cargo fmt`
- Add tests for new features
- Update documentation as needed
- Ensure all tests pass before submitting

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [Rust](https://www.rust-lang.org/) 🦀
- Uses [RocksDB](https://rocksdb.org/) for storage
- Inspired by Bitcoin and Ethereum architectures

---

**Happy Blockchain Building! 🚀⛓️**
