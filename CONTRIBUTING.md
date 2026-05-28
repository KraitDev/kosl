# Contributing to KOSL

Thank you for your interest in contributing to the Krait Object Serialization Language (KOSL) project. This guide outlines the development workflow, code standards, and submission guidelines.

## Development Setup

The repository is organized as a Rust workspace alongside a VSCode extension.

### Prerequisites

- **Rust**: Ensure you have the stable toolchain installed (1.70+ recommended).
- **Node.js & npm**: Required to build, run, and package the VSCode extension.

### Building and Testing

To verify changes in the Rust crates:

```bash
# Clone the repository
git clone https://github.com/kosl-org/kosl.git
cd kosl

# Build all workspace crates
cargo build

# Run unit and integration tests
cargo test
```

To run and verify code style locally:

```bash
# Check formatting
cargo fmt --all -- --check

# Run linter
cargo clippy --all-targets --all-features -- -D warnings
```

## Code Style and Standards

### Rust Code

- Follow standard idiomatic Rust practices.
- Run `cargo fmt` before submitting your pull request.
- Keep dependency additions minimal; avoid bloated libraries in parser modules.

### Parser & Language Changes

- Changes to the parser logic or type inference must align with the formal grammar rules in `KOSL_SPEC.md`.
- Any syntax changes or feature additions require corresponding test updates in the `tests/` directory and updates to the VSCode syntax highlighting file (`kosl.tmLanguage.json`).

## Submitting a Pull Request

1. **Fork the repository** and create a feature branch from `main`.
2. **Add tests for your** changes. Parser modifications should include negative test cases (malformed input handling).
3. **Keep PRs focused**. Avoid mixing refactoring, syntax additions, and documentation updates in a single PR.
4. **Ensure green CI**. Your PR must pass formatting, Clippy, and unit tests.
5. **Update documentation** where relevant (such as the Specification, CLI help texts, or README).