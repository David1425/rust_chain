use crate::crypto::keys::generate_keypair;
use std::collections::HashMap;
use sha2::{Sha256, Digest};
use rand::RngCore;

/// HD Wallet implementing simplified hierarchical deterministic key generation
pub struct Wallet {
    /// Master seed for key derivation
    master_seed: [u8; 32],
    /// Generated addresses with their derivation paths
    addresses: HashMap<String, u32>,
    /// Current address index for key derivation
    current_index: u32,
    /// Mnemonic-like seed phrase (simplified)
    seed_phrase: String,
}

impl Wallet {
    /// Create a new HD wallet with a random seed
    pub fn new() -> Self {
        let mut seed = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut seed);
        
        // Generate a simple seed phrase (simplified version of BIP-39)
        let seed_phrase = Self::generate_seed_phrase(&seed);
        
        Wallet {
            master_seed: seed,
            addresses: HashMap::new(),
            current_index: 0,
            seed_phrase,
        }
    }

    /// Create HD wallet from existing seed
    pub fn from_seed(seed: [u8; 32]) -> Self {
        let seed_phrase = Self::generate_seed_phrase(&seed);
        
        Wallet {
            master_seed: seed,
            addresses: HashMap::new(),
            current_index: 0,
            seed_phrase,
        }
    }

    /// Create HD wallet from seed phrase
    pub fn from_seed_phrase(phrase: &str) -> Result<Self, String> {
        let seed = Self::seed_from_phrase(phrase)?;
        Ok(Self::from_seed(seed))
    }

    /// Get the seed phrase for wallet backup
    pub fn get_seed_phrase(&self) -> &str {
        &self.seed_phrase
    }

    /// Generate a deterministic seed phrase from seed (simplified)
    fn generate_seed_phrase(seed: &[u8; 32]) -> String {
        // Simple word list for demonstration (in real implementation, use BIP-39 wordlist)
        let words = [
            "abandon", "ability", "able", "about", "above", "absent", "absorb", "abstract",
            "absurd", "abuse", "access", "accident", "account", "accuse", "achieve", "acid",
            "acoustic", "acquire", "across", "act", "action", "actor", "actress", "actual",
            "adapt", "add", "addict", "address", "adjust", "admit", "adult", "advance",
        ];
        
        // Convert seed to word indices deterministically
        let mut phrase_words = Vec::new();
        for i in 0..8 {
            let start_byte = i * 4;
            let chunk_bytes = [
                seed[start_byte],
                seed[start_byte + 1],
                seed[start_byte + 2],
                seed[start_byte + 3],
            ];
            let index = u32::from_be_bytes(chunk_bytes) as usize % words.len();
            phrase_words.push(words[index]);
        }
        
        phrase_words.join(" ")
    }

    /// Convert seed phrase back to seed (simplified)
    fn seed_from_phrase(phrase: &str) -> Result<[u8; 32], String> {
        let words: Vec<&str> = phrase.split_whitespace().collect();
        if words.len() != 8 {
            return Err("Seed phrase must contain exactly 8 words".to_string());
        }
        
        // Same word list used for generation
        let word_list = [
            "abandon", "ability", "able", "about", "above", "absent", "absorb", "abstract",
            "absurd", "abuse", "access", "accident", "account", "accuse", "achieve", "acid",
            "acoustic", "acquire", "across", "act", "action", "actor", "actress", "actual",
            "adapt", "add", "addict", "address", "adjust", "admit", "adult", "advance",
        ];
        
        // Convert words back to indices and then to bytes
        let mut seed = [0u8; 32];
        for (i, word) in words.iter().enumerate() {
            let index = word_list.iter().position(|&w| w == *word)
                .ok_or_else(|| format!("Unknown word in seed phrase: {}", word))?;
            
            let index_bytes = (index as u32).to_be_bytes();
            let start_byte = i * 4;
            seed[start_byte] = index_bytes[0];
            seed[start_byte + 1] = index_bytes[1];
            seed[start_byte + 2] = index_bytes[2];
            seed[start_byte + 3] = index_bytes[3];
        }
        
        Ok(seed)
    }

    /// Generate a new address using deterministic key derivation
    pub fn generate_address(&mut self) -> Result<String, String> {
        let derived_key = self.derive_key(self.current_index)?;
        let address = hex::encode(&derived_key);
        
        self.addresses.insert(address.clone(), self.current_index);
        self.current_index += 1;
        
        Ok(address)
    }

    /// Derive a key for a specific index using HMAC-based derivation
    fn derive_key(&self, index: u32) -> Result<[u8; 32], String> {
        let mut hasher = Sha256::new();
        hasher.update(&self.master_seed);
        hasher.update(&index.to_be_bytes());
        hasher.update(b"blockchain_wallet_derivation");
        
        let hash = hasher.finalize();
        let mut key = [0u8; 32];
        key.copy_from_slice(&hash);
        Ok(key)
    }

    /// Get the current primary address (generates one if none exists)
    pub fn address(&mut self) -> String {
        if self.addresses.is_empty() {
            self.generate_address().expect("Failed to generate address")
        } else {
            // Return the first address (index 0)
            self.get_address_by_index(0).expect("Address should exist")
        }
    }

    /// Get the current primary address without mutation (read-only)
    pub fn get_current_address(&self) -> Option<String> {
        if self.addresses.is_empty() {
            None
        } else {
            self.get_address_by_index(0)
        }
    }

    /// Get a new address for read-only contexts (generates deterministically)
    pub fn get_new_address_readonly(&self) -> String {
        let derived_key = self.derive_key(self.current_index).expect("Key derivation failed");
        hex::encode(&derived_key)
    }

    /// Get master seed (for internal use)
    pub fn get_master_seed(&self) -> [u8; 32] {
        self.master_seed
    }

    /// Get address by derivation index
    pub fn get_address_by_index(&self, index: u32) -> Option<String> {
        self.addresses.iter()
            .find(|(_, addr_index)| **addr_index == index)
            .map(|(address, _)| address.clone())
    }

    /// Get all generated addresses
    pub fn get_all_addresses(&self) -> Vec<String> {
        let mut addresses: Vec<_> = self.addresses.iter().collect();
        addresses.sort_by_key(|(_, index)| *index);
        addresses.into_iter().map(|(addr, _)| addr.clone()).collect()
    }

    /// Get the derived private key for a specific address
    pub fn get_private_key(&self, address: &str) -> Result<[u8; 32], String> {
        let index = self.addresses.get(address)
            .ok_or_else(|| "Address not found in wallet".to_string())?;
        
        self.derive_key(*index)
    }

    /// Get wallet statistics
    pub fn get_stats(&self) -> WalletStats {
        WalletStats {
            total_addresses: self.addresses.len(),
            next_index: self.current_index,
            master_fingerprint: hex::encode(&self.master_seed[..8]),
        }
    }

    /// Legacy method for backwards compatibility
    pub fn legacy_new() -> LegacyWallet {
        let keypair = generate_keypair();
        let address = hex::encode(keypair.verifying_key().as_bytes());
        LegacyWallet { address }
    }
}

/// Legacy wallet structure for backwards compatibility
pub struct LegacyWallet {
    pub address: String,
}

/// Wallet statistics
#[derive(Debug, serde::Serialize)]
pub struct WalletStats {
    pub total_addresses: usize,
    pub next_index: u32,
    pub master_fingerprint: String,
}
