# ADR 0007: Pipeline Abstraction via Matcher and Filter Traits

Status: Approved

## Context

The scanner must process HTTP responses and determine which results
are significant. To support extensibility and future features such
such as plugins or rule engines, the processing pipeline must be 
modular.

We need abstractions that separate signal detection from result 
visibility while remaining thread-safe and independent from the HTTP 
execution engine.


## Decision

Introduce two pipeline traits:

Matcher:
    fn matches(&self, response: &ScanResponse) -> bool;

Filter:
    fn allow(&self, result: &ScanResult) -> bool;

Both traits must implement Send + Sync so that they can be safely 
shared across worker threads.

## Alternatives Considered

### Single Combined trait

Rejected because matching and filtering represent different pipeline
responsibilities and should remain independently composable.

### Async trait

Rejected because matching and filtering should remain CPU-bound 
operations.

### Dynamic rule engine

Deferred for a later milestone.

## Consequences

### Positive

- Clear separation of concerns
- Supports future plugin systems
- Thread-safe pipeline primitives
- Flexible rule composition

### Negative

- Adds an additional abstraction layer
- Requires orchestration logic in future pipeline stage
