# Architecture Decision Records

This directory contains Architecture Decision Records (ADRs) for GhostKeys.

## What is an ADR?

An Architecture Decision Record (ADR) captures an important architectural decision made along with its context and consequences. ADRs help teams understand:

- Why certain decisions were made
- What alternatives were considered
- What the trade-offs and consequences are

## Format

We use a simplified version of [Michael Nygard's ADR template](http://thinkrelevance.com/blog/2011/11/15/documenting-architecture-decisions), which is the most popular format in open-source projects.

Each ADR contains:

1. **Title** - Short present-tense statement
2. **Status** - Proposed, Accepted, Rejected, Deprecated, Superseded
3. **Date** - When the decision was made
4. **Deciders** - Who was involved
5. **Context** - What is the issue motivating this decision
6. **Decision** - What is the change we're proposing
7. **Alternatives Considered** - Other options evaluated
8. **Consequences** - What becomes easier or harder

## Naming Convention

ADRs are numbered sequentially and use lowercase with dashes:

- `0001-use-windows-rs-for-keyboard-hooks.md`
- `0002-position-based-mapping-strategy.md`

## Creating a New ADR

1. Copy `template.md` to a new file with the next number
2. Fill in the sections
3. Start with status "Proposed"
4. After team discussion, change to "Accepted" or "Rejected"
5. Commit with message: `docs: add ADR-XXXX for [topic]`

## Index

| ADR | Title | Status |
|-----|-------|--------|
| [0001](0001-use-windows-rs-for-keyboard-hooks.md) | Use windows-rs for Keyboard Hooks | Accepted |
| [0002](0002-position-based-mapping-strategy.md) | Use Position-Based Mapping Strategy | Accepted |
| [0003](0003-thread-local-mapper-state.md) | Use Thread-Local Storage for Mapper State | Accepted |
| [0004](0004-cargo-xwin-for-cross-compilation.md) | Use cargo-xwin for Cross-Compilation | Accepted |
