use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;

use clap::{Parser, Subcommand};
use tokio::sync::Mutex;
use tokio::{signal, task};
use tracing::{debug, error, info, info_span, trace};

use engine::engine::HttpEngine;
use gremlin_core::config::ScanConfig;
use gremlin_core::generator::JobGenerator;
use gremlin_core::logging;
use gremlin_core::metrics::Metrics;
use gremlin_core::pipeline::executor::Pipeline;
use gremlin_core::queue::bounded;
use gremlin_core::rate_limiter::TokenBucket;
use gremlin_core::request::ScanRequest;

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
}

#[tokio::main]
async fn main() {
    logging::init();

    let cli = Cli::parse();

    let mut shutdown = tokio::spawn(async {
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
        } => match ScanConfig::new(
            url,
            wordlist,
            concurrency,
            match_status,
            match_regex,
            filter_size_min,
            filter_size_max,
            rate_limit,
        ) {
            Ok(config) => {
                let concurrency = config.concurrency;

                let (sender, receiver) = bounded::<ScanRequest>(concurrency);

                let engine = match HttpEngine::new() {
                    Ok(engine) => Arc::new(engine),
                    Err(e) => {
                        eprintln!("engine init failed: {}", e);
                        std::process::exit(1);
                    }
                };

                let receiver = Arc::new(Mutex::new(receiver));

                let mut handles = Vec::new();

                let matchers = config.build_matchers();
                let filters = config.build_filters();

                let pipeline = Arc::new(Pipeline::new(matchers, filters));

                let rate = rate_limit.unwrap_or(100);

                let limiter = Arc::new(Mutex::new(TokenBucket::new(rate)));

                let metrics = Metrics::new();

                for _ in 0..concurrency {
                    let rx = receiver.clone();
                    let engine = engine.clone();
                    let pipeline = pipeline.clone();
                    let limiter = limiter.clone();
                    let metrics = metrics.clone();
                    let handle = task::spawn(async move {
                        loop {
                            metrics.record_request();
                            let start = Instant::now();

                            let request_opt = {
                                let mut locked = rx.lock().await;
                                locked.recv().await
                            };

                            let request = match request_opt {
                                Some(request) => request,
                                None => break,
                            };

                            let span = info_span!(
                                "scan_request",
                                request_id = %request.id,
                                url = %request.url,
                            );

                            let _enter = span.enter();

                            limiter.lock().await.acquire().await;
                            let response = engine.execute(request).await;

                            if let Some(status) = response.status {
                                debug!(status = %status, "response received");
                            }

                            match response.error {
                                Some(e) => {
                                    metrics.record_error();
                                    error!(error=%e, "request failed");
                                }
                                None => match pipeline.process(response) {
                                    Some(result) => {
                                        metrics.record_success();

                                        debug!(
                                            request_id = %result.request_id,
                                            matched = result.matched,
                                            "pipeline emitted result",
                                        );
                                    }
                                    None => {
                                        metrics.record_success();
                                        trace!("response filtered");
                                    }
                                },
                            }

                            let elapsed = start.elapsed().as_nanos() as u64;
                            metrics.record_latency(elapsed);
                            span.record("latency_ns", elapsed);
                        }
                    });

                    handles.push(handle);
                }

                let mut generator = JobGenerator::new(config)
                    .await
                    .expect("generator init failed");

                loop {
                    tokio::select! {
                        _ = &mut shutdown => {
                            println!("shutdown signal received");
                            break;
                        }

                        job = generator.next() => {
                            match job {
                                Ok(Some(request)) => {
                                    if sender.send(request).await.is_err() {
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
