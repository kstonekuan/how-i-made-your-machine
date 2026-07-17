# Type Design

Use these rules when designing or reviewing domain models and function signatures.

## Make invalid states hard to represent

A type-safe language only helps when the model gives the compiler useful facts. Prefer types that describe the valid state space directly instead of types that require callers to remember informal rules.

- Use a union, enum, or variant when a value set is finite.
- Use one state type when states are mutually exclusive; avoid combinations of booleans that can contradict each other.
- Give identifiers and other domain values distinct types when mixing them would be meaningful and dangerous.
- Represent unknown external values explicitly rather than silently treating them as a known default.
- Avoid `any`, broad casts, raw dictionaries, untyped JSON, and other flows that discard compiler guarantees.

## Put domain types in function signatures

Function signatures should communicate the operation's domain contract.

Under-modeled:

```text
schedule_message(channel: string, is_immediate: boolean, is_scheduled: boolean)
```

Better modeled:

```text
Channel = Email | Sms | Push
DeliveryMode = Immediate | Scheduled
schedule_message(channel: Channel, delivery_mode: DeliveryMode)
```

Parse or construct these values at boundaries. Do not make every business function accept raw primitives and repeat the same checks.

## Prefer exhaustive handling

Structure variants so the language's type checker can verify that every case is handled. Prefer exhaustive `match` or `switch` expressions over default branches that hide newly added variants.

When an external system can introduce new values, convert the raw value into a domain type containing an explicit unknown variant. This keeps internal matching exhaustive without pretending the external contract is closed.

## Preserve learned information

If a function verifies a property, return a type that carries the result. A void-returning validator leaves the caller holding the same unrefined value, so downstream code must either trust it blindly or validate it again.

Use opaque types, newtypes, branded types, constrained models, or equivalent constructors for non-structural invariants such as:

- Numeric ranges
- Non-empty collections
- Valid identifiers
- Normalized paths
- Formatted email addresses

## Keep one owner for each fact

Duplicated facts eventually disagree. Derive a value when possible, or choose one structure as its authoritative owner.

This applies to both data and state transitions: do not store several flags that restate one state, and do not maintain parallel representations without an explicit boundary conversion.
