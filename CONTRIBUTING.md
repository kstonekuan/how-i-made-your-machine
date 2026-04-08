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
Its `SKILL.md` is a manual copy of the style guide (`src/style-guide.md`) with skill-specific additions.
When updating the style guide, also update the skill to match while preserving the differences required by the spec:

- **YAML frontmatter** — required by the spec (`name`, `description`, and any optional fields)
- **Agent-specific sections** — instructions for LLMs on how to apply the guide (e.g., "Agent Usage", "Agent Workflow"), inserted before the guide content
- **Bundled resources** — the skill may include files in optional spec directories (e.g., `references/`, `scripts/`, `assets/`) and reference them with relative paths from `SKILL.md`, where the style guide only uses external URLs
