use crate::blockchain::block::Transaction;
use crate::cli::{CLI, BlockchainCommands};
use crate::mempool::ValidationError;

/// Trait for mempool-related commands
pub trait MempoolCommands {
    fn add_transaction_to_mempool(&mut self, transaction: Transaction) -> Result<(), String>;
    fn show_mempool_stats(&self);
    fn show_pending_transactions(&self);
    fn mine_block_from_mempool(&mut self) -> Result<(), String>;
    fn clear_mempool(&mut self);
    fn demo_mempool(&mut self) -> Result<(), String>;
}

impl MempoolCommands for CLI {
    /// Add a transaction to the mempool
    fn add_transaction_to_mempool(&mut self, transaction: Transaction) -> Result<(), String> {
        let utxo_state = self.get_current_utxo_state();
        
        match self.mempool.add_transaction(transaction.clone(), &utxo_state) {
            Ok(()) => {
                println!("Transaction added to mempool successfully!");
                println!("  From: {}", transaction.from);
                println!("  To: {}", transaction.to);
                println!("  Amount: {}", transaction.amount);
                println!("  Current mempool size: {}", self.mempool.size());
                Ok(())
            },
            Err(ValidationError::InsufficientFunds) => {
                Err("Transaction rejected: Insufficient funds".to_string())
            },
            Err(ValidationError::DuplicateTransaction) => {
                Err("Transaction rejected: Duplicate transaction".to_string())
            },
            Err(ValidationError::SelfTransfer) => {
                Err("Transaction rejected: Cannot send to yourself".to_string())
            },
            Err(ValidationError::InvalidAddress) => {
                Err("Transaction rejected: Invalid address".to_string())
            },
            Err(ValidationError::InvalidSignature) => {
                Err("Transaction rejected: Invalid signature".to_string())
            },
            Err(ValidationError::NegativeAmount) => {
                Err("Transaction rejected: Negative amount".to_string())
            },
            Err(ValidationError::EmptyTransaction) => {
                Err("Transaction rejected: Empty transaction".to_string())
            },
        }
    }
    
    /// Show mempool statistics
    fn show_mempool_stats(&self) {
        let stats = self.mempool.get_stats();
        println!("=== Mempool Statistics ===");
        println!("Total transactions: {}", stats.total_transactions);
        println!("Pending transactions: {}", stats.pending_count);
        println!("Total size: {} bytes", stats.total_size_bytes);
        println!("Oldest transaction age: {} seconds", stats.oldest_transaction_age_seconds);
        println!("Average fee per byte: {:.6}", stats.average_fee_per_byte);
        
        if stats.total_transactions > 0 {
            println!("\nSample pending transactions:");
            let pending = self.mempool.get_pending_transactions();
            for (i, tx) in pending.iter().take(5).enumerate() {
                println!("  {}. {} -> {} ({})", i + 1, tx.from, tx.to, tx.amount);
            }
            if pending.len() > 5 {
                println!("  ... and {} more", pending.len() - 5);
            }
        }
    }
    
    /// Show all pending transactions in mempool
    fn show_pending_transactions(&self) {
        let pending = self.mempool.get_pending_transactions();
        
        if pending.is_empty() {
            println!("No pending transactions in mempool.");
            return;
        }
        
        println!("=== Pending Transactions ===");
        println!("Total: {} transactions", pending.len());
        println!("{:-<80}", "");
        
        for (i, tx) in pending.iter().enumerate() {
            println!("Transaction #{}: ", i + 1);
            println!("  From: {}", tx.from);
            println!("  To: {}", tx.to);
            println!("  Amount: {}", tx.amount);
            println!("  Signature: {} bytes", tx.signature.len());
            println!("{:-<80}", "");
        }
    }
    
    /// Mine a block using transactions from mempool
    fn mine_block_from_mempool(&mut self) -> Result<(), String> {
        let utxo_state = self.get_current_utxo_state();
        
        // Get transactions from mempool for the block
        let transactions = self.mempool.get_transactions_for_block(10, &utxo_state);
        
        if transactions.is_empty() {
            return Err("No valid transactions in mempool to mine".to_string());
        }
        
        println!("Mining block with {} transactions from mempool...", transactions.len());
        
        // Mine the block
        let previous_hash = self.chain.blocks.last()
            .map(|b| b.header.hash.clone())
            .unwrap_or_default();
        let height = self.chain.blocks.len() as u64;
        
        let result = self.mining_pool.mine_block(
            previous_hash,
            transactions.clone(),
            height,
        );
        
        println!("Block mined! Nonce: {}, Attempts: {}, Time: {}ms", 
                 result.nonce, result.attempts, result.elapsed_ms);
        
        // Add block to chain
        if self.chain.add_block(result.block.clone()) {
            // Store the block
            if let Err(e) = self.block_store.store_block(&result.block) {
                eprintln!("Warning: Failed to store block: {}", e);
            }
            
            // Remove mined transactions from mempool
            self.mempool.remove_transactions(&transactions);
            
            println!("Block successfully mined and added to chain!");
            println!("  Hash: {}", result.hash);
            println!("  Nonce: {}", result.nonce);
            println!("  Attempts: {}", result.attempts);
            println!("  Time: {}ms", result.elapsed_ms);
            println!("  Transactions included: {}", transactions.len());
            println!("  Remaining in mempool: {}", self.mempool.size());
            Ok(())
        } else {
            Err("Failed to add mined block to chain".to_string())
        }
    }
    
    /// Clear all transactions from mempool
    fn clear_mempool(&mut self) {
        let count = self.mempool.size();
        self.mempool.clear();
        println!("Cleared {} transactions from mempool.", count);
    }
    
    /// Demonstrate mempool functionality with a complete workflow
    fn demo_mempool(&mut self) -> Result<(), String> {
        println!("=== Mempool Demo ===");
        
        // Show initial state
        println!("\n1. Initial state:");
        self.show_mempool_stats();
        
        // Add some transactions
        println!("\n2. Adding transactions to mempool:");
        
        let tx1 = Transaction {
            from: "alice".to_string(),
            to: "charlie".to_string(),
            amount: 100,
            signature: vec![],
        };
        
        let tx2 = Transaction {
            from: "alice".to_string(),
            to: "david".to_string(),
            amount: 150,
            signature: vec![],
        };
        
        let tx3 = Transaction {
            from: "bob".to_string(),
            to: "alice".to_string(),
            amount: 75,
            signature: vec![],
        };
        
        // Add transactions
        match self.add_transaction_to_mempool(tx1) {
            Ok(()) => println!("✓ Transaction 1 added successfully"),
            Err(e) => println!("✗ Transaction 1 failed: {}", e),
        }
        
        match self.add_transaction_to_mempool(tx2) {
            Ok(()) => println!("✓ Transaction 2 added successfully"),
            Err(e) => println!("✗ Transaction 2 failed: {}", e),
        }
        
        match self.add_transaction_to_mempool(tx3) {
            Ok(()) => println!("✓ Transaction 3 added successfully"),
            Err(e) => println!("✗ Transaction 3 failed: {}", e),
        }
        
        // Show mempool state
        println!("\n3. Mempool state after adding transactions:");
        self.show_mempool_stats();
        
        println!("\n4. Pending transactions:");
        self.show_pending_transactions();
        
        // Mine a block from mempool
        println!("\n5. Mining block from mempool:");
        match self.mine_block_from_mempool() {
            Ok(()) => println!("✓ Block mined successfully from mempool"),
            Err(e) => println!("✗ Mining failed: {}", e),
        }
        
        // Show final state
        println!("\n6. Final mempool state:");
        self.show_mempool_stats();
        
        println!("\n7. Current blockchain state:");
        self.show_stats();
        
        Ok(())
    }
}

impl CLI {
    /// Get current UTXO state from the blockchain
    pub fn get_current_utxo_state(&self) -> crate::blockchain::state::UTXOState {
        use crate::blockchain::state::UTXOState;
        
        let mut state = UTXOState::new();
        
        // Process all transactions in all blocks to build current state
        for block in &self.chain.blocks {
            for tx in &block.transactions {
                // Subtract from sender (if not genesis)
                if !tx.from.is_empty() && tx.from != "genesis" {
                    state.update_balance(&tx.from, -(tx.amount as i64));
                }
                
                // Add to receiver
                state.update_balance(&tx.to, tx.amount as i64);
            }
        }
        
        state
    }
}
