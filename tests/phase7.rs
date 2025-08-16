use rust_chain::network::PeerDiscovery;
use rust_chain::rpc::{RpcConfig, BlockchainRpcHandler, JsonRpcRequest, RpcHandler};
use rust_chain::blockchain::chain::Chain;
use rust_chain::mempool::Mempool;
use rust_chain::wallet::keychain::Wallet;
use serde_json::Value;

#[test]
fn test_peer_discovery_with_multiple_peers() {
    let local_addr = "127.0.0.1:8333".parse().unwrap();
    let mut discovery = PeerDiscovery::new(local_addr, "rust-chain-v1.0".to_string());
    
    // Add multiple peers with different chain heights
    let peer1 = rust_chain::network::PeerInfo::new(
        "127.0.0.1:8334".parse().unwrap(),
        "rust-chain-v1.0".to_string(),
        100
    );
    let peer2 = rust_chain::network::PeerInfo::new(
        "127.0.0.1:8335".parse().unwrap(),
        "rust-chain-v1.0".to_string(),
        150
    );
    let peer3 = rust_chain::network::PeerInfo::new(
        "127.0.0.1:8336".parse().unwrap(),
        "rust-chain-v1.0".to_string(),
        120
    );
    
    assert!(discovery.add_peer(peer1));
    assert!(discovery.add_peer(peer2));
    assert!(discovery.add_peer(peer3));
    
    // Test stats
    let stats = discovery.get_stats();
    assert_eq!(stats.total_peers, 3);
    assert_eq!(stats.active_peers, 3);
    assert_eq!(stats.max_chain_height, 150);
    assert_eq!(stats.avg_chain_height, 123); // (100 + 150 + 120) / 3 = 123
    
    // Test getting best peers
    let best_peers = discovery.get_best_peers(2);
    assert_eq!(best_peers.len(), 2);
    assert_eq!(best_peers[0].chain_height, 150);
    assert_eq!(best_peers[1].chain_height, 120);
}

#[test]
fn test_peer_discovery_seed_nodes() {
    let local_addr = "127.0.0.1:8333".parse().unwrap();
    let mut discovery = PeerDiscovery::new(local_addr, "rust-chain-v1.0".to_string());
    
    let seed_nodes = vec![
        "127.0.0.1:8334".parse().unwrap(),
        "127.0.0.1:8335".parse().unwrap(),
        "127.0.0.1:8336".parse().unwrap(),
    ];
    
    discovery.add_seed_nodes(seed_nodes);
    
    let seeds = discovery.get_seed_nodes();
    assert_eq!(seeds.len(), 3);
    assert!(seeds.contains(&"127.0.0.1:8334".parse().unwrap()));
    assert!(seeds.contains(&"127.0.0.1:8335".parse().unwrap()));
    assert!(seeds.contains(&"127.0.0.1:8336".parse().unwrap()));
}

#[test]
fn test_peer_discovery_cleanup() {
    let local_addr = "127.0.0.1:8333".parse().unwrap();
    let mut discovery = PeerDiscovery::new(local_addr, "rust-chain-v1.0".to_string());
    
    // Set max age to 1 second for testing
    // (Reusing the same discovery instance)
    
    // Add a peer and then mark it as stale
    let mut peer = rust_chain::network::PeerInfo::new(
        "127.0.0.1:8334".parse().unwrap(),
        "rust-chain-v1.0".to_string(),
        100
    );
    peer.last_seen = 0; // Very old timestamp
    
    discovery.add_peer(peer);
    assert_eq!(discovery.peer_count(), 1);
    
    // Cleanup should remove stale peers
    let removed = discovery.cleanup_stale_peers();
    assert!(removed > 0);
    assert_eq!(discovery.peer_count(), 0);
}

#[test]
fn test_rpc_handler_basic_methods() {
    let chain = Chain::new();
    let mempool = Mempool::new();
    let wallet = Wallet::new();
    let handler = BlockchainRpcHandler::new(chain, mempool, wallet);
    
    // Test getblockchaininfo
    let request = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        method: "getblockchaininfo".to_string(),
        params: None,
        id: Some(Value::Number(1.into())),
    };
    
    let response = handler.handle_request(request);
    assert!(response.result.is_some());
    assert!(response.error.is_none());
    
    if let Some(result) = response.result {
        assert_eq!(result["chain"], "rust-chain");
        assert_eq!(result["blocks"], 1); // Genesis block
    }
}

#[test]
fn test_rpc_handler_getblockhash() {
    let chain = Chain::new();
    let mempool = Mempool::new();
    let wallet = Wallet::new();
    let handler = BlockchainRpcHandler::new(chain, mempool, wallet);
    
    // Test getblockhash for genesis block (height 0)
    let request = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        method: "getblockhash".to_string(),
        params: Some(serde_json::json!([0])),
        id: Some(Value::Number(1.into())),
    };
    
    let response = handler.handle_request(request);
    assert!(response.result.is_some());
    assert!(response.error.is_none());
    
    // Should return a valid hash string
    if let Some(result) = response.result {
        if let Some(hash) = result.as_str() {
            assert!(!hash.is_empty());
            assert!(hash.len() > 0);
        } else {
            panic!("Expected hash string");
        }
    }
}

#[test]
fn test_rpc_handler_invalid_method() {
    let chain = Chain::new();
    let mempool = Mempool::new();
    let wallet = Wallet::new();
    let handler = BlockchainRpcHandler::new(chain, mempool, wallet);
    
    let request = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        method: "invalidmethod".to_string(),
        params: None,
        id: Some(Value::Number(1.into())),
    };
    
    let response = handler.handle_request(request);
    assert!(response.result.is_none());
    assert!(response.error.is_some());
    
    if let Some(error) = response.error {
        assert_eq!(error.code, -32601); // METHOD_NOT_FOUND
        assert!(error.message.contains("not found"));
    }
}

#[test]
fn test_rpc_handler_mempool_info() {
    let chain = Chain::new();
    let mempool = Mempool::new();
    let wallet = Wallet::new();
    let handler = BlockchainRpcHandler::new(chain, mempool, wallet);
    
    let request = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        method: "getmempoolinfo".to_string(),
        params: None,
        id: Some(Value::Number(1.into())),
    };
    
    let response = handler.handle_request(request);
    assert!(response.result.is_some());
    assert!(response.error.is_none());
    
    if let Some(result) = response.result {
        assert!(result["size"].is_number());
        assert!(result["bytes"].is_number());
        assert!(result["maxmempool"].is_number());
    }
}

#[test]
fn test_rpc_config_creation() {
    let config = RpcConfig::default();
    assert_eq!(config.bind_address.port(), 8545);
    assert!(config.enable_cors);
    assert_eq!(config.max_request_size, 1024 * 1024);
    assert!(!config.allowed_origins.is_empty());
    
    // Test custom config
    let custom_config = RpcConfig {
        bind_address: "0.0.0.0:3000".parse().unwrap(),
        max_request_size: 2048,
        enable_cors: false,
        allowed_origins: vec!["localhost".to_string()],
    };
    
    assert_eq!(custom_config.bind_address.port(), 3000);
    assert!(!custom_config.enable_cors);
    assert_eq!(custom_config.max_request_size, 2048);
}

#[test]
fn test_discovery_message_handling() {
    let local_addr = "127.0.0.1:8333".parse().unwrap();
    let mut discovery = PeerDiscovery::new(local_addr, "rust-chain-v1.0".to_string());
    
    // Test ping message
    let ping_msg = rust_chain::network::DiscoveryMessage::Ping;
    let from_addr = "127.0.0.1:8334".parse().unwrap();
    
    let response = discovery.handle_discovery_message(ping_msg, from_addr);
    assert!(matches!(response, Some(rust_chain::network::DiscoveryMessage::Pong)));
    
    // Test peer announcement
    let peer_info = rust_chain::network::PeerInfo::new(
        "127.0.0.1:8335".parse().unwrap(),
        "rust-chain-v1.0".to_string(),
        100
    );
    let announcement = rust_chain::network::DiscoveryMessage::PeerAnnouncement { peer: peer_info };
    
    let response = discovery.handle_discovery_message(announcement, from_addr);
    assert!(response.is_none()); // Announcements don't require responses
    assert_eq!(discovery.peer_count(), 1); // But they should add the peer
}

#[test]
fn test_network_integration() {
    // Test that all Phase 7 components work together
    let local_addr = "127.0.0.1:8333".parse().unwrap();
    let mut discovery = PeerDiscovery::new(local_addr, "rust-chain-v1.0".to_string());
    
    // Add some peers
    let peer1 = rust_chain::network::PeerInfo::new(
        "127.0.0.1:8334".parse().unwrap(),
        "rust-chain-v1.0".to_string(),
        100
    );
    discovery.add_peer(peer1);
    
    // Set up blockchain state
    let chain = Chain::new();
    let mempool = Mempool::new();
    let wallet = Wallet::new();
    
    // Test RPC handler with blockchain state
    let handler = BlockchainRpcHandler::new(chain, mempool, wallet);
    
    // Test multiple RPC calls
    let methods = vec!["getblockchaininfo", "getblockcount", "getmempoolinfo", "getbalance"];
    
    for method in methods {
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: method.to_string(),
            params: None,
            id: Some(Value::Number(1.into())),
        };
        
        let response = handler.handle_request(request);
        assert!(response.result.is_some(), "Method {} should succeed", method);
        assert!(response.error.is_none(), "Method {} should not error", method);
    }
    
    // Verify peer discovery stats
    let stats = discovery.get_stats();
    assert_eq!(stats.total_peers, 1);
    assert_eq!(stats.active_peers, 1);
    assert_eq!(stats.max_chain_height, 100);
}
