use std::path::PathBuf;
use std::sync::Arc;

use clap::{Parser, Subcommand};
use tokio::sync::Mutex;
use tokio::task;
use tracing::{error, info};

use engine::engine::HttpEngine;
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
                let concurrency = config.concurrency;

                let (sender, receiver) = bounded(concurrency);

                let engine = match HttpEngine::new() {
                    Ok(engine) => Arc::new(engine),
                    Err(e) => {
                        eprintln!("engine init failed: {}", e);
                        std::process::exit(1);
                    }
                };

                let receiver = Arc::new(Mutex::new(receiver));

                let mut handles = Vec::new();

                for _ in 0..concurrency {
                    let rx = receiver.clone();
                    let engine = engine.clone();

                    let handle = task::spawn(async move {
                        loop {
                            let request_opt = {
                                let mut locked = rx.lock().await;
                                locked.recv().await
                            };

                            match request_opt {
                                Some(request) => {
                                    let response = engine.execute(request).await;

                                    if let Some(status) = response.status {
                                        info!("response status: {}", status);
                                    } else if let Some(e) = response.error {
                                        error!("respose error: {}", e);
                                    }
                                }
                                None => break,
                            }
                        }
                    });

                    handles.push(handle);
                }

                let mut generator = JobGenerator::new(config)
                    .await
                    .expect("generator init failed");

                while let Ok(Some(request)) = generator.next().await {
                    if sender.send(request).await.is_err() {
                        break;
                    }
                }

                drop(sender);

                for handle in handles {
                    let _ = handle.await;
                }

                info!("scan complete");
            }
            Err(e) => {
                eprintln!("Configuration error: {e}");
                std::process::exit(1);
            }
        },
    }
}
