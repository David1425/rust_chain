use crate::blockchain::block::{Block, Transaction};

/// Genesis block configuration
pub struct GenesisConfig {
    pub total_supply: u64,
    pub initial_allocations: Vec<(String, u64)>,
    pub genesis_message: String,
    pub timestamp: u64,
}

impl Default for GenesisConfig {
    fn default() -> Self {
        GenesisConfig {
            total_supply: 21_000_000, // Similar to Bitcoin's max supply
            initial_allocations: vec![
                ("1RustChainFoundation".to_string(), 2_100_000), // 10% to foundation
                ("1EarlyInvestors".to_string(), 1_050_000),      // 5% to early investors
                ("1Development".to_string(), 2_100_000),         // 10% to development
                ("1Community".to_string(), 15_750_000),          // 75% to community/mining
            ],
            genesis_message: "RustChain Genesis Block - A decentralized blockchain built in Rust".to_string(),
            timestamp: 1723804800, // August 16, 2024 00:00:00 UTC (example launch date)
        }
    }
}

/// Create a coinbase transaction (creates new coins from nothing)
fn create_coinbase_transaction(to: &str, amount: u64, message: Option<String>) -> Transaction {
    Transaction {
        from: "0000000000000000000000000000000000000000".to_string(), // Null address for coinbase
        to: to.to_string(),
        amount,
        signature: message.unwrap_or_default().into_bytes(), // Use signature field for genesis message
    }
}

/// Check if a transaction is a genesis message transaction
pub fn is_genesis_message_transaction(tx: &Transaction) -> bool {
    tx.from == "0000000000000000000000000000000000000000" && 
    tx.to == "0000000000000000000000000000000000000000" && 
    tx.amount == 0 && 
    !tx.signature.is_empty()
}

/// Extract genesis message from a transaction
pub fn get_genesis_message(tx: &Transaction) -> Option<String> {
    if is_genesis_message_transaction(tx) {
        String::from_utf8(tx.signature.clone()).ok()
    } else {
        None
    }
}

pub fn genesis_block() -> Block {
    genesis_block_with_config(GenesisConfig::default())
}

pub fn genesis_block_with_config(config: GenesisConfig) -> Block {
    let mut transactions = Vec::new();
    
    // Create coinbase transactions for initial allocations
    for (address, amount) in config.initial_allocations {
        let tx = create_coinbase_transaction(&address, amount, None);
        transactions.push(tx);
    }
    
    // Add a special transaction with the genesis message
    if !config.genesis_message.is_empty() {
        let message_tx = create_coinbase_transaction(
            "0000000000000000000000000000000000000000", // Burn address
            0, // No coins
            Some(config.genesis_message)
        );
        transactions.push(message_tx);
    }
    
    Block::new(
        "0000000000000000000000000000000000000000000000000000000000000000".to_string(), // 64 zeros
        transactions,
        0, // Genesis nonce is always 0
        config.timestamp,
        0, // Genesis block is at height 0
    )
}
