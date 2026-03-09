use std::time::{Duration, Instant};

use gremlin_core::rate_limiter::TokenBucket;

#[tokio::test]
async fn token_bucket_allows_requests() {
    let mut limiter = TokenBucket::new(5);

    for _ in 0..5 {
        limiter.acquire().await;
    }
}

#[tokio::test]
async fn token_bucket_limits_rate() {
    let mut limiter = TokenBucket::new(1);

    limiter.acquire().await;

    let start = Instant::now();
    limiter.acquire().await;

    assert!(start.elapsed().as_millis() >= 1000);
}

#[tokio::test]
async fn token_bucket_blocks_when_empty() {
    let mut limiter = TokenBucket::new(5);

    for _ in 0..5 {
        limiter.acquire().await;
    }

    // 6th request should not complete instantly
    let result = tokio::time::timeout(Duration::from_millis(10), limiter.acquire()).await;

    // timeout expired, meaning it blocked as expected
    assert!(result.is_err());
}

#[tokio::test]
async fn token_bucket_refills() {
    let mut limiter = TokenBucket::new(5);

    for _ in 0..5 {
        limiter.acquire().await;
    }

    tokio::time::sleep(Duration::from_millis(200)).await;

    let result = tokio::time::timeout(Duration::from_millis(10), limiter.acquire()).await;

    assert!(result.is_ok());
}
