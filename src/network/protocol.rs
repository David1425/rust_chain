use serde::{Serialize, Deserialize};
use crate::blockchain::block::Block;

/// Simplified block header for light clients
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlockHeader {
    pub height: u64,
    pub hash: String,
    pub previous_hash: String,
    pub timestamp: u64,
    pub nonce: u64,
    pub merkle_root: String,
}

/// Network protocol version
pub const PROTOCOL_VERSION: u32 = 1;

/// Magic bytes for message identification
pub const MAGIC_BYTES: [u8; 4] = [0x12, 0x34, 0x56, 0x78];

/// Maximum message size (1MB)
pub const MAX_MESSAGE_SIZE: usize = 1_048_576;

/// Network message types
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MessageType {
    /// Request for peer information
    GetPeers,
    /// Response with peer list
    Peers(Vec<PeerInfo>),
    /// Request for blocks starting from a specific hash
    GetBlocks { start_hash: String, count: u32 },
    /// Response with requested blocks
    Blocks(Vec<Block>),
    /// Announce a new block
    NewBlock(Block),
    /// Request the latest block hash and height
    GetChainInfo,
    /// Response with chain information
    ChainInfo { latest_hash: String, height: u64 },
    /// Ping message for connection keepalive
    Ping,
    /// Pong response to ping
    Pong,
    /// Handshake message with version and node info
    Handshake {
        version: u32,
        node_id: String,
        chain_height: u64,
    },
    /// **Phase 8 - Additional Message Types**
    /// Transaction broadcast message
    NewTransaction {
        transaction_data: String,
        from_address: String,
        to_address: String,
        amount: u64,
        signature: String,
    },
    /// Request for mempool contents
    GetMempool,
    /// Response with mempool transactions
    MempoolResponse {
        transactions: Vec<String>,
        count: usize,
    },
    /// Request for specific transaction by hash
    GetTransaction { tx_hash: String },
    /// Response with transaction details
    TransactionResponse {
        tx_hash: String,
        transaction_data: Option<String>,
        confirmed: bool,
        block_hash: Option<String>,
    },
    /// Node status announcement
    NodeStatus {
        uptime: u64,
        peer_count: usize,
        block_height: u64,
        memory_usage: u64,
        cpu_usage: f64,
    },
    /// Request for node statistics
    GetNodeStats,
    /// Response with node statistics
    NodeStatsResponse {
        version: String,
        uptime: u64,
        connections: usize,
        blocks: u64,
        transactions_processed: u64,
        bandwidth_in: u64,
        bandwidth_out: u64,
    },
    /// Peer quality report
    PeerReport {
        peer_id: String,
        latency_ms: u64,
        reliability_score: f64,
        last_message_time: u64,
    },
    /// Chain synchronization request
    SyncRequest {
        local_height: u64,
        local_best_hash: String,
    },
    /// Chain synchronization response
    SyncResponse {
        should_sync: bool,
        start_height: u64,
        end_height: u64,
        blocks_available: u32,
    },
    /// Block header only (for light clients)
    BlockHeaders {
        headers: Vec<BlockHeader>,
        start_height: u64,
    },
    /// Request for block headers
    GetBlockHeaders {
        start_height: u64,
        count: u32,
    },
    /// Network version negotiation
    VersionNegotiation {
        supported_versions: Vec<u32>,
        preferred_version: u32,
    },
    /// Address book sharing
    AddressBook {
        addresses: Vec<PeerInfo>,
        timestamp: u64,
    },
}

/// Peer information
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PeerInfo {
    pub address: String,
    pub port: u16,
    pub node_id: String,
    pub last_seen: u64,
    pub chain_height: u64,
}

/// Complete network message with header
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NetworkMessage {
    pub magic: [u8; 4],
    pub version: u32,
    pub message_type: MessageType,
    pub timestamp: u64,
    pub checksum: u32,
}

impl NetworkMessage {
    /// Create a new network message
    pub fn new(message_type: MessageType) -> Self {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        NetworkMessage {
            magic: MAGIC_BYTES,
            version: PROTOCOL_VERSION,
            message_type,
            timestamp,
            checksum: 0, // Will be calculated when serializing
        }
    }
    
    /// Serialize message to bytes
    pub fn to_bytes(&self) -> Result<Vec<u8>, String> {
        serde_json::to_vec(self)
            .map_err(|e| format!("Failed to serialize message: {}", e))
    }
    
    /// Deserialize message from bytes
    pub fn from_bytes(data: &[u8]) -> Result<Self, String> {
        if data.len() > MAX_MESSAGE_SIZE {
            return Err("Message too large".to_string());
        }
        
        serde_json::from_slice(data)
            .map_err(|e| format!("Failed to deserialize message: {}", e))
    }
    
    /// Validate message format and magic bytes
    pub fn validate(&self) -> bool {
        self.magic == MAGIC_BYTES && self.version <= PROTOCOL_VERSION
    }
}

/// Message handling result
#[derive(Debug)]
pub enum MessageResult {
    /// Message processed successfully
    Success,
    /// Message processed, response required
    Response(NetworkMessage),
    /// Multiple responses required
    MultipleResponses(Vec<NetworkMessage>),
    /// Error occurred
    Error(String),
}

/// Network error types
#[derive(Debug)]
pub enum NetworkError {
    ConnectionFailed(String),
    InvalidMessage(String),
    ProtocolError(String),
    Timeout,
    PeerDisconnected,
}

impl std::fmt::Display for NetworkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NetworkError::ConnectionFailed(msg) => write!(f, "Connection failed: {}", msg),
            NetworkError::InvalidMessage(msg) => write!(f, "Invalid message: {}", msg),
            NetworkError::ProtocolError(msg) => write!(f, "Protocol error: {}", msg),
            NetworkError::Timeout => write!(f, "Connection timeout"),
            NetworkError::PeerDisconnected => write!(f, "Peer disconnected"),
        }
    }
}

impl std::error::Error for NetworkError {}

/// Message priority levels for queue management
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MessagePriority {
    Low = 0,
    Normal = 1,
    High = 2,
    Critical = 3,
}

/// Protocol utilities for Phase 8 enhancements
impl NetworkMessage {
    /// Get message priority for queue management
    pub fn get_priority(&self) -> MessagePriority {
        match &self.message_type {
            MessageType::Ping | MessageType::Pong => MessagePriority::High,
            MessageType::NewBlock(_) => MessagePriority::Critical,
            MessageType::NewTransaction { .. } => MessagePriority::High,
            MessageType::GetChainInfo | MessageType::ChainInfo { .. } => MessagePriority::High,
            MessageType::SyncRequest { .. } | MessageType::SyncResponse { .. } => MessagePriority::High,
            MessageType::Handshake { .. } => MessagePriority::Critical,
            MessageType::NodeStatus { .. } => MessagePriority::Normal,
            _ => MessagePriority::Normal,
        }
    }

    /// Check if message requires response
    pub fn requires_response(&self) -> bool {
        matches!(
            &self.message_type,
            MessageType::GetPeers
                | MessageType::GetBlocks { .. }
                | MessageType::GetChainInfo
                | MessageType::GetMempool
                | MessageType::GetTransaction { .. }
                | MessageType::GetNodeStats
                | MessageType::SyncRequest { .. }
                | MessageType::GetBlockHeaders { .. }
                | MessageType::Ping
        )
    }

    /// Get estimated message size for bandwidth management
    pub fn estimated_size(&self) -> usize {
        match &self.message_type {
            MessageType::Blocks(blocks) => blocks.len() * 1000, // Rough estimate
            MessageType::Peers(peers) => peers.len() * 100,
            MessageType::BlockHeaders { headers, .. } => headers.len() * 200,
            MessageType::AddressBook { addresses, .. } => addresses.len() * 100,
            MessageType::MempoolResponse { transactions, .. } => transactions.len() * 500,
            _ => 200, // Base message size
        }
    }

    /// Create response message for a request
    pub fn create_response(&self, response_data: MessageType) -> Self {
        NetworkMessage::new(response_data)
    }
}

/// Protocol version compatibility check
pub fn is_compatible_version(local_version: u32, peer_version: u32) -> bool {
    // Allow communication with versions within 1 major version
    (local_version / 100) == (peer_version / 100) || 
    (local_version / 100).abs_diff(peer_version / 100) <= 1
}

/// Message routing for different node types
#[derive(Debug, Clone)]
pub enum NodeType {
    FullNode,
    LightClient,
    MiningNode,
    ArchiveNode,
}

impl NodeType {
    /// Check if this node type should handle a specific message
    pub fn should_handle(&self, message: &MessageType) -> bool {
        match (self, message) {
            // All nodes handle basic protocol messages
            (_, MessageType::Ping | MessageType::Pong | MessageType::Handshake { .. }) => true,
            
            // Full nodes handle everything
            (NodeType::FullNode, _) => true,
            
            // Light clients only handle headers and specific responses
            (NodeType::LightClient, MessageType::BlockHeaders { .. }) => true,
            (NodeType::LightClient, MessageType::GetBlockHeaders { .. }) => true,
            (NodeType::LightClient, MessageType::ChainInfo { .. }) => true,
            
            // Mining nodes prioritize new blocks and transactions
            (NodeType::MiningNode, MessageType::NewBlock(_)) => true,
            (NodeType::MiningNode, MessageType::NewTransaction { .. }) => true,
            (NodeType::MiningNode, MessageType::GetMempool) => true,
            (NodeType::MiningNode, MessageType::MempoolResponse { .. }) => true,
            
            // Archive nodes handle historical data requests
            (NodeType::ArchiveNode, MessageType::GetBlocks { .. }) => true,
            (NodeType::ArchiveNode, MessageType::Blocks(_)) => true,
            (NodeType::ArchiveNode, MessageType::GetTransaction { .. }) => true,
            
            _ => false,
        }
    }
}