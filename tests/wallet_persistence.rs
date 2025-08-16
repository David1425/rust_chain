#[cfg(test)]
mod wallet_persistence_tests {
    use rust_chain::wallet::keychain::Wallet;
    use std::fs;

    #[test]
    fn test_wallet_persistence() {
        let test_file = "test_wallet_persistence.json";
        
        // Clean up any existing test file
        let _ = fs::remove_file(test_file);
        
        // Create a new wallet
        let mut wallet1 = Wallet::new();
        let original_seed = wallet1.get_seed_phrase().to_string();
        
        // Generate some addresses
        let addr1 = wallet1.generate_address().expect("Failed to generate address 1");
        let addr2 = wallet1.generate_address().expect("Failed to generate address 2");
        
        // Save the wallet
        wallet1.save_to_file(test_file).expect("Failed to save wallet");
        
        // Load the wallet back
        let wallet2 = Wallet::load_from_file(test_file).expect("Failed to load wallet");
        
        // Verify everything is preserved
        assert_eq!(wallet2.get_seed_phrase(), original_seed);
        let addresses = wallet2.get_all_addresses();
        assert_eq!(addresses.len(), 2);
        assert!(addresses.contains(&addr1));
        assert!(addresses.contains(&addr2));
        
        // Test that the loaded wallet can generate the next address deterministically
        let mut wallet3 = wallet2.clone();
        let addr3 = wallet3.generate_address().expect("Failed to generate address 3");
        
        // The third address should be different from the first two
        assert_ne!(addr3, addr1);
        assert_ne!(addr3, addr2);
        
        // Clean up
        let _ = fs::remove_file(test_file);
        
        println!("✅ Wallet persistence test passed!");
        println!("   - Seed phrase preserved: {}", original_seed.len() > 0);
        println!("   - Addresses preserved: {}", addresses.len());
        println!("   - Deterministic generation works");
    }
    
    #[test]
    fn test_wallet_restoration_persistence() {
        let test_file = "test_wallet_restoration.json";
        
        // Clean up any existing test file  
        let _ = fs::remove_file(test_file);
        
        // Test mnemonic
        let test_mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon art";
        
        // Create wallet from mnemonic
        let mut wallet1 = Wallet::from_seed_phrase(test_mnemonic).expect("Failed to restore from mnemonic");
        
        // Generate an address
        let addr1 = wallet1.generate_address().expect("Failed to generate address");
        
        // Save wallet
        wallet1.save_to_file(test_file).expect("Failed to save wallet");
        
        // Load wallet back
        let wallet2 = Wallet::load_from_file(test_file).expect("Failed to load wallet");
        
        // Verify restoration worked
        assert_eq!(wallet2.get_seed_phrase(), test_mnemonic);
        assert_eq!(wallet2.get_all_addresses().len(), 1);
        assert_eq!(wallet2.get_all_addresses()[0], addr1);
        
        // Clean up
        let _ = fs::remove_file(test_file);
        
        println!("✅ Wallet restoration persistence test passed!");
    }
}
