// src/main.rs
/*
 * Main executable for BlockchainNftRegistryToolkitX
 */

use clap::Parser;
use blockchainnftregistrytoolkitx::{Result, run};

#[derive(Parser)]
#[command(version, about = "BlockchainNftRegistryToolkitX - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
