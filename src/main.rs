// src/main.rs
/*
 * Main executable for DesktopManager
 */

use clap::Parser;
use desktopmanager::{Result, run};

#[derive(Parser)]
#[command(version, about = "DesktopManager - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
