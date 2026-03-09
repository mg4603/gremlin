# Request and Response Processing Pipeline

## Overview

The scanner processes requests and responses through two connected
stages:

1. **Execution Pipeline** - generates and executes HTTP requests
2. **Processor Pipeline** - evaluates responses and produces results

---

# Execution Pipeline

The execution pipeline is responsible for producing `ScanResponse`
from generated requests.

```
Generator
   ↓
ScanRequest
   ↓
Bounded Queue(sender → receiver)
   ↓
Workers
   ↓
HTTP Engine(execute)
   ↓
ScanResponse
```

## Stage Descriptions:

### Generator

The generator reads entries from wordlist and produces `ScanRequest`
objects.

Responsibilities:

- FUZZ substitution
- Request ID assignment
- Streaming request generation

Output: `ScanRequest`

---

### Bounded Queue

The queue separates request generation from execution.

Responsibilities:

- Provide backpressure
- Prevent unbounded memory growth
- Decouple producer and workers

Implementation: bounded `mpsc` channel.

---

### Workers

Workers pull requests from the queue and execute them.

Responsibilities:

- Maintain bounded concurrency
- Execute HTTP requests
- Forward responses to the processing pipeline

---

### HTTP Engine

The HTTP engine performs the network request.

Input: `ScanRequest`
Output: `ScanResponse`

`ScanResponse` contains:

- request_id
- status
- headers
- body
- timing
- error

---

# Processing pipeline

After a response is produced, it is evaluated by the processing 
pipeline.

```
ScanResponse
   ↓
Matchers
   ↓
Filters
   ↓
Pipeline Executor
   ↓
ScanResult

```

---

## Matchers

Matchers evaluate `ScanResponse` and determine whether a response is
interesting.

Examples:

- `StatusMatcher`
- `RegexMatcher`

Matcher semantics:

- Each matcher returns `bool`
- **Any matcher may match**
- If no matcher matches, response is discarded

---

## Filters

Filters also evaluate `ScanResponse`.

Filters determine whether a matched response should be allowed to 
produce a result.

Example:

- `SizeFilter`

Filter semantics:

- Each filter returns `bool`
- **All filters must allow the response**
- If any filter rejects the response, the response is discarded

---

## Pipeline executor

The pipeline executor orchestrates matcher and filter stages.

If a response:

1. matches atleast one matcher
2. passes all filters

then the executor constructs a `ScanResult`.

`ScanResult` contains:

- request_id
- response (`ScanResponse`)
- matched (bool)
- notes ([])

The executor is responsible for converting responses into results.

---

# Worker Integration

Workers execute the full pipeline as follows:

```
request → engine.execute() → ScanResponse → pipeline.process() → Option<ScanResult>
```

Only `Some(result)` values are emitted.

---

# Backpressure

Backpressure occurs at the bounded queue.

If workers cannot keep up:

- queue fills
- request generation pauses
- memory remains bounded

The processing pipeline itself introduces **no additional buffers or
concurrency layers**.

---

# Future extensions

Potential improvements:

- matcher explanations stored in `ScanResult.notes`
- rule DSL for pipeline configuration
- plugin matcher/filter system
- configurable result sink


