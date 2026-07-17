---
name: how-i-made-your-machine
description: Apply the "How I Made Your Machine" coding style guide to implementation, refactoring, and code review across TypeScript, Rust, and Python. Use when a request asks for this style guide, when improving maintainability or type safety, when modeling domain concepts with explicit variants and refined types, when designing boundary parsing or recoverable outcomes, or when enforcing behavior-first testing.
---

# How I Made Your Machine

Apply this guide as a set of strong defaults. Respect explicit user tradeoffs and existing project constraints.

## Core defaults

- Make invalid states hard to represent.
- Model domain concepts with explicit types instead of ad-hoc primitives.
- Use the compiler and static analysis as first-line quality gates.
- Parse external input into domain types at system boundaries and trust refined types downstream.
- Represent expected, recoverable outcomes explicitly; propagate genuinely exceptional failures.
- Keep one source of truth for behavior shared by multiple entry points.
- Prefer readable intent, descriptive names, and comments that explain non-obvious decisions.
- Test business outcomes rather than framework internals, mocks, or third-party behavior.
- Keep unrelated code unchanged and avoid broad rewrites outside the task.

## Workflow

1. Inspect the repository's instructions, language, domain model, and established patterns.
2. Identify which concerns in the reference routing table apply.
3. Read only the references required for those concerns. Do not load every reference by default.
4. Implement or review using the applicable rules and language-specific patterns.
5. Run the repository's formatter, linter, type checker, and relevant behavior-focused tests.
6. Explain material tradeoffs or justified exceptions.

## Reference routing

Read each relevant topic reference before making design decisions in that area:

- For finite value sets, state modeling, domain-specific types, refined types, or exhaustive matching, read [Type Design](references/type-design.md).
- For external input, validation, smart constructors, unknown values, recoverable outcomes, or failure channels, read [Boundaries and Outcomes](references/boundaries-and-outcomes.md).
- For shared operations, duplicate entry points, comments, naming, or test design, read [Behavior, Structure, and Testing](references/behavior-structure-and-testing.md).
- Before adding a dependency or intentionally weakening a rule, read [Dependencies and Exceptions](references/dependencies-and-exceptions.md).
- When deeper rationale for boundary parsing is useful, read [Parse, Don't Validate](references/parse-dont-validate.md).
- When selecting static-analysis tools, read [Choosing Linters and Type Checkers](references/choosing-linters-and-type-checkers.md).

Read the matching language examples when an example would materially help implementation or review:

- [TypeScript Examples](references/typescript-examples.md)
- [Rust Examples](references/rust-examples.md)
- [Python Examples](references/python-examples.md)

## Review checklist

### Domain modeling

- Replace unconstrained strings with unions, enums, or variants when values are finite.
- Replace contradictory boolean combinations with one explicit state type.
- Accept domain types in function signatures instead of repeatedly accepting raw primitives.
- Return refined values from parsing functions instead of discarding knowledge with void validation.
- Derive duplicated facts or assign one clear owner.
- Avoid `any`, broad casts, untyped containers, and silent unknown-value fallbacks.

### Boundaries and failures

- Parse user input, API responses, configuration, and webhook payloads at entry points.
- Use smart constructors for invariants such as ranges, non-empty collections, and formatted strings.
- Put recoverable cases in an explicit success-side variant that callers must handle.
- Propagate unrecoverable failures through an opaque, contextual error channel.
- Introduce typed error variants only when callers genuinely branch on them.

### Structure and behavior

- Centralize shared validation, locking, mutation, and persistence in one core operation.
- Keep adapters and alternate entry points thin.
- Use descriptive, domain-aligned names even when they are verbose.
- Comment decisions, invariants, and tradeoffs; do not restate obvious code.
- Test observable state transitions, outputs, defaults, and fallbacks.
- Mock only unstable boundaries such as networks, filesystems, time, or processes.

### Exceptions

When constraints require an exception, document the rule being bent, the reason, and the safeguards such as boundary validation, logging, or behavior-focused tests.
