#!/bin/bash

# Test script for mempool persistence
echo "🧪 Testing Mempool Persistence"
echo "=============================="

# Clean up any existing mempool file
rm -f mempool.json

echo "Step 1: Add a transaction to mempool"
echo "Command: cargo run -- add-transaction alice bob 100"
echo ""

echo "Step 2: Check pending transactions"
echo "Command: cargo run -- pending-transactions"
echo ""

echo "Step 3: Check if transaction persists in a new CLI session"
echo "Command: cargo run -- pending-transactions (again)"
echo ""

echo "Step 4: Clear mempool and verify persistence"
echo "Command: cargo run -- clear-mempool"
echo "Command: cargo run -- pending-transactions (should be empty)"
echo ""

echo "Instructions:"
echo "============="
echo "1. Run the commands above one by one"
echo "2. Between steps 2 and 3, check if 'mempool.json' file exists"
echo "3. Verify that transactions persist between CLI sessions"
echo ""

echo "Expected results:"
echo "- Transaction added in step 1 should appear in step 2"
echo "- Same transaction should appear in step 3 (persistence test)"
echo "- After step 4, mempool should be empty"
echo "- The file 'mempool.json' should be created/updated after each operation"
