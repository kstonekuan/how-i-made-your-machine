# Python Examples

Use this reference when applying the style guide to Python code and a concrete example would help. Read only the sections relevant to the task.

## Contents

- [Example 1: Finite Value Set as Variants](#example-1-finite-value-set-as-variants)
- [Example 2: Boolean Combinations to State Variants](#example-2-boolean-combinations-to-state-variants)
- [Example 3: Functions Consume Domain Types](#example-3-functions-consume-domain-types)
- [Example 4: Return a Refined Type Instead of Void](#example-4-return-a-refined-type-instead-of-void)
- [Example 5: Smart Constructors for Non-Structural Invariants](#example-5-smart-constructors-for-non-structural-invariants)
- [Example 6: Outcome Variants on the Success Side](#example-6-outcome-variants-on-the-success-side)
- [Example 7: Decide in the Core, Act in the Shell](#example-7-decide-in-the-core-act-in-the-shell)
- [Example 8: Bulk Owns the Operation](#example-8-bulk-owns-the-operation)
- [Example 9: Document the Why, Not the Obvious What](#example-9-document-the-why-not-the-obvious-what)
- [Example 10: Test State Transitions and Outcomes](#example-10-test-state-transitions-and-outcomes)
- [Example 11: Mock Only Unstable Boundaries](#example-11-mock-only-unstable-boundaries)
- [Example 12: Prefer Libraries That Align With Your Type Design](#example-12-prefer-libraries-that-align-with-your-type-design)
- [Example 13: Re-implement When the Dependency Outweighs the Logic](#example-13-re-implement-when-the-dependency-outweighs-the-logic)
- [Example 14: External Contract Requires Looser Typing](#example-14-external-contract-requires-looser-typing)

## Example 1: Finite Value Set as Variants

```python
# Bad: unconstrained string allows invalid values.
under_modeled_status: str = "draff"

# Good: StrEnum constrains valid values.
from enum import StrEnum

class Status(StrEnum):
    DRAFT = "draft"
    PUBLISHED = "published"
    ARCHIVED = "archived"

modeled_status: Status = Status.DRAFT
```

## Example 2: Boolean Combinations to State Variants

```python
# Bad: booleans can represent impossible combinations.
is_draft = True
is_published = True
is_archived = False

# Good: Literal constrains to one valid state.
from typing import Literal

ContentState = Literal["draft", "published", "archived"]

content_state: ContentState = "published"
_ = (is_draft, is_published, is_archived, content_state)
```

## Example 3: Functions Consume Domain Types

```python
# Bad: primitive inputs accept invalid values and contradictory flags.
def schedule_message_under_modeled(
    channel: str,
    is_immediate: bool,
    is_scheduled: bool,
) -> None:
    _ = (channel, is_immediate, is_scheduled)

# Good: domain literals constrain allowed values.
from typing import Literal

Channel = Literal["email", "sms", "push"]
DeliveryMode = Literal["immediate", "scheduled"]

def schedule_message(channel: Channel, delivery_mode: DeliveryMode) -> None:
    _ = (channel, delivery_mode)
```

## Example 4: Return a Refined Type Instead of Void

```python
# Bad: validates but returns None — caller still holds a raw str.
def validate_email(input: str) -> None:
    if "@" not in input:
        raise ValueError("invalid email")

def send_welcome_under_modeled(email: str) -> None:
    validate_email(email)
    # email is still str — nothing prevents passing an unchecked value.
    _ = email

# Good: parse once at the boundary, carry proof in the type.
from typing import NewType

Email = NewType("Email", str)

def parse_email(input: str) -> Email:
    if "@" not in input:
        raise ValueError("invalid email")
    return Email(input)

def send_welcome(email: Email) -> None:
    # No re-validation needed — the type proves it was parsed.
    _ = email
```

## Example 5: Smart Constructors for Non-Structural Invariants

```python
# Bad: raw float allows out-of-range values.
def apply_discount_under_modeled(percentage: float) -> None:
    _ = percentage  # nothing prevents 150.0 or -3.0

# Good: NewType + constructor enforces the range.
from typing import NewType

Percentage = NewType("Percentage", float)

def make_percentage(value: float) -> Percentage:
    if not (0 <= value <= 100):
        raise ValueError(f"percentage out of range: {value}")
    return Percentage(value)

def apply_discount(percentage: Percentage) -> None:
    # Safe to use directly — construction guarantees 0-100.
    _ = percentage
```

## Example 6: Outcome Variants on the Success Side

```python
from dataclasses import dataclass

@dataclass(frozen=True)
class User:
    id: str
    name: str

# Bad: every recoverable case stuffed into a single exception type.
# Callers must remember which `kind` values to handle vs re-raise.
class FetchUserError(Exception):
    def __init__(self, kind: str, message: str) -> None:
        super().__init__(message)
        self.kind = kind

def fetch_user_under_modeled(id: str) -> User:
    raise FetchUserError("NotFound", f"no user with id {id}")

# Good: recoverable cases as a tagged union on the success side.
# Real errors are raised and propagate to the top-level handler.
@dataclass(frozen=True)
class UserFound:
    user: User

@dataclass(frozen=True)
class UserNotFound:
    pass

@dataclass(frozen=True)
class UserStale:
    pass

UserLookup = UserFound | UserNotFound | UserStale

def fetch_user(id: str) -> UserLookup:
    # Database/network failures are raised, not returned — the caller
    # pattern-matches on UserLookup and lets exceptions propagate.
    _ = id
    return UserNotFound()

def handle_request(id: str) -> str:
    match fetch_user(id):
        case UserFound(user):
            return f"welcome, {user.name}"
        case UserNotFound():
            return "no such user"
        case UserStale():
            return "please refresh"

_ = (fetch_user_under_modeled, handle_request)
```

## Example 7: Decide in the Core, Act in the Shell

```python
from dataclasses import dataclass
from datetime import datetime, timedelta

@dataclass(frozen=True)
class Subscription:
    customer_id: str
    price_in_cents: int
    expires_at: datetime
    auto_renews: bool

REMINDER_WINDOW = timedelta(days=7)

def load_subscription(subscription_id: str) -> Subscription:
    raise NotImplementedError(subscription_id)

def charge_customer(customer_id: str, amount_in_cents: int) -> None:
    _ = (customer_id, amount_in_cents)

def send_renewal_reminder(customer_id: str) -> None:
    _ = customer_id

# Bad: decisions and effects are interleaved. Testing any branch needs a
# fake database, clock, payment provider, and mailer.
def renew_subscription_under_modeled(subscription_id: str) -> None:
    subscription = load_subscription(subscription_id)
    time_until_expiry = subscription.expires_at - datetime.now()
    if subscription.auto_renews and time_until_expiry <= timedelta(0):
        charge_customer(subscription.customer_id, subscription.price_in_cents)
    elif not subscription.auto_renews and time_until_expiry <= REMINDER_WINDOW:
        send_renewal_reminder(subscription.customer_id)

# Good: the core is pure — values in, a decision out.
@dataclass(frozen=True)
class Charge:
    amount_in_cents: int

@dataclass(frozen=True)
class SendReminder:
    pass

@dataclass(frozen=True)
class NoAction:
    pass

RenewalDecision = Charge | SendReminder | NoAction

def decide_renewal(subscription: Subscription, now: datetime) -> RenewalDecision:
    time_until_expiry = subscription.expires_at - now
    if subscription.auto_renews and time_until_expiry <= timedelta(0):
        return Charge(subscription.price_in_cents)
    if not subscription.auto_renews and time_until_expiry <= REMINDER_WINDOW:
        return SendReminder()
    return NoAction()

# The shell gathers inputs, asks the core, and performs the effect it names.
def renew_subscription(subscription_id: str) -> None:
    subscription = load_subscription(subscription_id)
    match decide_renewal(subscription, datetime.now()):
        case Charge(amount_in_cents):
            charge_customer(subscription.customer_id, amount_in_cents)
        case SendReminder():
            send_renewal_reminder(subscription.customer_id)
        case NoAction():
            pass

_ = (renew_subscription_under_modeled, renew_subscription)
```

## Example 8: Bulk Owns the Operation

```python
from collections.abc import Iterator
from contextlib import contextmanager
from dataclasses import dataclass

@dataclass(frozen=True)
class GroupId:
    value: str

@dataclass(frozen=True)
class UserId:
    value: str

def validate_membership_cap(group: GroupId, add_count: int) -> None:
    _ = (group, add_count)

@contextmanager
def group_lock(group: GroupId) -> Iterator[None]:
    _ = group
    yield

def persist_membership(group: GroupId, users: list[UserId]) -> None:
    _ = (group, users)

# Bad: two entry points implement the same operation independently.
# A rule added to one path (audit log, rate cap, new invariant) silently
# skips the other.
def add_user_under_modeled(group: GroupId, user: UserId) -> None:
    validate_membership_cap(group, 1)
    with group_lock(group):
        persist_membership(group, [user])

def add_users_under_modeled(group: GroupId, users: list[UserId]) -> None:
    validate_membership_cap(group, len(users))
    with group_lock(group):
        persist_membership(group, users)

# Good: the bulk path owns the operation; the single wraps it.
# Any rule added to add_users automatically applies to add_user.
def add_users(group: GroupId, users: list[UserId]) -> None:
    validate_membership_cap(group, len(users))
    with group_lock(group):
        persist_membership(group, users)

def add_user(group: GroupId, user: UserId) -> None:
    add_users(group, [user])

_ = (add_user_under_modeled, add_users_under_modeled, add_users, add_user)
```

## Example 9: Document the Why, Not the Obvious What

```python
# Bad: restates obvious behavior.
def increment_retry_count(retry_count: int) -> int:
    # Increment retry count.
    return retry_count + 1

# Good: documents a non-obvious business constraint.
def is_refund_eligible(days_since_purchase: int, refund_window_days: int) -> bool:
    # Billing provider closes refunds at next-day midnight, so we keep a one-day grace period.
    refund_grace_period_in_days = 1
    return days_since_purchase <= refund_window_days + refund_grace_period_in_days

_ = increment_retry_count(0)
```

## Example 10: Test State Transitions and Outcomes

```python
def test_bad_asserts_implementation_detail_only() -> None:
    transition_helper_call_count = 1
    assert transition_helper_call_count == 1

def test_good_transition_order_state_moves_draft_to_published() -> None:
    next_state = transition_order_state("draft", "publish")
    assert next_state == "published"
```

## Example 11: Mock Only Unstable Boundaries

```python
def test_bad_asserts_internal_helper_behavior() -> None:
    internal_helper_call_count = 1
    assert internal_helper_call_count == 1

def test_good_uses_cached_rate_when_live_lookup_fails() -> None:
    gateway = CurrencyGateway(
        rate_client=FailingRateClient(),
        cache_store=CachedRateStore(rate=0.92),
    )

    rate = gateway.get_rate_or_fallback("USD", "EUR")

    assert rate == 0.92
```

## Example 12: Prefer Libraries That Align With Your Type Design

```python
from typing import Any

# Bad: library returns untyped dict — field access is unchecked.
def get_user_name_under_modeled(response: dict[str, Any]) -> str:
    return response["user"]["name"]  # KeyError if shape changes

# Good: library returns typed model — type checker catches field errors.
from dataclasses import dataclass

@dataclass
class User:
    name: str
    email: str

def get_user_name(user: User) -> str:
    return user.name
```

## Example 13: Re-implement When the Dependency Outweighs the Logic

```python
# Bad: importing a utility library for a one-liner adds supply-chain surface for no benefit.
# from math_utils_lib import clamp
# clamped_from_lib = clamp(value, 0, 100)

# Good: trivial, well-understood logic is cheaper to own than to import.
def clamp(value: float, lo: float, hi: float) -> float:
    return max(lo, min(hi, value))

clamped = clamp(150, 0, 100)
```

## Example 14: External Contract Requires Looser Typing

```python
from typing import Literal

# Bad: assumes the partner only ever sends known values.
UnderModeledPartnerStatus = Literal["accepted", "rejected"]

def parse_partner_status(raw_partner_status: UnderModeledPartnerStatus) -> UnderModeledPartnerStatus:
    return raw_partner_status

# Good: accept raw string input and map unknown values explicitly.
PartnerStatus = Literal["accepted", "rejected", "unknown_partner_status"]

def map_partner_status(raw_partner_status: str) -> PartnerStatus:
    if raw_partner_status in {"accepted", "rejected"}:
        return raw_partner_status

    print(f"unknown partner status: {raw_partner_status}")
    return "unknown_partner_status"
```
