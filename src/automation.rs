use crate::bridge::kosl_to_cargo_toml;
use std::env;
use std::fs;

/// Looks for `krait.kosl` in the current working directory, 
/// translates it, and immediately outputs a valid `Cargo.toml`.
pub fn generate_cargo_toml() -> std::io::Result<()> {
    let current_dir = env::current_dir()?;
    let kosl_path = current_dir.join("krait.kosl");

    if !kosl_path.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Failed to find 'krait.kosl' in the current working directory.",
        ));
    }

    let kosl_text = fs::read_to_string(&kosl_path)?;
    let toml_output = kosl_to_cargo_toml(&kosl_text);

    let toml_path = current_dir.join("Cargo.toml");
    fs::write(toml_path, toml_output)?;

    println!("✅ Successfully compiled krait.kosl into Cargo.toml!");
    Ok(())
}