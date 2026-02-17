use clap::Parser;
use libp2p::identity;
use network::service::NetworkService;
use std::path::PathBuf;
use storage::db::SledStore;
use tracing::{error, info};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Port to listen on (P2P)
    #[arg(short, long, default_value_t = 8000)]
    port: u16,

    /// Path to store blockchain data
    #[arg(short, long, default_value = "./data")]
    db_path: PathBuf,

    /// Run in light mode (reduced memory usage)
    #[arg(long, default_value_t = false)]
    light: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    let args = Args::parse();
    info!("Starting VajraChain Node on port {}", args.port);
    info!("Database path: {:?}", args.db_path);
    if args.light {
        info!("Running in LIGHT mode (Reduced Cache)");
    }

    // 1. Initialize Storage
    let cache_size = if args.light {
        16 * 1024 * 1024 // 16MB for light mode
    } else {
        1024 * 1024 * 1024 // 1GB default
    };

    let _storage = SledStore::new_with_cache(&args.db_path, cache_size).map_err(|e| {
        error!("Failed to open database: {}", e);
        e
    })?;
    info!("Storage initialized");

    // 2. Initialize Networking
    let local_key = identity::Keypair::generate_ed25519();
    let (service, _cmd_tx, _event_rx) = NetworkService::new(local_key).await?;

    // Spawn Network Service
    tokio::spawn(async move {
        service.run().await;
    });
    info!("Network service started");

    // 3. Keep Alive (Simulation of Consensus Loop)
    // In real implementation, we'd start the Consensus Engine here.
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
        info!("Node is running... (Heartbeat)");
    }
}
