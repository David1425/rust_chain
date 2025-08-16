#!/bin/bash

# RustChain Local Network Setup Script
# This script demonstrates how to set up a local blockchain network

echo "🦀 RustChain Local Network Setup 🦀"
echo "====================================="

# Check if cargo is available
if ! command -v cargo &> /dev/null; then
    echo "❌ Error: cargo not found. Please install Rust first."
    exit 1
fi

# Build the project
echo "📦 Building RustChain..."
cargo build --release

if [ $? -ne 0 ]; then
    echo "❌ Build failed!"
    exit 1
fi

echo "✅ Build successful!"
echo ""

echo "🔧 Setup Instructions:"
echo "====================="
echo ""

echo "1. Initialize the blockchain (run this once):"
echo "   cargo run --release -- init-chain"
echo ""

echo "2. Start the first node (Bootstrap Node):"
echo "   Terminal 1: cargo run --release -- start-node 0.0.0.0 8333"
echo ""

echo "3. Start RPC server (optional, for API access):"
echo "   Terminal 2: cargo run --release -- start-rpc 8545"
echo ""

echo "4. Start additional nodes:"
echo "   Terminal 3: cargo run --release -- start-node 0.0.0.0 8334"
echo "   Terminal 4: cargo run --release -- connect-peer 127.0.0.1 8333"
echo ""

echo "5. Test the network:"
echo "   - Add transactions: cargo run --release -- add-transaction alice bob 50"
echo "   - Mine blocks: cargo run --release -- mine-mempool"
echo "   - Check network: cargo run --release -- network-stats"
echo "   - View peers: cargo run --release -- show-peers"
echo ""

echo "🌐 Network Ports:"
echo "=================="
echo "• P2P Communication: 8333 (default), 8334, 8335..."
echo "• JSON-RPC API: 8545 (default)"
echo "• Health Check: http://localhost:8545/health"
echo "• Metrics: http://localhost:8545/metrics"
echo ""

echo "🔍 Monitoring Commands:"
echo "======================="
echo "• cargo run --release -- stats           # Blockchain stats"
echo "• cargo run --release -- network-stats   # Network status"
echo "• cargo run --release -- show-peers      # Connected peers"
echo "• cargo run --release -- mempool-stats   # Transaction pool"
echo ""

echo "🚀 Quick Start Example:"
echo "======================="
echo "# Terminal 1:"
echo "cargo run --release -- init-chain"
echo "cargo run --release -- start-node 0.0.0.0 8333"
echo ""
echo "# Terminal 2:"
echo "cargo run --release -- start-rpc 8545"
echo ""
echo "# Terminal 3:"
echo "cargo run --release -- add-transaction alice bob 100"
echo "cargo run --release -- mine-mempool"
echo "cargo run --release -- stats"
echo ""

echo "✅ Setup complete! Follow the instructions above to start your network."
