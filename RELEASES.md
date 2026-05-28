# Releases

All notable changes to the Krait Object Serialization Language (KOSL) ecosystem will be documented in this file.

## KOSL v0.1.0

### Initial Release

Welcome to the inaugural release of the **Krait Object Serialization Language (KOSL)**! This release establishes the core language specification, the official Rust-based toolchain, seamless Cargo compatibility, and a dedicated VS Code extension.

### Language Specification & Grammar
- **Added** formal EBNF grammar (`KOSL_SPEC.md`) defining a deterministic, whitespace-agnostic syntax.
- **Added** Implicit Array support: Comma-separated values at the top level or within data scopes automatically parse into arrays, enabling clean one-liners (e.g., `supported = mac, win, linux`).
- **Added** Strict Type Inference rules:
  - Valid `i64` evaluates to Integer.
  - Valid `f64` (strictly requiring a single decimal point) evaluates to Float.
  - Barewords like `true`, `false`, and `null` evaluate to their respective static types.
  - **Anti-Magic Fallback**: Ambiguous barewords (like semantic version `0.1.0` or bare alphanumeric strings) safely fall back to `String` without crashing or silently converting to dates.
- **Added** explicit scoping mechanisms using parentheses `()` for Objects and brackets `[]` for Arrays.
- **Added** support for `#` and `//` line comments.
- **Added** strict rejection of duplicate keys to ensure predictable data mapping.

### Core Toolchain (Rust Workspace)
- **Added** `kosl-ast`: The foundational Abstract Syntax Tree using `IndexMap` to preserve insertion order and safely serialize structural data.
- **Added** `kosl-parser`: A zero-magic, high-performance recursive descent parser with UTF-8 support and clean error bubbling via `anyhow`.
- **Added** `kosl-formatter`: A deterministic formatting engine to standardize KOSL syntax, indentation, and trailing commas. Highly optimized using character-level push routines.
- **Added** `kosl-transpiler`: A dedicated bridge engine translating KOSL AST directly to fully compliant TOML.

### Command Line Interface (`kosl-cli`)
- **Added** the official `kosl` binary with the following subcommands:
  - `kosl parse <file>`: Reads a `.kosl` file, validates syntax, and prints the raw AST structure for debugging.
  - `kosl format <file> [--write]`: Formats a `.kosl` document to stdout or overwrites the file in-place with standardized spacing and quotes.
  - `kosl transpile <file> [--output]`: Native translation command to turn KOSL configurations into target files (e.g., TOML).

### Cargo / Rust Integration
- **Added** native support for `Cargo.kosl` manifests as a cleaner, parenthetical alternative to `Cargo.toml`.
- **Added** accurate conversion handling for Rust specific paradigms:
  - Workspace definitions.
  - Complex nested dependency declarations (e.g., `serde = (version = 1.0, features = [derive])`).
  - Arrays of tables translation (e.g., translating `bin = [(name = cli, path = src/main.rs)]` directly into TOML `[[bin]]` tables).

### VS Code Extension (`kosl-lang`)
- **Added** custom language association for `.kosl` files.
- **Added** robust TextMate grammar (`kosl.tmLanguage.json`) for precise semantic highlighting of assignments, numbers, booleans, and nested structures.
- **Added** auto-closing bracket configurations for `()`, `[]`, and `""`.
- **Added** custom, monochrome-compatible SVG File Icons featuring the geometric Krait snake design for the VS Code file explorer.

### Documentation & Legal
- **Added** exhaustive Wiki material covering Core Principles, CLI Reference, and VS Code Setup.
- **Added** `CONTRIBUTING.md` with guidelines on PRs, architecture, and code style.
- **Added** `SECURITY.md` detailing the vulnerability disclosure policy.
- **Added** comprehensive `COPYRIGHT` and `TRADEMARK` policy documents clarifying the dual MIT/Apache-2.0 license, ecosystem rebranding rules, and independent extension rights.
- **Added** standardized GitHub Issue and Pull Request templates.

### CI/CD & Distribution
- **Added** strict GitHub Actions CI workflow to validate `rustfmt`, check `clippy` rules without warnings, and run all unit/integration tests across the workspace.
- **Added** multi-architecture Release workflow:
  - Cross-compiles completely static `musl` Linux binaries (x64, ARM64).
  - Cross-compiles macOS binaries (Intel, Apple Silicon).
  - Cross-compiles Windows executables (x64, ARM64).
- **Added** automatic `.vsix` packaging and release attachment for the VS Code Extension.
