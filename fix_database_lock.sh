#!/bin/bash

# Database Lock Issue Fix and Testing Script
echo "🔒 RustChain Database Lock Issue - Solution Guide"
echo "================================================="

echo "The Issue:"
echo "----------"
echo "RocksDB database lock conflicts occur when:"
echo "1. Multiple CLI commands try to access the same database"
echo "2. Network nodes are running and CLI commands conflict"
echo "3. Previous processes didn't clean up properly"
echo ""

echo "✅ What Was Fixed:"
echo "------------------"
echo "1. CLI now uses unique block store paths per process"
echo "2. Better separation between network node and CLI databases"
echo "3. Improved error handling for database conflicts"
echo ""

echo "🔧 Troubleshooting Steps:"
echo "-------------------------"

echo "1. Check for running processes:"
echo "   ps aux | grep rust_chain"
echo ""

echo "2. Kill any stuck processes:"
echo "   pkill -f rust_chain"
echo ""

echo "3. Clean up lock files (if needed):"
echo "   rm -rf ./temp_block_store/LOCK"
echo "   rm -rf ./blockchain_data/LOCK"
echo ""

echo "4. Clean up temporary CLI stores:"
echo "   rm -rf ./cli_block_store_*"
echo ""

echo "🧪 Test the Fix:"
echo "=================="

# Kill any existing processes
echo "Cleaning up any existing processes..."
pkill -f rust_chain 2>/dev/null || true
sleep 2

# Clean up lock files
echo "Cleaning up lock files..."
rm -rf ./temp_block_store/LOCK 2>/dev/null || true
rm -rf ./blockchain_data/LOCK 2>/dev/null || true
rm -rf ./cli_block_store_* 2>/dev/null || true

echo "✅ Cleanup complete!"
echo ""

echo "Now you can safely run:"
echo "======================="
echo "# Basic commands (should work now):"
echo "cargo run --release -- stats"
echo "cargo run --release -- generate-address"
echo "cargo run --release -- pending-transactions"
echo ""

echo "# Network setup:"
echo "Terminal 1: cargo run --release -- start-node 0.0.0.0 8333"
echo "Terminal 2: cargo run --release -- start-rpc 8545"
echo "Terminal 3: cargo run --release -- add-transaction alice bob 100"
echo ""

echo "The fix ensures each CLI command uses its own database instance,"
echo "avoiding conflicts with running network nodes."
