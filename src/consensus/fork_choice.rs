use crate::blockchain::block::Block;
use crate::blockchain::chain::Chain;
use std::collections::HashMap;

/// Fork choice implementation using longest chain rule
pub struct ForkChoice {
    /// All known chains by their tip hash
    chains: HashMap<String, Chain>,
    /// Current best chain hash
    best_chain_hash: Option<String>,
}

impl ForkChoice {
    /// Create new fork choice instance
    pub fn new() -> Self {
        ForkChoice {
            chains: HashMap::new(),
            best_chain_hash: None,
        }
    }
    
    /// Initialize with genesis chain
    pub fn with_genesis_chain(chain: Chain) -> Self {
        let mut fork_choice = Self::new();
        if let Some(tip) = chain.blocks.last() {
            let tip_hash = tip.header.hash.clone();
            fork_choice.chains.insert(tip_hash.clone(), chain);
            fork_choice.best_chain_hash = Some(tip_hash);
        }
        fork_choice
    }
    
    /// Add a new block and potentially update the best chain
    pub fn add_block(&mut self, block: Block) -> Result<bool, String> {
        let block_hash = block.header.hash.clone();
        let parent_hash = block.header.previous_hash.clone();
        
        // Special handling for genesis blocks (parent hash is "0")
        if parent_hash == "0" {
            let chain = Chain::from_blocks(vec![block]);
            let is_new_best = self.is_better_chain(&chain);
            
            self.chains.insert(block_hash.clone(), chain);
            
            if is_new_best {
                self.best_chain_hash = Some(block_hash);
            }
            
            return Ok(is_new_best);
        }
        
        // Find the parent chain
        let parent_chain = self.find_chain_with_tip(&parent_hash);
        
        match parent_chain {
            Some(mut chain) => {
                // Validate the block against the parent chain
                if !chain.validate_block(&block) {
                    return Err(format!("Invalid block: {}", block_hash));
                }
                
                // Add block to the chain
                chain.add_block(block);
                
                // Check if this is now the best chain
                let is_new_best = self.is_better_chain(&chain);
                
                // Update chains
                self.chains.insert(block_hash.clone(), chain);
                
                // Remove the old chain tip if it exists
                if parent_hash != "0" { // Don't remove genesis
                    self.chains.remove(&parent_hash);
                }
                
                if is_new_best {
                    self.best_chain_hash = Some(block_hash);
                }
                
                Ok(is_new_best)
            },
            None => {
                Err(format!("Parent block not found: {}", parent_hash))
            }
        }
    }
    
    /// Get the current best chain
    pub fn get_best_chain(&self) -> Option<&Chain> {
        match &self.best_chain_hash {
            Some(hash) => self.chains.get(hash),
            None => None,
        }
    }
    
    /// Get the current best chain (mutable)
    pub fn get_best_chain_mut(&mut self) -> Option<&mut Chain> {
        match &self.best_chain_hash {
            Some(hash) => {
                let hash = hash.clone(); // Clone to avoid borrowing issues
                self.chains.get_mut(&hash)
            },
            None => None,
        }
    }
    
    /// Get all known chains
    pub fn get_all_chains(&self) -> Vec<&Chain> {
        self.chains.values().collect()
    }
    
    /// Get chain by tip hash
    pub fn get_chain_by_tip(&self, tip_hash: &str) -> Option<&Chain> {
        self.chains.get(tip_hash)
    }
    
    /// Check if we have a specific block
    pub fn has_block(&self, block_hash: &str) -> bool {
        for chain in self.chains.values() {
            if chain.blocks.iter().any(|b| b.header.hash == block_hash) {
                return true;
            }
        }
        false
    }
    
    /// Get a specific block by hash
    pub fn get_block(&self, block_hash: &str) -> Option<&Block> {
        for chain in self.chains.values() {
            if let Some(block) = chain.blocks.iter().find(|b| b.header.hash == block_hash) {
                return Some(block);
            }
        }
        None
    }
    
    /// Handle chain reorganization
    pub fn handle_reorg(&mut self, new_blocks: Vec<Block>) -> Result<bool, String> {
        let mut reorg_occurred = false;
        
        for block in new_blocks {
            match self.add_block(block) {
                Ok(is_new_best) => {
                    if is_new_best {
                        reorg_occurred = true;
                        println!("Chain reorganization occurred!");
                    }
                },
                Err(e) => {
                    println!("Failed to add block during reorg: {}", e);
                }
            }
        }
        
        Ok(reorg_occurred)
    }
    
    /// Get chain statistics
    pub fn get_chain_stats(&self) -> ForkChoiceStats {
        let total_chains = self.chains.len();
        let best_height = self.get_best_chain()
            .map(|c| c.blocks.len() as u64 - 1)
            .unwrap_or(0);
        
        let mut max_height = 0;
        let mut total_blocks = 0;
        
        for chain in self.chains.values() {
            let height = chain.blocks.len() as u64 - 1;
            if height > max_height {
                max_height = height;
            }
            total_blocks += chain.blocks.len();
        }
        
        ForkChoiceStats {
            total_chains,
            best_chain_height: best_height,
            max_height,
            total_blocks,
            has_forks: total_chains > 1,
        }
    }
    
    /// Find chain that ends with the given block hash
    fn find_chain_with_tip(&self, tip_hash: &str) -> Option<Chain> {
        // First check if we have a chain ending with this hash
        if let Some(chain) = self.chains.get(tip_hash) {
            return Some(chain.clone());
        }
        
        // If not, look for a chain that contains this block
        for chain in self.chains.values() {
            if chain.blocks.iter().any(|b| b.header.hash == tip_hash) {
                // Create a new chain up to this block
                let mut new_chain_blocks = Vec::new();
                for block in &chain.blocks {
                    new_chain_blocks.push(block.clone());
                    if block.header.hash == tip_hash {
                        break;
                    }
                }
                return Some(Chain::from_blocks(new_chain_blocks));
            }
        }
        
        None
    }
    
    /// Determine if a chain is better than the current best chain
    fn is_better_chain(&self, chain: &Chain) -> bool {
        match self.get_best_chain() {
            Some(current_best) => {
                // Longest chain rule: more blocks wins
                let new_height = chain.blocks.len();
                let current_height = current_best.blocks.len();
                
                if new_height > current_height {
                    return true;
                }
                
                // If same length, use most work (sum of difficulty)
                if new_height == current_height {
                    // For now, just use the newer timestamp as tiebreaker
                    if let (Some(new_tip), Some(current_tip)) = (chain.blocks.last(), current_best.blocks.last()) {
                        return new_tip.header.timestamp > current_tip.header.timestamp;
                    }
                }
                
                false
            },
            None => true, // First chain is always the best
        }
    }
}

impl Default for ForkChoice {
    fn default() -> Self {
        Self::new()
    }
}

/// Fork choice statistics
#[derive(Debug, Clone)]
pub struct ForkChoiceStats {
    pub total_chains: usize,
    pub best_chain_height: u64,
    pub max_height: u64,
    pub total_blocks: usize,
    pub has_forks: bool,
}

/// Chain reorganization event
#[derive(Debug, Clone)]
pub struct ReorgEvent {
    pub old_tip: String,
    pub new_tip: String,
    pub depth: u64,
    pub added_blocks: Vec<String>,
    pub removed_blocks: Vec<String>,
}

/// Enhanced fork choice with reorganization tracking
pub struct ForkChoiceWithReorg {
    fork_choice: ForkChoice,
    reorg_history: Vec<ReorgEvent>,
    max_reorg_depth: u64,
}

impl ForkChoiceWithReorg {
    pub fn new(max_reorg_depth: u64) -> Self {
        ForkChoiceWithReorg {
            fork_choice: ForkChoice::new(),
            reorg_history: Vec::new(),
            max_reorg_depth,
        }
    }
    
    pub fn add_block(&mut self, block: Block) -> Result<Option<ReorgEvent>, String> {
        let old_best = self.fork_choice.get_best_chain()
            .and_then(|c| c.blocks.last())
            .map(|b| b.header.hash.clone());
        
        let is_new_best = self.fork_choice.add_block(block.clone())?;
        
        if is_new_best {
            let new_best = Some(block.header.hash.clone());
            
            if let (Some(old), Some(new)) = (old_best, new_best) {
                if old != new {
                    // A reorganization occurred
                    let reorg_event = ReorgEvent {
                        old_tip: old,
                        new_tip: new,
                        depth: 1, // Simplified for now
                        added_blocks: vec![block.header.hash],
                        removed_blocks: vec![], // Simplified for now
                    };
                    
                    self.reorg_history.push(reorg_event.clone());
                    
                    // Keep reorg history bounded
                    if self.reorg_history.len() > self.max_reorg_depth as usize {
                        self.reorg_history.remove(0);
                    }
                    
                    return Ok(Some(reorg_event));
                }
            }
        }
        
        Ok(None)
    }
    
    pub fn get_best_chain(&self) -> Option<&Chain> {
        self.fork_choice.get_best_chain()
    }
    
    pub fn get_reorg_history(&self) -> &Vec<ReorgEvent> {
        &self.reorg_history
    }
    
    pub fn get_stats(&self) -> ForkChoiceStats {
        self.fork_choice.get_chain_stats()
    }
}