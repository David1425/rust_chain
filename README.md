# RustChain - A Complete Blockchain Implementation in Rust

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Tests](https://img.shields.io/badge/tests-87%20passing-green.svg)](#testing)

A fully-featured blockchain implementation written in Rust, complete with proof-of-work consensus, P2P networking, persistent storage, HD wallets, and comprehensive analytics.

## Features

### 🔗 Core Blockchain
- **Genesis Block**: Automated blockchain initialization
- **Proof of Work**: Configurable difficulty mining algorithm
- **Block Validation**: Complete integrity checking and validation
- **Merkle Trees**: Efficient transaction verification
- **Chain Analytics**: Comprehensive blockchain statistics

### 💾 Storage & Persistence
- **RocksDB Backend**: High-performance persistent storage
- **Block Store**: Efficient block storage and retrieval
- **UTXO State**: Unspent transaction output tracking
- **Database Statistics**: Performance monitoring and compaction

### 🌐 Networking & P2P
- **TCP P2P Protocol**: Direct peer-to-peer connections
- **Peer Discovery**: Automatic network peer discovery
- **Block Synchronization**: Real-time blockchain sync across nodes
- **JSON-RPC Server**: RESTful API for external integrations
- **Network Statistics**: Connection and performance monitoring

### 💰 Wallet & Transactions
- **HD Wallets**: Hierarchical deterministic wallet support
- **Seed Phrases**: BIP39-compatible mnemonic generation
- **Multiple Addresses**: Generate unlimited wallet addresses
- **Transaction Pool**: Mempool with priority ordering
- **Digital Signatures**: Cryptographic transaction signing

### ⛏️ Mining & Consensus
- **Mining Pool**: Multi-threaded block mining
- **Difficulty Adjustment**: Dynamic mining difficulty
- **Fork Choice**: Longest chain consensus algorithm
- **Mining Statistics**: Performance tracking and analysis

## Quick Start

### Prerequisites

- **Rust** 1.70+ ([Install Rust](https://rustup.rs/))
- **System Dependencies**:
  ```bash
  # Ubuntu/Debian
  sudo apt update && sudo apt install -y clang libclang-dev cmake build-essential
  
  # macOS
  brew install cmake
  
  # Windows
  # Install Visual Studio Build Tools and CMake
  ```

### Installation

1. **Clone the repository**:
   ```bash
   git clone https://github.com/David1425/rust_chain.git
   cd rust_chain
   ```

2. **Build the project**:
   ```bash
   cargo build --release
   ```

3. **Run tests** (optional):
   ```bash
   cargo test
   ```

### Initialize Your Blockchain

```bash
# Initialize the blockchain with genesis block
cargo run -- init-chain

# View blockchain status
cargo run -- stats
```

## Usage Guide

### Starting a Blockchain Node

#### Single Node (Local Development)
```bash
# Start the main blockchain node
cargo run -- start-node

# In another terminal, start the RPC server
cargo run -- start-rpc 8545
```

#### Multi-Node Network
```bash
# Node 1 (Bootstrap node)
cargo run -- start-node 0.0.0.0 8333

# Node 2 (Connect to bootstrap)
cargo run -- init-chain
cargo run -- connect-peer <NODE1_IP> 8333
cargo run -- start-node 0.0.0.0 8334
```

### Blockchain Operations

#### Mining
```bash
# Mine a block with sample transaction
cargo run -- mine-block

# Mine using transactions from mempool
cargo run -- mine-mempool

# View mining statistics
cargo run -- mining-stats
```

#### Transactions
```bash
# Add transaction to mempool
cargo run -- add-transaction alice bob 100

# View pending transactions
cargo run -- pending-transactions

# View mempool statistics
cargo run -- mempool-stats
```

#### Blockchain Information
```bash
# Show blockchain statistics
cargo run -- stats

# Show all blocks
cargo run -- show-blocks

# Analyze blockchain
cargo run -- analyze-chain

# Validate blockchain integrity
cargo run -- validate-chain
```

### Wallet Management

#### Basic Wallet Operations
```bash
# Generate new wallet address
cargo run -- generate-address

# List all addresses
cargo run -- list-addresses

# Show wallet statistics
cargo run -- wallet-stats
```

#### Backup & Recovery
```bash
# Show seed phrase (keep safe!)
cargo run -- show-seed

# Backup wallet to file
cargo run -- backup-wallet wallet_backup.json

# Restore wallet from seed phrase
cargo run -- restore-wallet "your twelve word seed phrase here"
```

### Network Management

#### Peer Connections
```bash
# Connect to a specific peer
cargo run -- connect-peer 192.168.1.100 8333

# Discover peers using seed nodes
cargo run -- discover-peers node1.example.com:8333 node2.example.com:8333

# Show connected peers
cargo run -- show-peers

# Show network statistics
cargo run -- network-stats
```

### Analytics & Monitoring

```bash
# Comprehensive blockchain analysis
cargo run -- analyze-chain

# Block statistics (specific block)
cargo run -- block-stats 42

# Transaction statistics
cargo run -- transaction-stats

# Get specific block by hash
cargo run -- get-block <block_hash>
```

## Complete Command Reference

### Blockchain Commands
| Command | Description |
|---------|-------------|
| `init-chain` | Initialize blockchain with genesis block |
| `show-blocks` | Display all blocks in the chain |
| `stats` | Show blockchain statistics |
| `get-block <hash>` | Get specific block by hash |

### Mining Commands
| Command | Description |
|---------|-------------|
| `mine-block` | Mine a block with sample transaction |
| `mine-mempool` | Mine a block using mempool transactions |
| `mining-stats` | Show mining statistics |
| `fork-stats` | Show fork choice statistics |

### Transaction & Mempool
| Command | Description |
|---------|-------------|
| `add-transaction <from> <to> <amount>` | Add transaction to mempool |
| `mempool-stats` | Show mempool statistics |
| `pending-transactions` | Show all pending transactions |
| `clear-mempool` | Clear all transactions from mempool |

### Networking Commands
| Command | Description |
|---------|-------------|
| `start-node [addr] [port]` | Start P2P network node |
| `connect-peer <addr> <port>` | Connect to a peer |
| `start-rpc [port]` | Start JSON-RPC server |
| `discover-peers [seeds...]` | Discover peers using seed nodes |
| `show-peers` | Show connected peers |
| `network-stats` | Show network statistics |

### Wallet Commands
| Command | Description |
|---------|-------------|
| `generate-address` | Generate new wallet address |
| `list-addresses` | List all wallet addresses |
| `show-seed` | Show wallet seed phrase |
| `restore-wallet "<phrase>"` | Restore wallet from seed |
| `wallet-stats` | Show wallet statistics |
| `backup-wallet [path]` | Backup wallet to file |

### Analytics Commands
| Command | Description |
|---------|-------------|
| `analyze-chain` | Comprehensive blockchain analysis |
| `block-stats [height]` | Detailed block statistics |
| `transaction-stats` | Transaction statistics |
| `validate-chain` | Validate blockchain integrity |

## Network Architecture

### P2P Protocol
- **Port**: Default 8333 (configurable)
- **Protocol**: TCP-based custom protocol
- **Messages**: Handshake, Block, Transaction, Peer discovery
- **Sync**: Automatic blockchain synchronization

### JSON-RPC API
- **Port**: Default 8545 (configurable)
- **Methods**: Standard blockchain RPC methods
- **Format**: JSON-RPC 2.0 compliant

### Data Storage
- **Engine**: RocksDB
- **Location**: `./blockchain_data/` (configurable)
- **Features**: Compression, statistics, compaction

## Development

### Project Structure
```
src/
├── blockchain/          # Core blockchain logic
│   ├── block.rs        # Block and transaction structures
│   ├── chain.rs        # Blockchain implementation
│   ├── genesis.rs      # Genesis block creation
│   └── state.rs        # UTXO state management
├── consensus/           # Consensus algorithms
│   ├── pow.rs          # Proof of Work implementation
│   └── fork_choice.rs  # Fork choice rules
├── storage/             # Data persistence
│   ├── db.rs           # Database abstraction
│   └── block_store.rs  # Block storage
├── network/             # P2P networking
│   ├── server.rs       # Network server
│   ├── protocol.rs     # Network protocol
│   └── discovery.rs    # Peer discovery
├── wallet/              # Wallet functionality
│   ├── keychain.rs     # HD wallet implementation
│   └── signer.rs       # Transaction signing
├── mempool/             # Transaction pool
│   ├── pool.rs         # Mempool implementation
│   └── validator.rs    # Transaction validation
├── rpc/                 # JSON-RPC server
├── cli/                 # Command-line interface
└── crypto/              # Cryptographic functions
```

### Running Tests
```bash
# Run all tests
cargo test

# Run specific test suite
cargo test --test phase1_2
cargo test --test phase3
cargo test --test phase8

# Run with output
cargo test -- --nocapture
```

### Building for Production
```bash
# Optimized release build
cargo build --release

# Run the optimized binary
./target/release/rust_chain --help
```

## Configuration

### Environment Variables
- `RUST_LOG`: Set logging level (`debug`, `info`, `warn`, `error`)
- `BLOCKCHAIN_DATA_PATH`: Custom database path

### Default Ports
- **P2P Network**: 8333
- **JSON-RPC**: 8545

## Troubleshooting

### Common Issues

1. **Database Lock Error**:
   ```
   Error: IO error: lock hold by current process
   ```
   **Solution**: Ensure no other instance is running, or use different data paths for multiple nodes.

2. **Connection Refused**:
   ```
   Error connecting to peer: Connection refused
   ```
   **Solution**: Check if the target node is running and firewall settings.

3. **Build Errors**:
   **Solution**: Ensure all system dependencies are installed (clang, cmake, build-essential).

### Getting Help
- Check the command help: `cargo run -- help`
- Run with debug logging: `RUST_LOG=debug cargo run -- <command>`
- Review test files in `tests/` for usage examples

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- Uses [RocksDB](https://rocksdb.org/) for storage
- Inspired by Bitcoin and Ethereum architectures

---

**Happy Blockchain Building! 🚀**
