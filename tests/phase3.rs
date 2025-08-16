use rust_chain::blockchain::block::{Block, Transaction};
use rust_chain::blockchain::chain::Chain;
use rust_chain::storage::{block_store::BlockStore, db::Database};
use rust_chain::cli::{CLI, BlockchainCommands};

#[test]
fn test_database_operations() {
    let db = Database::new_with_path("./test_data/test_database_operations").expect("Failed to create database");
    
    // Test put and get
    db.put("key1".to_string(), b"value1".to_vec()).expect("Failed to put");
    assert_eq!(db.get("key1").expect("Failed to get"), Some(b"value1".to_vec()));
    
    // Test exists
    assert!(db.exists("key1").expect("Failed to check exists"));
    assert!(!db.exists("nonexistent").expect("Failed to check exists"));
    
    // Test delete
    assert!(db.delete("key1").expect("Failed to delete"));
    assert!(!db.exists("key1").expect("Failed to check exists"));
    assert_eq!(db.get("key1").expect("Failed to get"), None);
}

#[test]
fn test_block_store() {
    let store = BlockStore::new_with_path("./test_data/test_block_store").expect("Failed to create block store");
    
    // Create a test block
    let tx = Transaction {
        from: "alice".to_string(),
        to: "bob".to_string(),
        amount: 50,
        signature: vec![],
    };
    
    let block = Block::new("prev_hash".to_string(), vec![tx], 0, 0, 1);
    
    // Test storing and retrieving a block
    assert!(store.store_block(&block).is_ok());
    
    // Test get_block
    let retrieved = store.get_block(&block.header.hash).unwrap();
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().header.hash, block.header.hash);
    
    // Test get_block_by_height
    let by_height = store.get_block_by_height(1).unwrap();
    assert!(by_height.is_some());
    assert_eq!(by_height.unwrap().header.height, 1);
    
    // Test block_exists
    assert!(store.block_exists(&block.header.hash).unwrap());
    assert!(!store.block_exists("nonexistent").unwrap());
}

#[test]
fn test_cli_initialization() {
    let mut cli = CLI::new_with_path("./test_data/test_cli_initialization").expect("Failed to create CLI");
    
    // Test init chain
    assert!(cli.init_chain().is_ok());
    
    // Test adding a block
    let tx = Transaction {
        from: "test_sender".to_string(),
        to: "test_receiver".to_string(),
        amount: 100,
        signature: vec![],
    };
    
    assert!(cli.add_block(vec![tx]).is_ok());
}

#[test]
fn test_chain_with_storage() {
    let mut chain = Chain::new();
    let store = BlockStore::new_with_path("./test_data/test_chain_with_storage").expect("Failed to create block store");
    
    // Store genesis block
    let genesis = &chain.blocks[0];
    assert!(store.store_block(genesis).is_ok());
    
    // Create and add a new block
    let tx = Transaction {
        from: "alice".to_string(),
        to: "bob".to_string(),
        amount: 30,
        signature: vec![],
    };
    
    let new_block = Block::new(genesis.header.hash.clone(), vec![tx], 0, 0, 1);
    chain.add_block(new_block.clone());
    
    assert!(store.store_block(&new_block).is_ok());
    
    let stored_block = store.get_block(&new_block.header.hash).unwrap().unwrap();
    assert_eq!(stored_block.header.hash, new_block.header.hash);
    assert_eq!(stored_block.transactions.len(), 1);
}
