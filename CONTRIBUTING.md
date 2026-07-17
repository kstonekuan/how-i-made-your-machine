# Contributing

## Code Quality

### Rust

```bash
cargo clippy --all-targets --all-features  # Linting
cargo fmt                                   # Formatting
```

## Code Style & Philosophy

### Typing & Pattern Matching

- Prefer **explicit types** over raw dicts -- make invalid states unrepresentable where practical
- Prefer **typed variants over string literals** when the set of valid values is known
- Use **exhaustive pattern matching** (`match` in Python and Rust, `ts-pattern` in TypeScript) so the type checker can verify all cases are handled
- Structure types to enable exhaustive matching when handling variants
- Prefer **shared internal functions over factory patterns** when extracting common logic from hooks or functions -- keep each export explicitly defined for better IDE navigation and readability

### Self-Documenting Code

- **Verbose naming**: Variable and function naming should read like documentation
- **Strategic comments**: Only for non-obvious logic or architectural decisions; avoid restating what code shows

## Keeping the Skill in Sync

The skill at `skills/how-i-made-your-machine/` follows the [Agent Skills specification](https://agentskills.io/specification).
It is an agent-oriented, progressively disclosed version of the style guide rather than a complete copy of `src/style-guide.md`.
When updating the style guide, update the corresponding skill material while preserving the differences required by the spec:

- **YAML frontmatter** — required by the spec (`name`, `description`, and any optional fields)
- **Concise core instructions** — `SKILL.md` contains only the workflow, core defaults, reference routing, and review checklist that agents need on every task
- **Topic references** — detailed rationale lives in `references/` and is loaded only when the corresponding concern applies
- **Language examples** — examples from the style guide are grouped by language so an agent can load only the language relevant to its task
- **Direct routing** — every reference remains one level below and is linked directly from `SKILL.md` with a clear condition for reading it

Do not restore the full style guide to `SKILL.md`. Keep its body comfortably below 500 lines, avoid duplicating detailed material between it and the references, and add a table of contents to any reference longer than 100 lines.
