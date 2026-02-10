# How I Made Your Machine

_This is the style guide: the story of how I made your machine._

## Purpose

This document defines coding standards for maintainability, type safety, and behavior-focused testing across projects.
It sets strong defaults, with explicit room for justified exceptions.

## Core Principles

- Make invalid states hard to represent.
- Model domain concepts explicitly instead of using ad-hoc primitives.
- Use the type system and compiler as first-line quality gates.
- Optimize for readable intent over clever implementation.
- Test business behavior rather than framework or library internals.

## Type System and Compiler Standards

- Prefer domain types over raw strings and loosely typed containers.
- Use unions/enums/typed variants when valid values are finite.
- Use exhaustive pattern matching where possible.
- Represent unknown external values explicitly (`Unknown*`) instead of falling back silently.
- Avoid weakening types (`any`, broad casts, untyped flows) unless unavoidable.

### Type Design Signals

- Finite value set -> union/enum instead of `string`
- Mutually exclusive states -> one state union/enum instead of multiple booleans
- Function inputs representing domain concepts -> use those domain types directly

### Example 1: Finite Value Set as Variants

<Tabs groupId="finite-values">
  <TabItem value="pseudocode" label="Pseudocode" default>

```text
Under-modeled:
  status: string

Better modeled:
  Status = "draft" | "published" | "archived"
  status: Status
```

  </TabItem>
  <TabItem value="typescript" label="TypeScript">

```ts
type Status = "draft" | "published" | "archived";

const status: Status = "draft";
```

  </TabItem>
  <TabItem value="rust" label="Rust">

```rust
enum Status {
    Draft,
    Published,
    Archived,
}

let status = Status::Draft;
```

  </TabItem>
  <TabItem value="python" label="Python">

```python
from enum import StrEnum

class Status(StrEnum):
    DRAFT = "draft"
    PUBLISHED = "published"
    ARCHIVED = "archived"

status: Status = Status.DRAFT
```

  </TabItem>
</Tabs>

### Example 2: Boolean Combinations to State Variants

<Tabs groupId="boolean-state">
  <TabItem value="pseudocode" label="Pseudocode" default>

```text
Under-modeled:
  is_draft: boolean
  is_published: boolean
  is_archived: boolean

Better modeled:
  ContentState = "draft" | "published" | "archived"
  content_state: ContentState
```

  </TabItem>
  <TabItem value="typescript" label="TypeScript">

```ts
type ContentState = "draft" | "published" | "archived";

const contentState: ContentState = "published";
```

  </TabItem>
  <TabItem value="rust" label="Rust">

```rust
enum ContentState {
    Draft,
    Published,
    Archived,
}

let content_state = ContentState::Published;
```

  </TabItem>
  <TabItem value="python" label="Python">

```python
from typing import Literal

ContentState = Literal["draft", "published", "archived"]

content_state: ContentState = "published"
```

  </TabItem>
</Tabs>

### Example 3: Functions Consume Domain Types

<Tabs groupId="function-inputs">
  <TabItem value="pseudocode" label="Pseudocode" default>

```text
Under-modeled:
  schedule_message(channel: string, is_immediate: boolean, is_scheduled: boolean)

Better modeled:
  Channel = "email" | "sms" | "push"
  DeliveryMode = "immediate" | "scheduled"
  schedule_message(channel: Channel, delivery_mode: DeliveryMode)
```

  </TabItem>
  <TabItem value="typescript" label="TypeScript">

```ts
type Channel = "email" | "sms" | "push";
type DeliveryMode = "immediate" | "scheduled";

function scheduleMessage(channel: Channel, deliveryMode: DeliveryMode): void {
  // ...
}
```

  </TabItem>
  <TabItem value="rust" label="Rust">

```rust
enum Channel {
    Email,
    Sms,
    Push,
}

enum DeliveryMode {
    Immediate,
    Scheduled,
}

fn schedule_message(channel: Channel, delivery_mode: DeliveryMode) {
    let _ = (channel, delivery_mode);
}
```

  </TabItem>
  <TabItem value="python" label="Python">

```python
from typing import Literal

Channel = Literal["email", "sms", "push"]
DeliveryMode = Literal["immediate", "scheduled"]

def schedule_message(channel: Channel, delivery_mode: DeliveryMode) -> None:
    _ = (channel, delivery_mode)
```

  </TabItem>
</Tabs>

## Self-Documenting Code Standards

- Prefer descriptive, domain-aligned names even when verbose.
- Keep function and type names intention-revealing.
- Add comments only for non-obvious decisions, invariants, or tradeoffs.
- Avoid comments that restate what code already makes obvious.

## Testing Standards (Business Logic First)

- Test business outcomes and domain behavior first.
- Prioritize tests for state transitions, input-to-output rules, defaults, and fallback behavior.
- Do not test third-party/library internals.
- Avoid mocks by default.
- Use mocks only for unstable boundaries (network, filesystem, time, OS/process boundaries).
- Assert externally meaningful behavior, not private implementation details.

## Exceptions

Exceptions are acceptable when strict modeling creates disproportionate complexity, external contracts require looser typing, or performance/interoperability constraints apply.

When taking an exception, document:

- Which rule is being bent
- Why
- Which safeguards are in place (validation, logging, tests)
