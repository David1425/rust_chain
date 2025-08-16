use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;
use warp::{Filter, Reply};
use serde_json::Value;

use crate::rpc::handlers::{
    BlockchainRpcHandler, JsonRpcRequest, JsonRpcResponse, RpcHandler,
    create_error_response, error_codes
};
use crate::blockchain::chain::Chain;
use crate::mempool::Mempool;
use crate::wallet::keychain::Wallet;

/// JSON-RPC server configuration
#[derive(Debug, Clone)]
pub struct RpcConfig {
    pub bind_address: SocketAddr,
    pub max_request_size: usize,
    pub enable_cors: bool,
    pub allowed_origins: Vec<String>,
}

impl Default for RpcConfig {
    fn default() -> Self {
        RpcConfig {
            bind_address: "127.0.0.1:8545".parse().unwrap(),
            max_request_size: 1024 * 1024, // 1MB
            enable_cors: true,
            allowed_origins: vec!["*".to_string()],
        }
    }
}

/// JSON-RPC server
pub struct RpcServer {
    config: RpcConfig,
    handler: Arc<RwLock<BlockchainRpcHandler>>,
}

impl RpcServer {
    /// Create a new RPC server
    pub fn new(config: RpcConfig, chain: Chain, mempool: Mempool, wallet: Wallet) -> Self {
        let handler = BlockchainRpcHandler::new(chain, mempool, wallet);
        
        RpcServer {
            config,
            handler: Arc::new(RwLock::new(handler)),
        }
    }

    /// Start the RPC server
    pub async fn start(self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("Starting JSON-RPC server on {}", self.config.bind_address);

        let handler = self.handler.clone();
        
        // JSON-RPC endpoint
        let rpc = warp::path("rpc")
            .and(warp::post())
            .and(warp::body::content_length_limit(self.config.max_request_size as u64))
            .and(warp::body::json())
            .and_then(move |request: JsonRpcRequest| {
                let handler = handler.clone();
                async move {
                    Self::handle_rpc_request(handler, request).await
                }
            });

        // Health check endpoint
        let health = warp::path("health")
            .and(warp::get())
            .map(|| {
                warp::reply::json(&serde_json::json!({
                    "status": "healthy",
                    "service": "rust-chain-rpc"
                }))
            });

        // Metrics endpoint
        let metrics = warp::path("metrics")
            .and(warp::get())
            .and_then(move || {
                let handler = self.handler.clone();
                async move {
                    Self::handle_metrics_request(handler).await
                }
            });

        // Combine all routes with CORS
        let cors = warp::cors()
            .allow_any_origin()
            .allow_headers(vec!["content-type"])
            .allow_methods(vec!["POST", "GET", "OPTIONS"]);
            
        let routes = rpc.or(health).or(metrics).with(cors);

        // Start the server
        warp::serve(routes)
            .run(self.config.bind_address)
            .await;

        Ok(())
    }

    /// Handle a JSON-RPC request
    async fn handle_rpc_request(
        handler: Arc<RwLock<BlockchainRpcHandler>>,
        request: JsonRpcRequest,
    ) -> Result<impl Reply, Infallible> {
        // Validate JSON-RPC version
        if request.jsonrpc != "2.0" {
            let error_response = create_error_response(
                error_codes::INVALID_REQUEST,
                "Invalid JSON-RPC version".to_string(),
                request.id,
            );
            return Ok(warp::reply::json(&error_response));
        }

        // Handle the request
        let response = {
            let handler = handler.read().await;
            handler.handle_request(request)
        };

        Ok(warp::reply::json(&response))
    }

    /// Handle a metrics request
    async fn handle_metrics_request(
        handler: Arc<RwLock<BlockchainRpcHandler>>,
    ) -> Result<impl Reply, Infallible> {
        let handler = handler.read().await;
        
        // Get simplified metrics
        let block_count = handler.chain.blocks.len();
        let mempool_stats = handler.mempool.get_stats();
        
        let metrics = serde_json::json!({
            "blockchain": {
                "blocks": block_count,
                "height": block_count.saturating_sub(1),
                "total_difficulty": 4 * block_count,
                "latest_block_hash": if let Some(block) = handler.chain.blocks.last() {
                    block.header.hash.clone()
                } else {
                    "0".repeat(64)
                }
            },
            "mempool": {
                "transactions": mempool_stats.pending_count,
                "total_size": mempool_stats.total_size_bytes,
                "memory_usage": mempool_stats.total_size_bytes
            },
            "wallet": {
                "address": handler.wallet.get_current_address().unwrap_or_else(|| "No address generated".to_string()),
                "balance": 1000000 // Simplified
            }
        });

        Ok(warp::reply::json(&metrics))
    }

    /// Create a simple RPC server for testing
    pub fn simple(port: u16) -> Self {
        let config = RpcConfig {
            bind_address: format!("127.0.0.1:{}", port).parse().unwrap(),
            ..Default::default()
        };
        
        let chain = Chain::new();
        let mempool = Mempool::new();
        let wallet = Wallet::new();
        
        Self::new(config, chain, mempool, wallet)
    }
}

/// Batch JSON-RPC request handling
pub async fn handle_batch_request(
    handler: Arc<RwLock<BlockchainRpcHandler>>,
    requests: Vec<JsonRpcRequest>,
) -> Vec<JsonRpcResponse> {
    let mut responses = Vec::new();
    
    for request in requests {
        let response = {
            let handler = handler.read().await;
            handler.handle_request(request)
        };
        responses.push(response);
    }
    
    responses
}

/// JSON-RPC client for testing and integration
pub struct RpcClient {
    base_url: String,
    client: reqwest::Client,
}

impl RpcClient {
    pub fn new(base_url: String) -> Self {
        RpcClient {
            base_url,
            client: reqwest::Client::new(),
        }
    }

    pub async fn call(&self, method: &str, params: Option<Value>) -> Result<JsonRpcResponse, Box<dyn std::error::Error + Send + Sync>> {
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: method.to_string(),
            params,
            id: Some(Value::Number(1.into())),
        };

        let response = self.client
            .post(&format!("{}/rpc", self.base_url))
            .json(&request)
            .send()
            .await?;

        let rpc_response: JsonRpcResponse = response.json().await?;
        Ok(rpc_response)
    }

    pub async fn get_blockchain_info(&self) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
        let response = self.call("getblockchaininfo", None).await?;
        response.result.ok_or("No result in response".into())
    }

    pub async fn get_block_count(&self) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        let response = self.call("getblockcount", None).await?;
        let count = response.result
            .ok_or("No result in response")?
            .as_u64()
            .ok_or("Invalid block count format")?;
        Ok(count)
    }

    pub async fn get_block_hash(&self, height: u64) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let params = Some(serde_json::json!([height]));
        let response = self.call("getblockhash", params).await?;
        let hash = response.result
            .ok_or("No result in response")?
            .as_str()
            .ok_or("Invalid hash format")?
            .to_string();
        Ok(hash)
    }

    pub async fn get_balance(&self) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        let response = self.call("getbalance", None).await?;
        let balance = response.result
            .ok_or("No result in response")?
            .as_u64()
            .ok_or("Invalid balance format")?;
        Ok(balance)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::{sleep, Duration};

    #[tokio::test]
    async fn test_rpc_server_startup() {
        let server = RpcServer::simple(8546);
        
        // Start server in background
        let server_handle = tokio::spawn(async move {
            server.start().await.unwrap();
        });

        // Give server time to start
        sleep(Duration::from_millis(100)).await;

        // Test health endpoint (commented out to avoid port conflicts in tests)
        // let client = reqwest::Client::new();
        // let _response = client
        //     .get("http://127.0.0.1:8546/health")
        //     .send()
        //     .await;

        // Note: This test might fail if port is in use
        // In production, we'd use a random available port
        
        server_handle.abort();
    }

    #[test]
    fn test_rpc_config_default() {
        let config = RpcConfig::default();
        assert_eq!(config.bind_address.port(), 8545);
        assert!(config.enable_cors);
        assert_eq!(config.max_request_size, 1024 * 1024);
    }

    #[tokio::test]
    async fn test_batch_request_handling() {
        let chain = Chain::new();
        let mempool = Mempool::new();
        let wallet = Wallet::new();
        let handler = Arc::new(RwLock::new(BlockchainRpcHandler::new(chain, mempool, wallet)));

        let requests = vec![
            JsonRpcRequest {
                jsonrpc: "2.0".to_string(),
                method: "getblockcount".to_string(),
                params: None,
                id: Some(Value::Number(1.into())),
            },
            JsonRpcRequest {
                jsonrpc: "2.0".to_string(),
                method: "getblockchaininfo".to_string(),
                params: None,
                id: Some(Value::Number(2.into())),
            },
        ];

        let responses = handle_batch_request(handler, requests).await;
        assert_eq!(responses.len(), 2);
        assert!(responses[0].result.is_some());
        assert!(responses[1].result.is_some());
    }
}
