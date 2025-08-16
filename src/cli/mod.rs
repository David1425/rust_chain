use crate::blockchain::chain::Chain;
use crate::storage::block_store::BlockStore;
use crate::consensus::pow::MiningPool;
use crate::consensus::fork_choice::ForkChoice;
use crate::mempool::Mempool;
use crate::wallet::keychain::Wallet;

pub mod blockchain_commands;
pub mod mempool_commands;
pub mod mining_commands;
pub mod network_commands;
pub mod advanced_commands;
pub mod utils;

pub use blockchain_commands::BlockchainCommands;
pub use mempool_commands::MempoolCommands;
pub use mining_commands::MiningCommands;
pub use network_commands::NetworkCommands;
pub use advanced_commands::{WalletCommands, AnalyticsCommands, TransactionCommands};

/// Main CLI struct that holds all the blockchain components
pub struct CLI {
    pub chain: Chain,
    pub block_store: BlockStore,
    pub mining_pool: MiningPool,
    pub fork_choice: ForkChoice,
    pub mempool: Mempool,
    pub wallet: Wallet,
}

impl CLI {
    pub fn new() -> Result<Self, String> {
        // Use persistent chain
        let chain = Chain::new_persistent()?;
        let fork_choice = ForkChoice::with_genesis_chain(chain.clone());
        
        // Load existing wallet or create new one
        let wallet_path = "wallet.json";
        let wallet = if Wallet::wallet_exists(wallet_path) {
            Wallet::load_from_file(wallet_path).unwrap_or_else(|e| {
                eprintln!("Warning: Failed to load wallet: {}. Creating new wallet.", e);
                Wallet::new()
            })
        } else {
            Wallet::new()
        };
        
        // For persistent chains, we don't need a separate BlockStore since it's handled internally
        let block_store = BlockStore::new_with_path("./temp_block_store")?;
        
        // Create persistent mempool and load existing transactions
        let mut mempool = Mempool::new_persistent("./mempool.json".to_string());
        
        // Create CLI temporarily to get UTXO state for mempool loading
        let temp_cli = CLI {
            chain: chain.clone(),
            block_store: BlockStore::new_with_path("./temp_block_store")?,
            mining_pool: MiningPool::new(4),
            fork_choice: ForkChoice::with_genesis_chain(chain.clone()),
            mempool: Mempool::new(), // Temporary mempool
            wallet: wallet.clone(),
        };
        
        // Load mempool from persistence
        let utxo_state = temp_cli.get_current_utxo_state();
        if let Err(e) = mempool.load_from_file("./mempool.json", &utxo_state) {
            eprintln!("Warning: Failed to load mempool: {}", e);
        }
        
        let cli = CLI {
            chain,
            block_store,
            mining_pool: MiningPool::new(4), // Default difficulty of 4
            fork_choice,
            mempool,
            wallet,
        };
        
        // Save wallet to persist any changes
        if let Err(e) = cli.wallet.save_to_file(wallet_path) {
            eprintln!("Warning: Failed to save wallet: {}", e);
        }
        
        Ok(cli)
    }
    
    pub fn new_with_path(db_path: &str) -> Result<Self, String> {
        // Use persistent chain with custom path
        let chain = Chain::new_persistent_with_path(db_path)?;
        let fork_choice = ForkChoice::with_genesis_chain(chain.clone());
        
        // Load existing wallet or create new one (using custom path)
        let wallet_path = format!("{}/wallet.json", db_path);
        let wallet = if Wallet::wallet_exists(&wallet_path) {
            Wallet::load_from_file(&wallet_path).unwrap_or_else(|e| {
                eprintln!("Warning: Failed to load wallet: {}. Creating new wallet.", e);
                Wallet::new()
            })
        } else {
            Wallet::new()
        };

        // Use a different path for the CLI's block store to avoid conflicts
        let cli_block_store_path = format!("{}/cli_blocks", db_path);
        let cli = CLI {
            chain,
            block_store: BlockStore::new_with_path(&cli_block_store_path)?,
            mining_pool: MiningPool::new(4), // Default difficulty of 4
            fork_choice,
            mempool: Mempool::new(),
            wallet,
        };
        
        // Ensure directory exists and save wallet
        std::fs::create_dir_all(db_path).map_err(|e| format!("Failed to create directory: {}", e))?;
        if let Err(e) = cli.wallet.save_to_file(&wallet_path) {
            eprintln!("Warning: Failed to save wallet: {}", e);
        }
        
        Ok(cli)
    }
}

impl Default for CLI {
    fn default() -> Self {
        Self::new().expect("Failed to create default CLI")
    }
}