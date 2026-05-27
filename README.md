# KOSL (Krait Object Serialization Language)

KOSL is a minimal, human-readable, and blazing-fast data serialization format built entirely in Rust. Designed originally as the native configuration pipeline for the **Krait** programming language ecosystem, KOSL offers a flat, zero-nonsense alternative to JSON and TOML while retaining 100% compatibility with the Rust `cargo` build system.

## Features

- **Minimalist Syntax:** Clean `key=value` layout without the noise of trailing commas, outer curly braces, or forced quotes.
- **Native Grouped Objects:** Simple parentheses syntax `group=(key=value)` for handling nested structures without indentation traps.
- **Dynamic Type Parsing:** Automatic runtime handling of Strings, Integers, Floats, Booleans, and comma-separated Lists.
- **Built-in Cargo Bridge:** Read a `krait.kosl` file and automatically generate a fully valid `Cargo.toml` dynamically.
- **Blazing Fast:** Written in 100% safe, idiomatic Rust.

## Syntax Example

```coffee
# krait.kosl
name=my_krait_project
version=0.1.0
edition=2021

dependencies=(rand=0.8.5, serde=1.0)
supportedOS=windows, macOS, ubuntu

```

When run through the translation bridge, it automatically compiles into:

```toml
[package]
name = "my_krait_project"
version = "0.1.0"
edition = "2021"

[dependencies]
rand = "0.8.5"
serde = "1.0"

[package.metadata.krait]
supportedOS = ["windows", "macOS", "ubuntu"]

```

## Usage

Add KOSL to your `Cargo.toml`:

```toml
[dependencies]
kosl = { git = "[https://github.com/yourusername/kosl](https://github.com/yourusername/kosl)" }

```

### Parsing KOSL in Rust

```rust
use kosl::parser::parse_kosl;

fn main() {
    let kosl_text = "appName=Krait Dashboard\nversion=1.2";
    let parsed_data = parse_kosl(kosl_text);
    
    println!("{:#?}", parsed_data);
}

```

### Auto-Generating Cargo.toml

```rust
use kosl::automation::generate_cargo_toml;

fn main() {
    // Looks for 'krait.kosl' in the current directory and outputs 'Cargo.toml'
    if let Err(e) = generate_cargo_toml() {
        eprintln!("Automation failed: {}", e);
    }
}

```

## Contributing

We welcome contributions to KOSL. See CONTRIBUTING.md for details.

## License

KOSL is dual licensed under the terms of both the MIT License and the Apache License 2.0. See LICENSE-MIT and LICENSE-APACHE for details.
