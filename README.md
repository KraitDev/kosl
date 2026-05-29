<div align="center">

<img height="120" alt="kosl-logo" src="https://github.com/user-attachments/assets/20ff3874-5149-4406-88fb-3682a445e0d6" />

[![Version](https://img.shields.io/badge/-0D1117?style=flat-square&color=0D1117)](#)

**Krait Object Serialization Language (KOSL) is a modern, human-first serialization and configuration language. It combines the simplicity of** `.env` **files, the structure of JSON, and the usability of TOML, while completely avoiding the magical implicit conversions and whitespace sensitivity of YAML.**

</div>

## Features
- **Strict & Deterministic**: No indentation semantics, no implicit date parsing.
- **Minimal Syntax**: Parentheses `()` for objects, Brackets `[]` for explicit arrays.
- **Implicit Arrays**: Top-level commas create arrays naturally (`supported = win, mac, linux`).
- **Cargo Native**: Transpile `Cargo.kosl` directly to `Cargo.toml` with `kosl transpile`.
- **High Performance**: Zero-copy capable Rust parser.

## Example `Cargo.kosl`
```kosl
package=(
  name=my_project,
  version=0.1.0,
  edition=2021
)

dependencies=(
  rand=0.8.5,
  serde=(
    version=1.0,
    features=[derive]
  )
)
```

## CLI Usage
```bash
cargo install kosl-cli
kosl parse config.kosl
kosl format config.kosl
kosl transpile Cargo.kosl # Outputs Cargo.toml
```

## IDE Support
KOSL includes a first-class VSCode extension providing semantic highlighting, auto-formatting, and the official file icons for `.kosl` files.

## Contributing

We welcome contributions to KOSL. See [CONTRIBUTING.md](CONTRIBUTING.md) to learn more.

## License & Project Policies

**Krait Object Serialization Language (KOSL)** is an open-source project managed by **KraitDev**. The core files are dual-licensed under the terms of both the **MIT License** and the **Apache License 2.0**.

To protect the integrity of the ecosystem and ensure fair attribution, all users, forks, and contributors are bound by our official project policies:

* **Copyright & Ownership:** KraitDev retains exclusive ownership of the core KOSL codebase. Contributors maintain authorship of their specific code but grant KraitDev a permanent license to distribute it. Review the full fork and attribution boundaries in [COPYRIGHT](COPYRIGHT).
* **Trademark & Identity:** The phrase *"Krait Object Serialization Language"*, the word *"KOSL"* and the official logos are protected brand assets. You are fully permitted to use them to brand community tools, libraries, and extensions built for the ecosystem, provided it is clear they are independent creations. Deceptive core forks or commercial passing off are strictly prohibited. Review the full domain boundaries in [TRADEMARK](TRADEMARK).

See [LICENSE-MIT](LICENSE-MIT) and [LICENSE-APACHE](LICENSE-APACHE) for the underlying open-source license texts.
