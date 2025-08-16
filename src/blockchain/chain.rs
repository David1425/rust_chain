use crate::blockchain::block::{Block, Transaction};
use crate::blockchain::genesis::genesis_block;
use crate::storage::block_store::BlockStore;
use crate::storage::db::Database;
use serde::{Serialize, Deserialize};
use std::sync::{Arc, Mutex};

/// Persistent blockchain structure with RocksDB storage
pub struct Chain {
	pub blocks: Vec<Block>,
	block_store: Option<Arc<Mutex<BlockStore>>>,
	transaction_store: Option<Arc<Mutex<Database>>>,
	persistent: bool,
}

// Manual Clone implementation that doesn't clone the stores
impl Clone for Chain {
	fn clone(&self) -> Self {
		Chain {
			blocks: self.blocks.clone(),
			block_store: self.block_store.clone(),
			transaction_store: self.transaction_store.clone(),
			persistent: self.persistent,
		}
	}
}

/// Transaction index entry for efficient lookups
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransactionIndex {
	pub block_hash: String,
	pub block_height: u64,
	pub transaction_index: usize,
	pub from: String,
	pub to: String,
	pub amount: u64,
	pub timestamp: u64,
}

impl Chain {
	/// Create a new in-memory chain (for testing)
	pub fn new() -> Self {
		Chain { 
			blocks: vec![genesis_block()],
			block_store: None,
			transaction_store: None,
			persistent: false,
		}
	}

	/// Create a new persistent chain with storage
	pub fn new_persistent() -> Result<Self, String> {
		let block_store = BlockStore::new()?;
		let transaction_store = Database::new_with_path("./blockchain_data/transactions")
			.map_err(|e| format!("Failed to create transaction database: {}", e))?;
		
		let mut chain = Chain {
			blocks: Vec::new(),
			block_store: Some(Arc::new(Mutex::new(block_store))),
			transaction_store: Some(Arc::new(Mutex::new(transaction_store))),
			persistent: true,
		};

		// Load existing blockchain or create genesis
		chain.load_from_storage()?;
		Ok(chain)
	}

	/// Create a persistent chain with custom path
	pub fn new_persistent_with_path(path: &str) -> Result<Self, String> {
		let block_store = BlockStore::new_with_path(path)?;
		let tx_path = format!("{}/transactions", path);
		let transaction_store = Database::new_with_path(tx_path)
			.map_err(|e| format!("Failed to create transaction database: {}", e))?;
		
		let mut chain = Chain {
			blocks: Vec::new(),
			block_store: Some(Arc::new(Mutex::new(block_store))),
			transaction_store: Some(Arc::new(Mutex::new(transaction_store))),
			persistent: true,
		};

		chain.load_from_storage()?;
		Ok(chain)
	}

	/// Load blockchain from persistent storage
	fn load_from_storage(&mut self) -> Result<(), String> {
		if !self.persistent {
			return Ok(());
		}

		let block_store = self.block_store.as_ref().unwrap();
		let block_store_guard = block_store.lock()
			.map_err(|e| format!("Failed to lock block store: {}", e))?;
		
		// Check if we have any blocks stored
		match block_store_guard.get_latest_height()? {
			Some(latest_height) => {
				// Load all blocks from storage
				self.blocks.clear();
				for height in 0..=latest_height {
					if let Some(block) = block_store_guard.get_block_by_height(height)? {
						self.blocks.push(block);
					} else {
						return Err(format!("Missing block at height {}", height));
					}
				}
				println!("Loaded {} blocks from storage", self.blocks.len());
			},
			None => {
				// No blocks in storage, create and store genesis
				let genesis = genesis_block();
				self.blocks = vec![genesis.clone()];
				drop(block_store_guard); // Release lock before calling persist_block
				self.persist_block(&genesis)?;
				println!("Created new blockchain with genesis block");
			}
		}

		Ok(())
	}

	/// Add a block to the chain with persistence
	pub fn add_block(&mut self, block: Block) -> bool {
		if self.validate_block(&block) {
			// Persist the block if storage is enabled
			if self.persistent {
				if let Err(e) = self.persist_block(&block) {
					eprintln!("Failed to persist block: {}", e);
					return false;
				}
			}

			self.blocks.push(block);
			true
		} else {
			false
		}
	}

	/// Persist a block and its transactions to storage
	fn persist_block(&self, block: &Block) -> Result<(), String> {
		if !self.persistent {
			return Ok(());
		}

		let block_store = self.block_store.as_ref().unwrap();
		let tx_store = self.transaction_store.as_ref().unwrap();

		// Store the block
		{
			let block_store_guard = block_store.lock()
				.map_err(|e| format!("Failed to lock block store: {}", e))?;
			block_store_guard.store_block(block)?;
		}

		// Index all transactions in the block
		{
			let tx_store_guard = tx_store.lock()
				.map_err(|e| format!("Failed to lock transaction store: {}", e))?;

			for (tx_index, transaction) in block.transactions.iter().enumerate() {
				let tx_hash = crate::crypto::hash::sha256_hash(&format!("{:?}", transaction));
				
				// Create transaction index
				let tx_index_entry = TransactionIndex {
					block_hash: block.header.hash.clone(),
					block_height: block.header.height,
					transaction_index: tx_index,
					from: transaction.from.clone(),
					to: transaction.to.clone(),
					amount: transaction.amount,
					timestamp: block.header.timestamp,
				};

				// Store transaction by hash
				let tx_key = format!("tx:{}", tx_hash);
				let tx_data = serde_json::to_vec(&transaction)
					.map_err(|e| format!("Failed to serialize transaction: {}", e))?;
				tx_store_guard.put(tx_key, tx_data)
					.map_err(|e| format!("Failed to store transaction: {}", e))?;

				// Store transaction index
				let index_key = format!("tx_index:{}", tx_hash);
				let index_data = serde_json::to_vec(&tx_index_entry)
					.map_err(|e| format!("Failed to serialize transaction index: {}", e))?;
				tx_store_guard.put(index_key, index_data)
					.map_err(|e| format!("Failed to store transaction index: {}", e))?;

				// Index by sender address
				let from_key = format!("addr_from:{}:{}", transaction.from, tx_hash);
				tx_store_guard.put(from_key, vec![1])
					.map_err(|e| format!("Failed to index sender: {}", e))?;

				// Index by recipient address
				let to_key = format!("addr_to:{}:{}", transaction.to, tx_hash);
				tx_store_guard.put(to_key, vec![1])
					.map_err(|e| format!("Failed to index recipient: {}", e))?;
			}
		}

		Ok(())
	}

	/// Get a transaction by hash
	pub fn get_transaction(&self, tx_hash: &str) -> Result<Option<Transaction>, String> {
		if !self.persistent {
			// Search in-memory blocks
			for block in &self.blocks {
				for transaction in &block.transactions {
					let hash = crate::crypto::hash::sha256_hash(&format!("{:?}", transaction));
					if hash == tx_hash {
						return Ok(Some(transaction.clone()));
					}
				}
			}
			return Ok(None);
		}

		let tx_store = self.transaction_store.as_ref().unwrap();
		let tx_store_guard = tx_store.lock()
			.map_err(|e| format!("Failed to lock transaction store: {}", e))?;
		
		let tx_key = format!("tx:{}", tx_hash);
		
		match tx_store_guard.get(&tx_key) {
			Ok(Some(tx_data)) => {
				let transaction: Transaction = serde_json::from_slice(&tx_data)
					.map_err(|e| format!("Failed to deserialize transaction: {}", e))?;
				Ok(Some(transaction))
			},
			Ok(None) => Ok(None),
			Err(e) => Err(format!("Database error: {}", e)),
		}
	}

	/// Get transaction index information
	pub fn get_transaction_index(&self, tx_hash: &str) -> Result<Option<TransactionIndex>, String> {
		if !self.persistent {
			return Ok(None);
		}

		let tx_store = self.transaction_store.as_ref().unwrap();
		let tx_store_guard = tx_store.lock()
			.map_err(|e| format!("Failed to lock transaction store: {}", e))?;
		
		let index_key = format!("tx_index:{}", tx_hash);
		
		match tx_store_guard.get(&index_key) {
			Ok(Some(index_data)) => {
				let index: TransactionIndex = serde_json::from_slice(&index_data)
					.map_err(|e| format!("Failed to deserialize transaction index: {}", e))?;
				Ok(Some(index))
			},
			Ok(None) => Ok(None),
			Err(e) => Err(format!("Database error: {}", e)),
		}
	}

	/// Get all transactions for an address (both sent and received)
	pub fn get_transactions_for_address(&self, address: &str) -> Result<Vec<(String, Transaction)>, String> {
		let mut results = Vec::new();

		if !self.persistent {
			// Search in-memory blocks
			for block in &self.blocks {
				for transaction in &block.transactions {
					if transaction.from == address || transaction.to == address {
						let hash = crate::crypto::hash::sha256_hash(&format!("{:?}", transaction));
						results.push((hash, transaction.clone()));
					}
				}
			}
			return Ok(results);
		}

		let tx_store = self.transaction_store.as_ref().unwrap();
		let tx_store_guard = tx_store.lock()
			.map_err(|e| format!("Failed to lock transaction store: {}", e))?;
		
		// Get transactions where this address is the sender
		let from_keys = tx_store_guard.keys_with_prefix(&format!("addr_from:{}", address))
			.map_err(|e| format!("Database error: {}", e))?;
		
		// Get transactions where this address is the recipient
		let to_keys = tx_store_guard.keys_with_prefix(&format!("addr_to:{}", address))
			.map_err(|e| format!("Database error: {}", e))?;

		// Combine and extract transaction hashes
		let mut tx_hashes = std::collections::HashSet::new();
		
		for key in from_keys {
			if let Some(tx_hash) = key.split(':').last() {
				tx_hashes.insert(tx_hash.to_string());
			}
		}
		
		for key in to_keys {
			if let Some(tx_hash) = key.split(':').last() {
				tx_hashes.insert(tx_hash.to_string());
			}
		}

		// Release the guard before calling get_transaction
		drop(tx_store_guard);

		// Retrieve each transaction
		for tx_hash in tx_hashes {
			if let Some(transaction) = self.get_transaction(&tx_hash)? {
				results.push((tx_hash, transaction));
			}
		}

		Ok(results)
	}

	pub fn validate_block(&self, block: &Block) -> bool {
		let last_hash = self.blocks.last().map(|b| b.header.hash.clone()).unwrap_or_default();
		let expected_height = self.blocks.len() as u64;
		block.header.previous_hash == last_hash && block.header.height == expected_height
	}

	/// Get chain statistics
	pub fn get_stats(&self) -> ChainStats {
		let total_transactions: usize = self.blocks.iter().map(|b| b.transactions.len()).sum();
		
		ChainStats {
			total_blocks: self.blocks.len(),
			total_transactions,
			latest_block_hash: self.blocks.last().map(|b| b.header.hash.clone()).unwrap_or_default(),
			chain_height: self.blocks.len().saturating_sub(1),
			persistent: self.persistent,
		}
	}

	/// Create a chain from a vector of blocks (for fork choice)
	pub fn from_blocks(blocks: Vec<Block>) -> Self {
		Chain {
			blocks,
			block_store: None,
			transaction_store: None,
			persistent: false,
		}
	}

	/// Get the blocks (read-only access)
	pub fn get_blocks(&self) -> &[Block] {
		&self.blocks
	}
}

/// Chain statistics structure
#[derive(Debug, Serialize, Deserialize)]
pub struct ChainStats {
	pub total_blocks: usize,
	pub total_transactions: usize,
	pub latest_block_hash: String,
	pub chain_height: usize,
	pub persistent: bool,
}
