use clap::{Parser, Subcommand};
use std::path::PathBuf;
use tracing::info;

use gremlin_core::config::ScanConfig;
use gremlin_core::generator::JobGenerator;
use gremlin_core::logging;
use gremlin_core::queue::bounded;

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
                let (sender, _) = bounded(config.concurrency);

                let mut generator = JobGenerator::new(config)
                    .await
                    .expect("failed to initialize generator");

                loop {
                    match generator.next().await {
                        Ok(Some(request)) => {
                            if let Err(e) = sender.send(request).await {
                                eprintln!("queue send failed: {e}");
                                break;
                            }
                        }
                        Ok(None) => break,
                        Err(e) => {
                            eprintln!("generator error: {e}");
                            break;
                        }
                    }
                }

                info!("producer loop completed");
            }
            Err(e) => {
                eprintln!("Configuration error: {e}");
                std::process::exit(1);
            }
        },
    }
}
