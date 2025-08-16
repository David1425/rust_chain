use rust_chain::blockchain::block::Transaction;
use rust_chain::blockchain::state::UTXOState;
use rust_chain::mempool::{Mempool, TransactionValidator, ValidationError};

fn create_test_transaction(from: &str, to: &str, amount: u64) -> Transaction {
    Transaction {
        from: from.to_string(),
        to: to.to_string(),
        amount,
        signature: vec![],
    }
}

#[test]
fn test_transaction_validator_basic() {
    let mut validator = TransactionValidator::new();
    let mut state = UTXOState::new();
    
    // Add some balance to alice
    state.update_balance("alice", 100);
    
    let valid_tx = create_test_transaction("alice", "bob", 50);
    assert!(validator.validate_transaction(&valid_tx, &state).is_ok());
}

#[test]
fn test_transaction_validator_insufficient_funds() {
    let mut validator = TransactionValidator::new();
    let state = UTXOState::new(); // Empty state
    
    let invalid_tx = create_test_transaction("alice", "bob", 50);
    assert_eq!(
        validator.validate_transaction(&invalid_tx, &state),
        Err(ValidationError::InsufficientFunds)
    );
}

#[test]
fn test_transaction_validator_self_transfer() {
    let mut validator = TransactionValidator::new();
    let state = UTXOState::new();
    
    let self_tx = create_test_transaction("alice", "alice", 50);
    assert_eq!(
        validator.validate_transaction(&self_tx, &state),
        Err(ValidationError::SelfTransfer)
    );
}

#[test]
fn test_transaction_validator_duplicate() {
    let mut validator = TransactionValidator::new();
    let mut state = UTXOState::new();
    state.update_balance("alice", 100);
    
    let tx = create_test_transaction("alice", "bob", 50);
    
    // First validation should pass
    assert!(validator.validate_transaction(&tx, &state).is_ok());
    
    // Second validation should fail (duplicate)
    assert_eq!(
        validator.validate_transaction(&tx, &state),
        Err(ValidationError::DuplicateTransaction)
    );
}

#[test]
fn test_transaction_validator_empty_transaction() {
    let mut validator = TransactionValidator::new();
    let state = UTXOState::new();
    
    let empty_tx = create_test_transaction("alice", "bob", 0);
    assert_eq!(
        validator.validate_transaction(&empty_tx, &state),
        Err(ValidationError::EmptyTransaction)
    );
}

#[test]
fn test_transaction_validator_invalid_address() {
    let mut validator = TransactionValidator::new();
    let state = UTXOState::new();
    
    let invalid_tx = Transaction {
        from: "".to_string(), // Empty from address
        to: "bob".to_string(),
        amount: 50,
        signature: vec![],
    };
    
    assert_eq!(
        validator.validate_transaction(&invalid_tx, &state),
        Err(ValidationError::InvalidAddress)
    );
}

#[test]
fn test_mempool_add_transaction() {
    let mut mempool = Mempool::new();
    let mut state = UTXOState::new();
    state.update_balance("alice", 100);
    
    let tx = create_test_transaction("alice", "bob", 50);
    
    assert!(mempool.add_transaction(tx.clone(), &state).is_ok());
    assert_eq!(mempool.size(), 1);
    assert!(mempool.contains_transaction(&tx));
}

#[test]
fn test_mempool_duplicate_prevention() {
    let mut mempool = Mempool::new();
    let mut state = UTXOState::new();
    state.update_balance("alice", 100);
    
    let tx = create_test_transaction("alice", "bob", 50);
    
    // First add should succeed
    assert!(mempool.add_transaction(tx.clone(), &state).is_ok());
    
    // Second add should fail
    assert_eq!(
        mempool.add_transaction(tx, &state),
        Err(ValidationError::DuplicateTransaction)
    );
    
    assert_eq!(mempool.size(), 1);
}

#[test]
fn test_mempool_get_transactions_for_block() {
    let mut mempool = Mempool::new();
    let mut state = UTXOState::new();
    state.update_balance("alice", 1000);
    
    // Add multiple transactions
    let tx1 = create_test_transaction("alice", "bob", 100);
    let tx2 = create_test_transaction("alice", "charlie", 200);
    let tx3 = create_test_transaction("alice", "david", 300);
    
    mempool.add_transaction(tx1.clone(), &state).unwrap();
    mempool.add_transaction(tx2.clone(), &state).unwrap();
    mempool.add_transaction(tx3.clone(), &state).unwrap();
    
    assert_eq!(mempool.size(), 3);
    
    // Get transactions for block
    let block_txs = mempool.get_transactions_for_block(2, &state);
    assert_eq!(block_txs.len(), 2); // Should limit to 2 transactions
}

#[test]
fn test_mempool_remove_transactions() {
    let mut mempool = Mempool::new();
    let mut state = UTXOState::new();
    state.update_balance("alice", 1000);
    
    let tx1 = create_test_transaction("alice", "bob", 100);
    let tx2 = create_test_transaction("alice", "charlie", 200);
    
    mempool.add_transaction(tx1.clone(), &state).unwrap();
    mempool.add_transaction(tx2.clone(), &state).unwrap();
    assert_eq!(mempool.size(), 2);
    
    // Remove one transaction
    mempool.remove_transactions(&[tx1.clone()]);
    assert_eq!(mempool.size(), 1);
    assert!(!mempool.contains_transaction(&tx1));
    assert!(mempool.contains_transaction(&tx2));
}

#[test]
fn test_mempool_stats() {
    let mut mempool = Mempool::new();
    let mut state = UTXOState::new();
    state.update_balance("alice", 1000);
    
    // Empty mempool
    let stats = mempool.get_stats();
    assert_eq!(stats.total_transactions, 0);
    assert_eq!(stats.pending_count, 0);
    
    // Add a transaction
    let tx = create_test_transaction("alice", "bob", 50);
    mempool.add_transaction(tx, &state).unwrap();
    
    let stats = mempool.get_stats();
    assert_eq!(stats.total_transactions, 1);
    assert_eq!(stats.pending_count, 1);
    assert!(stats.total_size_bytes > 0);
}

#[test]
fn test_mempool_clear() {
    let mut mempool = Mempool::new();
    let mut state = UTXOState::new();
    state.update_balance("alice", 1000);
    
    let tx = create_test_transaction("alice", "bob", 50);
    mempool.add_transaction(tx, &state).unwrap();
    assert_eq!(mempool.size(), 1);
    
    mempool.clear();
    assert_eq!(mempool.size(), 0);
    assert!(mempool.is_empty());
}

#[test]
fn test_mempool_invalid_transaction_insufficient_funds() {
    let mut mempool = Mempool::new();
    let state = UTXOState::new(); // Empty state
    
    let tx = create_test_transaction("alice", "bob", 50);
    
    assert_eq!(
        mempool.add_transaction(tx, &state),
        Err(ValidationError::InsufficientFunds)
    );
    assert_eq!(mempool.size(), 0);
}

#[test]
fn test_mempool_transaction_ordering() {
    let mut mempool = Mempool::new();
    let mut state = UTXOState::new();
    state.update_balance("alice", 1000);
    
    // Add transactions (same fee, so should be ordered by timestamp)
    let tx1 = create_test_transaction("alice", "bob", 100);
    let tx2 = create_test_transaction("alice", "charlie", 200);
    
    mempool.add_transaction(tx1.clone(), &state).unwrap();
    mempool.add_transaction(tx2.clone(), &state).unwrap();
    
    let block_txs = mempool.get_transactions_for_block(10, &state);
    assert_eq!(block_txs.len(), 2);
    // Should maintain order (first added, first in block)
    assert_eq!(block_txs[0].amount, 100);
    assert_eq!(block_txs[1].amount, 200);
}

#[test]
fn test_utxo_state_operations() {
    let mut state = UTXOState::new();
    
    // Initially empty
    assert_eq!(state.get_balance("alice"), 0);
    
    // Update balance
    state.update_balance("alice", 100);
    assert_eq!(state.get_balance("alice"), 100);
    
    // Deduct balance
    state.update_balance("alice", -50);
    assert_eq!(state.get_balance("alice"), 50);
    
    // Can't go negative
    state.update_balance("alice", -100);
    assert_eq!(state.get_balance("alice"), 0);
    
    // Set balance directly
    state.set_balance("bob", 200);
    assert_eq!(state.get_balance("bob"), 200);
}

#[test]
fn test_mempool_with_limits() {
    let mut mempool = Mempool::with_limits(2, 3600); // Max 2 transactions
    let mut state = UTXOState::new();
    state.update_balance("alice", 1000);
    
    let tx1 = create_test_transaction("alice", "bob", 100);
    let tx2 = create_test_transaction("alice", "charlie", 200);
    let tx3 = create_test_transaction("alice", "david", 300);
    
    // Add three transactions (should be limited to 2)
    mempool.add_transaction(tx1, &state).unwrap();
    mempool.add_transaction(tx2, &state).unwrap();
    mempool.add_transaction(tx3, &state).unwrap();
    
    // Should only keep 2 transactions (highest priority)
    assert_eq!(mempool.size(), 2);
}
