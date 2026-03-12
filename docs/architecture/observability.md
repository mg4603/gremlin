# Observability

The scanner exposes observability through three mechanisms:

- logging
- tracing spans
- metrics counters

These provide visibility into request execution, pipeline behavior and
system shutdown.

---

# Logging

Logging uses the `tracing` ecosystem.

Logs capture high-level lifecycle events such as:

- scan start
- scan completion
- shutdown signals
- worker termination

Logging is intended for operational visibility and debugging.

---

# Tracing Spans

Tracing spans instrument the lifecycle of each request.

Each request execution creates a span:

Span flow:
```
ScanRequest
   ↓
HTTP execution
   ↓
ScanResponse
   ↓
Pipeline evaluation
```

Captured fields:
- `request_id`
- `url`
- `latency`
- `status`
- `error`

Example trace:

```
scan_request(request_id=42 url="/admin")
├── response received status 200
└── response not filtered
```

Errors are recorded as structured tracing events.

---

# Metrics

Metrics track request statistics using Atomic counters.

Counters include:

| Metric | Description |
|--------|-------------|
| `requests_total` | total requests executed |
| `responses_success` | successful HTTP responses |
| `responses_error` | transport failures |
| `responses_filtered` | responses rejected by pipeline |
| `latency_total_ns` | cumulative request latency |

Average latency can be derived as:

```
avg_latency = latency_total_ns / requests_total
```

Metrics use `AtomicU64` with relaxed ordering to minimize overhead.

---

# Progress Reporting

Progress is derived from request counter.

The CLI progress bar displays:

`requests_total / wordlist_len`

Progress reporting does not introduce additional synchronization.

--- 

# Shutdown flow

Graceful shutdown is triggered by termination signals (`SIGINT` / 
CTRL+C).

Shutdown sequence:
```
signal received
   ↓
generator stops producing requests
   ↓
queue sender dropped
   ↓
workers drain remaining requests
   ↓
worker tasks exit
   ↓
program terminates
```

This ensures no in-flight tasks are dropped.

---

# Performance Consideration

Observability mechanisms are designed to avoid impacting scan
throughput:

- metrics use atomic counters
- tracing is structured and level-controlled
- progress updates are throttled

The system maintains high concurrency without introducing locks in the
request execution path.
