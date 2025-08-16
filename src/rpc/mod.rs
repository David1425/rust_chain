//! JSON-RPC module for blockchain API
//! 
//! This module provides a complete JSON-RPC 2.0 implementation for interacting
//! with the blockchain, including:
//! - Blockchain queries (blocks, transactions, chain info)
//! - Mempool operations
//! - Wallet functionality
//! - Network statistics

pub mod handlers;
pub mod server;

pub use handlers::{
    BlockchainRpcHandler, 
    JsonRpcRequest, 
    JsonRpcResponse, 
    JsonRpcError,
    RpcHandler,
    error_codes,
    create_error_response,
    create_success_response
};

pub use server::{
    RpcServer,
    RpcConfig,
    RpcClient,
    handle_batch_request
};

/// Re-export common types for convenience
pub type RpcResult<T> = Result<T, JsonRpcError>;

/// Common RPC methods as constants
pub mod methods {
    pub const GET_BLOCKCHAIN_INFO: &str = "getblockchaininfo";
    pub const GET_BLOCK_COUNT: &str = "getblockcount";
    pub const GET_BLOCK_HASH: &str = "getblockhash";
    pub const GET_BLOCK: &str = "getblock";
    pub const GET_TRANSACTION: &str = "gettransaction";
    pub const GET_MEMPOOL_INFO: &str = "getmempoolinfo";
    pub const GET_RAW_MEMPOOL: &str = "getrawmempool";
    pub const SEND_RAW_TRANSACTION: &str = "sendrawtransaction";
    pub const GET_BALANCE: &str = "getbalance";
    pub const GET_NEW_ADDRESS: &str = "getnewaddress";
    pub const LIST_TRANSACTIONS: &str = "listtransactions";
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blockchain::chain::Chain;
    use crate::mempool::Mempool;
    use crate::wallet::keychain::Wallet;

    #[test]
    fn test_module_exports() {
        // Test that we can create basic RPC components
        let chain = Chain::new();
        let mempool = Mempool::new();
        let wallet = Wallet::new();
        
        let _handler = BlockchainRpcHandler::new(chain, mempool, wallet);
        let _config = RpcConfig::default();
        
        // Test method constants
        assert_eq!(methods::GET_BLOCKCHAIN_INFO, "getblockchaininfo");
        assert_eq!(methods::GET_BLOCK_COUNT, "getblockcount");
    }
}
