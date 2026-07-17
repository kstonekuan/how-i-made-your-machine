# Dependencies and Exceptions

Use these rules before introducing dependencies or intentionally relaxing another part of the guide.

## Choose dependencies deliberately

Imported code carries maintenance, compatibility, and supply-chain costs.

- Prefer popular, actively maintained libraries backed by established organizations or open-source communities.
- Consider recent maintenance, adoption, security history, release cadence, and transitive dependencies.
- Prefer APIs that return well-typed domain objects over `any`, raw dictionaries, or untyped JSON.
- Prefer libraries whose type design aligns with the application's domain model.
- Use the standard library or a small local implementation when the needed algorithm is well known and only a few lines long.
- Reconsider a dependency when extensive adapters are required to make its types fit the domain.

The goal is not to avoid dependencies categorically. It is to ensure the dependency removes more risk and work than it introduces.

Read [Choosing Linters and Type Checkers](choosing-linters-and-type-checkers.md) for static-analysis tool selection.

## Make exceptions explicit

Strict modeling can create disproportionate complexity, or external contracts, performance, and interoperability constraints may require looser types.

When taking an exception, document:

1. Which rule is being bent.
2. Why the constraint makes the exception worthwhile.
3. Which safeguards preserve correctness.

Safeguards may include boundary parsing, explicit unknown variants, logging, targeted assertions, or behavior-focused tests.

Keep the weaker representation close to the constrained boundary. Convert to the stronger internal model as soon as practical instead of allowing the exception to spread through the codebase.
