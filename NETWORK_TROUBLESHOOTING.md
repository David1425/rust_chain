# RustChain Network Troubleshooting Guide

## Common Issues and Solutions

### 1. "Broken pipe" Error During Connection

**Problem**: Connection established but breaks during handshake
```
Connection error: Connection failed: Failed to write message data: Broken pipe (os error 32)
```

**Solution**: This was fixed in the latest version. The issue was that the client connection was closing before the server could send the handshake response.

**What was changed**:
- Modified `connect_to_peer()` to wait for handshake response
- Added proper connection timeout handling
- Improved peer state management

### 2. "No peers available for sync" Warning

**Problem**: Connection appears successful but sync fails
```
Warning: Blockchain sync failed: Connection failed: No peers available for sync
```

**Root cause**: The CLI creates a temporary server instance for connections, which doesn't maintain persistent peer state.

**Workaround**: This is expected behavior with the current architecture. The connection is established, but the temporary server instance is cleaned up after the command completes.

### 3. Connected Peers Shows 0

**Problem**: After connecting, `network-stats` shows 0 connected peers

**Explanation**: This is due to the CLI architecture where each command creates its own server instance. The peer connection is established but not maintained between commands.

**Solution for persistent connections**: Use the server mode:
```bash
# Terminal 1: Start persistent server
cargo run -- start-node 0.0.0.0 8333

# Terminal 2: Connect from another node
cargo run -- start-node 0.0.0.0 8334
# Then connect manually or use peer discovery
```

### 4. Port Already in Use

**Problem**: 
```
Error: Failed to bind to 0.0.0.0:8333: Address already in use
```

**Solutions**:
1. Use different ports: `cargo run -- start-node 0.0.0.0 8334`
2. Kill existing process: `pkill -f rust_chain`
3. Check what's using the port: `lsof -i :8333`

### 5. Connection Refused

**Problem**: Cannot connect to peer
```
Error: Failed to connect to 127.0.0.1:8333: Connection refused
```

**Solutions**:
1. Ensure target node is running
2. Check firewall settings
3. Verify correct IP and port
4. Use `netstat -tlnp | grep 8333` to confirm server is listening

## Network Architecture Notes

### Current Limitations

1. **Stateless CLI Commands**: Each CLI command creates its own server instance, so peer connections aren't persistent across commands.

2. **Connection Lifecycle**: Connections are established for the duration of a single command, then cleaned up.

3. **Peer Discovery**: Basic peer discovery is implemented but requires manual configuration.

### Recommended Network Setup

For a persistent local network, use this approach:

#### Method 1: Multiple Persistent Nodes
```bash
# Node 1 (Bootstrap)
cd node1_directory
cargo run -- init-chain
cargo run -- start-node 0.0.0.0 8333

# Node 2
cd node2_directory  
cargo run -- init-chain
cargo run -- start-node 0.0.0.0 8334

# Node 3
cd node3_directory
cargo run -- init-chain  
cargo run -- start-node 0.0.0.0 8335

# Connect nodes (each in their own terminal)
# From node2: cargo run -- connect-peer 127.0.0.1 8333
# From node3: cargo run -- connect-peer 127.0.0.1 8333
```

#### Method 2: Single Node with RPC
```bash
# Terminal 1: Blockchain node
cargo run -- start-node 0.0.0.0 8333

# Terminal 2: RPC server  
cargo run -- start-rpc 8545

# Terminal 3: Interact via CLI
cargo run -- add-transaction alice bob 100
cargo run -- mine-mempool
cargo run -- stats

# Or via curl to RPC:
curl -X POST http://localhost:8545/rpc \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"getblockcount","id":1}'
```

## Testing Connection Fix

To test if the connection issue is resolved:

1. **Start first node**:
```bash
cargo run -- start-node 0.0.0.0 8334
```

2. **In another terminal, connect**:
```bash
cargo run -- connect-peer 127.0.0.1 8334
```

3. **Expected output** (after fix):
```
Connecting to peer at 127.0.0.1:8334...
Connected to peer at 127.0.0.1:8334 successfully
Network Status:
  Connected peers: 1
  Our chain height: 1
  Max peer height: 1  
  Synchronized: Yes
Connected! Attempting blockchain synchronization...
Blockchain synchronization completed successfully
```

## Advanced Debugging

### Enable Debug Logging
```bash
RUST_LOG=debug cargo run -- start-node 0.0.0.0 8333
```

### Network Monitoring
```bash
# Check listening ports
sudo netstat -tlnp | grep rust_chain

# Monitor connections
sudo ss -tuln | grep 833

# Check firewall (Ubuntu)
sudo ufw status
```

### RPC API Testing
```bash
# Health check
curl http://localhost:8545/health

# Get blockchain info
curl -X POST http://localhost:8545/rpc \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"getblockcount","id":1}'
```

## Performance Notes

- **Development Mode**: Use `cargo run` for testing
- **Production Mode**: Use `cargo run --release` for better performance
- **Resource Usage**: Each node maintains its own database and state
- **Concurrent Connections**: Server supports multiple simultaneous connections

## Future Improvements

To make the networking more robust, consider:

1. **Persistent Peer Store**: Maintain peer connections across CLI commands
2. **Automatic Reconnection**: Implement connection recovery
3. **Better Discovery**: Enhanced peer discovery mechanisms
4. **Connection Pooling**: Manage multiple peer connections efficiently
5. **Network Health Monitoring**: Real-time network status
