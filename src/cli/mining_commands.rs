use crate::blockchain::block::Transaction;
use crate::cli::CLI;

/// Trait for mining-related commands
pub trait MiningCommands {
    fn mine_block(&mut self, transactions: Vec<Transaction>) -> Result<(), String>;
    fn show_mining_stats(&self);
    fn show_fork_stats(&self);
}

impl MiningCommands for CLI {
    /// Mine a new block
    fn mine_block(&mut self, transactions: Vec<Transaction>) -> Result<(), String> {
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
    fn show_mining_stats(&self) {
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
    fn show_fork_stats(&self) {
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
}
