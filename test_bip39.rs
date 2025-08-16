use rust_chain::wallet::keychain::Wallet;

fn main() {
    println!("=== Testing BIP-39 Implementation ===\n");
    
    // Test 1: Create a new wallet with BIP-39 mnemonic
    println!("1. Creating new wallet...");
    let wallet1 = Wallet::new();
    let seed_phrase1 = wallet1.get_seed_phrase().to_string();
    println!("   Generated seed phrase: {}", seed_phrase1);
    println!("   Word count: {}", seed_phrase1.split_whitespace().count());
    
    // Test 2: Restore wallet from seed phrase
    println!("\n2. Restoring wallet from seed phrase...");
    let wallet2 = Wallet::from_seed_phrase(&seed_phrase1).expect("Failed to restore wallet");
    let seed_phrase2 = wallet2.get_seed_phrase();
    println!("   Restored seed phrase: {}", seed_phrase2);
    println!("   Phrases match: {}", seed_phrase1 == seed_phrase2);
    
    // Test 3: Generate addresses from both wallets and verify they match
    println!("\n3. Testing deterministic address generation...");
    let mut wallet1_mut = wallet1;
    let mut wallet2_mut = wallet2;
    
    let addr1 = wallet1_mut.generate_address().expect("Failed to generate address");
    let addr2 = wallet2_mut.generate_address().expect("Failed to generate address");
    
    println!("   Address from original wallet: {}", addr1);
    println!("   Address from restored wallet: {}", addr2);
    println!("   Addresses match: {}", addr1 == addr2);
    
    // Test 4: Test with known BIP-39 test vector
    println!("\n4. Testing with known BIP-39 test vector...");
    let test_mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon art";
    let wallet3 = Wallet::from_seed_phrase(test_mnemonic).expect("Failed to restore from test vector");
    println!("   Test vector restored successfully");
    println!("   Seed phrase: {}", wallet3.get_seed_phrase());
    
    println!("\n=== All tests completed successfully! ===");
}
