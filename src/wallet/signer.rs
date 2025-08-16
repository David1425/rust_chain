use ed25519_dalek::{SigningKey, Signer};

pub fn sign_message(signing_key: &SigningKey, message: &[u8]) -> Vec<u8> {
	let signature = signing_key.sign(message);
	signature.to_bytes().to_vec()
}

