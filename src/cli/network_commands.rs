use crate::cli::CLI;
use crate::network::{NetworkServer, PeerDiscovery};
use std::net::SocketAddr;
use std::thread;

/// Trait for network-related commands
pub trait NetworkCommands {
    fn start_node(&self, listen_address: String, listen_port: u16) -> Result<(), String>;
    fn connect_peer(&self, address: String, port: u16) -> Result<(), String>;
    fn start_rpc_server(&self, rpc_port: u16) -> Result<(), String>;
    fn discover_peers(&self, seed_nodes: Vec<String>) -> Result<(), String>;
    fn show_peers(&self) -> Result<(), String>;
    fn show_network_stats(&self) -> Result<(), String>;
}

impl NetworkCommands for CLI {
    /// Start network node
    fn start_node(&self, listen_address: String, listen_port: u16) -> Result<(), String> {
        println!("Starting network node on {}:{}...", listen_address, listen_port);
        
        let server = NetworkServer::new(self.chain.clone(), listen_address, listen_port);
        
        // Start server in a separate thread
        let server_handle = thread::spawn(move || {
            if let Err(e) = server.start() {
                eprintln!("Server error: {}", e);
            }
        });
        
        println!("Network node started. Press Ctrl+C to stop.");
        
        // Wait for the server thread (this will block until the server stops)
        if let Err(e) = server_handle.join() {
            eprintln!("Server thread error: {:?}", e);
        }
        
        Ok(())
    }
    
    /// Connect to a peer
    fn connect_peer(&self, address: String, port: u16) -> Result<(), String> {
        println!("Connecting to peer at {}:{}...", address, port);
        
        // Create a network server with proper configuration
        let server = NetworkServer::new(self.chain.clone(), "127.0.0.1".to_string(), 8333);
        
        server.connect_to_peer(&address, port)
            .map_err(|e| format!("Failed to connect to peer: {}", e))?;
        
        // Give the connection a moment to establish properly
        std::thread::sleep(std::time::Duration::from_millis(200));
        
        // Show network statistics
        let stats = server.get_network_stats();
        println!("Network Status:");
        println!("  Connected peers: {}", stats.connected_peers);
        println!("  Our chain height: {}", stats.our_chain_height);
        println!("  Max peer height: {}", stats.max_peer_height);
        println!("  Synchronized: {}", if stats.is_synced { "Yes" } else { "No" });
        
        // After showing initial stats, attempt to sync blockchain
        if stats.connected_peers > 0 {
            println!("Connected! Attempting blockchain synchronization...");
            if let Err(e) = server.sync_blockchain() {
                eprintln!("Warning: Blockchain sync failed: {}", e);
            } else {
                println!("Blockchain synchronization completed successfully");
            }
        } else {
            println!("Warning: No peers connected after handshake");
        }
        
        Ok(())
    }

    /// Start JSON-RPC server
    fn start_rpc_server(&self, rpc_port: u16) -> Result<(), String> {
        println!("Starting production JSON-RPC server on port {}...", rpc_port);
        
        // Create RPC config
        let config = crate::rpc::server::RpcConfig {
            bind_address: format!("127.0.0.1:{}", rpc_port).parse()
                .map_err(|e| format!("Invalid address: {}", e))?,
            max_request_size: 1_048_576, // 1MB
            enable_cors: true,
            allowed_origins: vec!["*".to_string()],
        };
        
        // Use existing CLI components instead of creating new ones
        // This avoids the database lock conflict
        let server = crate::rpc::server::RpcServer::new(
            config,
            self.chain.clone(),
            self.mempool.clone(),
            self.wallet.clone(),
        );
        
        println!("✓ RPC server configured successfully!");
        println!("Server Details:");
        println!("  Endpoint: http://127.0.0.1:{}/rpc", rpc_port);
        println!("  Health check: http://127.0.0.1:{}/health", rpc_port);
        println!("  Metrics: http://127.0.0.1:{}/metrics", rpc_port);
        println!("  Using existing CLI components (shared state)");
        
        println!("Available JSON-RPC methods:");
        println!("  getblockcount - Get current block height");
        println!("  getblockhash <height> - Get block hash by height");
        println!("  getblock <hash> - Get block details");
        println!("  getmempoolinfo - Get mempool statistics");
        println!("  sendrawtransaction <hex> - Submit transaction");
        println!("  getnewaddress - Generate new wallet address");
        
        println!("Note: Server runs with:");
        println!("  ✓ Shared blockchain state with CLI");
        println!("  ✓ Shared mempool state with CLI");
        println!("  ✓ Shared wallet state with CLI");
        println!("  ✓ CORS enabled");
        println!("  ✓ Request size limits (1MB)");
        
        // Start the server in an async runtime
        println!("\nStarting server...");
        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| format!("Failed to create async runtime: {}", e))?;
        
        rt.block_on(async {
            server.start().await
                .map_err(|e| format!("Failed to start server: {}", e))
        })?;
        
        Ok(())
    }

    /// Discover peers using seed nodes
    fn discover_peers(&self, seed_nodes: Vec<String>) -> Result<(), String> {
        println!("Starting peer discovery...");
        
        let local_addr: SocketAddr = "127.0.0.1:8333".parse()
            .map_err(|e| format!("Invalid local address: {}", e))?;
        
        let mut discovery = PeerDiscovery::new(local_addr, "rust-chain-v1.0".to_string());
        
        // Parse and add seed nodes
        let mut seed_addrs = Vec::new();
        for seed in seed_nodes {
            let addr: SocketAddr = seed.parse()
                .map_err(|e| format!("Invalid seed node address '{}': {}", seed, e))?;
            seed_addrs.push(addr);
        }
        
        if seed_addrs.is_empty() {
            return Err("No valid seed nodes provided".to_string());
        }
        
        discovery.add_seed_nodes(seed_addrs);
        
        // Update discovery with current chain height
        let chain_height = self.chain.blocks.len() as u64;
        discovery.update_chain_height(chain_height);
        
        println!("Added {} seed nodes for discovery", discovery.get_seed_nodes().len());
        println!("Current chain height: {}", chain_height);
        
        // In a real implementation, we would start the discovery process
        // and connect to peers. For now, just show the configuration.
        println!("Peer discovery configured successfully");
        
        Ok(())
    }

    /// Show connected peers
    fn show_peers(&self) -> Result<(), String> {
        println!("\n=== Connected Peers ===");
        
        // Create a sample discovery instance for demonstration
        let local_addr: SocketAddr = "127.0.0.1:8333".parse().unwrap();
        let discovery = PeerDiscovery::new(local_addr, "rust-chain-v1.0".to_string());
        
        let active_peers = discovery.get_active_peers();
        
        if active_peers.is_empty() {
            println!("No active peers found");
        } else {
            println!("Active peers: {}", active_peers.len());
            for (i, peer) in active_peers.iter().enumerate() {
                println!("  {}. {} (height: {}, version: {})", 
                    i + 1, peer.address, peer.chain_height, peer.version);
            }
        }
        
        let stats = discovery.get_stats();
        println!("\nDiscovery Statistics:");
        println!("  Total peers: {}", stats.total_peers);
        println!("  Active peers: {}", stats.active_peers);
        println!("  Max chain height: {}", stats.max_chain_height);
        println!("  Average chain height: {}", stats.avg_chain_height);
        println!("  Seed nodes: {}", stats.seed_nodes);
        
        Ok(())
    }

    /// Show network statistics
    fn show_network_stats(&self) -> Result<(), String> {
        println!("\n=== Network Statistics ===");
        
        // Get blockchain stats
        println!("Blockchain:");
        println!("  Block count: {}", self.chain.blocks.len());
        println!("  Chain height: {}", self.chain.blocks.len().saturating_sub(1));
        
        // Get mempool stats
        let mempool_stats = self.mempool.get_stats();
        println!("\nMempool:");
        println!("  Pending transactions: {}", mempool_stats.pending_count);
        println!("  Total transactions: {}", mempool_stats.total_transactions);
        println!("  Total size: {} bytes", mempool_stats.total_size_bytes);
        println!("  Average fee per byte: {}", mempool_stats.average_fee_per_byte);
        
        // Network connectivity (simplified)
        println!("\nNetwork:");
        println!("  Protocol version: 1");
        println!("  Network ID: rust-chain-mainnet");
        println!("  Default ports: P2P=8333, RPC=8545");
        
        Ok(())
    }
}
