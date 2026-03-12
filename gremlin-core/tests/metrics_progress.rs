use gremlin_core::metrics::Metrics;
use std::sync::atomic::Ordering;

#[test]
fn request_total_counter_increments() {
    let metrics = Metrics::new();

    metrics.record_request();
    metrics.record_request();

    assert_eq!(metrics.requests_total.load(Ordering::Relaxed), 2);
}

#[test]
fn responses_success_counter_increments() {
    let metrics = Metrics::new();

    metrics.record_success();
    metrics.record_success();
    metrics.record_success();

    assert_eq!(metrics.responses_success.load(Ordering::Relaxed), 3);
}

#[test]
fn responses_error_counter_increments() {
    let metrics = Metrics::new();

    metrics.record_error();
    metrics.record_error();
    metrics.record_error();

    assert_eq!(metrics.responses_error.load(Ordering::Relaxed), 3);
}

#[test]
fn responses_filtered_counter_increments() {
    let metrics = Metrics::new();

    metrics.record_filtered();
    metrics.record_filtered();
    metrics.record_filtered();

    assert_eq!(metrics.responses_filtered.load(Ordering::Relaxed), 3);
}

#[test]
fn latency_total_ns_update() {
    let metrics = Metrics::new();

    metrics.record_latency(100);
    metrics.record_latency(1000);

    assert_eq!(metrics.latency_total_ns.load(Ordering::Relaxed), 1100);
}
