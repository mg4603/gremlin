# ADR 0004: Use Streaming Processing Model for Scan Pipeline

Status: Accepted

## Context

The scanner is designed to handle large volumes of HTTPS requests and
responses, potentially reaching hundreds of thousands or millions of 
operations in a single run. Processing results in baches or storing all
responses in memory before analysis could lead to excessive memory 
usage and reduced responsiveness.

The system needs a processing model that allows responses to be handled
incrementally as they are produced, enabling efficient resource usage
and real-time analysis.

## Decision

Adopt a streaming processing model where scan results flow through 
pipeline stages as they are produced rather than being accumulated in 
memory.

Responses will be processed incrementally through matchers, filers, and
result sinks, allowing the system to operate with a stable memory 
footprint regardless of scan size.

## Alternatives considered

### Batch processing

Collecting responses into batches before processing would simplify 
implementation but could significantly increase memory usage and delay 
result availability, especially for large scans.

### Store-all-then-process model

Storing all responses before processing would provide flexibility but
would not scale well for large workloads and could lead to memory 
exhaustion.

### Hybrid model

A hybrid approach was considered but introduces additional complexity 
without clear advantages over a fully streaming design for this use 
case.


## Consequences

### Positive

- Stable memory independent of scan size
- Results available in real time
- Improved scalability for large workloads
- Reduced latency between request and analysis
- Aligns with asynchronous processing model
- Enables flexible pipeline composition

### Negative

- Slightly more complex control flow
- Requires careful error handling across stages
- Debugging may require tracing pipeline flow

Overall, a streaming processing model provides the best balance of 
scalability, performance, and responsiveness for a high-throughput 
scanning engine.
