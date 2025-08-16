use bip39::{Mnemonic, Seed};
use hdwallet::{ExtendedPrivKey, ExtendedPubKey};

pub struct Keychain {
    master_key: ExtendedPrivKey,
}

impl Keychain {
    pub fn from_mnemonic(phrase: &str) -> Self {
        let mnemonic = Mnemonic::from_phrase(phrase, Language::English).unwrap();
        let seed = Seed::new(&mnemonic, "");
        let master_key = ExtendedPrivKey::derive_from_path(&seed, &"m/44'/0'/0'".parse().unwrap()).unwrap();
        Keychain { master_key }
    }
}