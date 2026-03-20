mod benchmark;
mod generator;
mod scan;
mod worker;

use std::path::PathBuf;

use clap::{Parser, Subcommand};
use tokio::signal;

use benchmark::benchmark;
use gremlin_core::logging;
use scan::scan;

/// HTTP scanning engine
#[derive(Parser)]
#[command(name = "gremlin")]
#[command(version)]
#[command(about = "High-performance HTTP scanning tool")]
struct Cli {
    /// Set logging level to Error
    #[arg(long, default_value_t = false)]
    quiet: bool,

    /// Hide progress bar
    #[arg(long, default_value_t = false)]
    no_progress: bool,

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

        /// Include results where [field] matches PATTERN
        #[arg(short = 'r', long, value_name = "PATTERN")]
        match_regex: Option<String>,

        /// Include results with this status code
        #[arg(long, value_name = "CODE")]
        match_status: Option<u16>,

        /// Exclude results smaller than N bytes
        #[arg(long, value_name = "BYTES")]
        filter_size_min: Option<usize>,

        /// Exclude results larger than N bytes
        #[arg(long, value_name = "BYTES")]
        filter_size_max: Option<usize>,

        /// Requests per Second
        #[arg(long, value_name = "RATE")]
        rate_limit: Option<u64>,
    },

    Benchmark {
        #[arg(long)]
        url: String,

        #[arg(long, default_value_t = 10000)]
        requests: usize,

        #[arg(long, default_value_t = 50)]
        concurrency: usize,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    logging::init(cli.quiet);

    let shutdown = tokio::spawn(async {
        signal::ctrl_c()
            .await
            .expect("failed to listen for SIGTERM");
    });

    match cli.command {
        Commands::Scan {
            url,
            wordlist,
            concurrency,
            match_regex,
            match_status,
            filter_size_min,
            filter_size_max,
            rate_limit,
        } => {
            scan(
                url,
                wordlist,
                concurrency,
                cli.no_progress,
                match_status,
                match_regex,
                filter_size_min,
                filter_size_max,
                rate_limit,
                shutdown,
            )
            .await;
        }
        Commands::Benchmark {
            url,
            requests,
            concurrency,
        } => {
            benchmark(url, requests, concurrency, cli.no_progress, shutdown).await;
        }
    }
}
