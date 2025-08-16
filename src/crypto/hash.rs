use sha2::{Sha256, Digest};

pub fn sha256_hash(data: &str) -> String {
	let mut hasher = Sha256::new();
	hasher.update(data.as_bytes());
	let result = hasher.finalize();
	hex::encode(result)
}
