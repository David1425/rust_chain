use rust_chain::network::protocol::{
    NetworkMessage, MessageType, PeerInfo, PROTOCOL_VERSION, MAGIC_BYTES
};
use rust_chain::network::server::NetworkServer;
use rust_chain::blockchain::chain::Chain;
use rust_chain::blockchain::block::{Block, Transaction};

#[test]
fn test_network_message_creation() {
    let message = NetworkMessage::new(MessageType::Ping);
    
    assert_eq!(message.magic, MAGIC_BYTES);
    assert_eq!(message.version, PROTOCOL_VERSION);
    assert!(matches!(message.message_type, MessageType::Ping));
    assert!(message.timestamp > 0);
}

#[test]
fn test_network_message_serialization() {
    let original = NetworkMessage::new(MessageType::GetChainInfo);
    
    // Serialize and deserialize
    let bytes = original.to_bytes().unwrap();
    let deserialized = NetworkMessage::from_bytes(&bytes).unwrap();
    
    assert_eq!(original.magic, deserialized.magic);
    assert_eq!(original.version, deserialized.version);
    assert!(matches!(deserialized.message_type, MessageType::GetChainInfo));
}

#[test]
fn test_network_message_validation() {
    let valid_message = NetworkMessage::new(MessageType::Pong);
    assert!(valid_message.validate());
    
    // Test invalid magic bytes
    let mut invalid_message = NetworkMessage::new(MessageType::Ping);
    invalid_message.magic = [0x00, 0x00, 0x00, 0x00];
    assert!(!invalid_message.validate());
}

#[test]
fn test_peer_info_serialization() {
    let peer = PeerInfo {
        address: "192.168.1.100".to_string(),
        port: 8333,
        node_id: "test_node_123".to_string(),
        last_seen: 1640995200,
        chain_height: 42,
    };
    
    let message = NetworkMessage::new(MessageType::Peers(vec![peer.clone()]));
    let bytes = message.to_bytes().unwrap();
    let deserialized = NetworkMessage::from_bytes(&bytes).unwrap();
    
    if let MessageType::Peers(peers) = deserialized.message_type {
        assert_eq!(peers.len(), 1);
        assert_eq!(peers[0].address, peer.address);
        assert_eq!(peers[0].port, peer.port);
        assert_eq!(peers[0].node_id, peer.node_id);
        assert_eq!(peers[0].chain_height, peer.chain_height);
    } else {
        panic!("Expected Peers message type");
    }
}

#[test]
fn test_block_message_serialization() {
    let tx = Transaction {
        from: "alice".to_string(),
        to: "bob".to_string(),
        amount: 100,
        signature: vec![1, 2, 3, 4],
    };
    
    let block = Block::new(
        "previous_hash".to_string(),
        vec![tx],
        42,
        1640995200,
        1,
    );
    
    let message = NetworkMessage::new(MessageType::NewBlock(block.clone()));
    let bytes = message.to_bytes().unwrap();
    let deserialized = NetworkMessage::from_bytes(&bytes).unwrap();
    
    if let MessageType::NewBlock(received_block) = deserialized.message_type {
        assert_eq!(received_block.header.hash, block.header.hash);
        assert_eq!(received_block.header.height, block.header.height);
        assert_eq!(received_block.transactions.len(), 1);
    } else {
        panic!("Expected NewBlock message type");
    }
}

#[test]
fn test_handshake_message() {
    let handshake = MessageType::Handshake {
        version: PROTOCOL_VERSION,
        node_id: "test_node".to_string(),
        chain_height: 10,
    };
    
    let message = NetworkMessage::new(handshake);
    let bytes = message.to_bytes().unwrap();
    let deserialized = NetworkMessage::from_bytes(&bytes).unwrap();
    
    if let MessageType::Handshake { version, node_id, chain_height } = deserialized.message_type {
        assert_eq!(version, PROTOCOL_VERSION);
        assert_eq!(node_id, "test_node");
        assert_eq!(chain_height, 10);
    } else {
        panic!("Expected Handshake message type");
    }
}

#[test]
fn test_get_blocks_message() {
    let get_blocks = MessageType::GetBlocks {
        start_hash: "genesis_hash".to_string(),
        count: 5,
    };
    
    let message = NetworkMessage::new(get_blocks);
    let bytes = message.to_bytes().unwrap();
    let deserialized = NetworkMessage::from_bytes(&bytes).unwrap();
    
    if let MessageType::GetBlocks { start_hash, count } = deserialized.message_type {
        assert_eq!(start_hash, "genesis_hash");
        assert_eq!(count, 5);
    } else {
        panic!("Expected GetBlocks message type");
    }
}

#[test]
fn test_network_server_creation() {
    let chain = Chain::new();
    let _server = NetworkServer::new(chain, "127.0.0.1".to_string(), 8333);
    
    // Just test that we can create a server without errors
    // The actual networking tests would require more complex setup
    assert!(true); // Server creation successful
}

#[test]
fn test_chain_info_message() {
    let chain_info = MessageType::ChainInfo {
        latest_hash: "latest_block_hash".to_string(),
        height: 42,
    };
    
    let message = NetworkMessage::new(chain_info);
    let bytes = message.to_bytes().unwrap();
    let deserialized = NetworkMessage::from_bytes(&bytes).unwrap();
    
    if let MessageType::ChainInfo { latest_hash, height } = deserialized.message_type {
        assert_eq!(latest_hash, "latest_block_hash");
        assert_eq!(height, 42);
    } else {
        panic!("Expected ChainInfo message type");
    }
}
