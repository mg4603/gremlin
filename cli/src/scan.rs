use std::path::PathBuf;
use std::sync::Arc;

use indicatif::{ProgressBar, ProgressStyle};
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use tracing::info;

use engine::engine::HttpEngine;
use gremlin_core::config::ScanConfig;
use gremlin_core::generator::ScanJobGenerator;
use gremlin_core::metrics::Metrics;
use gremlin_core::pipeline::executor::Pipeline;
use gremlin_core::queue::bounded;
use gremlin_core::rate_limiter::TokenBucket;
use gremlin_core::request::ScanRequest;
use gremlin_core::wordlist::WordlistReader;

use crate::generator::run_generator;
use crate::worker::spawn_workers;

#[allow(clippy::too_many_arguments)]
pub async fn scan(
    url: String,
    wordlist: PathBuf,
    concurrency: usize,
    no_progress: bool,
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

    let limiter = rate_limit.map(|rate| Arc::new(Mutex::new(TokenBucket::new(rate))));

    let metrics = Metrics::new();

    let wordlist_len = match WordlistReader::count_lines(&config.wordlist) {
        Ok(l) => l as u64,
        Err(e) => {
            eprintln!("failed to count number of lines: {e}");
            std::process::exit(1);
        }
    };

    let pb = if !no_progress {
        Some(ProgressBar::new(wordlist_len))
    } else {
        None
    };

    if let Some(pb) = &pb {
        pb.set_style(
            ProgressStyle::with_template("[{elapsed_precise}] [{wide_bar}] {pos}/{len} ({eta})")
                .unwrap(),
        );
    }

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

    let generator = ScanJobGenerator::new(config)
        .await
        .expect("generator init failed");

    run_generator(generator, sender, shutdown).await;

    for handle in handles {
        let _ = handle.await;
    }

    if let Some(pb) = &pb {
        pb.finish_with_message("scan complete");
    }

    info!("scan complete");
}
