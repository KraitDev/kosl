# Contributing to KOSL

First off, thank you for taking the time to contribute! KOSL aims to be the cleanest configuration and serialization tool in the systems ecosystem, and community inputs keep it bulletproof.

## How Can I Contribute?

### Reporting Bugs
- Check the Issues tab to ensure the bug hasn't already been reported.
- Open a new issue utilizing a clear title, a description of the unexpected behavior, and a minimal reproducible snippet of the `.kosl` code causing the failure.

### Suggesting Enhancements
- Open an issue outlining your proposed syntax feature or bridge upgrade.
- Explain *why* this change benefits the wider ecosystem (both Krait and non-Krait projects).

### Pull Requests
1. Fork the repository and create your branch from `main`.
2. Ensure your code follows idiomatic Rust style guidelines (`cargo fmt` is your friend).
3. If you alter parsing logic, write matching regression tests inside the `tests` module in `lib.rs`.
4. Run `cargo test` locally to ensure all specifications pass.
5. Open a Pull Request with a comprehensive description of your changes.

## Development Setup
To test your local environment:
```bash
git clone [https://github.com/yourusername/kosl.git](https://github.com/yourusername/kosl.git)
cd kosl
cargo test