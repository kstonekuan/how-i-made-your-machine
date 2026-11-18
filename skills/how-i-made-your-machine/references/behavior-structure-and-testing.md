# Behavior, Structure, and Testing

Use these rules when separating pure logic from side effects, when several entry points share an operation, when naming or commenting code, and when designing tests.

## Separate pure decisions from side effects

Keep decisions in pure functions that take values and return values. Put I/O (database, network, clock, filesystem) in a thin shell that gathers inputs, calls the core, and performs what it returns.

- Return a value describing what should happen, ideally a variant the shell matches exhaustively.
- Pass time, randomness, and loaded records into the core as arguments instead of fetching them inside it.
- Keep the shell nearly branch-free: load, decide, perform.

The core is then testable with plain values and no mocks. A shared operation from the next section is a shell entry point that calls this core.

For the full rationale, see Gary Bernhardt's [Functional Core, Imperative Shell](https://www.destroyallsoftware.com/screencasts/catalog/functional-core-imperative-shell). A concrete example is in the language examples reference.

## Keep one source of truth for behavior

When one operation is available through several entry points, implement the behavior once and make the other entry points thin adapters.

Examples include:

- Single-item and bulk APIs
- HTTP handlers and command-line commands
- User-interface actions and background jobs
- Public functions and internal wrappers

The shared operation should own shared validation, locking, mutation, and persistence. Return domain values instead of transport-specific responses so every adapter can use it.

```text
add_users(group, users) owns the operation
add_user(group, user) delegates to add_users(group, [user])
```

When a one-item batch has the same semantics as a single-item call, pass a one-element collection to the shared operation and adapt its result to the single-item contract. This also applies to shared SDK behavior such as waiting for completion and handling cancellation.

For bulk operations, choose the atomicity model deliberately and express it in the return type:

- Atomic operations succeed completely or report where they failed.
- Tolerant operations report applied and rejected items explicitly.

If the entry points differ in validation, side effects, or failure behavior, keep those differences at their entry points and share the steps with the same contract.

## Write self-documenting code

- Prefer descriptive, domain-aligned names even when they are verbose.
- Keep function and type names intention-revealing.
- Add comments for non-obvious decisions, invariants, external constraints, and tradeoffs.
- Remove comments that merely restate the next line of code.

Comments go stale independently of code. Names and structure must evolve with the implementation, so use them as the primary documentation.

## Test business behavior first

Tests should protect outcomes that matter to the system:

- State transitions
- Input-to-output rules
- Defaults and fallbacks
- Boundary behavior
- Domain invariants
- Meaningful failure and recovery behavior

Avoid tests that only assert helper call counts, private control flow, mock interactions, or third-party library behavior.

Use mocks only for unstable boundaries such as networks, filesystems, time, operating-system processes, or external services. Prefer small fakes with meaningful behavior when practical.

```text
Brittle:
  assert transition_helper_called_once()

Behavior-focused:
  result = transition_order_state(Draft, Publish)
  assert result == Published
```

An implementation refactor should not break a test when the observable business behavior remains unchanged.
