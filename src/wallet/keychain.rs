use crate::crypto::keys::generate_keypair;

pub struct Wallet {
	pub address: String,
}

impl Wallet {
	pub fn new() -> Self {
		let keypair = generate_keypair();
	let address = hex::encode(keypair.verifying_key().as_bytes());
		Wallet { address }
	}
}
