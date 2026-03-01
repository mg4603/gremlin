# ADR 0006: HTTP Client Selection for Execution Engine

Status: Accepted

## Context

The HTTP execution engine requires an asynchronous HTTP client to 
execute `ScanRequest` values under high concurrency.

The client must:

- Integrate with Tokio
- Support connection pooling
- Support HTTPS with TLS
- Provide reasonable performance
- Be production-ready and well maintained

This decision affects performance, dependency footprint, error handling
and long-term maintainability of the engine.

## Decision

We will use `reqwest` with `rust-tls` feature enabled as the HTTP 
client for the execution engine.

## Alternatives Considered

### hyper (direct usage)

Pros:

- Lower-level control
- Smaller abstraction overhead
- Greater customization flexibility

Cons:

- Higher implementation complexity
- More boilerplate
- Manual connection and body handling
- Increased maintenace burden for MVP

Rejected for MVP due to complexity.

### surf

Pros:

- Async HTTP client
- Clean API

Cons:

- Less commonly used in high-performance scanning tools
- Smaller ecosystem compared to reqwest

Rejected due to ecosystem maturity concerns.

### isahc (libcurl-based)

Pros:

- Mature underlying C implementation
- HTTP/2 support

Cons:

- Native dependency complexity
- Less seamless Tokio integration
- Increased build surface

Rejected due to native dependency overhead.

## Consequences

### Positive

- Mature and widely adopted async client
- Built-in connection pooling
- Clean integration with Tokio runtime
- Pure Rust TLS via rustls
- Faster development velocity for MVP

### Negative

- Larger dependency tree
- Slight abstraction overhead compared to raw hyper
- Less fine-grained transport control

This decision may be revisited if profiling reveals performance
limitations or need for lower-level control.
