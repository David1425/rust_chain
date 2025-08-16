use crate::blockchain::block::{Block, Transaction};
use std::time::{SystemTime, UNIX_EPOCH};

/// Proof of Work difficulty target
pub const DEFAULT_DIFFICULTY: u32 = 4; // Number of leading zeros required
pub const MAX_NONCE: u64 = u64::MAX;

/// Proof of Work mining result
#[derive(Debug, Clone)]
pub struct MiningResult {
    pub block: Block,
    pub nonce: u64,
    pub hash: String,
    pub attempts: u64,
    pub elapsed_ms: u128,
}

/// Proof of Work implementation
pub struct ProofOfWork {
    difficulty: u32,
}

impl ProofOfWork {
    /// Create new PoW instance with default difficulty
    pub fn new() -> Self {
        ProofOfWork {
            difficulty: DEFAULT_DIFFICULTY,
        }
    }
    
    /// Create new PoW instance with custom difficulty
    pub fn with_difficulty(difficulty: u32) -> Self {
        ProofOfWork { difficulty }
    }
    
    /// Mine a block using Proof of Work
    pub fn mine_block(
        &self,
        previous_hash: String,
        transactions: Vec<Transaction>,
        height: u64,
    ) -> MiningResult {
        let start_time = SystemTime::now();
        let timestamp = start_time
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let target = self.calculate_target();
        let mut attempts = 0u64;
        
        println!("Mining block with difficulty {}...", self.difficulty);
        
        for nonce in 0..MAX_NONCE {
            attempts += 1;
            
            let block = Block::new(
                previous_hash.clone(),
                transactions.clone(),
                nonce,
                timestamp,
                height,
            );
            
            if self.validate_proof(&block.header.hash, &target) {
                let elapsed = start_time.elapsed().unwrap().as_millis();
                let hash = block.header.hash.clone();
                println!("Block mined! Nonce: {}, Attempts: {}, Time: {}ms", nonce, attempts, elapsed);
                
                return MiningResult {
                    block,
                    nonce,
                    hash,
                    attempts,
                    elapsed_ms: elapsed,
                };
            }
            
            // Progress indicator for long mining sessions
            if attempts % 100000 == 0 {
                println!("Mining... attempts: {}", attempts);
            }
        }
        
        // This should theoretically never happen with reasonable difficulty
        panic!("Failed to mine block: exhausted all nonces");
    }
    
    /// Validate a block's proof of work
    pub fn validate_block(&self, block: &Block) -> bool {
        let target = self.calculate_target();
        self.validate_proof(&block.header.hash, &target)
    }
    
    /// Check if a hash meets the difficulty target
    fn validate_proof(&self, hash: &str, target: &str) -> bool {
        hash < target
    }
    
    /// Calculate the target hash for current difficulty
    fn calculate_target(&self) -> String {
        let mut target = String::from("0".repeat(self.difficulty as usize));
        target.push_str(&"f".repeat(64 - self.difficulty as usize));
        target
    }
    
    /// Get current difficulty
    pub fn get_difficulty(&self) -> u32 {
        self.difficulty
    }
    
    /// Set new difficulty
    pub fn set_difficulty(&mut self, difficulty: u32) {
        self.difficulty = difficulty;
    }
    
    /// Calculate difficulty adjustment based on block times
    /// This is a simple implementation - in a real blockchain you'd have more sophisticated logic
    pub fn adjust_difficulty(
        &mut self,
        last_blocks: &[Block],
        target_block_time_seconds: u64,
    ) -> u32 {
        if last_blocks.len() < 2 {
            return self.difficulty;
        }
        
        // Calculate average time between blocks
        let mut total_time = 0u64;
        for i in 1..last_blocks.len() {
            let time_diff = last_blocks[i].header.timestamp - last_blocks[i-1].header.timestamp;
            total_time += time_diff;
        }
        
        let avg_block_time = total_time / (last_blocks.len() - 1) as u64;
        
        // Adjust difficulty based on whether blocks are coming too fast or too slow
        if avg_block_time < target_block_time_seconds / 2 {
            // Blocks too fast, increase difficulty
            self.difficulty += 1;
        } else if avg_block_time > target_block_time_seconds * 2 {
            // Blocks too slow, decrease difficulty (but never below 1)
            if self.difficulty > 1 {
                self.difficulty -= 1;
            }
        }
        
        println!("Difficulty adjusted to {} (avg block time: {}s)", self.difficulty, avg_block_time);
        self.difficulty
    }
    
    /// Estimate mining time for current difficulty
    pub fn estimate_mining_time(&self, hash_rate_per_second: u64) -> f64 {
        let target_calculations = 16u64.pow(self.difficulty);
        target_calculations as f64 / hash_rate_per_second as f64
    }
}

impl Default for ProofOfWork {
    fn default() -> Self {
        Self::new()
    }
}

/// Mining statistics
#[derive(Debug, Clone)]
pub struct MiningStats {
    pub total_blocks_mined: u64,
    pub total_attempts: u64,
    pub total_time_ms: u128,
    pub average_attempts_per_block: f64,
    pub average_time_per_block_ms: f64,
    pub current_hash_rate: f64, // hashes per second
}

/// Mining pool for tracking mining statistics
pub struct MiningPool {
    stats: MiningStats,
    pow: ProofOfWork,
}

impl MiningPool {
    pub fn new(difficulty: u32) -> Self {
        MiningPool {
            stats: MiningStats {
                total_blocks_mined: 0,
                total_attempts: 0,
                total_time_ms: 0,
                average_attempts_per_block: 0.0,
                average_time_per_block_ms: 0.0,
                current_hash_rate: 0.0,
            },
            pow: ProofOfWork::with_difficulty(difficulty),
        }
    }
    
    pub fn mine_block(
        &mut self,
        previous_hash: String,
        transactions: Vec<Transaction>,
        height: u64,
    ) -> MiningResult {
        let result = self.pow.mine_block(previous_hash, transactions, height);
        
        // Update statistics
        self.stats.total_blocks_mined += 1;
        self.stats.total_attempts += result.attempts;
        self.stats.total_time_ms += result.elapsed_ms;
        
        self.stats.average_attempts_per_block = 
            self.stats.total_attempts as f64 / self.stats.total_blocks_mined as f64;
        
        self.stats.average_time_per_block_ms = 
            self.stats.total_time_ms as f64 / self.stats.total_blocks_mined as f64;
        
        if result.elapsed_ms > 0 {
            self.stats.current_hash_rate = 
                result.attempts as f64 / (result.elapsed_ms as f64 / 1000.0);
        } else {
            // Very fast mining, estimate based on attempts
            self.stats.current_hash_rate = result.attempts as f64 * 1000.0; // Assume 1ms
        }
        
        result
    }
    
    pub fn get_stats(&self) -> &MiningStats {
        &self.stats
    }
    
    pub fn get_difficulty(&self) -> u32 {
        self.pow.get_difficulty()
    }
    
    pub fn adjust_difficulty(&mut self, last_blocks: &[Block], target_time: u64) {
        self.pow.adjust_difficulty(last_blocks, target_time);
    }
}