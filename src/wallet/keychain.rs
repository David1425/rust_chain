use crate::crypto::keys::generate_keypair;
use std::collections::HashMap;
use sha2::{Sha256, Digest};
use bip39::{Mnemonic, Language};
use serde::{Serialize, Deserialize};
use std::fs;
use std::path::Path;

/// HD Wallet implementing simplified hierarchical deterministic key generation
#[derive(Serialize, Deserialize, Clone)]
pub struct Wallet {
    /// Master seed for key derivation
    #[serde(skip)]
    master_seed: [u8; 32],
    /// Master seed as hex string for persistence
    master_seed_hex: String,
    /// Generated addresses with their derivation paths
    addresses: HashMap<String, u32>,
    /// Current address index for key derivation
    current_index: u32,
    /// Mnemonic-like seed phrase (simplified)
    seed_phrase: String,
}

impl Wallet {
    /// Create a new HD wallet with a random BIP-39 mnemonic
    pub fn new() -> Self {
        use rand::RngCore;
        let mut entropy = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut entropy);
        let mnemonic = Mnemonic::from_entropy(&entropy).expect("Failed to generate mnemonic");
        let seed = mnemonic.to_seed_normalized("");
        let mut master_seed = [0u8; 32];
        master_seed.copy_from_slice(&seed[..32]);
        Wallet {
            master_seed,
            master_seed_hex: hex::encode(master_seed),
            addresses: HashMap::new(),
            current_index: 0,
            seed_phrase: mnemonic.to_string(),
        }
    }

    /// Create HD wallet from existing seed
    pub fn from_seed(seed: [u8; 32]) -> Self {
        let mnemonic = Mnemonic::from_entropy(&seed).unwrap_or_else(|_| {
            use rand::RngCore;
            let mut entropy = [0u8; 32];
            rand::thread_rng().fill_bytes(&mut entropy);
            Mnemonic::from_entropy(&entropy).expect("Failed to generate mnemonic")
        });
        Wallet {
            master_seed: seed,
            master_seed_hex: hex::encode(seed),
            addresses: HashMap::new(),
            current_index: 0,
            seed_phrase: mnemonic.to_string(),
        }
    }

    /// Create HD wallet from BIP-39 seed phrase
    pub fn from_seed_phrase(phrase: &str) -> Result<Self, String> {
        let mnemonic = Mnemonic::parse_in(Language::English, phrase)
            .map_err(|e| format!("Invalid mnemonic: {}", e))?;
        let seed = mnemonic.to_seed_normalized("");
        let mut master_seed = [0u8; 32];
        master_seed.copy_from_slice(&seed[..32]);
        Ok(Wallet {
            master_seed,
            master_seed_hex: hex::encode(master_seed),
            addresses: HashMap::new(),
            current_index: 0,
            seed_phrase: phrase.to_string(),
        })
    }

    /// Get the seed phrase for wallet backup
    pub fn get_seed_phrase(&self) -> &str {
        &self.seed_phrase
    }

    // ...existing code...

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

    /// Save wallet to file
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), String> {
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize wallet: {}", e))?;
        
        fs::write(path, json)
            .map_err(|e| format!("Failed to write wallet file: {}", e))?;
        
        Ok(())
    }

    /// Load wallet from file
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let json = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read wallet file: {}", e))?;
        
        let mut wallet: Wallet = serde_json::from_str(&json)
            .map_err(|e| format!("Failed to deserialize wallet: {}", e))?;
        
        // Restore master_seed from hex string
        wallet.master_seed = hex::decode(&wallet.master_seed_hex)
            .map_err(|e| format!("Invalid hex in master seed: {}", e))?
            .try_into()
            .map_err(|_| "Master seed must be exactly 32 bytes")?;
        
        Ok(wallet)
    }

    /// Check if wallet file exists
    pub fn wallet_exists<P: AsRef<Path>>(path: P) -> bool {
        path.as_ref().exists()
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
