use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    /// Start the node
    Start {
        #[arg(short, long)]
        p2p_port: Option<u16>,
    },
    /// Check balance
    Balance {
        address: String,
    },
}