# Agents

## Overview

This document outlines the agents used in the project, their purpose, and guidelines for contribution.

## Development Tools

-   This project must be developed using [Kiro.dev](https://kiro.dev)
-   Agents must become world-class experts in Kiro through its documentation: https://kiro.dev/docs.

## Hackathon Rules

-   This project is part of Kiroween and must follow the rules at: https://kiroween.devpost.com/rules

## Language Requirements

**Everything saved in the repository MUST be in English.** This includes:

-   All code (variable names, function names, etc.)
-   All comments and documentation
-   All commit messages
-   All README files and documentation

Chat conversations or AI interactions MAY be conducted in other languages, but any artifacts saved to the repository MUST be in English.

## Coding Standards

-   **Clarity and maintainability** over cleverness or optimization
-   Code should be well-commented, especially for complex logic
-   Use idiomatic patterns for the chosen language (Rust)
-   Comment "why" not "what" (code should be self-documenting for "what")
-   Keep functions small and focused
-   Use descriptive names for variables, functions, and types

## Design Principles

1.  **Zero friction** - Minimize steps from "install" to "working"
2.  **Transparency** - Open source, well-documented, no magic
3.  **Security first** - Handle input interception responsibly
4.  **Minimal footprint** - Low resource usage, no unnecessary dependencies

## Security Considerations

-   **Input handling:** This project intercepts keystrokes - handle with extreme care
-   **Dependencies:** Use only trusted, well-supported libraries - no obscure or unmaintained packages
-   **Local processing:** All data must be processed locally, no network requirement
-   **Minimal access:** Request only the minimum permissions needed
-   **Audit-friendly:** Code should be easy to audit for security

## Commit Messages

-   Commit messages must follow the [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) specification.

Use these prefixes:

-   `feat:` - New features
-   `fix:` - Bug fixes
-   `docs:` - Documentation only
-   `style:` - Formatting, missing semicolons, etc.
-   `refactor:` - Code restructuring without changing behavior
-   `test:` - Adding or updating tests
-   `chore:` - Updating build tasks, package manager configs, etc.
-   `perf:` - Performance improvements

### Commit Message Guidelines

**Commit messages should be clear and describe user impact:**

```bash
# ❌ Bad - too vague, internal-focused
git commit -m "fix: update code"

# ✅ Good - clear, describes user impact
git commit -m "fix: resolve incorrect accent mapping for tilde key"

# ❌ Bad - implementation detail
git commit -m "feat: add hashmap"

# ✅ Good - describes the feature
git commit -m "feat: add ABNT2 to US layout key mapping for Portuguese accents"
```

## When to Ask for Clarification

-   If the requirement is ambiguous or contradictory
-   If you need to make a significant architectural decision
-   If you're about to introduce a breaking change
-   If security or privacy concerns arise
-   If the change could affect cross-platform compatibility
