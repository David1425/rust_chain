use rust_chain::cli::{CLI, WalletCommands, AnalyticsCommands, BlockchainCommands, MiningCommands};
use rust_chain::wallet::keychain::Wallet;
use rust_chain::blockchain::block::Transaction;
use std::time::{SystemTime, UNIX_EPOCH};

fn get_unique_test_path(base_name: &str) -> String {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!("./test_data/{}_{}", base_name, timestamp)
}

#[test]
fn test_hd_wallet_creation() {
    let wallet = Wallet::new();
    let seed_phrase = wallet.get_seed_phrase();
    
    // Seed phrase should not be empty
    assert!(!seed_phrase.is_empty());
    
    // Should be exactly 24 words (BIP-39 standard for 32 bytes entropy)
    let words: Vec<&str> = seed_phrase.split_whitespace().collect();
    assert_eq!(words.len(), 24);
}

#[test]
fn test_hd_wallet_restoration() {
    let wallet1 = Wallet::new();
    let seed_phrase = wallet1.get_seed_phrase().to_string();
    
    // Restore wallet from seed phrase
    let wallet2 = Wallet::from_seed_phrase(&seed_phrase).expect("Failed to restore wallet");
    
    // Should have the same seed phrase
    assert_eq!(wallet1.get_seed_phrase(), wallet2.get_seed_phrase());
}

#[test]
fn test_hd_wallet_address_generation() {
    let mut wallet = Wallet::new();
    
    // Generate multiple addresses
    let addr1 = wallet.generate_address().expect("Failed to generate address 1");
    let addr2 = wallet.generate_address().expect("Failed to generate address 2");
    let addr3 = wallet.generate_address().expect("Failed to generate address 3");
    
    // Addresses should be different
    assert_ne!(addr1, addr2);
    assert_ne!(addr2, addr3);
    assert_ne!(addr1, addr3);
    
    // Should be hex strings
    assert!(addr1.chars().all(|c| c.is_ascii_hexdigit()));
    assert!(addr2.chars().all(|c| c.is_ascii_hexdigit()));
    assert!(addr3.chars().all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn test_wallet_commands() {
    let test_path = get_unique_test_path("test_wallet_commands");
    let mut cli = CLI::new_with_path(&test_path).expect("Failed to create CLI");
    
    // Generate new address
    let addr1 = cli.generate_new_address().expect("Failed to generate address");
    let addr2 = cli.generate_new_address().expect("Failed to generate address");
    
    // List addresses
    let addresses = cli.list_addresses();
    assert_eq!(addresses.len(), 2);
    assert!(addresses.contains(&addr1));
    assert!(addresses.contains(&addr2));
    
    // Get wallet stats
    let stats = cli.get_wallet_stats();
    assert_eq!(stats.total_addresses, 2);
    assert_eq!(stats.next_index, 2);
    assert!(!stats.master_fingerprint.is_empty());
}

#[test]
fn test_seed_phrase_backup_restore() {
    let test_path1 = get_unique_test_path("test_seed_phrase_cli1");
    let mut cli1 = CLI::new_with_path(&test_path1).expect("Failed to create CLI");
    
    // Generate some addresses
    cli1.generate_new_address().expect("Failed to generate address");
    cli1.generate_new_address().expect("Failed to generate address");
    
    let seed_phrase = cli1.show_seed_phrase();
    
    // Create new CLI and restore
    let test_path2 = get_unique_test_path("test_seed_phrase_cli2");
    let mut cli2 = CLI::new_with_path(&test_path2).expect("Failed to create CLI");
    cli2.restore_from_seed(&seed_phrase).expect("Failed to restore wallet");
    
    // Should have the same seed phrase
    assert_eq!(cli1.show_seed_phrase(), cli2.show_seed_phrase());
}

#[test]
fn test_wallet_backup() {
    let test_path = get_unique_test_path("test_wallet_backup");
    let mut cli = CLI::new_with_path(&test_path).expect("Failed to create CLI");
    cli.generate_new_address().expect("Failed to generate address");
    
    let backup_path = "/tmp/test_wallet_backup.json";
    cli.backup_wallet(backup_path).expect("Failed to backup wallet");
    
    // Check that file exists and contains JSON
    let backup_content = std::fs::read_to_string(backup_path).expect("Failed to read backup file");
    let _backup_json: serde_json::Value = serde_json::from_str(&backup_content)
        .expect("Backup file is not valid JSON");
    
    // Clean up
    std::fs::remove_file(backup_path).ok();
}

#[test]
fn test_chain_analytics() {
    let test_path = get_unique_test_path("test_chain_analytics");
    let mut cli = CLI::new_with_path(&test_path).expect("Failed to create CLI");
    cli.init_chain().expect("Failed to initialize chain");
    
    // Add some blocks with transactions
    let tx1 = Transaction {
        from: "alice".to_string(),
        to: "bob".to_string(),
        amount: 100,
        signature: vec![],
    };
    let tx2 = Transaction {
        from: "bob".to_string(),
        to: "charlie".to_string(),
        amount: 50,
        signature: vec![],
    };
    
    cli.mine_block(vec![tx1]).expect("Failed to mine block 1");
    cli.mine_block(vec![tx2]).expect("Failed to mine block 2");
    
    // Test chain analytics
    let analytics = cli.analyze_chain();
    assert_eq!(analytics.total_blocks, 3); // Genesis + 2 mined blocks
    assert_eq!(analytics.total_transactions, 4); // 2 in genesis + 2 transactions added
    assert!(analytics.total_size_bytes > 0);
}

#[test]
fn test_block_statistics() {
    let test_path = get_unique_test_path("test_block_statistics");
    let mut cli = CLI::new_with_path(&test_path).expect("Failed to create CLI");
    cli.init_chain().expect("Failed to initialize chain");
    
    let tx = Transaction {
        from: "alice".to_string(),
        to: "bob".to_string(),
        amount: 100,
        signature: vec![],
    };
    
    cli.mine_block(vec![tx]).expect("Failed to mine block");
    
    // Test block stats for the latest block
    let stats = cli.get_block_stats(None).expect("Failed to get block stats");
    assert_eq!(stats.height, 1); // Genesis is 0, this is block 1
    assert_eq!(stats.transaction_count, 1);
    assert!(!stats.hash.is_empty());
    assert!(stats.size_bytes > 0);
}

#[test]
fn test_transaction_statistics() {
    let test_path = get_unique_test_path("test_transaction_statistics");
    let mut cli = CLI::new_with_path(&test_path).expect("Failed to create CLI");
    cli.init_chain().expect("Failed to initialize chain");
    
    // Add transactions
    let tx1 = Transaction {
        from: "alice".to_string(),
        to: "bob".to_string(),
        amount: 100,
        signature: vec![],
    };
    let tx2 = Transaction {
        from: "bob".to_string(),
        to: "charlie".to_string(),
        amount: 50,
        signature: vec![],
    };
    let tx3 = Transaction {
        from: "alice".to_string(),
        to: "charlie".to_string(),
        amount: 25,
        signature: vec![],
    };
    
    cli.mine_block(vec![tx1, tx2]).expect("Failed to mine block 1");
    cli.mine_block(vec![tx3]).expect("Failed to mine block 2");
    
    // Test transaction stats
    let stats = cli.get_transaction_stats();
    assert_eq!(stats.total_transactions, 5); // 2 in genesis + 3 added
    assert_eq!(stats.total_value_transferred, 1675); // 1000+500 (genesis) + 100 + 50 + 25
    assert_eq!(stats.unique_addresses, 4); // genesis, alice, bob, charlie
    assert_eq!(stats.average_transaction_value, 335); // 1675 / 5 = 335
}

#[test]
fn test_chain_integrity_validation() {
    let test_path = get_unique_test_path("test_chain_integrity_validation");
    let mut cli = CLI::new_with_path(&test_path).expect("Failed to create CLI");
    cli.init_chain().expect("Failed to initialize chain");
    
    let tx = Transaction {
        from: "alice".to_string(),
        to: "bob".to_string(),
        amount: 100,
        signature: vec![],
    };
    
    cli.mine_block(vec![tx]).expect("Failed to mine block");
    
    // Test chain integrity
    let report = cli.validate_chain_integrity();
    assert_eq!(report.total_blocks, 2); // Genesis + 1 mined block
    assert_eq!(report.valid_blocks, 2); // Both should be valid
    assert!(report.is_valid);
    assert!(report.issues.is_empty());
}

#[test]
fn test_wallet_private_key_derivation() {
    let mut wallet = Wallet::new();
    
    // Generate an address
    let address = wallet.generate_address().expect("Failed to generate address");
    
    // Get private key for the address
    let private_key = wallet.get_private_key(&address).expect("Failed to get private key");
    
    // Private key should be 32 bytes
    assert_eq!(private_key.len(), 32);
    
    // Should be able to derive the same key again
    let private_key2 = wallet.get_private_key(&address).expect("Failed to get private key again");
    assert_eq!(private_key, private_key2);
}

#[test]
fn test_wallet_deterministic_generation() {
    let wallet1 = Wallet::new();
    let seed = wallet1.get_master_seed();
    
    // Create two wallets from the same seed
    let mut wallet2 = Wallet::from_seed(seed);
    let mut wallet3 = Wallet::from_seed(seed);
    
    // Generate addresses - should be the same
    let addr2 = wallet2.generate_address().expect("Failed to generate address");
    let addr3 = wallet3.generate_address().expect("Failed to generate address");
    
    assert_eq!(addr2, addr3);
}

#[test]
fn test_advanced_wallet_operations() {
    let test_path = get_unique_test_path("test_advanced_wallet_operations");
    let mut cli = CLI::new_with_path(&test_path).expect("Failed to create CLI");
    
    // Test import private key (simplified)
    let imported_addr = cli.import_private_key("dummy_private_key")
        .expect("Failed to import private key");
    assert!(!imported_addr.is_empty());
    
    // Test address listing after import
    let addresses = cli.list_addresses();
    assert!(!addresses.is_empty());
}
