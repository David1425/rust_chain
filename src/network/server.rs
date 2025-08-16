use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream, SocketAddr};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::blockchain::chain::Chain;
use crate::blockchain::block::Block;
use crate::network::protocol::{
    NetworkMessage, MessageType, MessageResult, NetworkError, PeerInfo, PROTOCOL_VERSION
};

/// Network server for handling P2P connections
pub struct NetworkServer {
    chain: Arc<Mutex<Chain>>,
    peers: Arc<Mutex<HashMap<String, PeerInfo>>>,
    node_id: String,
    listen_address: String,
    listen_port: u16,
    running: Arc<Mutex<bool>>,
}

impl NetworkServer {
    /// Create a new network server
    pub fn new(chain: Chain, listen_address: String, listen_port: u16) -> Self {
        let node_id = format!("node_{}", rand::random::<u32>());
        
        NetworkServer {
            chain: Arc::new(Mutex::new(chain)),
            peers: Arc::new(Mutex::new(HashMap::new())),
            node_id,
            listen_address,
            listen_port,
            running: Arc::new(Mutex::new(false)),
        }
    }
    
    /// Start the server
    pub fn start(&self) -> Result<(), NetworkError> {
        let bind_address = format!("{}:{}", self.listen_address, self.listen_port);
        let listener = TcpListener::bind(&bind_address)
            .map_err(|e| NetworkError::ConnectionFailed(format!("Failed to bind to {}: {}", bind_address, e)))?;
        
        println!("Network server listening on {}", bind_address);
        
        *self.running.lock().unwrap() = true;
        
        for stream in listener.incoming() {
            if !*self.running.lock().unwrap() {
                break;
            }
            
            match stream {
                Ok(stream) => {
                    let chain = Arc::clone(&self.chain);
                    let peers = Arc::clone(&self.peers);
                    let node_id = self.node_id.clone();
                    
                    thread::spawn(move || {
                        if let Err(e) = Self::handle_connection(stream, chain, peers, node_id) {
                            eprintln!("Connection error: {}", e);
                        }
                    });
                },
                Err(e) => {
                    eprintln!("Failed to accept connection: {}", e);
                }
            }
        }
        
        Ok(())
    }
    
    /// Stop the server
    pub fn stop(&self) {
        *self.running.lock().unwrap() = false;
    }
    
    /// Handle a single connection
    fn handle_connection(
        mut stream: TcpStream,
        chain: Arc<Mutex<Chain>>,
        peers: Arc<Mutex<HashMap<String, PeerInfo>>>,
        node_id: String,
    ) -> Result<(), NetworkError> {
        let peer_addr = stream.peer_addr()
            .map_err(|e| NetworkError::ConnectionFailed(format!("Failed to get peer address: {}", e)))?;
        
        println!("New connection from {}", peer_addr);
        
        // Set read timeout
        stream.set_read_timeout(Some(Duration::from_secs(30)))
            .map_err(|e| NetworkError::ConnectionFailed(format!("Failed to set timeout: {}", e)))?;
        
        loop {
            match Self::read_message(&mut stream) {
                Ok(message) => {
                    if !message.validate() {
                        return Err(NetworkError::InvalidMessage("Invalid message format".to_string()));
                    }
                    
                    match Self::handle_message(message, &chain, &peers, &node_id, &peer_addr) {
                        MessageResult::Success => {},
                        MessageResult::Response(response) => {
                            Self::send_message(&mut stream, response)?;
                        },
                        MessageResult::MultipleResponses(responses) => {
                            for response in responses {
                                Self::send_message(&mut stream, response)?;
                            }
                        },
                        MessageResult::Error(err) => {
                            eprintln!("Message handling error: {}", err);
                            break;
                        }
                    }
                },
                Err(NetworkError::Timeout) => {
                    // Send ping to check if connection is alive
                    let ping = NetworkMessage::new(MessageType::Ping);
                    Self::send_message(&mut stream, ping)?;
                },
                Err(NetworkError::PeerDisconnected) => {
                    println!("Peer {} disconnected", peer_addr);
                    break;
                },
                Err(e) => {
                    eprintln!("Error reading message: {}", e);
                    break;
                }
            }
        }
        
        Ok(())
    }
    
    /// Read a message from the stream
    fn read_message(stream: &mut TcpStream) -> Result<NetworkMessage, NetworkError> {
        let mut length_bytes = [0u8; 4];
        stream.read_exact(&mut length_bytes)
            .map_err(|e| {
                if e.kind() == std::io::ErrorKind::UnexpectedEof {
                    NetworkError::PeerDisconnected
                } else if e.kind() == std::io::ErrorKind::TimedOut {
                    NetworkError::Timeout
                } else {
                    NetworkError::ConnectionFailed(format!("Failed to read message length: {}", e))
                }
            })?;
        
        let length = u32::from_be_bytes(length_bytes) as usize;
        if length > crate::network::protocol::MAX_MESSAGE_SIZE {
            return Err(NetworkError::InvalidMessage("Message too large".to_string()));
        }
        
        let mut buffer = vec![0u8; length];
        stream.read_exact(&mut buffer)
            .map_err(|e| NetworkError::ConnectionFailed(format!("Failed to read message data: {}", e)))?;
        
        NetworkMessage::from_bytes(&buffer)
            .map_err(|e| NetworkError::InvalidMessage(e))
    }
    
    /// Send a message to the stream
    fn send_message(stream: &mut TcpStream, message: NetworkMessage) -> Result<(), NetworkError> {
        let data = message.to_bytes()
            .map_err(|e| NetworkError::ProtocolError(e))?;
        
        let length = data.len() as u32;
        stream.write_all(&length.to_be_bytes())
            .map_err(|e| NetworkError::ConnectionFailed(format!("Failed to write message length: {}", e)))?;
        
        stream.write_all(&data)
            .map_err(|e| NetworkError::ConnectionFailed(format!("Failed to write message data: {}", e)))?;
        
        stream.flush()
            .map_err(|e| NetworkError::ConnectionFailed(format!("Failed to flush stream: {}", e)))?;
        
        Ok(())
    }
    
    /// Handle an incoming message
    fn handle_message(
        message: NetworkMessage,
        chain: &Arc<Mutex<Chain>>,
        peers: &Arc<Mutex<HashMap<String, PeerInfo>>>,
        node_id: &str,
        peer_addr: &SocketAddr,
    ) -> MessageResult {
        println!("Received message: {:?}", message.message_type);
        
        match message.message_type {
            MessageType::Handshake { version, node_id: peer_node_id, chain_height } => {
                if version > PROTOCOL_VERSION {
                    return MessageResult::Error("Unsupported protocol version".to_string());
                }
                
                // Add peer to peer list
                let peer_info = PeerInfo {
                    address: peer_addr.ip().to_string(),
                    port: peer_addr.port(),
                    node_id: peer_node_id,
                    last_seen: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
                    chain_height,
                };
                
                peers.lock().unwrap().insert(peer_info.node_id.clone(), peer_info);
                
                // Respond with our handshake
                let chain_guard = chain.lock().unwrap();
                let our_height = chain_guard.blocks.len() as u64 - 1;
                drop(chain_guard);
                
                let response = NetworkMessage::new(MessageType::Handshake {
                    version: PROTOCOL_VERSION,
                    node_id: node_id.to_string(),
                    chain_height: our_height,
                });
                
                MessageResult::Response(response)
            },
            
            MessageType::GetChainInfo => {
                let chain_guard = chain.lock().unwrap();
                let latest_block = chain_guard.blocks.last().unwrap();
                let response = NetworkMessage::new(MessageType::ChainInfo {
                    latest_hash: latest_block.header.hash.clone(),
                    height: latest_block.header.height,
                });
                drop(chain_guard);
                
                MessageResult::Response(response)
            },
            
            MessageType::GetBlocks { start_hash, count } => {
                let chain_guard = chain.lock().unwrap();
                let mut blocks = Vec::new();
                let mut found_start = start_hash == "0"; // Genesis case
                
                for block in &chain_guard.blocks {
                    if found_start && blocks.len() < count as usize {
                        blocks.push(block.clone());
                    }
                    if block.header.hash == start_hash {
                        found_start = true;
                    }
                }
                drop(chain_guard);
                
                let response = NetworkMessage::new(MessageType::Blocks(blocks));
                MessageResult::Response(response)
            },
            
            MessageType::GetPeers => {
                let peers_guard = peers.lock().unwrap();
                let peer_list: Vec<PeerInfo> = peers_guard.values().cloned().collect();
                drop(peers_guard);
                
                let response = NetworkMessage::new(MessageType::Peers(peer_list));
                MessageResult::Response(response)
            },
            
            MessageType::NewBlock(block) => {
                // Simple validation and addition
                let mut chain_guard = chain.lock().unwrap();
                if chain_guard.validate_block(&block) {
                    chain_guard.add_block(block);
                    println!("Added new block from peer");
                }
                drop(chain_guard);
                
                MessageResult::Success
            },
            
            MessageType::Ping => {
                let response = NetworkMessage::new(MessageType::Pong);
                MessageResult::Response(response)
            },
            
            MessageType::Pong => {
                // Connection is alive
                MessageResult::Success
            },
            
            _ => {
                MessageResult::Success // Handle other message types as needed
            }
        }
    }
    
    /// Connect to a peer
    pub fn connect_to_peer(&self, address: &str, port: u16) -> Result<(), NetworkError> {
        let peer_address = format!("{}:{}", address, port);
        let mut stream = TcpStream::connect(&peer_address)
            .map_err(|e| NetworkError::ConnectionFailed(format!("Failed to connect to {}: {}", peer_address, e)))?;
        
        // Set timeout for handshake
        stream.set_read_timeout(Some(Duration::from_secs(10)))
            .map_err(|e| NetworkError::ConnectionFailed(format!("Failed to set timeout: {}", e)))?;
        
        // Send handshake
        let chain_guard = self.chain.lock().unwrap();
        let chain_height = chain_guard.blocks.len() as u64 - 1;
        drop(chain_guard);
        
        let handshake = NetworkMessage::new(MessageType::Handshake {
            version: PROTOCOL_VERSION,
            node_id: self.node_id.clone(),
            chain_height,
        });
        
        Self::send_message(&mut stream, handshake)?;
        
        // Wait for handshake response
        match Self::read_message(&mut stream) {
            Ok(response) => {
                if let MessageType::Handshake { version, node_id: peer_node_id, chain_height: peer_height } = response.message_type {
                    println!("Received handshake response from peer {} (version: {}, height: {})", 
                        peer_node_id, version, peer_height);
                    
                    // Add peer to our peer list
                    let peer_info = PeerInfo {
                        address: address.to_string(),
                        port,
                        node_id: peer_node_id,
                        last_seen: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
                        chain_height: peer_height,
                    };
                    
                    self.peers.lock().unwrap().insert(peer_info.node_id.clone(), peer_info);
                    println!("Connected to peer at {} successfully", peer_address);
                } else {
                    return Err(NetworkError::ProtocolError("Expected handshake response".to_string()));
                }
            },
            Err(e) => {
                return Err(NetworkError::ConnectionFailed(format!("Failed to receive handshake response: {}", e)));
            }
        }
        
        // Keep connection alive for a short time to establish the peer relationship
        // In a real implementation, this would be managed by a connection pool
        thread::sleep(Duration::from_millis(100));
        
        Ok(())
    }

    /// Synchronize blockchain with peers
    pub fn sync_blockchain(&self) -> Result<(), NetworkError> {
        let peers_guard = self.peers.lock().unwrap();
        if peers_guard.is_empty() {
            return Err(NetworkError::ConnectionFailed("No peers available for sync".to_string()));
        }

        // Find the best peer (highest chain height)
        let best_peer = peers_guard.values()
            .max_by_key(|peer| peer.chain_height)
            .cloned();
        drop(peers_guard);

        if let Some(peer) = best_peer {
            let chain_guard = self.chain.lock().unwrap();
            let our_height = chain_guard.blocks.len() as u64;
            drop(chain_guard);

            if peer.chain_height > our_height {
                println!("Syncing with peer {} (height: {} vs our height: {})", 
                    peer.address, peer.chain_height, our_height);
                
                // Request blocks from where we left off
                let peer_address = format!("{}:{}", peer.address, peer.port);
                self.request_blocks_from_peer(&peer_address, our_height)?;
            } else {
                println!("Blockchain is up to date");
            }
        }

        Ok(())
    }

    /// Request blocks from a specific peer
    fn request_blocks_from_peer(&self, peer_address: &str, _start_height: u64) -> Result<(), NetworkError> {
        let mut stream = TcpStream::connect(peer_address)
            .map_err(|e| NetworkError::ConnectionFailed(format!("Failed to connect for sync: {}", e)))?;

        // Get our latest block hash
        let chain_guard = self.chain.lock().unwrap();
        let start_hash = if let Some(block) = chain_guard.blocks.last() {
            block.header.hash.clone()
        } else {
            "0".repeat(64) // Genesis hash
        };
        drop(chain_guard);

        // Request blocks
        let get_blocks = NetworkMessage::new(MessageType::GetBlocks {
            start_hash,
            count: 100, // Request up to 100 blocks at a time
        });

        Self::send_message(&mut stream, get_blocks)?;

        // Read response
        match Self::read_message(&mut stream)? {
            message if matches!(message.message_type, MessageType::Blocks(_)) => {
                if let MessageType::Blocks(blocks) = message.message_type {
                    self.process_sync_blocks(blocks)?;
                }
            },
            _ => {
                return Err(NetworkError::ProtocolError("Unexpected response to GetBlocks".to_string()));
            }
        }

        Ok(())
    }

    /// Process blocks received during sync
    fn process_sync_blocks(&self, blocks: Vec<Block>) -> Result<(), NetworkError> {
        let mut chain_guard = self.chain.lock().unwrap();
        let mut synced_count = 0;

        for block in blocks {
            // Validate and add block
            if chain_guard.validate_block(&block) {
                chain_guard.blocks.push(block.clone());
                synced_count += 1;
                println!("Synced block {} (height: {})", block.header.hash, block.header.height);
            } else {
                println!("Warning: Invalid block received during sync: {}", block.header.hash);
            }
        }

        drop(chain_guard);
        println!("Successfully synced {} blocks", synced_count);
        Ok(())
    }

    /// Broadcast a block to all connected peers
    pub fn broadcast_block(&self, block: &Block) -> Result<(), NetworkError> {
        let peers_guard = self.peers.lock().unwrap();
        let peers: Vec<_> = peers_guard.values().cloned().collect();
        drop(peers_guard);

        for peer in peers {
            let peer_address = format!("{}:{}", peer.address, peer.port);
            if let Err(e) = self.send_block_to_peer(&peer_address, block) {
                eprintln!("Failed to broadcast block to peer {}: {}", peer_address, e);
                // Continue with other peers
            }
        }

        Ok(())
    }

    /// Send a block to a specific peer
    fn send_block_to_peer(&self, peer_address: &str, block: &Block) -> Result<(), NetworkError> {
        let mut stream = TcpStream::connect(peer_address)
            .map_err(|e| NetworkError::ConnectionFailed(format!("Failed to connect to peer: {}", e)))?;

        let new_block = NetworkMessage::new(MessageType::NewBlock(block.clone()));
        Self::send_message(&mut stream, new_block)?;

        println!("Broadcasted block {} to {}", block.header.hash, peer_address);
        Ok(())
    }

    /// Get list of connected peers
    pub fn get_connected_peers(&self) -> Vec<PeerInfo> {
        let peers_guard = self.peers.lock().unwrap();
        peers_guard.values().cloned().collect()
    }

    /// Get network statistics
    pub fn get_network_stats(&self) -> NetworkStats {
        let peers_guard = self.peers.lock().unwrap();
        let chain_guard = self.chain.lock().unwrap();
        
        let connected_peers = peers_guard.len();
        let our_height = chain_guard.blocks.len() as u64;
        let max_peer_height = peers_guard.values()
            .map(|p| p.chain_height)
            .max()
            .unwrap_or(0);

        NetworkStats {
            connected_peers,
            our_chain_height: our_height,
            max_peer_height,
            is_synced: our_height >= max_peer_height,
            node_id: self.node_id.clone(),
        }
    }
}

/// Network statistics
#[derive(Debug, Clone)]
pub struct NetworkStats {
    pub connected_peers: usize,
    pub our_chain_height: u64,
    pub max_peer_height: u64,
    pub is_synced: bool,
    pub node_id: String,
}