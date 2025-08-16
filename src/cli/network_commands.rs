use crate::cli::CLI;
use crate::network::{NetworkServer, PeerDiscovery};
use crate::rpc::RpcConfig;
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
        let server = NetworkServer::new(self.chain.clone(), "127.0.0.1".to_string(), 0);
        
        server.connect_to_peer(&address, port)
            .map_err(|e| format!("Failed to connect to peer: {}", e))?;
        
        println!("Successfully connected to peer at {}:{}", address, port);
        Ok(())
    }

    /// Start JSON-RPC server
    fn start_rpc_server(&self, rpc_port: u16) -> Result<(), String> {
        println!("Starting JSON-RPC server on port {}...", rpc_port);
        
        let config = RpcConfig {
            bind_address: format!("127.0.0.1:{}", rpc_port).parse()
                .map_err(|e| format!("Invalid address: {}", e))?,
            ..Default::default()
        };
        
        println!("RPC server would start on {}", config.bind_address);
        println!("Available endpoints:");
        println!("  POST /rpc - JSON-RPC 2.0 endpoint");
        println!("  GET /health - Health check");
        println!("  GET /metrics - Blockchain metrics");
        
        // For now, just show what would be started
        // In production, we'd need to handle the async runtime properly
        println!("RPC server configuration completed successfully");
        
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
