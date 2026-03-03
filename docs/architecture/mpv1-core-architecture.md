# MVP1 Core Architecture

## Overview

MVP1 implements a streaming, bounded-concurrency HTTP scanner with
explicit backpressure and strict execution limits.

The execution pipeline is:

Generator -> Bounded Queue -> Worker Pool (Fixed size) -> HttpEngine

## Components

### ScanConfig (gremin-core)

Responsible for:
- Validated runtime configuration
- URL parsing
- Wordlist existence validation
- Concurrency validation

Prevents invalid runtime state.

---

### WordlistReader (gremin-core)

- Async line-by-line streaming
- No full-file buffering
- Memory usage proportional to line size

Ensures scalability for large wordlists.

---

### JobGenerator (gremin-core)

- Consumes WordlistReader
- Performs FUZZ replacement
- Generates `ScanRequest`
- Assigns monotonic RequestId
- Streams requests without buffering

Error boundary: GeneratorError

---

### Bounded Queue (gremin-core)

- Tokio mpsc channel with fixed capacity
- Applies backpressure
- Prevents unbounded memory growth
- Does not expose Tokio types directly

Capacity is currently tied to concurrency.

--- 

### HttpEngine (engine)

- Reusable reqwest client
- Executes `ScanRequest`
- Returns `ScanResponse`
- Maps reqwest errors -> `ResponseError`
- No panics
- No reqwest error leakage

---

### Fixed Worker Pool (cli orchestration)

- Spawn N workers
- Receiver wrapped in Arc<Mutex<_>>
- Only recv() is synchronized
- Exection happens outside lock
- Strictly bounds in-flight HTTP requests

Maximum concurrent executions = configured concurrency.

---

## Concurrency Model

- Producer generates requests sequentially.
- Queue provides bounded buffering.
- Workers pull requests sequentially but execute concurrently.
- Execution concurrency is fixed and explicit.

No unbounded task spawning exists in MVP1.

---

## Backpressure Guarantees

Backpressure is enforced by the bounded queue.

If workers are slower than producer:

- Queue fills
- send().await blocks
- Producer pauses

This ensure memory stability.

---

## Error Boundaries

- ConfigError -> configuration layer
- GeneratorError -> generation layer
- ResponseError -> transport layer
- EngineError -> engine construction only

No lower-level error types are leaked upward.

---

## Panic Policy

- No panics in core or engine crates.
- Only CLI (orchestration layer) may exit process.
- All other layers propagate structured errors.

---

## Limitations (MVP1)

- No rate limiting
- No retries
- No timeout customization
- No result aggregation
- No structured output
- No graceful shutdown signals
- No metrics
- No cancellation support

These are intentionally deferred.

---

## Future Evolution

Potential improvements:

- Independent queue capacity vs. worker count
- Rate limiter in dispatcher layer
- Retry strategy with exponential backoff
- Structured result sink
- Metrics and observability
- Cancellation via shutdown signal
