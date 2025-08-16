use rust_chain::cli::commands::CLI;
use rust_chain::blockchain::block::Transaction;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_help();
        return;
    }
    
    let mut cli = CLI::new();
    
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
    println!("Usage:");
    println!("  rust_chain init-chain               Initialize a new blockchain");
    println!("  rust_chain show-blocks              Show all blocks in the chain");
    println!("  rust_chain stats                    Show blockchain statistics");
    println!("  rust_chain chain-info               Show blockchain information (alias for stats)");
    println!("  rust_chain mine-block               Mine a new block with sample transaction");
    println!("  rust_chain mining-stats             Show mining statistics");
    println!("  rust_chain fork-stats               Show fork choice statistics");
    println!("  rust_chain add-block                Add a new block with sample transaction");
    println!("  rust_chain add-transaction <from> <to> <amount> Add transaction to mempool");
    println!("  rust_chain mempool-stats             Show mempool statistics");
    println!("  rust_chain pending-transactions      Show all pending transactions");
    println!("  rust_chain mine-mempool              Mine a block using mempool transactions");
    println!("  rust_chain clear-mempool             Clear all transactions from mempool
  rust_chain demo-mempool              Demonstrate complete mempool workflow");
    println!("  rust_chain get-block <hash>         Get block by hash");
    println!("  rust_chain start-node [addr] [port] Start P2P network node (default: 127.0.0.1:8333)");
    println!("  rust_chain connect-peer <addr> <port> Connect to a peer");
    println!("  rust_chain help                     Show this help message");
}
