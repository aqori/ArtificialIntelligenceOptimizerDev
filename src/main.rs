// src/main.rs
/*
 * Main executable for ArtificialIntelligenceOptimizerDev
 */

use clap::Parser;
use artificialintelligenceoptimizerdev::{Result, run};

#[derive(Parser)]
#[command(version, about = "ArtificialIntelligenceOptimizerDev - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
