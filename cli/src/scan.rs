use std::path::PathBuf;
use std::sync::Arc;

use indicatif::{ProgressBar, ProgressStyle};
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use tracing::info;

use engine::engine::HttpEngine;
use gremlin_core::config::ScanConfig;
use gremlin_core::generator::JobGenerator;
use gremlin_core::metrics::Metrics;
use gremlin_core::pipeline::executor::Pipeline;
use gremlin_core::queue::{TaskSender, bounded};
use gremlin_core::rate_limiter::TokenBucket;
use gremlin_core::request::ScanRequest;
use gremlin_core::wordlist::WordlistReader;

use crate::worker::spawn_workers;

#[allow(clippy::too_many_arguments)]
pub async fn scan(
    url: String,
    wordlist: PathBuf,
    concurrency: usize,
    match_status: Option<u16>,
    match_regex: Option<String>,
    filter_size_min: Option<usize>,
    filter_size_max: Option<usize>,
    rate_limit: Option<u64>,
    shutdown: JoinHandle<()>,
) {
    let config = match ScanConfig::new(
        url,
        wordlist,
        concurrency,
        match_status,
        match_regex,
        filter_size_min,
        filter_size_max,
        rate_limit,
    ) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Configuration error: {e}");
            std::process::exit(1);
        }
    };

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

    let matchers = config.build_matchers();
    let filters = config.build_filters();

    let pipeline = Arc::new(Pipeline::new(matchers, filters));

    let rate = rate_limit.unwrap_or(100);

    let limiter = Arc::new(Mutex::new(TokenBucket::new(rate)));

    let metrics = Metrics::new();

    let wordlist_len = match WordlistReader::count_lines(&config.wordlist) {
        Ok(l) => l as u64,
        Err(e) => {
            eprintln!("failed to count number of lines: {e}");
            std::process::exit(1);
        }
    };

    let pb = ProgressBar::new(wordlist_len);

    pb.set_style(
        ProgressStyle::with_template("[{elapsed_precise}] [{wide_bar}] {pos}/{len} ({eta})")
            .unwrap(),
    );

    info!(
        concurrency = concurrency,
        wordlist_len = wordlist_len,
        "scan started"
    );

    let handles = spawn_workers(
        concurrency,
        receiver,
        engine,
        pipeline,
        limiter,
        metrics,
        pb.clone(),
    );

    run_generator(config, sender, shutdown).await;

    for handle in handles {
        let _ = handle.await;
    }

    pb.finish_with_message("scan complete");
    info!("scan complete");
}

pub async fn run_generator(
    config: ScanConfig,
    sender: TaskSender<ScanRequest>,
    mut shutdown: JoinHandle<()>,
) {
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
}
