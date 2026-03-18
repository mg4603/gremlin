use gremlin_core::config::{BenchmarkConfig, ScanConfig};
use gremlin_core::error::ConfigError;

fn build_scan_config(
    status_code: Option<u16>,
    regex: Option<String>,
    concurrency: usize,
    min: Option<usize>,
    max: Option<usize>,
    rate: Option<u64>,
) -> Result<ScanConfig, ConfigError> {
    let file = tempfile::NamedTempFile::new().unwrap();
    ScanConfig::new(
        "https://example.com".to_string(),
        file.path().to_path_buf(),
        concurrency,
        status_code,
        regex,
        min,
        max,
        rate,
    )
}

#[test]
fn reject_zero_concurrency_scan_config() {
    let result = build_scan_config(None, None, 0, None, None, None);
    assert!(matches!(result, Err(ConfigError::InvalidConcurrency(0))));
}

#[test]
fn reject_invalid_size_range() {
    let result = build_scan_config(None, None, 10, Some(200), Some(100), Some(2));
    assert!(matches!(result, Err(ConfigError::InvalidSizeRange { .. })));
}

#[test]
fn reject_invalid_rate_limit() {
    let result = build_scan_config(None, None, 10, None, None, Some(0));

    assert!(matches!(result, Err(ConfigError::InvalidRateLimit(0))))
}

#[test]
fn reject_invalid_status_code() {
    let result = build_scan_config(Some(1000), None, 10, None, None, None);
    assert!(matches!(result, Err(ConfigError::InvalidStatusCode(1000))));
}

#[test]
fn reject_invalid_regex() {
    let regex = "[abc";
    let result = build_scan_config(None, Some(regex.to_string()), 10, None, None, None);
    assert!(matches!(result, Err(ConfigError::InvalidRegex(..))));
}

fn build_benchmark_config(
    requests: usize,
    concurrency: usize,
) -> Result<BenchmarkConfig, ConfigError> {
    BenchmarkConfig::new("https://example.com".to_string(), requests, concurrency)
}

#[test]
fn reject_zero_concurrency_benchmark_config() {
    let result = build_benchmark_config(100, 0);
    assert!(matches!(result, Err(ConfigError::InvalidConcurrency(0))));
}

#[test]
fn reject_invalid_number_of_requests() {
    let result = build_benchmark_config(0, 10);
    assert!(matches!(
        result,
        Err(ConfigError::InvalidNumberOfRequests(0))
    ));
}
