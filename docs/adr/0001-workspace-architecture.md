# ADR 0001: Use Rust Workspace Architecture

Status: Accepted

## Context:

The project is a high-performance HTTP scanning engine that will consist
of multiple components such as a CLI interface, core domain models, 
execution engine, and processing pipeline.

As the system grows, maintaining clear boundaries between components 
will be important for scalability, testability, and maintainability. 
A single crate structure could lead to tight coupling, slower compile
times, and difficulty manadging dependencies across different parts of
the system.

We need an architecture that supports modular development while keeping
the project easy to navigate and evolve.

## Decision

Use a Rust workspace with multiple crate to represent major system 
components.

Initial crates include:

- `cli` - application entrypoint and command interface
- `gremlin-core` - shared domain models and foundational types
- `engine` - request execution logic
- `pipeline` - response processing logic

Additional crates may be added as the system evolves.

## Alternative included

### Single crate with modules

This approach would simplify setup but would make it harder to enforce
architectural boundaries as the project grows. It could also increase
compile times and create tighter coupling between components.

### Workspace with complete modularization

Creating too many crates upfront was avoided to reduce unnecessary 
complexity. The workspace will evolve incrementally instead.

## Consequences

### Positive

- Clear separation of concerns between system components
- Improved compile times through crate isolation
- Easier testing of individual components
- Scalable architecture that supports future features
- Cleaner dependency management
- Aligns with common practies in large Rust projects

### Negative
- More initial setup overhead
- Requires managing inter-crate dependencies
- Developers must understand workspace structure

Overall, the benefits of modularity and scalability outweight the added
complexity.
