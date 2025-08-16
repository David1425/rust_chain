use rust_chain::cli::{CLI, BlockchainCommands, MempoolCommands, MiningCommands, NetworkCommands, WalletCommands, AnalyticsCommands};
use rust_chain::blockchain::block::Transaction;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_help();
        return;
    }
    
    let mut cli = match CLI::new() {
        Ok(cli) => cli,
        Err(e) => {
            eprintln!("Error creating CLI: {}", e);
            return;
        }
    };
    
    match args[1].as_str() {
        "init-chain" => {
            if let Err(e) = cli.init_chain() {
                eprintln!("Error initializing chain: {}", e);
            }
        },
        "show-blocks" => {
            cli.show_blocks();
        },
        "stats" | "chain-info" => {
            cli.show_stats();
        },
        "mine-block" => {
            // Mine a block with a sample transaction
            let tx = Transaction {
                from: "alice".to_string(),
                to: "bob".to_string(),
                amount: 10,
                signature: vec![],
            };
            
            if let Err(e) = cli.mine_block(vec![tx]) {
                eprintln!("Error mining block: {}", e);
            }
        },
        "mining-stats" => {
            cli.show_mining_stats();
        },
        "fork-stats" => {
            cli.show_fork_stats();
        },
        "add-block" => {
            // Simple example: add a block with a sample transaction
            let tx = Transaction {
                from: "alice".to_string(),
                to: "bob".to_string(),
                amount: 10,
                signature: vec![],
            };
            
            if let Err(e) = cli.add_block(vec![tx]) {
                eprintln!("Error adding block: {}", e);
            }
        },
        "get-block" => {
            if args.len() < 3 {
                eprintln!("Usage: {} get-block <hash>", args[0]);
                return;
            }
            
            if let Err(e) = cli.get_block(&args[2]) {
                eprintln!("Error getting block: {}", e);
            }
        },
        "start-node" => {
            let address = args.get(2).unwrap_or(&"127.0.0.1".to_string()).clone();
            let port = args.get(3)
                .and_then(|s| s.parse::<u16>().ok())
                .unwrap_or(8333);
            
            if let Err(e) = cli.start_node(address, port) {
                eprintln!("Error starting node: {}", e);
            }
        },
        "connect-peer" => {
            if args.len() < 4 {
                eprintln!("Usage: {} connect-peer <address> <port>", args[0]);
                return;
            }
            
            let address = args[2].clone();
            let port = match args[3].parse::<u16>() {
                Ok(p) => p,
                Err(_) => {
                    eprintln!("Invalid port number: {}", args[3]);
                    return;
                }
            };
            
            if let Err(e) = cli.connect_peer(address, port) {
                eprintln!("Error connecting to peer: {}", e);
            }
        },
        "start-rpc" => {
            let port = args.get(2)
                .and_then(|s| s.parse::<u16>().ok())
                .unwrap_or(8545);
            
            if let Err(e) = cli.start_rpc_server(port) {
                eprintln!("Error starting RPC server: {}", e);
            }
        },
        "discover-peers" => {
            let seed_nodes = if args.len() > 2 {
                args[2..].to_vec()
            } else {
                vec!["127.0.0.1:8334".to_string(), "127.0.0.1:8335".to_string()]
            };
            
            if let Err(e) = cli.discover_peers(seed_nodes) {
                eprintln!("Error discovering peers: {}", e);
            }
        },
        "show-peers" => {
            if let Err(e) = cli.show_peers() {
                eprintln!("Error showing peers: {}", e);
            }
        },
        "network-stats" => {
            if let Err(e) = cli.show_network_stats() {
                eprintln!("Error showing network stats: {}", e);
            }
        },
        "add-transaction" => {
            if args.len() < 5 {
                eprintln!("Usage: {} add-transaction <from> <to> <amount>", args[0]);
                return;
            }
            
            let amount = match args[4].parse::<u64>() {
                Ok(a) => a,
                Err(_) => {
                    eprintln!("Invalid amount: {}", args[4]);
                    return;
                }
            };
            
            let tx = Transaction {
                from: args[2].clone(),
                to: args[3].clone(),
                amount,
                signature: vec![],
            };
            
            if let Err(e) = cli.add_transaction_to_mempool(tx) {
                eprintln!("Error adding transaction: {}", e);
            }
        },
        "mempool-stats" => {
            cli.show_mempool_stats();
        },
        "pending-transactions" => {
            cli.show_pending_transactions();
        },
        "mine-mempool" => {
            if let Err(e) = cli.mine_block_from_mempool() {
                eprintln!("Error mining from mempool: {}", e);
            }
        },
        "clear-mempool" => {
            cli.clear_mempool();
        },
        "demo-mempool" => {
            if let Err(e) = cli.demo_mempool() {
                eprintln!("Error in mempool demo: {}", e);
            }
        },
        // **Phase 8 - Advanced Wallet Commands**
        "generate-address" => {
            match cli.generate_new_address() {
                Ok(address) => println!("New address generated: {}", address),
                Err(e) => eprintln!("Error generating address: {}", e),
            }
        },
        "list-addresses" => {
            let addresses = cli.list_addresses();
            if addresses.is_empty() {
                println!("No addresses found in wallet");
            } else {
                println!("Wallet addresses:");
                for (i, addr) in addresses.iter().enumerate() {
                    println!("  {}: {}", i, addr);
                }
            }
        },
        "show-seed" => {
            println!("IMPORTANT: Keep this seed phrase safe and private!");
            println!("Seed phrase: {}", cli.show_seed_phrase());
        },
        "restore-wallet" => {
            if args.len() < 3 {
                eprintln!("Usage: {} restore-wallet \"<seed phrase>\"", args[0]);
                return;
            }
            
            match cli.restore_from_seed(&args[2]) {
                Ok(_) => println!("Wallet restored successfully"),
                Err(e) => eprintln!("Error restoring wallet: {}", e),
            }
        },
        "wallet-stats" => {
            let stats = cli.get_wallet_stats();
            println!("Wallet Statistics:");
            println!("  Total addresses: {}", stats.total_addresses);
            println!("  Next index: {}", stats.next_index);
            println!("  Master fingerprint: {}", stats.master_fingerprint);
        },
        "backup-wallet" => {
            let path = if args.len() > 2 {
                &args[2]
            } else {
                "wallet_backup.json"
            };
            
            match cli.backup_wallet(path) {
                Ok(_) => println!("Wallet backed up to: {}", path),
                Err(e) => eprintln!("Error backing up wallet: {}", e),
            }
        },
        // **Phase 8 - Analytics Commands**
        "analyze-chain" => {
            let analytics = cli.analyze_chain();
            println!("Blockchain Analysis:");
            println!("  Total blocks: {}", analytics.total_blocks);
            println!("  Total transactions: {}", analytics.total_transactions);
            println!("  Total size: {} bytes", analytics.total_size_bytes);
            println!("  Average block time: {} seconds", analytics.average_block_time_seconds);
            println!("  Chain start time: {}", analytics.chain_start_time);
            println!("  Latest block time: {}", analytics.chain_latest_time);
        },
        "block-stats" => {
            let height = if args.len() > 2 {
                args[2].parse::<u64>().ok()
            } else {
                None
            };
            
            match cli.get_block_stats(height) {
                Ok(stats) => {
                    println!("Block Statistics:");
                    println!("  Height: {}", stats.height);
                    println!("  Hash: {}", stats.hash);
                    println!("  Timestamp: {}", stats.timestamp);
                    println!("  Transactions: {}", stats.transaction_count);
                    println!("  Size: {} bytes", stats.size_bytes);
                    println!("  Nonce: {}", stats.nonce);
                    println!("  Previous hash: {}", stats.previous_hash);
                },
                Err(e) => eprintln!("Error getting block stats: {}", e),
            }
        },
        "transaction-stats" => {
            let stats = cli.get_transaction_stats();
            println!("Transaction Statistics:");
            println!("  Total transactions: {}", stats.total_transactions);
            println!("  Total value transferred: {}", stats.total_value_transferred);
            println!("  Unique addresses: {}", stats.unique_addresses);
            println!("  Average transaction value: {}", stats.average_transaction_value);
        },
        "validate-chain" => {
            let report = cli.validate_chain_integrity();
            println!("Chain Integrity Report:");
            println!("  Total blocks: {}", report.total_blocks);
            println!("  Valid blocks: {}", report.valid_blocks);
            println!("  Is valid: {}", report.is_valid);
            
            if !report.issues.is_empty() {
                println!("  Issues found:");
                for issue in &report.issues {
                    println!("    - {}", issue);
                }
            }
        },
        "help" | "--help" | "-h" => {
            print_help();
        },
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            print_help();
        }
    }
}

fn print_help() {
    println!("Rust Chain - Simple Blockchain Implementation");
    println!();
    println!("BASIC COMMANDS:");
    println!("  init-chain               Initialize a new blockchain");
    println!("  show-blocks              Show all blocks in the chain");
    println!("  stats                    Show blockchain statistics");
    println!("  chain-info               Show blockchain information (alias for stats)");
    println!("  help                     Show this help message");
    println!();
    println!("MINING COMMANDS:");
    println!("  mine-block               Mine a new block with sample transaction");
    println!("  mining-stats             Show mining statistics");
    println!("  fork-stats               Show fork choice statistics");
    println!("  add-block                Add a new block with sample transaction");
    println!("  mine-mempool             Mine a block using mempool transactions");
    println!();
    println!("TRANSACTION & MEMPOOL:");
    println!("  add-transaction <from> <to> <amount> Add transaction to mempool");
    println!("  mempool-stats            Show mempool statistics");
    println!("  pending-transactions     Show all pending transactions");
    println!("  clear-mempool            Clear all transactions from mempool");
    println!("  demo-mempool             Demonstrate complete mempool workflow");
    println!();
    println!("NETWORKING COMMANDS:");
    println!("  start-node [addr] [port] Start P2P network node (default: 127.0.0.1:8333)");
    println!("  connect-peer <addr> <port> Connect to a peer");
    println!("  start-rpc [port]         Start JSON-RPC server (default: 8545)");
    println!("  discover-peers [seeds...] Discover peers using seed nodes");
    println!("  show-peers               Show connected peers");
    println!("  network-stats            Show network statistics");
    println!();
    println!("WALLET COMMANDS:");
    println!("  generate-address         Generate a new wallet address");
    println!("  list-addresses           List all wallet addresses");
    println!("  show-seed                Show wallet seed phrase (keep safe!)");
    println!("  restore-wallet \"<phrase>\" Restore wallet from seed phrase");
    println!("  wallet-stats             Show wallet statistics");
    println!("  backup-wallet [path]     Backup wallet to file (default: wallet_backup.json)");
    println!();
    println!("ANALYTICS COMMANDS:");
    println!("  analyze-chain            Comprehensive blockchain analysis");
    println!("  block-stats [height]     Detailed statistics for a block");
    println!("  transaction-stats        Transaction statistics across the chain");
    println!("  validate-chain           Validate blockchain integrity");
    println!("  get-block <hash>         Get block by hash");
}
