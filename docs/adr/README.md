# Architecture Decision Records

This directory contains Architecture Decision Records (ADRs) for the 
project.

ADRs capture important architectural decisions, including the context,
alternatives considered, and the consequences of each choice. They 
serve as a historical record of how the system's design has evolved 
over time.

## Purpose

The goal of ADRs is to:

- Document why significant technical decisions were made
- Provide context for future contributors
- Preserve architectural history
- Make tradeoffs explicit
- Guide future development

ADRs are not implementation documentation. They describe decisions, not
code.

## Format

Each ADR follows a consistent structure:

- **Context** - background and problem statement
- **Decision** - chosen approach
- **Alternatives Considered** - other options evaluated
- **Consequences** - tradeoffs and implications

## Status Lifecycle

Each ADR includes a status field indicating its current state:

- **Proposed** - decision under consideration
- **Accepted** - decision is active and guiding implementation
- **Superseded** - replaced by a newer decision
- **Rejected** - considered but not adopted

## Naming convention

ADRs are named using a sequential numbering scheme:

NNNN-short-title.md

Numbers are never reused or renumbered to preserve history.

## When to write an ADR

An ADR should be created when a decision is:

- Cross-cutting or foundational
- Difficult to change later
- Likely to affect multiple components
- Significant enough to warrant documentation

## Index

- [0001 - Workspace architecture](./0001-workspace-architecture.md)
- [0002 - Tokio runtime](./0002-tokio-runtime.md)
- [0003 - Tracing logging](./0003-tracing-logging.md)
- [0004 - Streaming processing model](./0004-streaming-processing-model.md)
- [0005 - Bounded queues](./0005-bounded-queues.md)

## Updating ADRs

Existing ADRs should not be modified to reflect new decisions.
Instead, create a new ADR and mark the previous one as supreseded if 
necessary. 

## Scope

ADRs document architectural decision only. They are not intended for: 

- Feature specification
- Implementation details
- Roadmaps
- Task tracking
