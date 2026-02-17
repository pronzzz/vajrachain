use clap::{Parser, Subcommand};
use ed25519_dalek::{SigningKey, VerifyingKey};
use rand::rngs::OsRng;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a new Ed25519 keypair
    Keygen,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Keygen => {
            let mut csprng = OsRng;
            let signing_key = SigningKey::generate(&mut csprng);
            let verifying_key: VerifyingKey = signing_key.verifying_key();

            println!("New Keypair Generated:");
            println!(
                "Private Key (Keep Safe!): {}",
                hex::encode(signing_key.to_bytes())
            );
            println!(
                "Public Key (Address):     {}",
                hex::encode(verifying_key.to_bytes())
            );
        }
    }
}
