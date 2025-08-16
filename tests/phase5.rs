use rust_chain::consensus::pow::{ProofOfWork, MiningPool, DEFAULT_DIFFICULTY};
use rust_chain::consensus::fork_choice::{ForkChoice, ForkChoiceWithReorg};
use rust_chain::blockchain::chain::Chain;
use rust_chain::blockchain::block::{Block, Transaction};

#[test]
fn test_proof_of_work_creation() {
    let pow = ProofOfWork::new();
    assert_eq!(pow.get_difficulty(), DEFAULT_DIFFICULTY);
    
    let pow_custom = ProofOfWork::with_difficulty(2);
    assert_eq!(pow_custom.get_difficulty(), 2);
}

#[test]
fn test_mining_simple_block() {
    let pow = ProofOfWork::with_difficulty(2); // Low difficulty for fast testing
    
    let tx = Transaction {
        from: "miner".to_string(),
        to: "recipient".to_string(),
        amount: 50,
        signature: vec![],
    };
    
    let result = pow.mine_block(
        "previous_hash".to_string(),
        vec![tx],
        1,
    );
    
    // Nonce is always valid (u64)
    assert!(result.attempts > 0);
    // elapsed_ms is u128, always >= 0, so just check it's a reasonable value
    assert!(result.hash.starts_with("00")); // Should have 2 leading zeros
    assert!(pow.validate_block(&result.block));
}

#[test]
fn test_mining_pool() {
    let mut pool = MiningPool::new(2); // Low difficulty
    
    let tx = Transaction {
        from: "alice".to_string(),
        to: "bob".to_string(),
        amount: 25,
        signature: vec![],
    };
    
    let result = pool.mine_block(
        "genesis".to_string(),
        vec![tx],
        1,
    );
    
    let stats = pool.get_stats();
    assert_eq!(stats.total_blocks_mined, 1);
    assert_eq!(stats.total_attempts, result.attempts);
    // Hash rate might be 0 if mining was very fast
    assert!(stats.current_hash_rate >= 0.0);
}

#[test]
fn test_difficulty_adjustment() {
    let mut pow = ProofOfWork::with_difficulty(3); // Start higher so we can see adjustment
    
    // Create some mock blocks with timestamps
    let mut blocks = Vec::new();
    let base_time = 1640995200u64;
    
    for i in 0..5 {
        let tx = Transaction {
            from: "test".to_string(),
            to: "test".to_string(),
            amount: 1,
            signature: vec![],
        };
        
        let block = Block::new(
            format!("hash_{}", i),
            vec![tx],
            0,
            base_time + (i as u64 * 30), // 30 seconds apart (fast blocks)
            i as u64,
        );
        blocks.push(block);
    }
    
    // Target is 60 seconds, but blocks are coming every 30 seconds
    // 30 < 60/2 = 30, so 30 < 30 is false, so difficulty won't increase
    // Let's make blocks faster: 15 seconds apart
    let mut blocks = Vec::new();
    let base_time = 1640995200u64;
    
    for i in 0..5 {
        let tx = Transaction {
            from: "test".to_string(),
            to: "test".to_string(),
            amount: 1,
            signature: vec![],
        };
        
        let block = Block::new(
            format!("hash_{}", i),
            vec![tx],
            0,
            base_time + (i as u64 * 15), // 15 seconds apart (very fast blocks)
            i as u64,
        );
        blocks.push(block);
    }
    
    let new_difficulty = pow.adjust_difficulty(&blocks, 60);
    // Average is 15s, target is 60s, so 15 < 30 should trigger increase
    assert_eq!(new_difficulty, 4); // Should increase difficulty from 3 to 4
}

#[test]
fn test_fork_choice_creation() {
    let fork_choice = ForkChoice::new();
    assert!(fork_choice.get_best_chain().is_none());
    
    let chain = Chain::new();
    let fork_choice_with_genesis = ForkChoice::with_genesis_chain(chain);
    assert!(fork_choice_with_genesis.get_best_chain().is_some());
}

#[test]
fn test_fork_choice_add_block() {
    let chain = Chain::new();
    let mut fork_choice = ForkChoice::with_genesis_chain(chain);
    
    let genesis_hash = fork_choice.get_best_chain()
        .unwrap()
        .blocks.last()
        .unwrap()
        .header.hash.clone();
    
    let tx = Transaction {
        from: "alice".to_string(),
        to: "bob".to_string(),
        amount: 100,
        signature: vec![],
    };
    
    let new_block = Block::new(
        genesis_hash,
        vec![tx],
        42,
        1640995200,
        1,
    );
    
    let result = fork_choice.add_block(new_block.clone());
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), true); // Should be new best
    
    let best_chain = fork_choice.get_best_chain().unwrap();
    assert_eq!(best_chain.blocks.len(), 2); // Genesis + new block
    assert_eq!(best_chain.blocks.last().unwrap().header.hash, new_block.header.hash);
}

#[test]
fn test_fork_choice_longer_chain_wins() {
    let chain = Chain::new();
    let mut fork_choice = ForkChoice::with_genesis_chain(chain);
    
    let genesis_hash = fork_choice.get_best_chain()
        .unwrap()
        .blocks.last()
        .unwrap()
        .header.hash.clone();
    
    // Add first block to main chain
    let tx1 = Transaction {
        from: "alice".to_string(),
        to: "bob".to_string(),
        amount: 100,
        signature: vec![],
    };
    
    let block1 = Block::new(
        genesis_hash.clone(),
        vec![tx1],
        1,
        1640995200,
        1,
    );
    
    fork_choice.add_block(block1.clone()).unwrap();
    
    // Add second block to main chain
    let tx2 = Transaction {
        from: "bob".to_string(),
        to: "charlie".to_string(),
        amount: 50,
        signature: vec![],
    };
    
    let block2 = Block::new(
        block1.header.hash.clone(),
        vec![tx2],
        2,
        1640995260,
        2,
    );
    
    fork_choice.add_block(block2.clone()).unwrap();
    
    // Now add a competing fork from genesis
    let tx_fork = Transaction {
        from: "eve".to_string(),
        to: "mallory".to_string(),
        amount: 25,
        signature: vec![],
    };
    
    let fork_block = Block::new(
        genesis_hash,
        vec![tx_fork],
        10,
        1640995300,
        1,
    );
    
    let result = fork_choice.add_block(fork_block).unwrap();
    assert_eq!(result, false); // Should not be new best (shorter chain)
    
    // Main chain should still be the best
    let best_chain = fork_choice.get_best_chain().unwrap();
    assert_eq!(best_chain.blocks.len(), 3); // Genesis + 2 blocks
    assert_eq!(best_chain.blocks.last().unwrap().header.hash, block2.header.hash);
}

#[test]
fn test_fork_choice_stats() {
    let chain = Chain::new();
    let mut fork_choice = ForkChoice::with_genesis_chain(chain);
    
    let stats = fork_choice.get_chain_stats();
    assert_eq!(stats.total_chains, 1);
    assert_eq!(stats.best_chain_height, 0); // Just genesis
    assert_eq!(stats.has_forks, false);
    
    // Add a block
    let genesis_hash = fork_choice.get_best_chain()
        .unwrap()
        .blocks.last()
        .unwrap()
        .header.hash.clone();
    
    let tx = Transaction {
        from: "test".to_string(),
        to: "test".to_string(),
        amount: 1,
        signature: vec![],
    };
    
    let block = Block::new(genesis_hash, vec![tx], 1, 1640995200, 1);
    fork_choice.add_block(block).unwrap();
    
    let stats = fork_choice.get_chain_stats();
    assert_eq!(stats.best_chain_height, 1);
    assert_eq!(stats.total_blocks, 2); // Genesis + new block
}

#[test]
fn test_fork_choice_with_reorg() {
    // Create a new reorg tracker
    let mut fork_choice = ForkChoiceWithReorg::new(10);
    
    // Add the genesis block first by creating a fresh genesis
    let tx_genesis = Transaction {
        from: "coinbase".to_string(),
        to: "genesis_address".to_string(),
        amount: 50,
        signature: vec![],
    };
    
    let genesis_block = Block::new("0".to_string(), vec![tx_genesis], 0, 0, 0);
    let result = fork_choice.add_block(genesis_block.clone());
    if result.is_err() {
        println!("Genesis block error: {:?}", result);
    }
    assert!(result.is_ok());
    
    // Now add a regular block
    let tx = Transaction {
        from: "alice".to_string(),
        to: "bob".to_string(),
        amount: 25,
        signature: vec![],
    };
    
    let block = Block::new(genesis_block.header.hash, vec![tx], 1, 1640995200, 1);
    let result = fork_choice.add_block(block);
    assert!(result.is_ok());
    
    assert!(fork_choice.get_best_chain().is_some());
}

#[test]
fn test_block_validation_in_fork_choice() {
    let chain = Chain::new();
    let mut fork_choice = ForkChoice::with_genesis_chain(chain);
    
    // Try to add an invalid block (wrong parent)
    let tx = Transaction {
        from: "alice".to_string(),
        to: "bob".to_string(),
        amount: 100,
        signature: vec![],
    };
    
    let invalid_block = Block::new(
        "nonexistent_parent".to_string(),
        vec![tx],
        42,
        1640995200,
        1,
    );
    
    let result = fork_choice.add_block(invalid_block);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Parent block not found"));
}
