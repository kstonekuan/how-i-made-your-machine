# How I Made Your Machine

mdBook site for publishing my personal coding style guide.
This repo also includes a Rust preprocessor for Docusaurus-style language tabs.

## Local development

```bash
cargo build --bin mdbook-language-tabs-preprocessor
mdbook serve --hostname 0.0.0.0 --port 3000
```

## Quality checks

```bash
cargo fmt
cargo clippy --all-targets --all-features
cargo check --workspace
```

## GitHub Pages publishing

1. Push this repository to GitHub.
2. In repository settings, set Pages source to `GitHub Actions`.
3. Push to `main` to trigger `.github/workflows/deploy.yml`.

## LLM-friendly endpoints

- `/llms.txt`
- `/style-guide.md.txt`
