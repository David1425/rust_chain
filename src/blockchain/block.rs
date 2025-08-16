use serde::{Serialize, Deserialize};
use crate::crypto::hash::sha256_hash;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
	pub from: String,
	pub to: String,
	pub amount: u64,
	pub signature: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlockHeader {
	pub previous_hash: String,
	pub timestamp: u64,
	pub nonce: u64,
	pub merkle_root: String,
	pub hash: String,
	pub height: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
	pub header: BlockHeader,
	pub transactions: Vec<Transaction>,
}

fn calculate_merkle_root(transactions: &Vec<Transaction>) -> String {
	if transactions.is_empty() {
		return sha256_hash("");
	}
	let mut hashes: Vec<String> = transactions.iter()
		.map(|tx| sha256_hash(&format!("{:?}", tx)))
		.collect();
	while hashes.len() > 1 {
		let mut next_level = Vec::new();
		for i in (0..hashes.len()).step_by(2) {
			let left = &hashes[i];
			let right = if i + 1 < hashes.len() { &hashes[i + 1] } else { left };
			next_level.push(sha256_hash(&(left.clone() + right)));
		}
		hashes = next_level;
	}
	hashes[0].clone()
}

impl Block {
	pub fn new(previous_hash: String, transactions: Vec<Transaction>, nonce: u64, timestamp: u64, height: u64) -> Self {
		let merkle_root = calculate_merkle_root(&transactions);
		let mut header = BlockHeader {
			previous_hash,
			timestamp,
			nonce,
			merkle_root,
			hash: String::new(), // Will be calculated below
			height,
		};
		header.hash = sha256_hash(&format!("{:?}{:?}", &header, &transactions));
		Block { header, transactions }
	}

	/// Calculate the hash of this block (matches the original calculation)
	pub fn calculate_hash(&self) -> String {
		// Recreate the header without the hash field for calculation
		let temp_header = BlockHeader {
			previous_hash: self.header.previous_hash.clone(),
			timestamp: self.header.timestamp,
			nonce: self.header.nonce,
			merkle_root: self.header.merkle_root.clone(),
			hash: String::new(), // Empty hash for calculation
			height: self.header.height,
		};
		sha256_hash(&format!("{:?}{:?}", &temp_header, &self.transactions))
	}
}
