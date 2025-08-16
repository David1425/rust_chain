use std::collections::HashMap;
use std::net::SocketAddr;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Serialize, Deserialize};

/// Information about a peer in the network
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PeerInfo {
    pub address: SocketAddr,
    pub last_seen: u64,
    pub version: String,
    pub chain_height: u64,
    pub is_active: bool,
}

impl PeerInfo {
    pub fn new(address: SocketAddr, version: String, chain_height: u64) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        PeerInfo {
            address,
            last_seen: timestamp,
            version,
            chain_height,
            is_active: true,
        }
    }

    pub fn update_last_seen(&mut self) {
        self.last_seen = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }

    pub fn is_stale(&self, max_age_seconds: u64) -> bool {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        current_time.saturating_sub(self.last_seen) > max_age_seconds
    }
}

/// Discovery messages for peer communication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiscoveryMessage {
    PeerRequest,
    PeerResponse { peers: Vec<PeerInfo> },
    PeerAnnouncement { peer: PeerInfo },
    Ping,
    Pong,
}

/// Peer discovery and management system
pub struct PeerDiscovery {
    /// Our own address
    local_address: SocketAddr,
    
    /// Known peers with their information
    peers: HashMap<SocketAddr, PeerInfo>,
    
    /// Seed nodes for bootstrapping
    seed_nodes: Vec<SocketAddr>,
    
    /// Maximum number of peers to maintain
    max_peers: usize,
    
    /// Maximum age for peer information (in seconds)
    max_peer_age: u64,
    
    /// Our blockchain version
    version: String,
    
    /// Current chain height
    chain_height: u64,
}

impl PeerDiscovery {
    /// Create a new peer discovery instance
    pub fn new(local_address: SocketAddr, version: String) -> Self {
        PeerDiscovery {
            local_address,
            peers: HashMap::new(),
            seed_nodes: Vec::new(),
            max_peers: 50,
            max_peer_age: 3600, // 1 hour
            version,
            chain_height: 0,
        }
    }

    /// Add seed nodes for bootstrapping
    pub fn add_seed_nodes(&mut self, seeds: Vec<SocketAddr>) {
        self.seed_nodes.extend(seeds);
    }

    /// Add a new peer
    pub fn add_peer(&mut self, peer: PeerInfo) -> bool {
        // Don't add ourselves
        if peer.address == self.local_address {
            return false;
        }

        // Don't add if we're at capacity and this peer isn't better
        if self.peers.len() >= self.max_peers {
            if let Some(worst_peer) = self.find_worst_peer() {
                if peer.chain_height <= worst_peer.chain_height {
                    return false;
                }
                // Remove the worst peer to make room
                self.peers.remove(&worst_peer.address);
            }
        }

        self.peers.insert(peer.address, peer);
        true
    }

    /// Remove a peer
    pub fn remove_peer(&mut self, address: &SocketAddr) -> bool {
        self.peers.remove(address).is_some()
    }

    /// Update peer information
    pub fn update_peer(&mut self, address: &SocketAddr, chain_height: u64) {
        if let Some(peer) = self.peers.get_mut(address) {
            peer.chain_height = chain_height;
            peer.update_last_seen();
            peer.is_active = true;
        }
    }

    /// Mark a peer as inactive
    pub fn mark_peer_inactive(&mut self, address: &SocketAddr) {
        if let Some(peer) = self.peers.get_mut(address) {
            peer.is_active = false;
        }
    }

    /// Get all active peers
    pub fn get_active_peers(&self) -> Vec<PeerInfo> {
        self.peers.values()
            .filter(|p| p.is_active && !p.is_stale(self.max_peer_age))
            .cloned()
            .collect()
    }

    /// Get a random subset of peers
    pub fn get_random_peers(&self, count: usize) -> Vec<PeerInfo> {
        use rand::seq::SliceRandom;
        use rand::thread_rng;
        
        let mut active_peers = self.get_active_peers();
        active_peers.shuffle(&mut thread_rng());
        active_peers.into_iter().take(count).collect()
    }

    /// Get the best peers (highest chain height)
    pub fn get_best_peers(&self, count: usize) -> Vec<PeerInfo> {
        let mut active_peers = self.get_active_peers();
        active_peers.sort_by(|a, b| b.chain_height.cmp(&a.chain_height));
        active_peers.into_iter().take(count).collect()
    }

    /// Get seed nodes for bootstrapping
    pub fn get_seed_nodes(&self) -> &[SocketAddr] {
        &self.seed_nodes
    }

    /// Clean up stale peers
    pub fn cleanup_stale_peers(&mut self) -> usize {
        let stale_addresses: Vec<SocketAddr> = self.peers.iter()
            .filter(|(_, peer)| peer.is_stale(self.max_peer_age))
            .map(|(addr, _)| *addr)
            .collect();
        
        let removed_count = stale_addresses.len();
        for addr in stale_addresses {
            self.peers.remove(&addr);
        }
        
        removed_count
    }

    /// Update our chain height
    pub fn update_chain_height(&mut self, height: u64) {
        self.chain_height = height;
    }

    /// Get our current chain height
    pub fn get_chain_height(&self) -> u64 {
        self.chain_height
    }

    /// Get peer count
    pub fn peer_count(&self) -> usize {
        self.peers.len()
    }

    /// Get active peer count
    pub fn active_peer_count(&self) -> usize {
        self.get_active_peers().len()
    }

    /// Find the worst peer (for replacement)
    fn find_worst_peer(&self) -> Option<PeerInfo> {
        self.peers.values()
            .filter(|p| p.is_active)
            .min_by_key(|p| p.chain_height)
            .cloned()
    }

    /// Create a discovery message
    pub fn create_discovery_message(&self, msg_type: DiscoveryMessageType) -> DiscoveryMessage {
        match msg_type {
            DiscoveryMessageType::PeerRequest => DiscoveryMessage::PeerRequest,
            DiscoveryMessageType::PeerResponse => {
                let peers = self.get_random_peers(20); // Share up to 20 peers
                DiscoveryMessage::PeerResponse { peers }
            },
            DiscoveryMessageType::Announcement => {
                let our_info = PeerInfo::new(self.local_address, self.version.clone(), self.chain_height);
                DiscoveryMessage::PeerAnnouncement { peer: our_info }
            },
            DiscoveryMessageType::Ping => DiscoveryMessage::Ping,
            DiscoveryMessageType::Pong => DiscoveryMessage::Pong,
        }
    }

    /// Handle incoming discovery message
    pub fn handle_discovery_message(&mut self, message: DiscoveryMessage, from: SocketAddr) -> Option<DiscoveryMessage> {
        match message {
            DiscoveryMessage::PeerRequest => {
                // Respond with our known peers
                Some(self.create_discovery_message(DiscoveryMessageType::PeerResponse))
            },
            DiscoveryMessage::PeerResponse { peers } => {
                // Add the new peers to our list
                for peer in peers {
                    self.add_peer(peer);
                }
                None
            },
            DiscoveryMessage::PeerAnnouncement { peer } => {
                // Add the announcing peer
                self.add_peer(peer);
                None
            },
            DiscoveryMessage::Ping => {
                // Update peer info and respond with pong
                self.update_peer(&from, 0); // Height unknown from ping
                Some(DiscoveryMessage::Pong)
            },
            DiscoveryMessage::Pong => {
                // Update peer as active
                self.update_peer(&from, 0); // Height unknown from pong
                None
            },
        }
    }

    /// Get discovery statistics
    pub fn get_stats(&self) -> DiscoveryStats {
        let active_peers = self.get_active_peers();
        let max_height = active_peers.iter().map(|p| p.chain_height).max().unwrap_or(0);
        let avg_height = if !active_peers.is_empty() {
            active_peers.iter().map(|p| p.chain_height).sum::<u64>() / active_peers.len() as u64
        } else {
            0
        };

        DiscoveryStats {
            total_peers: self.peers.len(),
            active_peers: active_peers.len(),
            max_chain_height: max_height,
            avg_chain_height: avg_height,
            seed_nodes: self.seed_nodes.len(),
        }
    }
}

/// Types of discovery messages
pub enum DiscoveryMessageType {
    PeerRequest,
    PeerResponse,
    Announcement,
    Ping,
    Pong,
}

/// Discovery statistics
#[derive(Debug, Clone)]
pub struct DiscoveryStats {
    pub total_peers: usize,
    pub active_peers: usize,
    pub max_chain_height: u64,
    pub avg_chain_height: u64,
    pub seed_nodes: usize,
}

impl Default for PeerDiscovery {
    fn default() -> Self {
        PeerDiscovery::new(
            "127.0.0.1:8333".parse().unwrap(),
            "rust-chain-v1.0".to_string()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_peer(port: u16, height: u64) -> PeerInfo {
        let addr = format!("127.0.0.1:{}", port).parse().unwrap();
        PeerInfo::new(addr, "test-v1.0".to_string(), height)
    }

    #[test]
    fn test_peer_discovery_creation() {
        let discovery = PeerDiscovery::new(
            "127.0.0.1:8333".parse().unwrap(),
            "test-v1.0".to_string()
        );
        
        assert_eq!(discovery.peer_count(), 0);
        assert_eq!(discovery.active_peer_count(), 0);
    }

    #[test]
    fn test_add_peer() {
        let mut discovery = PeerDiscovery::new(
            "127.0.0.1:8333".parse().unwrap(),
            "test-v1.0".to_string()
        );
        
        let peer = create_test_peer(8334, 100);
        assert!(discovery.add_peer(peer));
        assert_eq!(discovery.peer_count(), 1);
    }

    #[test]
    fn test_dont_add_self() {
        let mut discovery = PeerDiscovery::new(
            "127.0.0.1:8333".parse().unwrap(),
            "test-v1.0".to_string()
        );
        
        let self_peer = create_test_peer(8333, 100);
        assert!(!discovery.add_peer(self_peer));
        assert_eq!(discovery.peer_count(), 0);
    }

    #[test]
    fn test_get_best_peers() {
        let mut discovery = PeerDiscovery::new(
            "127.0.0.1:8333".parse().unwrap(),
            "test-v1.0".to_string()
        );
        
        discovery.add_peer(create_test_peer(8334, 100));
        discovery.add_peer(create_test_peer(8335, 200));
        discovery.add_peer(create_test_peer(8336, 150));
        
        let best_peers = discovery.get_best_peers(2);
        assert_eq!(best_peers.len(), 2);
        assert_eq!(best_peers[0].chain_height, 200);
        assert_eq!(best_peers[1].chain_height, 150);
    }

    #[test]
    fn test_cleanup_stale_peers() {
        let mut discovery = PeerDiscovery::new(
            "127.0.0.1:8333".parse().unwrap(),
            "test-v1.0".to_string()
        );
        discovery.max_peer_age = 1; // 1 second for testing
        
        let mut peer = create_test_peer(8334, 100);
        peer.last_seen = 0; // Very old timestamp
        discovery.add_peer(peer);
        
        assert_eq!(discovery.peer_count(), 1);
        let removed = discovery.cleanup_stale_peers();
        assert_eq!(removed, 1);
        assert_eq!(discovery.peer_count(), 0);
    }

    #[test]
    fn test_discovery_messages() {
        let mut discovery = PeerDiscovery::new(
            "127.0.0.1:8333".parse().unwrap(),
            "test-v1.0".to_string()
        );
        
        // Test ping response
        let response = discovery.handle_discovery_message(
            DiscoveryMessage::Ping,
            "127.0.0.1:8334".parse().unwrap()
        );
        
        assert!(matches!(response, Some(DiscoveryMessage::Pong)));
    }
}
