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
        "stats" => {
            cli.show_stats();
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
    println!("  rust_chain init-chain      Initialize a new blockchain");
    println!("  rust_chain show-blocks     Show all blocks in the chain");
    println!("  rust_chain stats           Show blockchain statistics");
    println!("  rust_chain add-block       Add a new block with sample transaction");
    println!("  rust_chain get-block <hash> Get block by hash");
    println!("  rust_chain help            Show this help message");
}
