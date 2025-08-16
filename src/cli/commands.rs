use crate::blockchain::chain::Chain;
use crate::blockchain::block::{Block, Transaction};
use crate::storage::block_store::BlockStore;
use crate::network::server::NetworkServer;
use std::time::{SystemTime, UNIX_EPOCH};
use std::thread;

pub struct CLI {
    chain: Chain,
    block_store: BlockStore,
}

impl CLI {
    pub fn new() -> Self {
        CLI {
            chain: Chain::new(),
            block_store: BlockStore::new(),
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
}

impl Default for CLI {
    fn default() -> Self {
        Self::new()
    }
}