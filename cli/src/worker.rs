use std::sync::Arc;
use std::time::Instant;

use indicatif::ProgressBar;
use tokio::sync::Mutex;
use tokio::task::{self, JoinHandle};
use tracing::{Instrument, Span, debug, error, info_span, trace};

use engine::engine::HttpEngine;
use gremlin_core::metrics::Metrics;
use gremlin_core::pipeline::executor::Pipeline;
use gremlin_core::queue::TaskReceiver;
use gremlin_core::rate_limiter::TokenBucket;
use gremlin_core::request::ScanRequest;

pub fn spawn_workers(
    concurrency: usize,
    receiver: Arc<Mutex<TaskReceiver<ScanRequest>>>,
    engine: Arc<HttpEngine>,
    pipeline: Arc<Pipeline>,
    limiter: Option<Arc<Mutex<TokenBucket>>>,
    metrics: Arc<Metrics>,
    pb: Option<ProgressBar>,
) -> Vec<JoinHandle<()>> {
    let mut handles = Vec::new();
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

                    if let Some(limiter) = &limiter {
                        limiter.lock().await.acquire().await;
                    }

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

                    if let Some(pb) = &pb {
                        pb.inc(1);
                    }
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
    handles
}
