# ADR 0002: Use Tokio as Async Runtime

Status: Accepted

## Context

The scanner is a high-throughput networked application that must handle
large numbers of concurrent HTTP requests efficiently. It requires an
asynchronous runtime capable of managing I/O bounded tasks, connection
pooling, timers and task scheduling with predictable performance 
characteristics.

Choosing the runtime early is important because it affects nearly all
parts of the system, including networking, concurrency patterns, and
library compatability.

## Decision

Use tokio as the async runtime for the project.

Tokio will provide the task scheduler, async I/O primitives, timing 
utilities, and ecosystem integrations needed for building the scanning
engine.

## Alternatives Considered

### async-std

Provides a simpler API and lighter mental model but has a smaller 
ecosystem and fewer production-grade integrations compared to Tokio.
Many networking libraries are primarily designed around Tokio.

### smol

Lightweight and minimal, but not as widely adopted and lacks the 
ecosystem maturity required for a network-heavy system.

### Custom runtime / blocking model

A blocking or custom runtime would significantly limit scalability and
make high concurrency hard to achieve efficiently.

## Consequences

### Positive

- Mature and widely adopted runtime
- Strong ecosystem support
- Excellent networking performance
- Compatible with major async libraries
- Well-documented and battle-tested
- Supports structured concurrency patterns

### Negative

- Larger dependency footprint
- Requires understanding async programming model
- More configuration options compared to simpler runtimes

Overall, tokio provides the best balance of performance, ecosystem 
support, and long-term stability for a high-concurrency scanning tool.
