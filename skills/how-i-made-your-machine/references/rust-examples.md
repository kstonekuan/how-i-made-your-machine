# Rust Examples

Use this reference when applying the style guide to Rust code and a concrete example would help. Read only the sections relevant to the task.

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

```rust
// Bad: free-form string can carry invalid values.
let under_modeled_status = "draff";

// Good: enum constrains the value set.
enum Status {
    Draft,
    Published,
    Archived,
}

let modeled_status = Status::Draft;
let _ = (under_modeled_status, modeled_status);
```

## Example 2: Boolean Combinations to State Variants

```rust
// Bad: booleans can contradict each other.
struct UnderModeledContentState {
    is_draft: bool,
    is_published: bool,
    is_archived: bool,
}

// Good: enum enforces one state at a time.
enum ContentState {
    Draft,
    Published,
    Archived,
}

let under_modeled_content_state = UnderModeledContentState {
    is_draft: true,
    is_published: true,
    is_archived: false,
};
let modeled_content_state = ContentState::Published;
let _ = (
    under_modeled_content_state.is_draft,
    under_modeled_content_state.is_published,
    under_modeled_content_state.is_archived,
    modeled_content_state,
);
```

## Example 3: Functions Consume Domain Types

```rust
// Bad: primitives allow invalid channels and conflicting mode flags.
fn schedule_message_under_modeled(channel: &str, is_immediate: bool, is_scheduled: bool) {
    let _ = (channel, is_immediate, is_scheduled);
}

// Good: domain enums constrain function inputs.
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

## Example 4: Return a Refined Type Instead of Void

```rust
// Bad: validates but returns nothing — caller still holds a raw &str.
fn validate_email(input: &str) {
    if !input.contains('@') {
        panic!("invalid email");
    }
}

fn send_welcome_under_modeled(email: &str) {
    validate_email(email);
    // email is still &str — nothing prevents passing an unchecked value.
    let _ = email;
}

// Good: parse once at the boundary, carry proof in the type.
struct Email(String);

impl Email {
    fn parse(input: &str) -> Result<Self, String> {
        if !input.contains('@') {
            return Err(format!("invalid email: {input}"));
        }
        Ok(Self(input.to_owned()))
    }
}

fn send_welcome(email: &Email) {
    // No re-validation needed — the type proves it was parsed.
    let _ = email;
}

let _ = (send_welcome_under_modeled as fn(&str), send_welcome as fn(&Email));
```

## Example 5: Smart Constructors for Non-Structural Invariants

```rust
// Bad: raw f64 allows out-of-range values.
fn apply_discount_under_modeled(percentage: f64) {
    let _ = percentage; // nothing prevents 150.0 or -3.0
}

// Good: newtype with a constructor enforces the range.
struct Percentage(f64);

impl Percentage {
    fn new(value: f64) -> Result<Self, String> {
        if !(0.0..=100.0).contains(&value) {
            return Err(format!("percentage out of range: {value}"));
        }
        Ok(Self(value))
    }

    fn value(&self) -> f64 {
        self.0
    }
}

fn apply_discount(percentage: &Percentage) {
    // Safe to use directly — construction guarantees 0-100.
    let _ = percentage.value();
}

let _ = apply_discount_under_modeled;
```

## Example 6: Outcome Variants on the Success Side

```rust
struct UserId(String);
struct User {
    name: String,
}

// Bad: recoverable cases mixed into the error type. Callers must remember
// which variants are bubble-up vs handle-here, and the signature suggests
// they're equally important.
enum FetchUserError {
    NotFound,
    Stale,
    Database(String),
    Network(String),
}

fn fetch_user_under_modeled(id: &UserId) -> Result<User, FetchUserError> {
    let _ = id;
    Err(FetchUserError::NotFound)
}

// Good: recoverable outcomes are variants on the success side.
// Real errors flow through anyhow to the top-level handler.
enum UserLookup {
    Found(User),
    NotFound,
    Stale,
}

fn fetch_user(id: &UserId) -> anyhow::Result<UserLookup> {
    // Database/network failures use `?` and propagate as anyhow::Error.
    let _ = id;
    Ok(UserLookup::NotFound)
}

fn handle_request(id: &UserId) -> anyhow::Result<String> {
    Ok(match fetch_user(id)? {
        UserLookup::Found(user) => format!("welcome, {}", user.name),
        UserLookup::NotFound => "no such user".to_owned(),
        UserLookup::Stale => "please refresh".to_owned(),
    })
}

let _ = (
    fetch_user_under_modeled as fn(&UserId) -> Result<User, FetchUserError>,
    handle_request as fn(&UserId) -> anyhow::Result<String>,
);
```

## Example 7: Bulk Owns the Operation

```rust
struct GroupId(String);
struct UserId(String);

fn validate_membership_cap(_group: &GroupId, _add_count: usize) -> anyhow::Result<()> {
    Ok(())
}

fn with_group_lock(_group: &GroupId, body: impl FnOnce() -> anyhow::Result<()>) -> anyhow::Result<()> {
    body()
}

fn persist_membership(_group: &GroupId, _users: &[UserId]) -> anyhow::Result<()> {
    Ok(())
}

// Bad: two entry points implement the same operation independently.
// A rule added to one path (audit log, rate cap, new invariant) silently
// skips the other.
fn add_user_under_modeled(group: GroupId, user: UserId) -> anyhow::Result<()> {
    validate_membership_cap(&group, 1)?;
    with_group_lock(&group, || persist_membership(&group, &[user]))
}

fn add_users_under_modeled(group: GroupId, users: Vec<UserId>) -> anyhow::Result<()> {
    validate_membership_cap(&group, users.len())?;
    with_group_lock(&group, || persist_membership(&group, &users))
}

// Good: the bulk path owns the operation; the single wraps it.
// Any rule added to add_users automatically applies to add_user.
fn add_users(group: GroupId, users: Vec<UserId>) -> anyhow::Result<()> {
    validate_membership_cap(&group, users.len())?;
    with_group_lock(&group, || persist_membership(&group, &users))
}

fn add_user(group: GroupId, user: UserId) -> anyhow::Result<()> {
    add_users(group, vec![user])
}

let _ = (
    add_user_under_modeled as fn(GroupId, UserId) -> anyhow::Result<()>,
    add_users_under_modeled as fn(GroupId, Vec<UserId>) -> anyhow::Result<()>,
    add_user as fn(GroupId, UserId) -> anyhow::Result<()>,
    add_users as fn(GroupId, Vec<UserId>) -> anyhow::Result<()>,
);
```

## Example 8: Document the Why, Not the Obvious What

```rust
// Bad: restates obvious behavior.
fn increment_retry_count(retry_count: u32) -> u32 {
    // Increment retry count.
    retry_count + 1
}

// Good: documents a non-obvious business constraint.
fn is_refund_eligible(days_since_purchase: u32, refund_window_days: u32) -> bool {
    // Billing provider closes refunds at next-day midnight, so we keep a one-day grace period.
    let refund_grace_period_in_days = 1;
    days_since_purchase <= refund_window_days + refund_grace_period_in_days
}

let _ = increment_retry_count(0);
```

## Example 9: Test State Transitions and Outcomes

```rust
#[test]
fn bad_asserts_implementation_detail_only() {
    let transition_helper_call_count = 1;
    assert_eq!(transition_helper_call_count, 1);
}

#[test]
fn good_transitions_draft_to_published_when_publish_is_requested() {
    let next_state = transition_order_state("draft", "publish");
    assert_eq!(next_state, "published");
}
```

## Example 10: Mock Only Unstable Boundaries

```rust
#[test]
fn bad_asserts_internal_helper_behavior() {
    let internal_helper_call_count = 1;
    assert_eq!(internal_helper_call_count, 1);
}

#[test]
fn good_uses_cached_rate_when_live_lookup_fails() {
    let gateway = CurrencyGateway::new(FailingRateClient, CachedRateStore::with_rate(0.92));
    let rate = gateway.get_rate_or_fallback("USD", "EUR");
    assert_eq!(rate, 0.92);
}
```

## Example 11: Prefer Libraries That Align With Your Type Design

```rust
use serde::Deserialize;
use serde_json::Value;

// Bad: library returns serde_json::Value — caller must chain fallible accessors.
fn get_user_name_under_modeled(response: &Value) -> Option<&str> {
    response.get("user")?.get("name")?.as_str()
}

// Good: library deserializes into a typed struct — field access is compile-checked.
#[derive(Deserialize)]
struct User {
    name: String,
    email: String,
}

fn get_user_name(user: &User) -> &str {
    &user.name
}

let _ = get_user_name_under_modeled;
```

## Example 12: Re-implement When the Dependency Outweighs the Logic

```rust
// Bad: pulling in an external crate for something the standard library already provides.
// use num::clamp;
// let clamped_from_crate = clamp(150.0, 0.0, 100.0);

// Good: the standard library already has f64::clamp — no dependency needed.
let clamped = 150.0_f64.clamp(0.0, 100.0);
let _ = clamped;
```

## Example 13: External Contract Requires Looser Typing

```rust
// Bad: panics on unexpected partner status values.
enum UnderModeledPartnerStatus {
    Accepted,
    Rejected,
}

fn parse_partner_status(raw_partner_status: &str) -> UnderModeledPartnerStatus {
    match raw_partner_status {
        "accepted" => UnderModeledPartnerStatus::Accepted,
        "rejected" => UnderModeledPartnerStatus::Rejected,
        _ => panic!("unsupported partner status: {raw_partner_status}"),
    }
}

// Good: capture unknown values explicitly and continue safely.
enum PartnerStatus {
    Accepted,
    Rejected,
    UnknownPartnerStatus,
}

fn map_partner_status(raw_partner_status: &str) -> PartnerStatus {
    match raw_partner_status {
        "accepted" => PartnerStatus::Accepted,
        "rejected" => PartnerStatus::Rejected,
        _ => {
            eprintln!("unknown partner status: {raw_partner_status}");
            PartnerStatus::UnknownPartnerStatus
        }
    }
}

let _ = parse_partner_status("accepted");
```
