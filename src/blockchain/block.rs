use serde::{Serialize, Deserialize};
use crate::crypto::hash::sha256_hash;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlockHeader {
	pub previous_hash: String,
	pub timestamp: u64,
	pub nonce: u64,
	pub merkle_root: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
	pub header: BlockHeader,
	pub transactions: Vec<String>, // TODO: Replace String with Transaction struct
	pub hash: String,
}

impl Block {
	pub fn new(previous_hash: String, transactions: Vec<String>, nonce: u64, timestamp: u64) -> Self {
		let merkle_root = "TODO".to_string(); // TODO: Implement Merkle root calculation
		let header = BlockHeader {
			previous_hash,
			timestamp,
			nonce,
			merkle_root,
		};
		let hash = sha256_hash(&format!("{:?}{:?}", &header, &transactions));
		Block { header, transactions, hash }
	}
}
