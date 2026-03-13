use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;

use indicatif::{ProgressBar, ProgressStyle};
use tokio::sync::Mutex;
use tokio::task::{self, JoinHandle};
use tracing::{Instrument, Span, debug, error, info, info_span, trace};

use engine::engine::HttpEngine;
use gremlin_core::config::ScanConfig;
use gremlin_core::generator::JobGenerator;
use gremlin_core::metrics::Metrics;
use gremlin_core::pipeline::executor::Pipeline;
use gremlin_core::queue::bounded;
use gremlin_core::rate_limiter::TokenBucket;
use gremlin_core::request::ScanRequest;
use gremlin_core::wordlist::WordlistReader;

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
    mut shutdown: JoinHandle<()>,
) {
    match ScanConfig::new(
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

            let wordlist_len = match WordlistReader::count_lines(&config.wordlist) {
                Ok(l) => l as u64,
                Err(e) => {
                    eprintln!("failed to count number of lines: {e}");
                    std::process::exit(1);
                }
            };

            let pb = ProgressBar::new(wordlist_len);

            pb.set_style(
                ProgressStyle::with_template(
                    "[{elapsed_precise}] [{wide_bar}] {pos}/{len} ({eta})",
                )
                .unwrap(),
            );

            info!(
                concurrency = concurrency,
                wordlist_len = wordlist_len,
                "scan started"
            );

            for _ in 0..concurrency {
                let rx = receiver.clone();
                let engine = engine.clone();
                let pipeline = pipeline.clone();
                let limiter = limiter.clone();
                let metrics = metrics.clone();
                let pb = pb.clone();

                let handle = task::spawn(async move {
                    loop {
                        let request_opt = {
                            let mut locked = rx.lock().await;
                            locked.recv().await
                        };

                        let request = match request_opt {
                            Some(request) => request,
                            None => break,
                        };

                        metrics.record_request();
                        let request_id = request.id;
                        let request_url = request.url.clone();

                        async {
                            let start = Instant::now();

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
                                        metrics.record_filtered();
                                        trace!("response filtered");
                                    }
                                },
                            }

                            let elapsed = start.elapsed().as_nanos() as u64;
                            metrics.record_latency(elapsed);
                            Span::current().record("latency_ns", elapsed);
                            pb.inc(1);
                        }
                        .instrument(info_span!(
                            "scan_request",
                            request_id = request_id,
                            url = %request_url,
                            latency_ns = tracing::field::Empty,
                        ))
                        .await;
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

            pb.finish_with_message("scan complete");
            info!("scan complete");
        }
        Err(e) => {
            eprintln!("Configuration error: {e}");
            std::process::exit(1);
        }
    }
}
