use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::blockchain::chain::Chain;
use crate::mempool::Mempool;
use crate::wallet::keychain::Wallet;

/// JSON-RPC 2.0 request structure
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub method: String,
    pub params: Option<Value>,
    pub id: Option<Value>,
}

/// JSON-RPC 2.0 response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
    pub id: Option<Value>,
}

/// JSON-RPC 2.0 error structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcError {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

/// Error codes for JSON-RPC
pub mod error_codes {
    pub const PARSE_ERROR: i32 = -32700;
    pub const INVALID_REQUEST: i32 = -32600;
    pub const METHOD_NOT_FOUND: i32 = -32601;
    pub const INVALID_PARAMS: i32 = -32602;
    pub const INTERNAL_ERROR: i32 = -32603;
    
    // Custom application errors
    pub const BLOCK_NOT_FOUND: i32 = -1001;
    pub const TRANSACTION_NOT_FOUND: i32 = -1002;
    pub const INSUFFICIENT_FUNDS: i32 = -1003;
    pub const INVALID_ADDRESS: i32 = -1004;
    pub const MEMPOOL_FULL: i32 = -1005;
}

/// RPC method handler trait
pub trait RpcHandler: Send + Sync {
    fn handle_request(&self, request: JsonRpcRequest) -> JsonRpcResponse;
}

/// Main RPC handler implementation
pub struct BlockchainRpcHandler {
    pub chain: Chain,
    pub mempool: Mempool,
    pub wallet: Wallet,
}

impl BlockchainRpcHandler {
    pub fn new(chain: Chain, mempool: Mempool, wallet: Wallet) -> Self {
        BlockchainRpcHandler {
            chain,
            mempool,
            wallet,
        }
    }

    /// Get blockchain info
    fn get_blockchain_info(&self) -> Result<Value, JsonRpcError> {
        let block_count = self.chain.blocks.len();
        let latest_block = self.chain.blocks.last();
        let latest_hash = if let Some(block) = latest_block {
            block.header.hash.clone()
        } else {
            "0".repeat(64)
        };
        
        let info = serde_json::json!({
            "chain": "rust-chain",
            "blocks": block_count,
            "headers": block_count,
            "bestblockhash": latest_hash,
            "difficulty": 4,
            "mediantime": 0,
            "verificationprogress": 1.0,
            "chainwork": format!("{:016x}", block_count),
            "size_on_disk": block_count * 1000, // Approximate
            "pruned": false
        });
        Ok(info)
    }

    /// Get block count
    fn get_block_count(&self) -> Result<Value, JsonRpcError> {
        let height = self.chain.blocks.len().saturating_sub(1);
        Ok(Value::Number(height.into()))
    }

    /// Get block hash by height
    fn get_block_hash(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        let params_array = params.as_ref()
            .and_then(|p| p.as_array())
            .ok_or_else(|| JsonRpcError {
                code: error_codes::INVALID_PARAMS,
                message: "Invalid params format".to_string(),
                data: None,
            })?;
            
        let height = params_array.get(0)
            .and_then(|v| v.as_u64())
            .ok_or_else(|| JsonRpcError {
                code: error_codes::INVALID_PARAMS,
                message: "Invalid height parameter".to_string(),
                data: None,
            })?;

        if let Some(block) = self.chain.blocks.get(height as usize) {
            Ok(Value::String(block.header.hash.clone()))
        } else {
            Err(JsonRpcError {
                code: error_codes::BLOCK_NOT_FOUND,
                message: "Block not found".to_string(),
                data: None,
            })
        }
    }

    /// Get block by hash
    fn get_block(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        let params_array = params.as_ref()
            .and_then(|p| p.as_array())
            .ok_or_else(|| JsonRpcError {
                code: error_codes::INVALID_PARAMS,
                message: "Invalid params format".to_string(),
                data: None,
            })?;
            
        let hash_str = params_array.get(0)
            .and_then(|v| v.as_str())
            .ok_or_else(|| JsonRpcError {
                code: error_codes::INVALID_PARAMS,
                message: "Invalid hash parameter".to_string(),
                data: None,
            })?;

        for (i, block) in self.chain.blocks.iter().enumerate() {
            if block.header.hash == hash_str {
                let block_json = serde_json::json!({
                    "hash": block.header.hash.clone(),
                    "height": i,
                    "previousblockhash": block.header.previous_hash.clone(),
                    "merkleroot": block.header.merkle_root.clone(),
                    "time": block.header.timestamp,
                    "nonce": block.header.nonce,
                    "difficulty": 4, // Fixed difficulty for now
                    "tx": block.transactions.iter().enumerate().map(|(i, _)| format!("tx_{}", i)).collect::<Vec<_>>(),
                    "size": 1000, // Approximate
                    "weight": 4000 // Approximate
                });
                return Ok(block_json);
            }
        }

        Err(JsonRpcError {
            code: error_codes::BLOCK_NOT_FOUND,
            message: "Block not found".to_string(),
            data: None,
        })
    }

    /// Get mempool info
    fn get_mempool_info(&self) -> Result<Value, JsonRpcError> {
        let stats = self.mempool.get_stats();
        let info = serde_json::json!({
            "size": stats.pending_count,
            "bytes": stats.total_size_bytes,
            "usage": stats.total_size_bytes,
            "maxmempool": 100_000_000, // 100MB limit
            "mempoolminfee": 0.00001000,
            "minrelaytxfee": 0.00001000
        });
        Ok(info)
    }

    /// Get raw mempool
    fn get_raw_mempool(&self) -> Result<Value, JsonRpcError> {
        let transactions = self.mempool.get_pending_transactions();
        let txids: Vec<String> = transactions.iter()
            .enumerate()
            .map(|(i, _)| format!("mempool_tx_{}", i))
            .collect();
        Ok(Value::Array(txids.into_iter().map(Value::String).collect()))
    }

    /// Get wallet balance
    fn get_balance(&self) -> Result<Value, JsonRpcError> {
        // Simplified balance - in a real implementation this would check UTXOs
        let balance = 1000000; // 1 million satoshis
        Ok(Value::Number(serde_json::Number::from(balance)))
    }

    /// Create a new address
    fn get_new_address(&self) -> Result<Value, JsonRpcError> {
        // Return the wallet's address
        Ok(Value::String(self.wallet.address.clone()))
    }

    /// List transactions
    fn list_transactions(&self) -> Result<Value, JsonRpcError> {
        let mut transactions = Vec::new();
        
        // Add some sample transactions for demonstration
        for (i, block) in self.chain.blocks.iter().enumerate() {
            for (j, _tx) in block.transactions.iter().enumerate() {
                transactions.push(serde_json::json!({
                    "txid": format!("tx_{}_{}", i, j),
                    "amount": 1000,
                    "confirmations": self.chain.blocks.len() - i,
                    "time": block.header.timestamp,
                    "category": "receive"
                }));
            }
        }
        
        Ok(Value::Array(transactions))
    }
}

impl RpcHandler for BlockchainRpcHandler {
    fn handle_request(&self, request: JsonRpcRequest) -> JsonRpcResponse {
        let result = match request.method.as_str() {
            "getblockchaininfo" => self.get_blockchain_info(),
            "getblockcount" => self.get_block_count(),
            "getblockhash" => self.get_block_hash(request.params),
            "getblock" => self.get_block(request.params),
            "getmempoolinfo" => self.get_mempool_info(),
            "getrawmempool" => self.get_raw_mempool(),
            "getbalance" => self.get_balance(),
            "getnewaddress" => self.get_new_address(),
            "listtransactions" => self.list_transactions(),
            _ => Err(JsonRpcError {
                code: error_codes::METHOD_NOT_FOUND,
                message: format!("Method '{}' not found", request.method),
                data: None,
            }),
        };

        match result {
            Ok(value) => JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                result: Some(value),
                error: None,
                id: request.id,
            },
            Err(error) => JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                result: None,
                error: Some(error),
                id: request.id,
            },
        }
    }
}

/// Helper function to create error response
pub fn create_error_response(code: i32, message: String, id: Option<Value>) -> JsonRpcResponse {
    JsonRpcResponse {
        jsonrpc: "2.0".to_string(),
        result: None,
        error: Some(JsonRpcError {
            code,
            message,
            data: None,
        }),
        id,
    }
}

/// Helper function to create success response
pub fn create_success_response(result: Value, id: Option<Value>) -> JsonRpcResponse {
    JsonRpcResponse {
        jsonrpc: "2.0".to_string(),
        result: Some(result),
        error: None,
        id,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blockchain::chain::Chain;
    use crate::mempool::Mempool;
    use crate::wallet::keychain::Wallet;

    fn create_test_handler() -> BlockchainRpcHandler {
        let chain = Chain::new();
        let mempool = Mempool::new();
        let wallet = Wallet::new();
        BlockchainRpcHandler::new(chain, mempool, wallet)
    }

    #[test]
    fn test_get_blockchain_info() {
        let handler = create_test_handler();
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "getblockchaininfo".to_string(),
            params: None,
            id: Some(Value::Number(1.into())),
        };

        let response = handler.handle_request(request);
        assert!(response.result.is_some());
        assert!(response.error.is_none());
    }

    #[test]
    fn test_get_block_count() {
        let handler = create_test_handler();
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "getblockcount".to_string(),
            params: None,
            id: Some(Value::Number(1.into())),
        };

        let response = handler.handle_request(request);
        assert!(response.result.is_some());
        assert!(response.error.is_none());
    }

    #[test]
    fn test_unknown_method() {
        let handler = create_test_handler();
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "unknownmethod".to_string(),
            params: None,
            id: Some(Value::Number(1.into())),
        };

        let response = handler.handle_request(request);
        assert!(response.result.is_none());
        assert!(response.error.is_some());
        assert_eq!(response.error.unwrap().code, error_codes::METHOD_NOT_FOUND);
    }

    #[test]
    fn test_invalid_params() {
        let handler = create_test_handler();
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "getblockhash".to_string(),
            params: Some(Value::String("invalid".to_string())),
            id: Some(Value::Number(1.into())),
        };

        let response = handler.handle_request(request);
        assert!(response.result.is_none());
        assert!(response.error.is_some());
        assert_eq!(response.error.unwrap().code, error_codes::INVALID_PARAMS);
    }
}
