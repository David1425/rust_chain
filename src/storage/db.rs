use std::collections::HashMap;

/// Simple in-memory key-value storage
/// TODO: Replace with RocksDB in future phases
pub struct Database {
    data: HashMap<String, Vec<u8>>,
}

impl Database {
    pub fn new() -> Self {
        Database {
            data: HashMap::new(),
        }
    }
    
    pub fn put(&mut self, key: String, value: Vec<u8>) {
        self.data.insert(key, value);
    }
    
    pub fn get(&self, key: &str) -> Option<&Vec<u8>> {
        self.data.get(key)
    }
    
    pub fn delete(&mut self, key: &str) -> bool {
        self.data.remove(key).is_some()
    }
    
    pub fn exists(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }
    
    pub fn keys(&self) -> Vec<String> {
        self.data.keys().cloned().collect()
    }
}

impl Default for Database {
    fn default() -> Self {
        Self::new()
    }
}