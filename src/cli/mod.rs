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
pub mod advanced_commands; // Phase 8
pub mod utils;

pub use blockchain_commands::BlockchainCommands;
pub use mempool_commands::MempoolCommands;
pub use mining_commands::MiningCommands;
pub use network_commands::NetworkCommands;
pub use advanced_commands::{WalletCommands, AnalyticsCommands}; // Phase 8

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
        let chain = Chain::new();
        let fork_choice = ForkChoice::with_genesis_chain(chain.clone());
        
        Ok(CLI {
            chain,
            block_store: BlockStore::new()?,
            mining_pool: MiningPool::new(4), // Default difficulty of 4
            fork_choice,
            mempool: Mempool::new(),
            wallet: Wallet::new(),
        })
    }
    
    pub fn new_with_path(db_path: &str) -> Result<Self, String> {
        let chain = Chain::new();
        let fork_choice = ForkChoice::with_genesis_chain(chain.clone());
        
        Ok(CLI {
            chain,
            block_store: BlockStore::new_with_path(db_path)?,
            mining_pool: MiningPool::new(4), // Default difficulty of 4
            fork_choice,
            mempool: Mempool::new(),
            wallet: Wallet::new(),
        })
    }
}

impl Default for CLI {
    fn default() -> Self {
        Self::new().expect("Failed to create default CLI")
    }
}