// Re-export all command traits and CLI struct for backwards compatibility
pub use crate::cli::{CLI, BlockchainCommands, MempoolCommands, MiningCommands, NetworkCommands};

// This file exists for backwards compatibility with main.rs
// All command implementations have been moved to separate modules:
// - blockchain_commands.rs - Chain operations, blocks, stats
// - mempool_commands.rs - Transaction pool management 
// - mining_commands.rs - Proof of work mining
// - network_commands.rs - P2P networking
// - utils.rs - Shared utility functions