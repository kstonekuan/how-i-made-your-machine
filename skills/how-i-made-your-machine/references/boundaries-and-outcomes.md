# Boundaries and Outcomes

Use these rules for untrusted input, validation, recoverable cases, and error propagation.

## Parse at system boundaries

Parse user input, API responses, configuration files, database records, and webhook payloads as they enter the system. Return domain values that carry the established invariants, then let internal code rely on those values.

Avoid shotgun parsing, where business functions repeatedly validate fragments of raw input. That approach permits partial processing, duplicates rules, and makes it unclear where trust begins.

```text
Validation that discards knowledge:
  validate_email(raw: string) -> void
  send_welcome(email: string)

Parsing that preserves knowledge:
  parse_email(raw: string) -> Email | error
  send_welcome(email: Email)
```

Read [Parse, Don't Validate](parse-dont-validate.md) when the distinction needs deeper treatment.

## Use smart constructors for invariants

When structure alone cannot express an invariant, hide direct construction and expose a constructor that checks it.

```text
Percentage = opaque(number)
make_percentage(raw: number) -> Percentage | error
```

Downstream functions should accept `Percentage`, not a raw number that every caller must remember to constrain.

## Separate recoverable outcomes from errors

Not every unsuccessful result is an error.

- Put expected cases that callers branch on in an explicit success-side variant. Examples include not found, already exists, stale data, or a retry-later outcome.
- Propagate failures that callers cannot meaningfully recover from through an opaque, contextual error channel toward a top-level handler.

```text
fetch_user(id) -> Result<UserLookup, OpaqueError>
UserLookup = Found(User) | NotFound | Stale
```

The caller exhaustively handles `UserLookup`. Database, network, and other exceptional failures propagate without forcing every caller to enumerate error variants it cannot usefully handle.

Introduce a typed error enum only when a downstream caller genuinely needs to distinguish those failure modes. First consider whether the cases belong in the success type instead.

## Handle open external contracts explicitly

External systems may send undocumented values. Accept the loose representation at the boundary, then map it into a strict internal model containing an explicit unknown variant.

For an exception to strict modeling, document:

- The external constraint
- The rule being relaxed
- How unknown values are represented
- What is logged
- Which known and unknown behaviors are tested
