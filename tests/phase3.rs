use rust_chain::blockchain::block::{Block, Transaction};
use rust_chain::blockchain::chain::Chain;
use rust_chain::storage::{block_store::BlockStore, db::Database};
use rust_chain::cli::{CLI, BlockchainCommands};

#[test]
fn test_database_operations() {
    let mut db = Database::new();
    
    // Test put and get
    db.put("key1".to_string(), b"value1".to_vec());
    assert_eq!(db.get("key1"), Some(&b"value1".to_vec()));
    
    // Test exists
    assert!(db.exists("key1"));
    assert!(!db.exists("nonexistent"));
    
    // Test delete
    assert!(db.delete("key1"));
    assert!(!db.exists("key1"));
    assert_eq!(db.get("key1"), None);
}

#[test]
fn test_block_store() {
    let mut store = BlockStore::new();
    
    // Create a test block
    let tx = Transaction {
        from: "alice".to_string(),
        to: "bob".to_string(),
        amount: 25,
        signature: vec![],
    };
    
    let block = Block::new(
        "previous_hash".to_string(),
        vec![tx],
        123,
        1234567890,
        1,
    );
    
    // Store the block
    assert!(store.store_block(&block).is_ok());
    
    // Retrieve by hash
    let retrieved = store.get_block(&block.header.hash).unwrap();
    assert!(retrieved.is_some());
    let retrieved_block = retrieved.unwrap();
    assert_eq!(retrieved_block.header.hash, block.header.hash);
    assert_eq!(retrieved_block.header.height, block.header.height);
    
    // Retrieve by height
    let by_height = store.get_block_by_height(1).unwrap();
    assert!(by_height.is_some());
    let by_height_block = by_height.unwrap();
    assert_eq!(by_height_block.header.hash, block.header.hash);
    
    // Test exists
    assert!(store.block_exists(&block.header.hash));
    assert!(!store.block_exists("nonexistent"));
}

#[test]
fn test_cli_initialization() {
    let mut cli = CLI::new();
    
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
    let mut store = BlockStore::new();
    
    // Store genesis block
    let genesis = &chain.blocks[0];
    assert!(store.store_block(genesis).is_ok());
    
    // Create and add a new block
    let tx = Transaction {
        from: "miner".to_string(),
        to: "recipient".to_string(),
        amount: 50,
        signature: vec![],
    };
    
    let prev_hash = chain.blocks.last().unwrap().header.hash.clone();
    let new_block = Block::new(prev_hash, vec![tx], 0, 1234567890, 1);
    
    // Add to chain and store
    assert!(chain.add_block(new_block.clone()));
    assert!(store.store_block(&new_block).is_ok());
    
    // Verify storage
    let stored_block = store.get_block(&new_block.header.hash).unwrap().unwrap();
    assert_eq!(stored_block.header.hash, new_block.header.hash);
    assert_eq!(stored_block.transactions.len(), 1);
}
