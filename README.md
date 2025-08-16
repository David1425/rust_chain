# RustChain - A Complete Blockchain Implementation in Rust

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Tests](https://img.shields.io/badge/tests-87%20passing-green.svg)](#testing)

A production-ready, fully-featured blockchain implementation written in Rust, complete with proof-of-work consensus, P2P networking, persistent storage, HD wallets, comprehensive analytics, and enterprise-grade features for real-world deployment.

> **🏭 Production Ready**: This blockchain includes comprehensive production deployment guides, systemd service configurations, security hardening, monitoring setup, client integration examples, and automated backup strategies. See the [Production Deployment Guide](#-production-deployment-guide) for complete setup instructions.

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

## Table of Contents

- [Quick Start](#quick-start)
- [Production Deployment Guide](#-production-deployment-guide)
  - [Prerequisites for Production](#prerequisites-for-production)
  - [Production Build & Installation](#production-build--installation)
  - [Production Network Deployment](#production-network-deployment)
  - [Systemd Service Configuration](#systemd-service-configuration)
  - [Firewall Configuration](#firewall-configuration)
  - [SSL/TLS Configuration](#ssltls-configuration-recommended-for-production)
- [Client Setup & Integration Guide](#️-client-setup--integration-guide)
  - [Command Line Client](#command-line-client)
  - [HTTP API Client Examples](#http-api-client-examples)
  - [Wallet Integration](#wallet-integration)
  - [Monitoring and Analytics](#monitoring-and-analytics)
  - [Backup and Recovery](#backup-and-recovery)
  - [Performance Tuning](#performance-tuning)
- [Usage Guide](#usage-guide)
- [Network Architecture](#network-architecture)
- [Development](#development)
- [API Reference](#api-reference)
- [Testing](#testing)
- [Contributing](#contributing)

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

## 🏭 Production Deployment Guide

### Prerequisites for Production

#### System Requirements
- **Operating System**: Ubuntu 20.04+, CentOS 8+, or similar Linux distribution
- **CPU**: 4+ cores (8+ recommended for mining nodes)
- **RAM**: 8GB minimum (16GB+ recommended)
- **Storage**: SSD with 100GB+ available space
- **Network**: Static IP address with stable internet connection

#### System Dependencies
```bash
# Ubuntu/Debian
sudo apt update && sudo apt install -y clang libclang-dev cmake build-essential git curl

# CentOS/RHEL
sudo yum groupinstall -y "Development Tools"
sudo yum install -y clang cmake git curl

# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

### Production Build & Installation

#### 1. Clone and Build
```bash
# Clone repository
git clone https://github.com/David1425/rust_chain.git
cd rust_chain

# Build optimized release binary
cargo build --release

# The binary will be available at: target/release/rust_chain
```

#### 2. Create System User and Directories
```bash
# Create blockchain user
sudo useradd -r -s /bin/false blockchain

# Create directories
sudo mkdir -p /opt/rustchain/{bin,data,logs,config}
sudo mkdir -p /var/log/rustchain

# Copy binary
sudo cp target/release/rust_chain /opt/rustchain/bin/
sudo chmod +x /opt/rustchain/bin/rust_chain

# Set ownership
sudo chown -R blockchain:blockchain /opt/rustchain
sudo chown -R blockchain:blockchain /var/log/rustchain
```

#### 3. Create Shell Alias (Optional)
```bash
# Add to ~/.bashrc for easy access
echo 'alias rustchain="/opt/rustchain/bin/rust_chain"' >> ~/.bashrc
source ~/.bashrc
```

### Production Network Deployment

#### Single Node Setup (Development/Testing)
```bash
# Initialize blockchain
sudo -u blockchain /opt/rustchain/bin/rust_chain init-chain

# Start services manually (for testing)
sudo -u blockchain /opt/rustchain/bin/rust_chain start-node 0.0.0.0 8333 &
sudo -u blockchain /opt/rustchain/bin/rust_chain start-rpc 8545 &
```

#### Multi-Node Production Network

**Bootstrap Node (Primary Node)**
```bash
# Initialize the network
sudo -u blockchain /opt/rustchain/bin/rust_chain init-chain

# Configure as bootstrap node
sudo -u blockchain /opt/rustchain/bin/rust_chain start-node 0.0.0.0 8333 &
sudo -u blockchain /opt/rustchain/bin/rust_chain start-rpc 8545 &
```

**Additional Nodes**
```bash
# Connect to bootstrap node
sudo -u blockchain /opt/rustchain/bin/rust_chain connect-peer <BOOTSTRAP_IP> 8333
sudo -u blockchain /opt/rustchain/bin/rust_chain discover-peers <BOOTSTRAP_IP>:8333

# Start local services on different ports
sudo -u blockchain /opt/rustchain/bin/rust_chain start-node 0.0.0.0 8334 &
sudo -u blockchain /opt/rustchain/bin/rust_chain start-rpc 8546 &
```

### Systemd Service Configuration

#### 1. Create Network Node Service
Create `/etc/systemd/system/rustchain-node.service`:
```ini
[Unit]
Description=RustChain Blockchain Node
After=network.target
Wants=network.target

[Service]
Type=simple
User=blockchain
Group=blockchain
WorkingDirectory=/opt/rustchain
ExecStart=/opt/rustchain/bin/rust_chain start-node 0.0.0.0 8333
ExecReload=/bin/kill -HUP $MAINPID
Restart=always
RestartSec=5
TimeoutStopSec=30

# Environment variables
Environment=RUST_LOG=info
Environment=BLOCKCHAIN_DATA_PATH=/opt/rustchain/data

# Security settings
NoNewPrivileges=true
ProtectSystem=strict
ProtectHome=true
ReadWritePaths=/opt/rustchain/data /var/log/rustchain

# Logging
StandardOutput=journal
StandardError=journal
SyslogIdentifier=rustchain-node

[Install]
WantedBy=multi-user.target
```

#### 2. Create RPC Service
Create `/etc/systemd/system/rustchain-rpc.service`:
```ini
[Unit]
Description=RustChain JSON-RPC Server
After=network.target rustchain-node.service
Wants=network.target

[Service]
Type=simple
User=blockchain
Group=blockchain
WorkingDirectory=/opt/rustchain
ExecStart=/opt/rustchain/bin/rust_chain start-rpc 8545
ExecReload=/bin/kill -HUP $MAINPID
Restart=always
RestartSec=5
TimeoutStopSec=30

# Environment variables
Environment=RUST_LOG=info
Environment=BLOCKCHAIN_DATA_PATH=/opt/rustchain/data

# Security settings
NoNewPrivileges=true
ProtectSystem=strict
ProtectHome=true
ReadWritePaths=/opt/rustchain/data /var/log/rustchain

# Logging
StandardOutput=journal
StandardError=journal
SyslogIdentifier=rustchain-rpc

[Install]
WantedBy=multi-user.target
```

#### 3. Enable and Start Services
```bash
# Reload systemd configuration
sudo systemctl daemon-reload

# Enable services to start on boot
sudo systemctl enable rustchain-node rustchain-rpc

# Start services
sudo systemctl start rustchain-node
sudo systemctl start rustchain-rpc

# Check status
sudo systemctl status rustchain-node rustchain-rpc
```

### Firewall Configuration

```bash
# UFW (Ubuntu/Debian)
sudo ufw allow 8333/tcp comment 'RustChain P2P'
sudo ufw allow 8545/tcp comment 'RustChain RPC'
sudo ufw enable

# Firewalld (CentOS/RHEL)
sudo firewall-cmd --permanent --add-port=8333/tcp --add-port=8545/tcp
sudo firewall-cmd --reload

# For production, consider restricting RPC access:
# sudo ufw allow from <trusted_ip> to any port 8545
```

### SSL/TLS Configuration (Recommended for Production)

#### Using nginx as reverse proxy
Create `/etc/nginx/sites-available/rustchain`:
```nginx
server {
    listen 443 ssl http2;
    server_name your-blockchain-domain.com;

    ssl_certificate /path/to/your/certificate.crt;
    ssl_certificate_key /path/to/your/private.key;

    # SSL configuration
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers ECDHE-RSA-AES128-GCM-SHA256:ECDHE-RSA-AES256-GCM-SHA384;
    ssl_prefer_server_ciphers off;

    # Proxy to RustChain RPC
    location /rpc {
        proxy_pass http://127.0.0.1:8545/rpc;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }

    # Health and metrics endpoints
    location /health {
        proxy_pass http://127.0.0.1:8545/health;
    }

    location /metrics {
        proxy_pass http://127.0.0.1:8545/metrics;
        # Consider restricting access
        allow 10.0.0.0/8;
        deny all;
    }
}
```

### Environment Variables

Create `/opt/rustchain/config/environment`:
```bash
# Logging level (debug, info, warn, error)
RUST_LOG=info

# Custom data directory
BLOCKCHAIN_DATA_PATH=/opt/rustchain/data

# Network configuration
RUSTCHAIN_LISTEN_PORT=8333
RUSTCHAIN_RPC_PORT=8545
RUSTCHAIN_MAX_PEERS=50

# Performance tuning
RUSTCHAIN_CACHE_SIZE=512
RUSTCHAIN_WRITE_BUFFER_SIZE=64
```

Source in systemd services:
```ini
EnvironmentFile=/opt/rustchain/config/environment
```

## 🖥️ Client Setup & Integration Guide

### Command Line Client

#### Installation
```bash
# Option 1: Build from source
git clone https://github.com/David1425/rust_chain.git
cd rust_chain
cargo build --release

# Option 2: Use pre-built binary (if available)
wget https://github.com/David1425/rust_chain/releases/latest/download/rust_chain
chmod +x rust_chain
sudo mv rust_chain /usr/local/bin/rustchain
```

#### Basic Usage
```bash
# Connect to a running node
export RUSTCHAIN_RPC_URL="http://your-node-ip:8545"

# Initialize local blockchain (for development)
rustchain init-chain

# Check blockchain status
rustchain stats

# Show blockchain information
rustchain chain-info

# View all blocks
rustchain show-blocks
```

### HTTP API Client Examples

#### cURL Examples
```bash
# Get blockchain information
curl -X POST http://your-node:8545/rpc \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"getblockchaininfo","id":1}'

# Get block count
curl -X POST http://your-node:8545/rpc \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"getblockcount","id":1}'

# Get block by hash
curl -X POST http://your-node:8545/rpc \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"getblock","params":["<block_hash>"],"id":1}'

# Get mempool information
curl -X POST http://your-node:8545/rpc \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"getmempoolinfo","id":1}'

# Health check
curl http://your-node:8545/health

# Get metrics
curl http://your-node:8545/metrics
```

#### Python Client Example
```python
import requests
import json

class RustChainClient:
    def __init__(self, rpc_url="http://localhost:8545"):
        self.rpc_url = f"{rpc_url}/rpc"
        self.session = requests.Session()
        self.id_counter = 0

    def _call(self, method, params=None):
        self.id_counter += 1
        payload = {
            "jsonrpc": "2.0",
            "method": method,
            "params": params or [],
            "id": self.id_counter
        }
        
        response = self.session.post(
            self.rpc_url,
            json=payload,
            headers={"Content-Type": "application/json"}
        )
        
        result = response.json()
        if "error" in result:
            raise Exception(f"RPC Error: {result['error']}")
        return result.get("result")

    def get_blockchain_info(self):
        return self._call("getblockchaininfo")

    def get_block_count(self):
        return self._call("getblockcount")

    def get_block(self, block_hash):
        return self._call("getblock", [block_hash])

    def get_mempool_info(self):
        return self._call("getmempoolinfo")

    def get_balance(self, address=None):
        return self._call("getbalance", [address] if address else [])

# Usage example
client = RustChainClient("http://your-node:8545")
info = client.get_blockchain_info()
print(f"Blockchain height: {info.get('blocks', 0)}")
```

#### JavaScript/Node.js Client Example
```javascript
const axios = require('axios');

class RustChainClient {
    constructor(rpcUrl = 'http://localhost:8545') {
        this.rpcUrl = `${rpcUrl}/rpc`;
        this.idCounter = 0;
    }

    async call(method, params = []) {
        this.idCounter++;
        
        const payload = {
            jsonrpc: '2.0',
            method: method,
            params: params,
            id: this.idCounter
        };

        try {
            const response = await axios.post(this.rpcUrl, payload, {
                headers: { 'Content-Type': 'application/json' }
            });

            if (response.data.error) {
                throw new Error(`RPC Error: ${JSON.stringify(response.data.error)}`);
            }

            return response.data.result;
        } catch (error) {
            throw new Error(`Request failed: ${error.message}`);
        }
    }

    async getBlockchainInfo() {
        return await this.call('getblockchaininfo');
    }

    async getBlockCount() {
        return await this.call('getblockcount');
    }

    async getBlock(blockHash) {
        return await this.call('getblock', [blockHash]);
    }

    async getMempoolInfo() {
        return await this.call('getmempoolinfo');
    }

    async getBalance(address = null) {
        return await this.call('getbalance', address ? [address] : []);
    }
}

// Usage example
async function main() {
    const client = new RustChainClient('http://your-node:8545');
    
    try {
        const info = await client.getBlockchainInfo();
        console.log(`Blockchain height: ${info.blocks || 0}`);
        
        const mempool = await client.getMempoolInfo();
        console.log(`Pending transactions: ${mempool.size || 0}`);
    } catch (error) {
        console.error('Error:', error.message);
    }
}

main();
```

### Wallet Integration

#### Create and Manage Wallets
```bash
# Generate new wallet address
rustchain generate-address

# List all addresses
rustchain list-addresses

# Show seed phrase (keep secure!)
rustchain show-seed

# Restore wallet from seed phrase
rustchain restore-wallet "your twelve word seed phrase here"

# Check wallet balance
rustchain wallet-stats

# Backup wallet
rustchain backup-wallet /secure/path/wallet_backup.json
```

#### Transaction Management
```bash
# Add transaction to mempool
rustchain add-transaction <from_address> <to_address> <amount>

# View pending transactions
rustchain pending-transactions

# Check mempool status
rustchain mempool-stats

# Mine a block with pending transactions
rustchain mine-mempool
```

### Monitoring and Analytics

#### Health Monitoring
```bash
# Check node health
curl http://your-node:8545/health

# Get detailed metrics
curl http://your-node:8545/metrics

# View logs
sudo journalctl -u rustchain-node -f
sudo journalctl -u rustchain-rpc -f
```

#### Blockchain Analytics
```bash
# Comprehensive chain analysis
rustchain analyze-chain

# Block-specific statistics
rustchain block-stats <block_height>

# Transaction statistics
rustchain transaction-stats

# Validate chain integrity
rustchain validate-chain

# Network statistics
rustchain network-stats
```

### Backup and Recovery

#### Automated Backup Script
Create `/opt/rustchain/bin/backup.sh`:
```bash
#!/bin/bash

BACKUP_DIR="/opt/rustchain/backups"
DATE=$(date +%Y%m%d_%H%M%S)
DATA_DIR="/opt/rustchain/data"

mkdir -p "$BACKUP_DIR"

# Stop services
systemctl stop rustchain-rpc rustchain-node

# Create backup
tar -czf "$BACKUP_DIR/blockchain_backup_$DATE.tar.gz" -C "$DATA_DIR" .

# Restart services
systemctl start rustchain-node rustchain-rpc

# Keep only last 7 days of backups
find "$BACKUP_DIR" -name "blockchain_backup_*.tar.gz" -mtime +7 -delete

echo "Backup completed: blockchain_backup_$DATE.tar.gz"
```

Make executable and add to cron:
```bash
chmod +x /opt/rustchain/bin/backup.sh

# Add to crontab (daily backup at 2 AM)
echo "0 2 * * * /opt/rustchain/bin/backup.sh" | sudo -u blockchain crontab -
```

#### Recovery Process
```bash
# Stop services
sudo systemctl stop rustchain-rpc rustchain-node

# Restore from backup
cd /opt/rustchain/data
sudo -u blockchain tar -xzf /opt/rustchain/backups/blockchain_backup_YYYYMMDD_HHMMSS.tar.gz

# Restart services
sudo systemctl start rustchain-node rustchain-rpc

# Verify integrity
rustchain validate-chain
```

### Performance Tuning

#### System Optimization
```bash
# Increase file descriptor limits
echo "blockchain soft nofile 65536" | sudo tee -a /etc/security/limits.conf
echo "blockchain hard nofile 65536" | sudo tee -a /etc/security/limits.conf

# TCP optimization for P2P networking
cat >> /etc/sysctl.conf << EOF
net.core.rmem_max = 16777216
net.core.wmem_max = 16777216
net.ipv4.tcp_rmem = 4096 65536 16777216
net.ipv4.tcp_wmem = 4096 65536 16777216
net.core.netdev_max_backlog = 5000
EOF

sudo sysctl -p
```

#### RocksDB Tuning
```bash
# Environment variables for performance
export ROCKSDB_CACHE_SIZE=512000000  # 512MB cache
export ROCKSDB_WRITE_BUFFER_SIZE=67108864  # 64MB write buffer
export ROCKSDB_MAX_WRITE_BUFFER_NUMBER=3
export ROCKSDB_COMPRESSION_TYPE=lz4
```

### Production Checklist

#### Pre-Deployment
- [ ] **Hardware**: Adequate CPU, RAM, and SSD storage verified
- [ ] **Network**: Static IP configured, firewall rules set
- [ ] **Security**: Non-root user created, systemd services configured
- [ ] **SSL/TLS**: HTTPS configured for RPC endpoints (if external access)
- [ ] **Monitoring**: Health checks and log monitoring configured
- [ ] **Backup**: Automated backup strategy implemented

#### Post-Deployment
- [ ] **Services**: All systemd services running and enabled
- [ ] **Connectivity**: P2P network connections established
- [ ] **API**: RPC endpoints responding correctly
- [ ] **Synchronization**: Blockchain syncing with network peers
- [ ] **Monitoring**: Metrics collection and alerting active
- [ ] **Documentation**: Network configuration documented

#### Maintenance
- [ ] **Updates**: Process for updating blockchain software
- [ ] **Backups**: Regular backup verification
- [ ] **Monitoring**: Regular health check reviews
- [ ] **Security**: Regular security audit and updates
- [ ] **Performance**: Regular performance monitoring and optimization

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
