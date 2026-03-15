use std::sync::Arc;
use std::sync::atomic::Ordering;
use std::time::Instant;

use indicatif::{ProgressBar, ProgressStyle};
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use tracing::info;

use engine::engine::HttpEngine;
use gremlin_core::config::BenchmarkConfig;
use gremlin_core::generator::BenchmarkJobGenerator;
use gremlin_core::metrics::Metrics;
use gremlin_core::pipeline::executor::Pipeline;
use gremlin_core::queue::bounded;
use gremlin_core::rate_limiter::TokenBucket;
use gremlin_core::request::ScanRequest;

use crate::generator::run_generator;
use crate::worker::spawn_workers;

pub async fn benchmark(url: String, requests: usize, concurrency: usize, shutdown: JoinHandle<()>) {
    let config = match BenchmarkConfig::new(url, requests, concurrency) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Benchmark configuration error: {e}");
            std::process::exit(1);
        }
    };

    let start = Instant::now();
    let metrics = Metrics::new();

    let concurrency = config.concurrency;
    let (sender, receiver) = bounded::<ScanRequest>(concurrency);

    let engine = match HttpEngine::new() {
        Ok(e) => Arc::new(e),
        Err(e) => {
            eprintln!("engine init failed: {e}");
            std::process::exit(1);
        }
    };

    let receiver = Arc::new(Mutex::new(receiver));
    let pipeline = Arc::new(Pipeline::new(vec![], vec![]));
    let limiter = Arc::new(Mutex::new(TokenBucket::new(requests as u64)));

    let pb = ProgressBar::new(requests as u64);

    pb.set_style(
        ProgressStyle::with_template("[{elapsed_precise}] [{wide_bar}] {pos}/{len} ({eta})")
            .unwrap(),
    );

    info!(
        concurrency = concurrency,
        number_of_requests = requests,
        "benchmarking started"
    );

    let handles = spawn_workers(
        concurrency,
        receiver,
        engine,
        pipeline,
        limiter,
        metrics.clone(),
        pb.clone(),
    );

    let generator = BenchmarkJobGenerator::new(config).expect("generator init failed");

    run_generator(generator, sender, shutdown).await;

    for handle in handles {
        _ = handle.await;
    }

    pb.finish_with_message("benchmarking complete");
    info!("benchmarking complete");

    let elapsed = start.elapsed();
    let completed = metrics.requests_total.load(Ordering::Relaxed);
    let throughput = completed as f64 / elapsed.as_secs_f64();

    println!("Benchmark Results");
    println!("------------------");
    println!("Requests: {completed}");
    println!("Elapsed: {:.2?}", elapsed);
    println!("Throughput: {:.2?} req/sec", throughput);
}
