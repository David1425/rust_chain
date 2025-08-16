use crate::blockchain::chain::Chain;
use crate::blockchain::block::{Block, Transaction};
use crate::storage::block_store::BlockStore;
use crate::network::server::NetworkServer;
use crate::consensus::pow::MiningPool;
use crate::consensus::fork_choice::ForkChoice;
use crate::mempool::{Mempool, ValidationError};
use std::time::{SystemTime, UNIX_EPOCH};
use std::thread;

pub struct CLI {
    chain: Chain,
    block_store: BlockStore,
    mining_pool: MiningPool,
    fork_choice: ForkChoice,
    mempool: Mempool,
}

impl CLI {
    pub fn new() -> Self {
        let chain = Chain::new();
        let fork_choice = ForkChoice::with_genesis_chain(chain.clone());
        
        CLI {
            chain,
            block_store: BlockStore::new(),
            mining_pool: MiningPool::new(4), // Default difficulty of 4
            fork_choice,
            mempool: Mempool::new(),
        }
    }
    
    /// Initialize a new blockchain
    pub fn init_chain(&mut self) -> Result<(), String> {
        println!("Initializing new blockchain...");
        
        // Store genesis block
        let genesis = &self.chain.blocks[0];
        self.block_store.store_block(genesis)?;
        
        println!("Genesis block created with hash: {}", genesis.header.hash);
        println!("Blockchain initialized successfully!");
        Ok(())
    }
    
    /// Show all blocks in the chain
    pub fn show_blocks(&self) {
        println!("Blockchain contains {} blocks:", self.chain.blocks.len());
        println!("{:-<80}", "");
        
        for (i, block) in self.chain.blocks.iter().enumerate() {
            println!("Block #{}: {}", i, block.header.hash);
            println!("  Height: {}", block.header.height);
            println!("  Previous Hash: {}", block.header.previous_hash);
            println!("  Timestamp: {}", block.header.timestamp);
            println!("  Merkle Root: {}", block.header.merkle_root);
            println!("  Nonce: {}", block.header.nonce);
            println!("  Transactions: {}", block.transactions.len());
            
            for (j, tx) in block.transactions.iter().enumerate() {
                println!("    Tx #{}: {} -> {} ({})", j, tx.from, tx.to, tx.amount);
            }
            println!("{:-<80}", "");
        }
    }
    
    /// Add a new block with given transactions
    pub fn add_block(&mut self, transactions: Vec<Transaction>) -> Result<(), String> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let previous_hash = self.chain.blocks.last()
            .map(|b| b.header.hash.clone())
            .unwrap_or_default();
        
        let height = self.chain.blocks.len() as u64;
        
        let new_block = Block::new(
            previous_hash,
            transactions,
            0, // Simple nonce for now
            timestamp,
            height,
        );
        
        if self.chain.add_block(new_block.clone()) {
            self.block_store.store_block(&new_block)?;
            println!("Block added successfully with hash: {}", new_block.header.hash);
            Ok(())
        } else {
            Err("Failed to validate and add block".to_string())
        }
    }
    
    /// Show chain statistics
    pub fn show_stats(&self) {
        println!("=== Blockchain Statistics ===");
        println!("Total Blocks: {}", self.chain.blocks.len());
        println!("Chain Height: {}", self.chain.blocks.len() - 1);
        
        let total_transactions: usize = self.chain.blocks
            .iter()
            .map(|b| b.transactions.len())
            .sum();
        
        println!("Total Transactions: {}", total_transactions);
        
        if let Some(latest_block) = self.chain.blocks.last() {
            println!("Latest Block Hash: {}", latest_block.header.hash);
            println!("Latest Block Timestamp: {}", latest_block.header.timestamp);
        }
    }
    
    /// Get block by hash
    pub fn get_block(&self, hash: &str) -> Result<(), String> {
        match self.block_store.get_block(hash)? {
            Some(block) => {
                println!("Block found:");
                println!("  Hash: {}", block.header.hash);
                println!("  Height: {}", block.header.height);
                println!("  Previous Hash: {}", block.header.previous_hash);
                println!("  Timestamp: {}", block.header.timestamp);
                println!("  Transactions: {}", block.transactions.len());
                Ok(())
            },
            None => {
                println!("Block with hash '{}' not found", hash);
                Ok(())
            }
        }
    }
    
    /// Start network node
    pub fn start_node(&self, listen_address: String, listen_port: u16) -> Result<(), String> {
        println!("Starting network node on {}:{}...", listen_address, listen_port);
        
        let server = NetworkServer::new(self.chain.clone(), listen_address, listen_port);
        
        // Start server in a separate thread
        let server_handle = thread::spawn(move || {
            if let Err(e) = server.start() {
                eprintln!("Server error: {}", e);
            }
        });
        
        println!("Network node started. Press Ctrl+C to stop.");
        
        // Wait for the server thread (this will block until the server stops)
        if let Err(e) = server_handle.join() {
            eprintln!("Server thread error: {:?}", e);
        }
        
        Ok(())
    }
    
    /// Connect to a peer
    pub fn connect_peer(&self, address: String, port: u16) -> Result<(), String> {
        let server = NetworkServer::new(self.chain.clone(), "127.0.0.1".to_string(), 0);
        
        server.connect_to_peer(&address, port)
            .map_err(|e| format!("Failed to connect to peer: {}", e))?;
        
        println!("Successfully connected to peer at {}:{}", address, port);
        Ok(())
    }
    
    /// Mine a new block
    pub fn mine_block(&mut self, transactions: Vec<Transaction>) -> Result<(), String> {
        let previous_hash = self.chain.blocks.last()
            .map(|b| b.header.hash.clone())
            .unwrap_or_default();
        
        let height = self.chain.blocks.len() as u64;
        
        println!("Starting to mine block at height {}...", height);
        
        let result = self.mining_pool.mine_block(
            previous_hash,
            transactions,
            height,
        );
        
        // Add the mined block to the chain
        if self.chain.add_block(result.block.clone()) {
            self.block_store.store_block(&result.block)?;
            
            // Update fork choice
            match self.fork_choice.add_block(result.block.clone()) {
                Ok(_) => {
                    println!("Block successfully mined and added to chain!");
                    println!("  Hash: {}", result.hash);
                    println!("  Nonce: {}", result.nonce);
                    println!("  Attempts: {}", result.attempts);
                    println!("  Time: {}ms", result.elapsed_ms);
                    Ok(())
                },
                Err(e) => Err(format!("Failed to update fork choice: {}", e))
            }
        } else {
            Err("Failed to add mined block to chain".to_string())
        }
    }
    
    /// Show mining statistics
    pub fn show_mining_stats(&self) {
        let stats = self.mining_pool.get_stats();
        println!("=== Mining Statistics ===");
        println!("Total blocks mined: {}", stats.total_blocks_mined);
        println!("Total attempts: {}", stats.total_attempts);
        println!("Total time: {}ms", stats.total_time_ms);
        println!("Average attempts per block: {:.2}", stats.average_attempts_per_block);
        println!("Average time per block: {:.2}ms", stats.average_time_per_block_ms);
        println!("Current hash rate: {:.2} H/s", stats.current_hash_rate);
        println!("Current difficulty: {}", self.mining_pool.get_difficulty());
    }
    
    /// Show fork choice statistics
    pub fn show_fork_stats(&self) {
        let stats = self.fork_choice.get_chain_stats();
        println!("=== Fork Choice Statistics ===");
        println!("Total chains: {}", stats.total_chains);
        println!("Best chain height: {}", stats.best_chain_height);
        println!("Maximum height: {}", stats.max_height);
        println!("Total blocks: {}", stats.total_blocks);
        println!("Has forks: {}", stats.has_forks);
        
        if let Some(best_chain) = self.fork_choice.get_best_chain() {
            println!("Best chain tip: {}", 
                best_chain.blocks.last().unwrap().header.hash);
        }
    }
    
    /// Add a transaction to the mempool
    pub fn add_transaction_to_mempool(&mut self, transaction: Transaction) -> Result<(), String> {
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
    pub fn show_mempool_stats(&self) {
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
    pub fn show_pending_transactions(&self) {
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
    pub fn mine_block_from_mempool(&mut self) -> Result<(), String> {
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
    pub fn clear_mempool(&mut self) {
        let count = self.mempool.size();
        self.mempool.clear();
        println!("Cleared {} transactions from mempool.", count);
    }
    
    /// Demonstrate mempool functionality with a complete workflow
    pub fn demo_mempool(&mut self) -> Result<(), String> {
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
    
    /// Get current UTXO state from the blockchain
    fn get_current_utxo_state(&self) -> crate::blockchain::state::UTXOState {
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

impl Default for CLI {
    fn default() -> Self {
        Self::new()
    }
}