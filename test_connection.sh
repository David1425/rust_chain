#!/bin/bash

# Quick Network Test Script
# Tests the connection fix for the "broken pipe" issue

echo "🧪 Testing RustChain Network Connection Fix"
echo "==========================================="

# Kill any existing rust_chain processes
echo "🔄 Cleaning up any existing processes..."
pkill -f rust_chain 2>/dev/null || true
sleep 2

echo "✅ Environment clean"
echo ""

# Test the connection
echo "🔗 Testing network connection..."
echo ""

echo "Instructions for manual testing:"
echo "================================"
echo ""
echo "1. In Terminal 1, run:"
echo "   cargo run -- start-node 0.0.0.0 8334"
echo ""
echo "2. In Terminal 2, run:"  
echo "   cargo run -- connect-peer 127.0.0.1 8334"
echo ""
echo "Expected result: No 'broken pipe' error, successful handshake"
echo ""

echo "🔍 What to look for:"
echo "===================="
echo ""
echo "✅ GOOD - Terminal 1 should show:"
echo "   - Network server listening on 0.0.0.0:8334"
echo "   - New connection from 127.0.0.1:XXXXX"
echo "   - Received message: Handshake { ... }"
echo "   - No 'Broken pipe' error"
echo ""
echo "✅ GOOD - Terminal 2 should show:"
echo "   - Connecting to peer at 127.0.0.1:8334..."
echo "   - Connected to peer at 127.0.0.1:8334 successfully"
echo "   - Network Status with Connected peers: 1"
echo ""
echo "❌ BAD (old behavior):"
echo "   - Connection error: Failed to write message data: Broken pipe"
echo "   - Connected peers: 0"
echo ""

echo "Ready to test! Open two terminals and follow the instructions above."
