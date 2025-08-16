use crate::blockchain::block::Transaction;
use crate::blockchain::state::UTXOState;
use crate::mempool::validator::{TransactionValidator, ValidationError};
use std::collections::{HashMap, VecDeque};
use std::time::{SystemTime, UNIX_EPOCH};

/// Transaction with metadata for mempool management
#[derive(Debug, Clone)]
pub struct MempoolTransaction {
    pub transaction: Transaction,
    pub timestamp: u64,
    pub fee_per_byte: f64, // For future fee-based prioritization
    pub size_bytes: usize,
}

impl MempoolTransaction {
    pub fn new(transaction: Transaction) -> Self {
        let size_bytes = std::mem::size_of_val(&transaction);
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        MempoolTransaction {
            transaction,
            timestamp,
            fee_per_byte: 0.0, // Default fee
            size_bytes,
        }
    }

    pub fn with_fee(mut self, fee_per_byte: f64) -> Self {
        self.fee_per_byte = fee_per_byte;
        self
    }
}

/// Mempool statistics
#[derive(Debug, Clone)]
pub struct MempoolStats {
    pub total_transactions: usize,
    pub total_size_bytes: usize,
    pub oldest_transaction_age_seconds: u64,
    pub average_fee_per_byte: f64,
    pub pending_count: usize,
}

/// Transaction mempool for pending transactions
#[derive(Clone)]
pub struct Mempool {
    /// Pending transactions ordered by priority (fee, then timestamp)
    transactions: VecDeque<MempoolTransaction>,
    
    /// Quick lookup by transaction hash
    transaction_lookup: HashMap<String, usize>,
    
    /// Transaction validator
    validator: TransactionValidator,
    
    /// Maximum number of transactions in mempool
    max_size: usize,
    
    /// Maximum age of transactions in seconds
    max_age_seconds: u64,
}

impl Mempool {
    /// Create a new mempool with default settings
    pub fn new() -> Self {
        Mempool {
            transactions: VecDeque::new(),
            transaction_lookup: HashMap::new(),
            validator: TransactionValidator::new(),
            max_size: 1000, // Default max 1000 transactions
            max_age_seconds: 3600, // Default 1 hour expiry
        }
    }

    /// Create a new mempool with custom settings
    pub fn with_limits(max_size: usize, max_age_seconds: u64) -> Self {
        Mempool {
            transactions: VecDeque::new(),
            transaction_lookup: HashMap::new(),
            validator: TransactionValidator::new(),
            max_size,
            max_age_seconds,
        }
    }

    /// Add a transaction to the mempool
    pub fn add_transaction(
        &mut self,
        transaction: Transaction,
        utxo_state: &UTXOState,
    ) -> Result<(), ValidationError> {
        // Validate the transaction
        self.validator.validate_transaction(&transaction, utxo_state)?;
        
        // Create mempool transaction
        let mempool_tx = MempoolTransaction::new(transaction);
        let tx_hash = self.calculate_transaction_hash(&mempool_tx.transaction);
        
        // Check if already in mempool
        if self.transaction_lookup.contains_key(&tx_hash) {
            return Err(ValidationError::DuplicateTransaction);
        }
        
        // Add to mempool with priority ordering
        self.insert_with_priority(mempool_tx, tx_hash);
        
        // Clean up old transactions and enforce size limits
        self.cleanup();
        
        Ok(())
    }

    /// Get transactions for block creation (highest priority first)
    pub fn get_transactions_for_block(
        &self,
        max_transactions: usize,
        utxo_state: &UTXOState,
    ) -> Vec<Transaction> {
        let mut selected = Vec::new();
        let mut temp_state = utxo_state.clone();
        
        for mempool_tx in &self.transactions {
            if selected.len() >= max_transactions {
                break;
            }
            
            // Check if transaction is still valid against current state
            let mut temp_validator = TransactionValidator::new();
            if temp_validator.validate_transaction(&mempool_tx.transaction, &temp_state).is_ok() {
                // Apply transaction to temporary state
                self.apply_transaction_to_state(&mempool_tx.transaction, &mut temp_state);
                selected.push(mempool_tx.transaction.clone());
            }
        }
        
        selected
    }

    /// Remove transactions that have been included in a block
    pub fn remove_transactions(&mut self, transactions: &[Transaction]) {
        for tx in transactions {
            let tx_hash = self.calculate_transaction_hash(tx);
            if let Some(_index) = self.transaction_lookup.get(&tx_hash) {
                // Find the actual index in the deque (may have changed due to removals)
                if let Some(pos) = self.transactions.iter().position(|mtx| {
                    self.calculate_transaction_hash(&mtx.transaction) == tx_hash
                }) {
                    self.transactions.remove(pos);
                    self.transaction_lookup.remove(&tx_hash);
                    
                    // Update indices in lookup table
                    self.rebuild_lookup_table();
                }
            }
        }
    }

    /// Get mempool statistics
    pub fn get_stats(&self) -> MempoolStats {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let oldest_age = if let Some(oldest) = self.transactions.front() {
            current_time.saturating_sub(oldest.timestamp)
        } else {
            0
        };
        
        let total_size: usize = self.transactions.iter()
            .map(|tx| tx.size_bytes)
            .sum();
        
        let average_fee = if !self.transactions.is_empty() {
            self.transactions.iter()
                .map(|tx| tx.fee_per_byte)
                .sum::<f64>() / self.transactions.len() as f64
        } else {
            0.0
        };
        
        MempoolStats {
            total_transactions: self.transactions.len(),
            total_size_bytes: total_size,
            oldest_transaction_age_seconds: oldest_age,
            average_fee_per_byte: average_fee,
            pending_count: self.transactions.len(),
        }
    }

    /// Get all pending transactions
    pub fn get_pending_transactions(&self) -> Vec<Transaction> {
        self.transactions.iter()
            .map(|mtx| mtx.transaction.clone())
            .collect()
    }

    /// Check if mempool contains a specific transaction
    pub fn contains_transaction(&self, transaction: &Transaction) -> bool {
        let tx_hash = self.calculate_transaction_hash(transaction);
        self.transaction_lookup.contains_key(&tx_hash)
    }

    /// Clear all transactions from mempool
    pub fn clear(&mut self) {
        self.transactions.clear();
        self.transaction_lookup.clear();
        self.validator.clear_seen_transactions();
    }

    /// Get current mempool size
    pub fn size(&self) -> usize {
        self.transactions.len()
    }

    /// Check if mempool is empty
    pub fn is_empty(&self) -> bool {
        self.transactions.is_empty()
    }

    /// Insert transaction with priority ordering (higher fee first, then older timestamp)
    fn insert_with_priority(&mut self, mempool_tx: MempoolTransaction, tx_hash: String) {
        let insert_pos = self.transactions.iter().position(|existing| {
            // First priority: higher fee per byte
            if mempool_tx.fee_per_byte > existing.fee_per_byte {
                return true;
            }
            
            // Second priority: older timestamp (if fees are equal)
            if mempool_tx.fee_per_byte == existing.fee_per_byte 
                && mempool_tx.timestamp < existing.timestamp {
                return true;
            }
            
            false
        }).unwrap_or(self.transactions.len());
        
        self.transactions.insert(insert_pos, mempool_tx);
        self.transaction_lookup.insert(tx_hash, insert_pos);
        
        // Rebuild lookup table to maintain correct indices
        self.rebuild_lookup_table();
    }

    /// Rebuild the lookup table with correct indices
    fn rebuild_lookup_table(&mut self) {
        self.transaction_lookup.clear();
        for (index, mempool_tx) in self.transactions.iter().enumerate() {
            let tx_hash = self.calculate_transaction_hash(&mempool_tx.transaction);
            self.transaction_lookup.insert(tx_hash, index);
        }
    }

    /// Clean up old transactions and enforce size limits
    fn cleanup(&mut self) {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        // Remove expired transactions
        while let Some(oldest) = self.transactions.front() {
            if current_time.saturating_sub(oldest.timestamp) > self.max_age_seconds {
                let removed = self.transactions.pop_front().unwrap();
                let tx_hash = self.calculate_transaction_hash(&removed.transaction);
                self.transaction_lookup.remove(&tx_hash);
            } else {
                break;
            }
        }
        
        // Enforce size limit (remove lowest priority transactions)
        while self.transactions.len() > self.max_size {
            let removed = self.transactions.pop_back().unwrap();
            let tx_hash = self.calculate_transaction_hash(&removed.transaction);
            self.transaction_lookup.remove(&tx_hash);
        }
        
        // Rebuild lookup table after cleanup
        if !self.transactions.is_empty() {
            self.rebuild_lookup_table();
        }
    }

    /// Apply transaction to UTXO state
    fn apply_transaction_to_state(&self, transaction: &Transaction, state: &mut UTXOState) {
        state.update_balance(&transaction.from, -(transaction.amount as i64));
        state.update_balance(&transaction.to, transaction.amount as i64);
    }

    /// Calculate transaction hash
    fn calculate_transaction_hash(&self, transaction: &Transaction) -> String {
        use crate::crypto::hash::sha256_hash;
        
        let tx_string = format!(
            "{}:{}:{}:{}",
            transaction.from,
            transaction.to,
            transaction.amount,
            hex::encode(&transaction.signature)
        );
        
        sha256_hash(&tx_string)
    }

    /// Save mempool state to disk for persistence
    pub fn save_to_file(&self, path: &str) -> Result<(), String> {
        use std::fs;
        use std::path::Path;
        
        // Create the directory if it doesn't exist
        if let Some(parent) = Path::new(path).parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create mempool directory: {}", e))?;
        }

        // Serialize mempool transactions (excluding validator state)
        let serializable_data = self.transactions.iter()
            .map(|mempool_tx| &mempool_tx.transaction)
            .collect::<Vec<_>>();
        
        let json_data = serde_json::to_string_pretty(&serializable_data)
            .map_err(|e| format!("Failed to serialize mempool: {}", e))?;
        
        fs::write(path, json_data)
            .map_err(|e| format!("Failed to write mempool file: {}", e))?;
        
        Ok(())
    }

    /// Load mempool state from disk
    pub fn load_from_file(&mut self, path: &str, utxo_state: &UTXOState) -> Result<(), String> {
        use std::fs;
        use std::path::Path;
        
        if !Path::new(path).exists() {
            return Ok(()); // No saved state, start fresh
        }

        let json_data = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read mempool file: {}", e))?;
        
        let transactions: Vec<Transaction> = serde_json::from_str(&json_data)
            .map_err(|e| format!("Failed to deserialize mempool: {}", e))?;
        
        // Clear current state
        self.clear();
        
        // Re-add transactions with validation
        let mut loaded_count = 0;
        for tx in transactions {
            match self.add_transaction(tx, utxo_state) {
                Ok(()) => loaded_count += 1,
                Err(_) => {
                    // Skip invalid transactions from saved state
                    continue;
                }
            }
        }
        
        println!("Loaded {} valid transactions from mempool persistence", loaded_count);
        Ok(())
    }

    /// Create a persistent mempool that auto-saves and loads
    pub fn new_persistent(_save_path: String) -> Self {
        let mempool = Self::new();
        // Note: Loading will be done separately when UTXO state is available
        // as we need it for validation
        mempool
    }
}

impl Default for Mempool {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blockchain::state::UTXOState;

    fn create_test_transaction(from: &str, to: &str, amount: u64) -> Transaction {
        Transaction {
            from: from.to_string(),
            to: to.to_string(),
            amount,
            signature: vec![],
        }
    }

    #[test]
    fn test_mempool_basic_operations() {
        let mut mempool = Mempool::new();
        let mut state = UTXOState::new();
        state.update_balance("alice", 100);
        
        let tx = create_test_transaction("alice", "bob", 50);
        
        // Add transaction
        assert!(mempool.add_transaction(tx.clone(), &state).is_ok());
        assert_eq!(mempool.size(), 1);
        assert!(mempool.contains_transaction(&tx));
        
        // Get transactions for block
        let block_txs = mempool.get_transactions_for_block(10, &state);
        assert_eq!(block_txs.len(), 1);
        
        // Remove transactions
        mempool.remove_transactions(&block_txs);
        assert_eq!(mempool.size(), 0);
        assert!(!mempool.contains_transaction(&tx));
    }

    #[test]
    fn test_mempool_priority_ordering() {
        let mut mempool = Mempool::new();
        let mut state = UTXOState::new();
        state.update_balance("alice", 1000);
        
        // Add transactions with different fees
        let tx1 = create_test_transaction("alice", "bob", 100);
        let tx2 = create_test_transaction("alice", "charlie", 200);
        
        mempool.add_transaction(tx1.clone(), &state).unwrap();
        mempool.add_transaction(tx2.clone(), &state).unwrap();
        
        let block_txs = mempool.get_transactions_for_block(10, &state);
        
        // Should be ordered by fee (higher fee first), but since we have same fees,
        // order should be by timestamp (first added first)
        assert_eq!(block_txs.len(), 2);
        assert_eq!(block_txs[0].amount, 100); // First added
        assert_eq!(block_txs[1].amount, 200); // Second added
    }

    #[test]
    fn test_mempool_invalid_transaction() {
        let mut mempool = Mempool::new();
        let state = UTXOState::new(); // Empty state, no funds
        
        let tx = create_test_transaction("alice", "bob", 50);
        
        // Should fail due to insufficient funds
        assert_eq!(
            mempool.add_transaction(tx, &state),
            Err(ValidationError::InsufficientFunds)
        );
        assert_eq!(mempool.size(), 0);
    }

    #[test]
    fn test_mempool_stats() {
        let mut mempool = Mempool::new();
        let mut state = UTXOState::new();
        state.update_balance("alice", 1000);
        
        assert_eq!(mempool.get_stats().total_transactions, 0);
        
        let tx = create_test_transaction("alice", "bob", 50);
        mempool.add_transaction(tx, &state).unwrap();
        
        let stats = mempool.get_stats();
        assert_eq!(stats.total_transactions, 1);
        assert_eq!(stats.pending_count, 1);
        assert!(stats.total_size_bytes > 0);
    }

    #[test]
    fn test_mempool_duplicate_prevention() {
        let mut mempool = Mempool::new();
        let mut state = UTXOState::new();
        state.update_balance("alice", 1000);
        
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
}
