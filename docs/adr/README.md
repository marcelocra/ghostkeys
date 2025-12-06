# Architecture Decision Records (ADRs)

This directory contains Architecture Decision Records for GhostKeys.

## What is an ADR?

An ADR documents an important architectural decision made along with its context and consequences.

## Format

Each ADR follows this structure:
- **Status**: Accepted, Deprecated, Superseded
- **Context**: The issue motivating this decision
- **Decision**: The change we're proposing
- **Rationale**: Why this decision was made
- **Consequences**: Positive and negative outcomes
- **Alternatives Considered**: Other options we evaluated

## Index

| ADR | Title | Status |
|-----|-------|--------|
| [001](001-use-windows-rs-for-keyboard-hooks.md) | Use windows-rs for Keyboard Hooks | Accepted |
| [002](002-position-based-mapping-strategy.md) | Position-Based Mapping Strategy | Accepted |
| [003](003-thread-local-mapper-state.md) | Thread-Local Mapper State | Accepted |
| [004](004-cargo-xwin-for-cross-compilation.md) | cargo-xwin for Cross-Compilation | Accepted |

## Creating a New ADR

1. Copy the template from an existing ADR
2. Number it sequentially (005, 006, etc.)
3. Fill in all sections
4. Update this index
5. Commit with message: `docs: add ADR-XXX for [topic]`
