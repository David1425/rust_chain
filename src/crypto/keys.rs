use ed25519_dalek::SigningKey;
use rand::rngs::OsRng;

pub fn generate_keypair() -> SigningKey {
	let mut csprng = OsRng;
	SigningKey::generate(&mut csprng)
}

