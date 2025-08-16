use crate::cli::CLI;
use crate::network::server::NetworkServer;
use std::thread;

/// Trait for network-related commands
pub trait NetworkCommands {
    fn start_node(&self, listen_address: String, listen_port: u16) -> Result<(), String>;
    fn connect_peer(&self, address: String, port: u16) -> Result<(), String>;
}

impl NetworkCommands for CLI {
    /// Start network node
    fn start_node(&self, listen_address: String, listen_port: u16) -> Result<(), String> {
        println!("Starting network node on {}:{}...", listen_address, listen_port);
        
        let server = NetworkServer::new(self.chain.clone(), listen_address, listen_port);
        
        // Start server in a separate thread
        let server_handle = thread::spawn(move || {
            if let Err(e) = server.start() {
                eprintln!("Server error: {}", e);
            }
        });
        
        println!("Network node started. Press Ctrl+C to stop.");
        
        // Wait for the server thread (this will block until the server stops)
        if let Err(e) = server_handle.join() {
            eprintln!("Server thread error: {:?}", e);
        }
        
        Ok(())
    }
    
    /// Connect to a peer
    fn connect_peer(&self, address: String, port: u16) -> Result<(), String> {
        let server = NetworkServer::new(self.chain.clone(), "127.0.0.1".to_string(), 0);
        
        server.connect_to_peer(&address, port)
            .map_err(|e| format!("Failed to connect to peer: {}", e))?;
        
        println!("Successfully connected to peer at {}:{}", address, port);
        Ok(())
    }
}
