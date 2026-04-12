### Choosing Linters and Type Checkers

The patterns in this guide — exhaustive matching, refined types, parse-at-boundary — only hold if the toolchain actually enforces them.
Linters and type checkers are dependencies too, and the same selection criteria apply: prefer popular, actively maintained tools backed by known organizations, and favor those whose design aligns with the values here.

- Enable the strictest practical rule set. Lenient defaults let the patterns this guide recommends slip through unchecked.
- Prefer tools written in stricter, compiled languages. Rust-based tooling tends to be faster and benefits from the same compiler-driven correctness this guide advocates.
- Keep the toolchain cohesive. Tools from the same ecosystem share conventions, reduce configuration friction, and are more likely to be maintained together.

Current examples of tools that fit these criteria:

- **TypeScript** — Biome (linter and formatter, written in Rust) or oxlint (linter from the Oxc project, written in Rust).
- **Rust** — Clippy with pedantic lints enabled. The default lint set is intentionally conservative; pedantic raises the bar to match the strictness this guide expects.
- **Python** — The Astral ecosystem: uv (package manager), ruff (linter and formatter), and ty (type checker), all written in Rust and designed to work together.
- **Documentation links** — Lychee (link checker for Markdown, HTML, and plain text, written in Rust). Catches broken internal and external links before they rot in published docs.
- **Dockerfiles** — hadolint (linter that parses Dockerfiles into an AST and applies ShellCheck rules to RUN instructions, written in Haskell).
- **Shell scripts** — ShellCheck (static analyzer for sh/bash/dash/ksh, written in Haskell). Catches quoting errors, unused variables, and subtle POSIX pitfalls that are invisible to manual review.
- **Terraform** — OpenTofu for formatting and validation, paired with tflint for Terraform-specific static analysis. This keeps infrastructure checks close to the language semantics instead of relying on generic text-based rules.
