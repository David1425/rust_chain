use crate::blockchain::block::Block;
use crate::storage::db::Database;

/// Block storage interface
pub struct BlockStore {
    db: Database,
}

impl BlockStore {
    pub fn new() -> Self {
        BlockStore {
            db: Database::new(),
        }
    }
    
    /// Store a block by its hash
    pub fn store_block(&mut self, block: &Block) -> Result<(), String> {
        let block_data = serde_json::to_vec(block)
            .map_err(|e| format!("Failed to serialize block: {}", e))?;
        
        let key = format!("block:{}", block.header.hash);
        self.db.put(key, block_data);
        
        // Also store height mapping
        let height_key = format!("height:{}", block.header.height);
        self.db.put(height_key, block.header.hash.as_bytes().to_vec());
        
        Ok(())
    }
    
    /// Retrieve a block by its hash
    pub fn get_block(&self, hash: &str) -> Result<Option<Block>, String> {
        let key = format!("block:{}", hash);
        
        match self.db.get(&key) {
            Some(block_data) => {
                let block: Block = serde_json::from_slice(block_data)
                    .map_err(|e| format!("Failed to deserialize block: {}", e))?;
                Ok(Some(block))
            },
            None => Ok(None),
        }
    }
    
    /// Get block by height
    pub fn get_block_by_height(&self, height: u64) -> Result<Option<Block>, String> {
        let height_key = format!("height:{}", height);
        
        match self.db.get(&height_key) {
            Some(hash_bytes) => {
                let hash = String::from_utf8(hash_bytes.clone())
                    .map_err(|e| format!("Invalid hash encoding: {}", e))?;
                self.get_block(&hash)
            },
            None => Ok(None),
        }
    }
    
    /// Check if a block exists
    pub fn block_exists(&self, hash: &str) -> bool {
        let key = format!("block:{}", hash);
        self.db.exists(&key)
    }
    
    /// Get all block hashes
    pub fn get_all_block_hashes(&self) -> Vec<String> {
        self.db.keys()
            .into_iter()
            .filter(|key| key.starts_with("block:"))
            .map(|key| key.strip_prefix("block:").unwrap().to_string())
            .collect()
    }
}

impl Default for BlockStore {
    fn default() -> Self {
        Self::new()
    }
}