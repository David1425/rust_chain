use rocksdb::{DB, Options, Error};
use std::path::Path;

/// RocksDB-based persistent key-value storage
/// Upgraded from in-memory HashMap to persistent RocksDB
pub struct Database {
    db: DB,
}

impl Database {
    /// Create a new database instance with default path
    pub fn new() -> Result<Self, Error> {
        Self::new_with_path("./blockchain_data")
    }
    
    /// Create a new database instance with custom path
    pub fn new_with_path<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        
        // Optimize for blockchain use case
        opts.set_max_open_files(10000);
        opts.set_use_fsync(false);
        opts.set_bytes_per_sync(1048576);
        
        let db = DB::open(&opts, path)?;
        
        Ok(Database { db })
    }
    
    /// Store a key-value pair
    pub fn put(&self, key: String, value: Vec<u8>) -> Result<(), Error> {
        self.db.put(key.as_bytes(), value)
    }
    
    /// Retrieve a value by key
    pub fn get(&self, key: &str) -> Result<Option<Vec<u8>>, Error> {
        self.db.get(key.as_bytes())
    }
    
    /// Delete a key-value pair
    pub fn delete(&self, key: &str) -> Result<bool, Error> {
        match self.db.delete(key.as_bytes()) {
            Ok(_) => Ok(true),
            Err(e) => Err(e),
        }
    }
    
    /// Check if a key exists
    pub fn exists(&self, key: &str) -> Result<bool, Error> {
        match self.db.get(key.as_bytes()) {
            Ok(Some(_)) => Ok(true),
            Ok(None) => Ok(false),
            Err(e) => Err(e),
        }
    }
    
    /// Get all keys (use with caution on large datasets)
    pub fn keys(&self) -> Result<Vec<String>, Error> {
        let mut keys = Vec::new();
        let iter = self.db.iterator(rocksdb::IteratorMode::Start);
        
        for item in iter {
            let (key, _) = item?;
            keys.push(String::from_utf8_lossy(&key).to_string());
        }
        
        Ok(keys)
    }
    
    /// Get keys with a specific prefix
    pub fn keys_with_prefix(&self, prefix: &str) -> Result<Vec<String>, Error> {
        let mut keys = Vec::new();
        let iter = self.db.prefix_iterator(prefix.as_bytes());
        
        for item in iter {
            let (key, _) = item?;
            keys.push(String::from_utf8_lossy(&key).to_string());
        }
        
        Ok(keys)
    }
    
    /// Batch operations for better performance
    pub fn batch_put(&self, operations: Vec<(String, Vec<u8>)>) -> Result<(), Error> {
        use rocksdb::WriteBatch;
        
        let mut batch = WriteBatch::default();
        for (key, value) in operations {
            batch.put(key.as_bytes(), value);
        }
        
        self.db.write(batch)
    }
    
    /// Get database statistics
    pub fn stats(&self) -> Result<DatabaseStats, Error> {
        let db_stats = self.db.property_value("rocksdb.stats")?;
        let size_estimate = self.db.property_int_value("rocksdb.estimate-live-data-size")?;
        let num_keys = self.db.property_int_value("rocksdb.estimate-num-keys")?;
        
        Ok(DatabaseStats {
            estimated_size_bytes: size_estimate.unwrap_or(0),
            estimated_keys: num_keys.unwrap_or(0),
            stats_string: db_stats.unwrap_or_else(|| "No stats available".to_string()),
        })
    }
    
    /// Compact the database to reclaim space
    pub fn compact(&self) -> Result<(), Error> {
        self.db.compact_range::<&[u8], &[u8]>(None, None);
        Ok(())
    }
}

impl Default for Database {
    fn default() -> Self {
        Self::new().expect("Failed to create default database")
    }
}

/// Database statistics
#[derive(Debug)]
pub struct DatabaseStats {
    pub estimated_size_bytes: u64,
    pub estimated_keys: u64,
    pub stats_string: String,
}