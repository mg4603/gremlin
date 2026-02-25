# ADR 0005: Use Bounded Queues for Task Scheduling

Status: Accepted

## Context

The scanner will generate and process a potentially very large number 
of requests concurrently. Without flow control, producers such as the
job generator could create tasks faster than workers can procss them 
leading to unbounded memory growth and eventual instability.

The system requires a mechanism to provide backpressure so that task 
production naturally slows down when processing capcaity is reached.

## Decision

Use bounded asynchronous queues for task scheduling and communication 
between components.

Bounded channels will limit the number of in-flight tasks and ensure 
that producers wait when the queue reaches capacity, maintaining stable
memory usage under load.

## Alternatives considered

### Unbounded queues

Unbounded channels simplify implementation but can lead to uncontrollable
memory growth if producers outpace consumers, especially during large
scans.

### Custom backpressure mechanisms

Implementing a custom flow control system would increase complexity
without providing clear advantages over built-in bounded channels.

### Batch submission with manual throttling

Manually controlling submission rates would be less reliable and more 
complex than relying on queue backpressure.

## Consequences

### Positive

- Prevents memory exhaustion under high load
- Provides natural backpressure between components
- Improves system stability
- Simplifies concurrency control
- Aligns with streaming processing model
- Easier to reason about system throughput

### Negative

- Requires tuning queue size
- Producers may block when capacity is reached
- Slightly more complex coordination between components

Overall, bounded queues provide a simple and reliable mechanism to 
maintain system stability when supporting high concurrency.
