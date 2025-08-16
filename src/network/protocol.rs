use serde::{Serialize, Deserialize};
use crate::blockchain::block::Block;

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
}

/// Peer information
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PeerInfo {
    pub address: String,
    pub port: u16,
    pub node_id: String,
    pub last_seen: u64,
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