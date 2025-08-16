use crate::cli::CLI;
use crate::wallet::keychain::WalletStats;
use crate::blockchain::block::Transaction;

/// Transaction lookup and persistence commands
pub trait TransactionCommands {
    fn get_transaction(&self, tx_hash: &str) -> Result<Option<Transaction>, String>;
    fn get_transaction_info(&self, tx_hash: &str) -> Result<Option<TransactionInfo>, String>;
    fn get_address_transactions(&self, address: &str) -> Result<Vec<AddressTransaction>, String>;
    fn get_address_balance(&self, address: &str) -> Result<AddressBalance, String>;
}

impl TransactionCommands for CLI {
    /// Get a transaction by its hash
    fn get_transaction(&self, tx_hash: &str) -> Result<Option<Transaction>, String> {
        self.chain.get_transaction(tx_hash)
    }
    
    /// Get detailed transaction information including block context
    fn get_transaction_info(&self, tx_hash: &str) -> Result<Option<TransactionInfo>, String> {
        if let Some(transaction) = self.chain.get_transaction(tx_hash)? {
            let index = self.chain.get_transaction_index(tx_hash)?;
            
            Ok(Some(TransactionInfo {
                hash: tx_hash.to_string(),
                transaction,
                block_hash: index.as_ref().map(|i| i.block_hash.clone()),
                block_height: index.as_ref().map(|i| i.block_height),
                transaction_index: index.as_ref().map(|i| i.transaction_index),
                timestamp: index.as_ref().map(|i| i.timestamp),
            }))
        } else {
            Ok(None)
        }
    }
    
    /// Get all transactions for an address
    fn get_address_transactions(&self, address: &str) -> Result<Vec<AddressTransaction>, String> {
        let transactions = self.chain.get_transactions_for_address(address)?;
        
        let mut result = Vec::new();
        for (tx_hash, transaction) in transactions {
            let index = self.chain.get_transaction_index(&tx_hash)?;
            
            let is_sender = transaction.from == address;
            let is_recipient = transaction.to == address;
            
            result.push(AddressTransaction {
                hash: tx_hash,
                from: transaction.from,
                to: transaction.to,
                amount: transaction.amount,
                is_sender,
                is_recipient,
                block_hash: index.as_ref().map(|i| i.block_hash.clone()),
                block_height: index.as_ref().map(|i| i.block_height),
                timestamp: index.as_ref().map(|i| i.timestamp),
            });
        }
        
        // Sort by block height (most recent first)
        result.sort_by(|a, b| b.block_height.cmp(&a.block_height));
        
        Ok(result)
    }
    
    /// Get address balance and transaction summary
    fn get_address_balance(&self, address: &str) -> Result<AddressBalance, String> {
        let transactions = self.get_address_transactions(address)?;
        
        let mut balance: i64 = 0;
        let mut sent = 0u64;
        let mut received = 0u64;
        let tx_count = transactions.len();
        
        for tx in &transactions {
            if tx.is_sender && !tx.is_recipient {
                // Only sent
                sent += tx.amount;
                balance -= tx.amount as i64;
            } else if tx.is_recipient && !tx.is_sender {
                // Only received
                received += tx.amount;
                balance += tx.amount as i64;
            }
            // If both sender and recipient (self-transfer), balance doesn't change
        }
        
        Ok(AddressBalance {
            address: address.to_string(),
            balance: balance.max(0) as u64,
            total_sent: sent,
            total_received: received,
            transaction_count: tx_count,
        })
    }
}

/// Transaction information with block context
#[derive(Debug)]
pub struct TransactionInfo {
    pub hash: String,
    pub transaction: Transaction,
    pub block_hash: Option<String>,
    pub block_height: Option<u64>,
    pub transaction_index: Option<usize>,
    pub timestamp: Option<u64>,
}

/// Address transaction with context
#[derive(Debug)]
pub struct AddressTransaction {
    pub hash: String,
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub is_sender: bool,
    pub is_recipient: bool,
    pub block_hash: Option<String>,
    pub block_height: Option<u64>,
    pub timestamp: Option<u64>,
}

/// Address balance and summary
#[derive(Debug)]
pub struct AddressBalance {
    pub address: String,
    pub balance: u64,
    pub total_sent: u64,
    pub total_received: u64,
    pub transaction_count: usize,
}

/// Wallet management commands for Phase 8
pub trait WalletCommands {
    fn generate_new_address(&mut self) -> Result<String, String>;
    fn list_addresses(&self) -> Vec<String>;
    fn show_seed_phrase(&self) -> String;
    fn restore_from_seed(&mut self, seed_phrase: &str) -> Result<(), String>;
    fn get_wallet_stats(&self) -> WalletStats;
    fn backup_wallet(&self, path: &str) -> Result<(), String>;
    fn import_private_key(&mut self, private_key: &str) -> Result<String, String>;
}

impl WalletCommands for CLI {
    /// Generate a new address for the wallet
    fn generate_new_address(&mut self) -> Result<String, String> {
        let address = self.wallet.generate_address()?;
        
        // Save wallet after modification
        let wallet_path = "wallet.json";
        if let Err(e) = self.wallet.save_to_file(wallet_path) {
            eprintln!("Warning: Failed to save wallet: {}", e);
        }
        
        Ok(address)
    }

    /// List all addresses in the wallet
    fn list_addresses(&self) -> Vec<String> {
        self.wallet.get_all_addresses()
    }

    /// Show the wallet's seed phrase for backup
    fn show_seed_phrase(&self) -> String {
        self.wallet.get_seed_phrase().to_string()
    }

    /// Restore wallet from seed phrase
    fn restore_from_seed(&mut self, seed_phrase: &str) -> Result<(), String> {
        use crate::wallet::keychain::Wallet;
        
        let new_wallet = Wallet::from_seed_phrase(seed_phrase)?;
        self.wallet = new_wallet;
        
        // Save the restored wallet
        let wallet_path = "wallet.json";
        if let Err(e) = self.wallet.save_to_file(wallet_path) {
            eprintln!("Warning: Failed to save wallet: {}", e);
        }
        
        Ok(())
    }

    /// Get wallet statistics
    fn get_wallet_stats(&self) -> WalletStats {
        self.wallet.get_stats()
    }

    /// Backup wallet to file
    fn backup_wallet(&self, path: &str) -> Result<(), String> {
        use std::fs;
        
        let backup_data = serde_json::json!({
            "seed_phrase": self.wallet.get_seed_phrase(),
            "addresses": self.wallet.get_all_addresses(),
            "stats": self.wallet.get_stats(),
            "backup_time": std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        });
        
        fs::write(path, backup_data.to_string())
            .map_err(|e| format!("Failed to write backup: {}", e))?;
        
        Ok(())
    }

    /// Import a private key (simplified implementation)
    fn import_private_key(&mut self, _private_key: &str) -> Result<String, String> {
        // For simplicity, just generate a new address
        // In a real implementation, this would derive the address from the private key
        self.generate_new_address()
    }
}

/// Advanced blockchain analysis commands
pub trait AnalyticsCommands {
    fn analyze_chain(&self) -> ChainAnalytics;
    fn get_block_stats(&self, height: Option<u64>) -> Result<BlockStats, String>;
    fn get_transaction_stats(&self) -> TransactionStats;
    fn validate_chain_integrity(&self) -> ChainIntegrityReport;
}

impl AnalyticsCommands for CLI {
    /// Analyze the blockchain for various metrics
    fn analyze_chain(&self) -> ChainAnalytics {
        let blocks = self.chain.get_blocks();
        let total_blocks = blocks.len();
        
        let mut total_transactions = 0;
        let mut total_size = 0;
        let mut min_time = u64::MAX;
        let mut max_time = 0;
        
        for block in blocks {
            total_transactions += block.transactions.len();
            total_size += serde_json::to_string(block).unwrap_or_default().len();
            min_time = min_time.min(block.header.timestamp);
            max_time = max_time.max(block.header.timestamp);
        }
        
        let average_block_time = if total_blocks > 1 {
            (max_time - min_time) / (total_blocks as u64 - 1)
        } else {
            0
        };
        
        ChainAnalytics {
            total_blocks,
            total_transactions,
            total_size_bytes: total_size,
            average_block_time_seconds: average_block_time,
            chain_start_time: min_time,
            chain_latest_time: max_time,
        }
    }

    /// Get statistics for a specific block
    fn get_block_stats(&self, height: Option<u64>) -> Result<BlockStats, String> {
        let block = if let Some(h) = height {
            self.chain.get_blocks().get(h as usize)
                .ok_or_else(|| format!("Block at height {} not found", h))?
        } else {
            self.chain.get_blocks().last()
                .ok_or_else(|| "No blocks in chain".to_string())?
        };
        
        let block_size = serde_json::to_string(block).unwrap_or_default().len();
        
        Ok(BlockStats {
            height: height.unwrap_or(self.chain.blocks.len() as u64 - 1),
            hash: block.header.hash.clone(),
            timestamp: block.header.timestamp,
            transaction_count: block.transactions.len(),
            size_bytes: block_size,
            nonce: block.header.nonce,
            previous_hash: block.header.previous_hash.clone(),
        })
    }

    /// Get transaction statistics across the chain
    fn get_transaction_stats(&self) -> TransactionStats {
        let mut total_transactions = 0;
        let mut total_value = 0;
        let mut unique_addresses = std::collections::HashSet::new();
        
        for block in &self.chain.blocks {
            for tx in &block.transactions {
                total_transactions += 1;
                total_value += tx.amount;
                unique_addresses.insert(tx.from.clone());
                unique_addresses.insert(tx.to.clone());
            }
        }
        
        TransactionStats {
            total_transactions,
            total_value_transferred: total_value,
            unique_addresses: unique_addresses.len(),
            average_transaction_value: if total_transactions > 0 {
                total_value / total_transactions as u64
            } else {
                0
            },
        }
    }

    /// Validate the integrity of the entire blockchain
    fn validate_chain_integrity(&self) -> ChainIntegrityReport {
        let mut issues = Vec::new();
        let mut valid_blocks = 0;
        
        for (i, block) in self.chain.blocks.iter().enumerate() {
            // Check block hash
            let calculated_hash = block.calculate_hash();
            if calculated_hash != block.header.hash {
                issues.push(format!("Block {} has invalid hash: expected {}, got {}", 
                                    i, block.header.hash, calculated_hash));
                continue;
            }
            
            // Check previous hash linkage
            if i > 0 {
                let prev_block = &self.chain.blocks[i - 1];
                if block.header.previous_hash != prev_block.header.hash {
                    issues.push(format!("Block {} has invalid previous hash", i));
                    continue;
                }
            }
            
            // Check timestamp ordering
            if i > 0 {
                let prev_block = &self.chain.blocks[i - 1];
                if block.header.timestamp < prev_block.header.timestamp {
                    issues.push(format!("Block {} has timestamp before previous block", i));
                }
            }
            
            valid_blocks += 1;
        }
        
        ChainIntegrityReport {
            total_blocks: self.chain.blocks.len(),
            valid_blocks,
            is_valid: issues.is_empty(),
            issues,
        }
    }
}

/// Chain analytics data structures
#[derive(Debug)]
pub struct ChainAnalytics {
    pub total_blocks: usize,
    pub total_transactions: usize,
    pub total_size_bytes: usize,
    pub average_block_time_seconds: u64,
    pub chain_start_time: u64,
    pub chain_latest_time: u64,
}

#[derive(Debug)]
pub struct BlockStats {
    pub height: u64,
    pub hash: String,
    pub timestamp: u64,
    pub transaction_count: usize,
    pub size_bytes: usize,
    pub nonce: u64,
    pub previous_hash: String,
}

#[derive(Debug)]
pub struct TransactionStats {
    pub total_transactions: usize,
    pub total_value_transferred: u64,
    pub unique_addresses: usize,
    pub average_transaction_value: u64,
}

#[derive(Debug)]
pub struct ChainIntegrityReport {
    pub total_blocks: usize,
    pub valid_blocks: usize,
    pub issues: Vec<String>,
    pub is_valid: bool,
}
