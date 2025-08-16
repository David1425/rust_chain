#[cfg(test)]
mod bip39_integration_tests {
    use rust_chain::wallet::keychain::Wallet;

    #[test]
    fn test_bip39_full_implementation() {
        println!("=== Testing BIP-39 Implementation ===\n");
        
        // Test 1: Create a new wallet with BIP-39 mnemonic
        println!("1. Creating new wallet...");
        let wallet1 = Wallet::new();
        let seed_phrase1 = wallet1.get_seed_phrase().to_string();
        println!("   Generated seed phrase: {}", seed_phrase1);
        let word_count = seed_phrase1.split_whitespace().count();
        println!("   Word count: {}", word_count);
        assert_eq!(word_count, 24, "Should generate 24-word BIP-39 mnemonic");
        
        // Test 2: Restore wallet from seed phrase
        println!("\n2. Restoring wallet from seed phrase...");
        let wallet2 = Wallet::from_seed_phrase(&seed_phrase1).expect("Failed to restore wallet");
        let seed_phrase2 = wallet2.get_seed_phrase();
        println!("   Restored seed phrase: {}", seed_phrase2);
        assert_eq!(seed_phrase1, seed_phrase2, "Seed phrases should match");
        
        // Test 3: Generate addresses from both wallets and verify they match
        println!("\n3. Testing deterministic address generation...");
        let mut wallet1_mut = wallet1;
        let mut wallet2_mut = wallet2;
        
        let addr1 = wallet1_mut.generate_address().expect("Failed to generate address");
        let addr2 = wallet2_mut.generate_address().expect("Failed to generate address");
        
        println!("   Address from original wallet: {}", addr1);
        println!("   Address from restored wallet: {}", addr2);
        assert_eq!(addr1, addr2, "Addresses should match for same seed");
        
        // Test 4: Test with known BIP-39 test vector
        println!("\n4. Testing with known BIP-39 test vector...");
        let test_mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon art";
        let wallet3 = Wallet::from_seed_phrase(test_mnemonic).expect("Failed to restore from test vector");
        println!("   Test vector restored successfully");
        println!("   Seed phrase: {}", wallet3.get_seed_phrase());
        assert_eq!(wallet3.get_seed_phrase(), test_mnemonic);
        
        println!("\n=== All tests completed successfully! ===");
    }

    #[test] 
    fn test_bip39_word_validation() {
        // Test invalid mnemonic
        let invalid_mnemonic = "invalid word list that should fail validation test";
        let result = Wallet::from_seed_phrase(invalid_mnemonic);
        assert!(result.is_err(), "Should fail with invalid words");
        
        // Test wrong word count
        let wrong_count = "abandon abandon abandon";
        let result2 = Wallet::from_seed_phrase(wrong_count);
        assert!(result2.is_err(), "Should fail with wrong word count");
    }
}
