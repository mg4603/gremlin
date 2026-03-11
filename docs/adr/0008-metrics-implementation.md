# ADR 0008: Metrics Implementation Using Atomic Counters

Status: Accepted

## Context

The scanner process a large number of HTTP requests concurrently 
through worker tasks. Observability is required to track runtime
statistics such as:

- total requests
- successful responses
- failed responses
- request latency

Metrics collection must not introduce contention or measurable
performance overhead, since workers operate at high throughput.

The metrics system must also remain simple for MVP3, while leaving
room for extensibility.

## Decision

Metrics will be implemented using in-process atomic counters.

Each metric will be represented by `AtomicU64` and be updated using
`fetch_add` with `Ordering::Relaxed`.

Example metrics include:

- `requests_total`
- `responses_success`
- `responses_error`
- `latency_total_ns`

Metrics are shared across workers using `Arc<Metrics>`.

Latency is recorded as the cumulative nanoseconds across requests.
Average latency can be derived as:

```avg_latency = latency_total_ns / requests_total```

## Alternatives Considered

### Channel-based metrics aggregation

Workers could send metrics events to a dedicated metrics task through 
a channel.

Rejected because:

- introduces additional scheduling overhead
- adds queue pressure under high request rates
- increases implementation complexity

### External metrics libraries (Prometheus / OpenTelemetry)

Using a thrid-party metrics library would provide more features
such as exporters and labels.

Rejected for MVP3 because:

- adds dependencies
- increases runtime overhead
- not required for current observability needs

### Mutex-protected counters

Metrics could be stored in a struct protected by `Mutex`.

Rejected because:

- introduces lock contention across workers
- unnecessary for simple counters

## Consequences

### Positive

- Extremely low overhead metrics collection.
- Lock-free updates using atomic operations
- Simple implementation
- Safe to share across worker threads

### Negative

- Metrics aggregation is limited to simple counters
- No built-in exporter support
- Future integration with external metrics systems may require 
  refactoring
