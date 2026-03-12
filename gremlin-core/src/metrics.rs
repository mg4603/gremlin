use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

pub struct Metrics {
    pub requests_total: AtomicU64,
    pub responses_success: AtomicU64,
    pub responses_error: AtomicU64,
    pub responses_filtered: AtomicU64,
    pub latency_total_ns: AtomicU64,
}

impl Metrics {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            requests_total: AtomicU64::new(0),
            responses_success: AtomicU64::new(0),
            responses_error: AtomicU64::new(0),
            responses_filtered: AtomicU64::new(0),
            latency_total_ns: AtomicU64::new(0),
        })
    }

    pub fn record_request(&self) {
        self.requests_total.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_success(&self) {
        self.responses_success.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_error(&self) {
        self.responses_error.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_filtered(&self) {
        self.responses_filtered.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_latency(&self, nano: u64) {
        self.latency_total_ns.fetch_add(nano, Ordering::Relaxed);
    }
}
