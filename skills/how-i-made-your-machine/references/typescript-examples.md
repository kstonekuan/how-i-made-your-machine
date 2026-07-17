# TypeScript Examples

Use this reference when applying the style guide to TypeScript code and a concrete example would help. Read only the sections relevant to the task.

## Contents

- [Example 1: Finite Value Set as Variants](#example-1-finite-value-set-as-variants)
- [Example 2: Boolean Combinations to State Variants](#example-2-boolean-combinations-to-state-variants)
- [Example 3: Functions Consume Domain Types](#example-3-functions-consume-domain-types)
- [Example 4: Return a Refined Type Instead of Void](#example-4-return-a-refined-type-instead-of-void)
- [Example 5: Smart Constructors for Non-Structural Invariants](#example-5-smart-constructors-for-non-structural-invariants)
- [Example 6: Outcome Variants on the Success Side](#example-6-outcome-variants-on-the-success-side)
- [Example 7: Bulk Owns the Operation](#example-7-bulk-owns-the-operation)
- [Example 8: Document the Why, Not the Obvious What](#example-8-document-the-why-not-the-obvious-what)
- [Example 9: Test State Transitions and Outcomes](#example-9-test-state-transitions-and-outcomes)
- [Example 10: Mock Only Unstable Boundaries](#example-10-mock-only-unstable-boundaries)
- [Example 11: Prefer Libraries That Align With Your Type Design](#example-11-prefer-libraries-that-align-with-your-type-design)
- [Example 12: Re-implement When the Dependency Outweighs the Logic](#example-12-re-implement-when-the-dependency-outweighs-the-logic)
- [Example 13: External Contract Requires Looser Typing](#example-13-external-contract-requires-looser-typing)

## Example 1: Finite Value Set as Variants

```ts
// Bad: unconstrained string allows invalid values.
const underModeledStatus: string = "draff";

// Good: finite union prevents invalid states.
type Status = "draft" | "published" | "archived";
const modeledStatus: Status = "draft";
```

## Example 2: Boolean Combinations to State Variants

```ts
// Bad: boolean combinations allow impossible states.
const isDraft = true;
const isPublished = true;
const isArchived = false;

// Good: one variant encodes one valid state.
type ContentState = "draft" | "published" | "archived";
const contentState: ContentState = "published";
void [isDraft, isPublished, isArchived];
```

## Example 3: Functions Consume Domain Types

```ts
// Bad: primitives hide domain intent and allow contradictions.
function scheduleMessageUnderModeled(
  channel: string,
  isImmediate: boolean,
  isScheduled: boolean,
): void {
  void [channel, isImmediate, isScheduled];
}

// Good: explicit domain types make valid states clear.
type Channel = "email" | "sms" | "push";
type DeliveryMode = "immediate" | "scheduled";

function scheduleMessage(channel: Channel, deliveryMode: DeliveryMode): void {
  void [channel, deliveryMode];
}
```
## Example 4: Return a Refined Type Instead of Void

```ts
// Bad: validates but returns void — caller still holds a raw string.
function validateEmail(input: string): void {
  if (!input.includes("@")) {
    throw new Error("invalid email");
  }
}

function sendWelcomeUnderModeled(email: string): void {
  validateEmail(email);
  // email is still string — nothing prevents passing an unchecked value.
  void email;
}

// Good: parse once at the boundary, carry proof in the type.
type Email = string & { readonly __brand: "Email" };

function parseEmail(input: string): Email {
  if (!input.includes("@")) {
    throw new Error("invalid email");
  }
  return input as Email;
}

function sendWelcome(email: Email): void {
  // No re-validation needed — the type proves it was parsed.
  void email;
}
```

## Example 5: Smart Constructors for Non-Structural Invariants

```ts
// Bad: raw number allows out-of-range values.
function applyDiscountUnderModeled(percentage: number): void {
  void percentage; // nothing prevents 150 or -3
}

// Good: branded type enforces range at construction.
type Percentage = number & { readonly __brand: "Percentage" };

function makePercentage(value: number): Percentage {
  if (value < 0 || value > 100) {
    throw new RangeError(`percentage out of range: ${value}`);
  }
  return value as Percentage;
}

function applyDiscount(percentage: Percentage): void {
  // Safe to use directly — construction guarantees 0-100.
  void percentage;
}
```

## Example 6: Outcome Variants on the Success Side

```ts
interface User {
  id: string;
  name: string;
}

// Bad: every recoverable case stuffed into the error channel.
// Callers must triage which thrown shapes to handle and which to rethrow.
class FetchUserError extends Error {
  constructor(
    readonly kind: "NotFound" | "Stale" | "Database" | "Network",
    message: string,
  ) {
    super(message);
  }
}

async function fetchUserUnderModeled(id: string): Promise<User> {
  throw new FetchUserError("NotFound", `no user with id ${id}`);
}

// Good: recoverable cases as a discriminated union on the success side.
// Real errors are thrown and propagate to the top-level handler.
type UserLookup =
  | { kind: "Found"; user: User }
  | { kind: "NotFound" }
  | { kind: "Stale" };

async function fetchUser(id: string): Promise<UserLookup> {
  // Database/network failures are thrown, not returned — the caller
  // branches on UserLookup and lets exceptions propagate.
  void id;
  return { kind: "NotFound" };
}

async function handleRequest(id: string): Promise<string> {
  const lookup = await fetchUser(id);
  switch (lookup.kind) {
    case "Found":
      return `welcome, ${lookup.user.name}`;
    case "NotFound":
      return "no such user";
    case "Stale":
      return "please refresh";
  }
}

void (fetchUserUnderModeled as (id: string) => Promise<User>);
void handleRequest;
```

## Example 7: Bulk Owns the Operation

```ts
type GroupId = string & { readonly __brand: "GroupId" };
type UserId = string & { readonly __brand: "UserId" };

declare function validateMembershipCap(group: GroupId, addCount: number): Promise<void>;
declare function withGroupLock<T>(group: GroupId, fn: () => Promise<T>): Promise<T>;
declare function persistMembership(group: GroupId, users: UserId[]): Promise<void>;

// Bad: two entry points implement the same operation independently.
// A rule added to one path (audit log, rate cap, new invariant) silently
// skips the other.
async function addUserUnderModeled(group: GroupId, user: UserId): Promise<void> {
  await validateMembershipCap(group, 1);
  await withGroupLock(group, () => persistMembership(group, [user]));
}

async function addUsersUnderModeled(group: GroupId, users: UserId[]): Promise<void> {
  await validateMembershipCap(group, users.length);
  await withGroupLock(group, () => persistMembership(group, users));
}

// Good: the bulk path owns the operation; the single wraps it.
// Any rule added to addUsers automatically applies to addUser.
async function addUsers(group: GroupId, users: UserId[]): Promise<void> {
  await validateMembershipCap(group, users.length);
  await withGroupLock(group, () => persistMembership(group, users));
}

async function addUser(group: GroupId, user: UserId): Promise<void> {
  await addUsers(group, [user]);
}

void [addUserUnderModeled, addUsersUnderModeled, addUsers, addUser];
```

## Example 8: Document the Why, Not the Obvious What

```ts
// Bad: restates obvious code behavior without context.
function incrementRetryCount(retryCount: number): number {
  // Increment retry count.
  return retryCount + 1;
}

// Good: documents a non-obvious business constraint.
function isRefundEligible(
  daysSincePurchase: number,
  refundWindowDays: number,
): boolean {
  // Billing provider closes refunds at next-day midnight, so we keep a one-day grace period.
  const refundGracePeriodInDays = 1;
  return daysSincePurchase <= refundWindowDays + refundGracePeriodInDays;
}
```

## Example 9: Test State Transitions and Outcomes

```ts
import { describe, expect, it } from "vitest";

describe("transitionOrderStateUnderTest", () => {
  it("bad: asserts an implementation detail", () => {
    const transitionHelperCallCount = 1;
    expect(transitionHelperCallCount).toBe(1);
  });
});

describe("transitionOrderState", () => {
  it("good: asserts observable state transition", () => {
    const nextState = transitionOrderState("draft", "publish");
    expect(nextState).toBe("published");
  });
});
```

## Example 10: Mock Only Unstable Boundaries

```ts
import { expect, it, vi } from "vitest";

it("bad: mocks an internal helper and asserts internals", () => {
  const calculateDiscountWindow = vi.fn().mockReturnValue(true);
  calculateDiscountWindow();
  expect(calculateDiscountWindow).toHaveBeenCalledTimes(1);
});

it("good: mocks only unstable network boundary and asserts behavior", async () => {
  const fetchExchangeRate = vi.fn().mockRejectedValue(new Error("timeout"));
  const gateway = createCurrencyGateway({ fetchExchangeRate });

  const rate = await gateway.getRateOrFallback("USD", "EUR");

  expect(rate).toBe(0.92);
});
```

## Example 11: Prefer Libraries That Align With Your Type Design

```ts
// Bad: library returns `any` — caller must cast or assert at every access.
async function getUserNameUnderModeled(userId: string): Promise<string> {
  // biome-ignore lint/suspicious/noExplicitAny: demonstrating a bad library API
  const response: any = await untypedHttpClient.get(`/users/${userId}`);
  return response.data.name;
}

// Good: library returns typed response — compiler catches field errors.
import { z } from "zod";

const UserSchema = z.object({
  name: z.string(),
  email: z.string(),
});

type User = z.infer<typeof UserSchema>;

async function getUserName(userId: string): Promise<string> {
  const response = await typedHttpClient.get(`/users/${userId}`, UserSchema);
  return response.name;
}
```

## Example 12: Re-implement When the Dependency Outweighs the Logic

```ts
// Bad: importing a package for a one-liner adds supply-chain surface for no benefit.
// import { clamp } from "math-utils-lib";
// const clampedFromLib = clamp(value, 0, 100);

// Good: trivial, well-understood logic is cheaper to own than to import.
function clamp(value: number, min: number, max: number): number {
  return Math.max(min, Math.min(max, value));
}

const clamped = clamp(150, 0, 100);
void clamped;
```

## Example 13: External Contract Requires Looser Typing

```ts
// Bad: assumes external input already matches strict known values.
type UnderModeledPartnerStatus = "accepted" | "rejected";

function parsePartnerStatus(rawPartnerStatus: UnderModeledPartnerStatus): UnderModeledPartnerStatus {
  return rawPartnerStatus;
}

// Good: accept string input, validate known values, and map unknown values explicitly.
type KnownPartnerStatus = "accepted" | "rejected";
type PartnerStatus = KnownPartnerStatus | "unknown_partner_status";

function mapPartnerStatus(rawPartnerStatus: string): PartnerStatus {
  if (rawPartnerStatus === "accepted" || rawPartnerStatus === "rejected") {
    return rawPartnerStatus;
  }

  console.warn("unknown partner status", { rawPartnerStatus });
  return "unknown_partner_status";
}
```
