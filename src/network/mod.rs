//! Network module for P2P communication and peer discovery
//! 
//! This module handles all networking aspects of the blockchain:
//! - P2P protocol implementation
//! - Network server for handling connections
//! - Peer discovery and management
//! - Message routing and validation

pub mod protocol;
pub mod server;
pub mod discovery;

pub use discovery::{
    PeerDiscovery, 
    PeerInfo, 
    DiscoveryMessage, 
    DiscoveryMessageType,
    DiscoveryStats
};

pub use protocol::{
    NetworkMessage,
    MessageType,
    NetworkError
};

pub use server::{
    NetworkServer
};

/// Network configuration
#[derive(Debug, Clone)]
pub struct NetworkConfig {
    pub listen_port: u16,
    pub max_peers: usize,
    pub seed_nodes: Vec<String>,
    pub protocol_version: u32,
    pub network_id: String,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        NetworkConfig {
            listen_port: 8333,
            max_peers: 50,
            seed_nodes: vec![
                "127.0.0.1:8334".to_string(),
                "127.0.0.1:8335".to_string(),
            ],
            protocol_version: 1,
            network_id: "rust-chain-mainnet".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_config_default() {
        let config = NetworkConfig::default();
        assert_eq!(config.listen_port, 8333);
        assert_eq!(config.max_peers, 50);
        assert!(!config.seed_nodes.is_empty());
    }

    #[test]
    fn test_peer_discovery_creation() {
        let local_addr = "127.0.0.1:8333".parse().unwrap();
        let discovery = PeerDiscovery::new(local_addr, "test-v1.0".to_string());
        assert_eq!(discovery.peer_count(), 0);
    }
}