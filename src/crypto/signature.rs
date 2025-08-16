use ed25519_dalek::{VerifyingKey, Signature, Verifier};

pub fn verify_signature(public_key: &VerifyingKey, message: &[u8], signature: &[u8]) -> bool {
    if signature.len() != 64 {
        return false;
    }
    
    let sig_array: [u8; 64] = match signature.try_into() {
        Ok(arr) => arr,
        Err(_) => return false,
    };
    
    let sig = Signature::from_bytes(&sig_array);
    public_key.verify(message, &sig).is_ok()
}