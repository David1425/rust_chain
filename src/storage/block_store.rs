use crate::blockchain::block::Block;
use crate::storage::db::Database;

/// Block storage interface using RocksDB
pub struct BlockStore {
    db: Database,
}

impl BlockStore {
    /// Create a new BlockStore with default database path
    pub fn new() -> Result<Self, String> {
        let db = Database::new()
            .map_err(|e| format!("Failed to create database: {}", e))?;
        
        Ok(BlockStore { db })
    }
    
    /// Create a new BlockStore with custom database path
    pub fn new_with_path(path: &str) -> Result<Self, String> {
        let db = Database::new_with_path(path)
            .map_err(|e| format!("Failed to create database at {}: {}", path, e))?;
        
        Ok(BlockStore { db })
    }
    
    /// Store a block by its hash
    pub fn store_block(&self, block: &Block) -> Result<(), String> {
        let block_data = serde_json::to_vec(block)
            .map_err(|e| format!("Failed to serialize block: {}", e))?;
        
        let key = format!("block:{}", block.header.hash);
        self.db.put(key, block_data)
            .map_err(|e| format!("Failed to store block: {}", e))?;
        
        // Also store height mapping
        let height_key = format!("height:{}", block.header.height);
        self.db.put(height_key, block.header.hash.as_bytes().to_vec())
            .map_err(|e| format!("Failed to store height mapping: {}", e))?;
        
        // Store latest block height
        let latest_key = "latest_height".to_string();
        self.db.put(latest_key, block.header.height.to_be_bytes().to_vec())
            .map_err(|e| format!("Failed to store latest height: {}", e))?;
        
        Ok(())
    }
    
    /// Retrieve a block by its hash
    pub fn get_block(&self, hash: &str) -> Result<Option<Block>, String> {
        let key = format!("block:{}", hash);
        
        match self.db.get(&key) {
            Ok(Some(block_data)) => {
                let block: Block = serde_json::from_slice(&block_data)
                    .map_err(|e| format!("Failed to deserialize block: {}", e))?;
                Ok(Some(block))
            },
            Ok(None) => Ok(None),
            Err(e) => Err(format!("Database error: {}", e)),
        }
    }
    
    /// Get block by height
    pub fn get_block_by_height(&self, height: u64) -> Result<Option<Block>, String> {
        let height_key = format!("height:{}", height);
        
        match self.db.get(&height_key) {
            Ok(Some(hash_bytes)) => {
                let hash = String::from_utf8(hash_bytes)
                    .map_err(|e| format!("Invalid hash encoding: {}", e))?;
                self.get_block(&hash)
            },
            Ok(None) => Ok(None),
            Err(e) => Err(format!("Database error: {}", e)),
        }
    }
    
    /// Check if a block exists
    pub fn block_exists(&self, hash: &str) -> Result<bool, String> {
        let key = format!("block:{}", hash);
        self.db.exists(&key)
            .map_err(|e| format!("Database error: {}", e))
    }
    
    /// Get all block hashes
    pub fn get_all_block_hashes(&self) -> Result<Vec<String>, String> {
        let keys = self.db.keys_with_prefix("block:")
            .map_err(|e| format!("Database error: {}", e))?;
        
        Ok(keys.into_iter()
            .filter_map(|key| key.strip_prefix("block:").map(|s| s.to_string()))
            .collect())
    }
    
    /// Get the latest block height
    pub fn get_latest_height(&self) -> Result<Option<u64>, String> {
        match self.db.get("latest_height") {
            Ok(Some(height_bytes)) => {
                if height_bytes.len() == 8 {
                    let height_array: [u8; 8] = height_bytes.try_into()
                        .map_err(|_| "Invalid height data length".to_string())?;
                    Ok(Some(u64::from_be_bytes(height_array)))
                } else {
                    Err("Invalid height data".to_string())
                }
            },
            Ok(None) => Ok(None),
            Err(e) => Err(format!("Database error: {}", e)),
        }
    }
    
    /// Store multiple blocks in a batch operation
    pub fn store_blocks_batch(&self, blocks: &[Block]) -> Result<(), String> {
        let mut operations = Vec::new();
        let mut latest_height = 0;
        
        for block in blocks {
            let block_data = serde_json::to_vec(block)
                .map_err(|e| format!("Failed to serialize block: {}", e))?;
            
            let key = format!("block:{}", block.header.hash);
            operations.push((key, block_data));
            
            let height_key = format!("height:{}", block.header.height);
            operations.push((height_key, block.header.hash.as_bytes().to_vec()));
            
            latest_height = latest_height.max(block.header.height);
        }
        
        // Add latest height update
        operations.push(("latest_height".to_string(), latest_height.to_be_bytes().to_vec()));
        
        self.db.batch_put(operations)
            .map_err(|e| format!("Failed to store blocks in batch: {}", e))
    }
    
    /// Get database statistics
    pub fn get_stats(&self) -> Result<crate::storage::db::DatabaseStats, String> {
        self.db.stats()
            .map_err(|e| format!("Failed to get database stats: {}", e))
    }
    
    /// Compact the database
    pub fn compact(&self) -> Result<(), String> {
        self.db.compact()
            .map_err(|e| format!("Failed to compact database: {}", e))
    }
}

impl Default for BlockStore {
    fn default() -> Self {
        Self::new().expect("Failed to create default BlockStore")
    }
}
