use clap::{Parser, Subcommand};
use std::path::PathBuf;
use tracing::info;

use gremlin_core::config::ScanConfig;
use gremlin_core::logging;

/// HTTP scanning engine
#[derive(Parser)]
#[command(name = "gremlin")]
#[command(version)]
#[command(about = "High-performance HTTP scanning tool")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a scan
    Scan {
        /// Target URL (supports FUZZ placeholder)
        #[arg(short, long)]
        url: String,

        /// Path to wordlist
        #[arg(short, long)]
        wordlist: PathBuf,

        /// Number of concurrent workers
        #[arg(short, long, default_value_t = 10)]
        concurrency: usize,
    },
}

#[tokio::main]
async fn main() {
    logging::init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Scan {
            url,
            wordlist,
            concurrency,
        } => match ScanConfig::new(&url, &wordlist, concurrency) {
            Ok(config) => {
                info!(?config, "config validated");
                println!("Config validated successfully.");
            }
            Err(e) => {
                eprintln!("Configuration error: {e}");
                std::process::exit(1);
            }
        },
    }
}
