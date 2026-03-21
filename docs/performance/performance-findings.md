# Performance Findings

## Overview

This document summarizes performance characteristics, benchmarks, and
tuning decisions for the scanner.

---

## Benchmark Results

Environment:
- concurrency: 50
- requests: 10_000

Results:

| Metrics | Value |
|---------|-------|
| Throughput | 5295.15 req/sec |
| Peak memory | 1.6 MB |
| Avg latency | 0.1898 ms|

---

## Key Observations

- Throughput scales linearly upto moderate concurrency
- Diminishing returns beyond ~2-4x CPU count
- Memory usage remains stable under sustained load
- No long-term memory growth observed

---

## Hotspots Identified

- Regex matching on response body
- Response body allocations
- UTF-8 conversions (`from_utf8_lossy`)

---

## Optimizations applied

### Regex Optimization

- Switched to `regex::bytes::Regex`
- Removed UTF-8 conversions

Impact:

- reduced allocations
- improved matching speed

---

### Metrics

- Used atomic counters with relaxed ordering

Impact

- negligible overhead

---

## Concurrency Tuning

- concurrency is user-controlled
- Recommended range:
  - IO-bound: 2-4x CPU cores
- Excessive concurrency leads to:
  - increased latency
  - no throughput gain

---

## Observability Impact

- Tracing controlled via log level
- Most tracing overhead removed by `--quiet` flag
- ProgressBar can be removed via `--no-progress` flag for 
  high-throughput scans

---

## Recommendations

- Use `--quiet` for benchmarking
- Avoid regex unless necessary
- Tune concurrency based on workload (2-4x CPU cores)
- Use benchmark command for validation

---

## Future Work

- introduce rate-aware scheduling
- optimize body-handling
- evaluate zero-copy response processing
