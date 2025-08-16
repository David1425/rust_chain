use ed25519_dalek::{Keypair, PublicKey, SecretKey, Signature, Signer};
use rand::rngs::OsRng;

pub fn generate_keypair() -> Keypair {
	Keypair::generate(&mut OsRng)
}

// TODO: Implement signature verification and transaction signing in Phase 2
