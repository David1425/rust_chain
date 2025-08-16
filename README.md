# RustChain - A Complete Blockchain Implementation in Rust

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Tests](https://img.shields.io/badge/tests-87%20passing-green.svg)](#testing)

A production-ready, fully-featured blockchain implementation written in Rust, complete with proof-of-work consensus, P2P networking, persistent storage, HD wallets, comprehensive analytics, and enterprise-grade features for real-world deployment.

## Features

### 🔗 Core Blockchain
- **Genesis Block**: Automated blockchain initialization
- **Proof of Work**: Production-grade configurable difficulty mining algorithm
- **Block Validation**: Complete integrity checking and validation
- **Merkle Trees**: Efficient transaction verification
- **Chain Analytics**: Comprehensive blockchain statistics
- **Enhanced Mining**: Real proof-of-work with progress tracking and nonce calculation

### 💾 Storage & Persistence
- **RocksDB Backend**: High-performance persistent storage
- **Block Store**: Efficient block storage and retrieval
- **UTXO State**: Unspent transaction output tracking
- **Database Statistics**: Performance monitoring and compaction
- **Transaction Indexing**: Fast lookup by hash, address, and block
- **Cross-Session Persistence**: Automatic state recovery across restarts
- **Mempool Persistence**: JSON-based transaction pool persistence

### 🌐 Networking & P2P
- **TCP P2P Protocol**: Production-ready peer-to-peer connections
- **Peer Discovery**: Enhanced automatic network peer discovery
- **Block Synchronization**: Real-time blockchain sync across nodes
- **JSON-RPC Server**: Enterprise-grade RESTful API for external integrations
- **Network Statistics**: Advanced connection and performance monitoring
- **Blockchain Sync**: Automatic synchronization with network peers
- **Block Broadcasting**: Efficient block propagation to all connected peers
- **Peer Management**: Enhanced peer tracking with chain height monitoring

### 💰 Wallet & Transactions
- **HD Wallets**: Hierarchical deterministic wallet support with persistence
- **Seed Phrases**: BIP39-compatible mnemonic generation
- **Multiple Addresses**: Generate unlimited wallet addresses
- **Transaction Pool**: Enhanced mempool with persistence and priority ordering
- **Digital Signatures**: Production-grade cryptographic transaction validation
- **Transaction Validation**: Comprehensive signature and format validation
- **Address Validation**: Enhanced address format checking

### ⛏️ Mining & Consensus
- **Mining Pool**: Multi-threaded block mining
- **Difficulty Adjustment**: Dynamic mining difficulty
- **Fork Choice**: Longest chain consensus algorithm
- **Mining Statistics**: Performance tracking and analysis
- **Production Mining**: Real proof-of-work with difficulty-based nonce calculation
- **Mining Progress**: Real-time mining attempt tracking and feedback

### 🚀 Production-Ready Features
- **Enterprise Storage**: RocksDB with transaction indexing and fast lookups
- **Persistent State**: Automatic recovery of blockchain, mempool, and wallet state
- **Network Resilience**: Automatic peer discovery and blockchain synchronization
- **Production RPC**: Enhanced JSON-RPC server with persistence and comprehensive endpoints
- **Enhanced Validation**: Production-grade transaction and signature validation
- **Scalable Architecture**: Designed for multi-node production deployments
- **Monitoring & Analytics**: Comprehensive statistics and health monitoring

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

## Production Deployment

### Enterprise Setup

RustChain is production-ready with enterprise-grade features:

#### **Persistent Production Node**
```bash
# Initialize with persistent storage
cargo run -- init-chain

# Start production RPC server (port 8545)
cargo run -- start-rpc 8545

# Start P2P network node (port 8333)
cargo run -- start-node 0.0.0.0 8333
```

#### **Multi-Node Production Network**
```bash
# Bootstrap Node (Node 1)
cargo run -- init-chain
cargo run -- start-node 0.0.0.0 8333
cargo run -- start-rpc 8545

# Additional Nodes (Node 2, 3, etc.)
cargo run -- connect-peer <BOOTSTRAP_IP> 8333
cargo run -- discover-peers <BOOTSTRAP_IP>:8333
cargo run -- start-node 0.0.0.0 8334
cargo run -- start-rpc 8546
```

### Production Features

#### **Automatic State Recovery**
- ✅ **Blockchain**: Automatically loads from `./blockchain_data/`
- ✅ **Mempool**: Restores pending transactions from `./blockchain_data/mempool.json`
- ✅ **Wallet**: Persistent HD wallet with seed phrase backup
- ✅ **Transaction Index**: Fast lookups by hash, address, and block

#### **Enhanced Networking**
- ✅ **Blockchain Sync**: Automatic synchronization with network peers
- ✅ **Peer Discovery**: Dynamic peer management with health monitoring
- ✅ **Block Broadcasting**: Efficient propagation to all connected nodes
- ✅ **Network Stats**: Real-time monitoring of connections and sync status

#### **Production RPC API**
```bash
# Production RPC server with persistence
curl -X POST http://localhost:8545/rpc \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"getblockcount","id":1}'

# Health monitoring
curl http://localhost:8545/health

# Network metrics
curl http://localhost:8545/metrics
```

#### **Enhanced Security**
- ✅ **Transaction Validation**: Production-grade signature verification
- ✅ **Address Validation**: Enhanced format checking and security
- ✅ **Network Security**: Peer validation and secure connections
- ✅ **Data Integrity**: Comprehensive blockchain validation

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
| `mine-block` | Mine a block with sample transaction (demo mode) |
| `mine-mempool` | Mine a block using mempool transactions (production PoW) |
| `add-block` | Add block using mempool transactions (production mining) |
| `mining-stats` | Show comprehensive mining statistics |
| `fork-stats` | Show fork choice statistics |

### Transaction & Mempool
| Command | Description |
|---------|-------------|
| `add-transaction <from> <to> <amount>` | Add transaction to mempool with enhanced validation |
| `mempool-stats` | Show comprehensive mempool statistics |
| `pending-transactions` | Show all pending transactions with details |
| `clear-mempool` | Clear all transactions from mempool |
| `demo-mempool` | Demonstrate complete mempool workflow |
| `add-block` | Add block using mempool transactions (production mining) |

### Enhanced Transaction Persistence
| Command | Description |
|---------|-------------|
| `get-transaction <hash>` | Get transaction by hash from persistent storage |
| `get-transaction-info <hash>` | Get detailed transaction information with block context |
| `get-address-transactions <addr>` | Get all transactions for a specific address |
| `get-address-balance <addr>` | Get address balance and transaction summary |

### Networking Commands
| Command | Description |
|---------|-------------|
| `start-node [addr] [port]` | Start P2P network node with automatic sync |
| `connect-peer <addr> <port>` | Connect to peer with blockchain synchronization |
| `start-rpc [port]` | Start production JSON-RPC server with persistence |
| `discover-peers [seeds...]` | Enhanced peer discovery using seed nodes |
| `show-peers` | Show connected peers with chain heights |
| `network-stats` | Show comprehensive network statistics |

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
- **Protocol**: Production-grade TCP-based custom protocol
- **Messages**: Handshake, Block, Transaction, Peer discovery, Chain sync
- **Sync**: Automatic blockchain synchronization with peer height tracking
- **Features**: Block broadcasting, peer management, network statistics

### JSON-RPC API
- **Port**: Default 8545 (configurable)
- **Methods**: Complete blockchain RPC methods with persistence
- **Format**: JSON-RPC 2.0 compliant
- **Endpoints**: 
  - `POST /rpc` - JSON-RPC 2.0 endpoint
  - `GET /health` - Health check and status
  - `GET /metrics` - Blockchain and network metrics
- **Features**: CORS enabled, request size limits, persistent state

### Data Storage
- **Engine**: RocksDB with optimized configuration
- **Location**: `./blockchain_data/` (configurable)
- **Features**: Compression, statistics, compaction, transaction indexing
- **Persistence**: 
  - `./blockchain_data/` - Block storage
  - `./blockchain_data/transactions/` - Transaction indexing
  - `./blockchain_data/mempool.json` - Mempool persistence
  - `./blockchain_data/wallet.json` - Wallet persistence

## Development

### Project Structure
```
src/
├── blockchain/          # Core blockchain logic
│   ├── block.rs        # Block and transaction structures
│   ├── chain.rs        # Enhanced blockchain with persistence & indexing
│   ├── genesis.rs      # Genesis block creation
│   └── state.rs        # UTXO state management
├── consensus/           # Consensus algorithms
│   ├── pow.rs          # Production proof-of-work implementation
│   └── fork_choice.rs  # Fork choice rules
├── storage/             # Data persistence
│   ├── db.rs           # Database abstraction layer
│   └── block_store.rs  # Persistent block storage
├── network/             # P2P networking
│   ├── server.rs       # Enhanced network server with sync capabilities
│   ├── protocol.rs     # Network protocol with peer height tracking
│   └── discovery.rs    # Advanced peer discovery and management
├── wallet/              # Wallet functionality
│   ├── keychain.rs     # HD wallet with persistence
│   └── signer.rs       # Transaction signing
├── mempool/             # Transaction pool
│   ├── pool.rs         # Enhanced mempool with persistence
│   └── validator.rs    # Production-grade transaction validation
├── rpc/                 # JSON-RPC server
│   ├── server.rs       # Production RPC server with persistence
│   └── handlers.rs     # Enhanced RPC method handlers
├── cli/                 # Command-line interface
│   ├── advanced_commands.rs    # Transaction persistence commands
│   ├── blockchain_commands.rs  # Enhanced blockchain operations
│   ├── mempool_commands.rs     # Mempool management
│   ├── mining_commands.rs      # Production mining
│   ├── network_commands.rs     # Enhanced networking
│   └── utils.rs        # CLI utilities
└── crypto/              # Cryptographic functions
    ├── hash.rs         # SHA-256 hashing
    ├── keys.rs         # Key management
    └── signature.rs    # Digital signatures
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

## Production Features & Capabilities

### 🏭 Enterprise-Grade Components

#### **Enhanced Mining System**
- **Production PoW**: Real proof-of-work with configurable difficulty (default: 4)
- **Mining Progress**: Real-time attempt tracking and performance metrics
- **Mempool Integration**: Automatic transaction selection from persistent mempool
- **Block Broadcasting**: Efficient propagation to all network peers

#### **Advanced Persistence Layer**
- **RocksDB Storage**: High-performance persistent blockchain storage
- **Transaction Indexing**: Fast lookups by hash, sender, recipient, and block
- **Mempool Persistence**: JSON-based transaction pool state preservation
- **Wallet Persistence**: HD wallet state with automatic recovery
- **Cross-Session Recovery**: Seamless state restoration across restarts

#### **Production Networking**
- **Blockchain Synchronization**: Automatic sync with network peers
- **Peer Management**: Enhanced peer discovery with chain height tracking
- **Network Statistics**: Real-time monitoring of connections and sync status
- **Block Broadcasting**: Efficient block propagation to all connected nodes
- **Peer Health Monitoring**: Automatic cleanup of stale connections

#### **Enhanced Transaction System**
- **Production Validation**: Comprehensive signature and format validation
- **Address Verification**: Enhanced address format checking and security
- **Transaction Persistence**: Permanent storage with fast retrieval
- **UTXO Management**: Complete unspent transaction output tracking
- **Balance Calculation**: Real-time address balance computation

#### **Monitoring & Analytics**
- **Network Metrics**: Connection statistics, peer heights, sync status
- **Blockchain Analytics**: Block statistics, transaction analysis, chain validation
- **Performance Monitoring**: Mining statistics, database performance, memory usage
- **Health Checks**: System status, data integrity, network connectivity

### 🔧 Production Deployment Modes

#### **Single Node (Development/Testing)**
```bash
# Initialize and start complete node
cargo run -- init-chain
cargo run -- start-rpc 8545    # Production RPC with persistence
cargo run -- start-node 0.0.0.0 8333  # P2P networking
```

#### **Multi-Node Network (Production)**
```bash
# Bootstrap Node
cargo run -- init-chain
cargo run -- start-rpc 8545
cargo run -- start-node 0.0.0.0 8333

# Additional Nodes (automatic sync)
cargo run -- connect-peer <bootstrap_ip> 8333
cargo run -- start-rpc 8546
cargo run -- start-node 0.0.0.0 8334
```

#### **Production API Server**
```bash
# Start persistent RPC server
cargo run -- start-rpc 8545

# Available endpoints:
# POST /rpc          - JSON-RPC 2.0 API
# GET  /health       - System health check  
# GET  /metrics      - Blockchain metrics
```

### 🛡️ Security & Validation

#### **Transaction Security**
- **Signature Validation**: Production-grade cryptographic verification
- **Format Validation**: Comprehensive transaction and address format checking
- **Duplicate Prevention**: Automatic detection and rejection of duplicate transactions
- **Balance Verification**: Real-time UTXO validation before transaction acceptance

#### **Network Security**
- **Peer Validation**: Secure handshake and version compatibility checking
- **Protocol Validation**: Message format and size limits
- **Connection Management**: Secure peer-to-peer communication
- **Data Integrity**: Blockchain validation and corruption detection

## Troubleshooting

### Common Issues

1. **Database Lock Error**:
   ```
   Error: IO error: lock hold by current process
   ```
   **Solution**: This is expected behavior when multiple instances try to access the same database. Each node should use a separate data directory, or ensure only one instance accesses `./blockchain_data/` at a time.

2. **Connection Refused**:
   ```
   Error connecting to peer: Connection refused
   ```
   **Solution**: Check if the target node is running and accessible. Verify firewall settings and network connectivity.

3. **Mempool Persistence Warning**:
   ```
   Warning: Could not load mempool state
   ```
   **Solution**: This is normal for first startup. The mempool will be created automatically.

4. **Build Errors**:
   **Solution**: Ensure all system dependencies are installed:
   ```bash
   # Ubuntu/Debian
   sudo apt install clang libclang-dev cmake build-essential
   
   # macOS  
   brew install cmake
   ```

5. **Mining Performance**:
   ```
   Mining... attempts: 10000
   ```
   **Information**: This shows real proof-of-work in progress. Higher difficulty requires more attempts.

6. **Network Sync Issues**:
   ```
   Warning: Blockchain sync failed
   ```
   **Solution**: Ensure peers are reachable and running compatible versions. Check network connectivity.

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
