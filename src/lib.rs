pub mod parser;
pub mod bridge;
pub mod automation;

#[cfg(test)]
mod tests {
    use crate::bridge::kosl_to_cargo_toml;

    #[test]
    fn test_kosl_to_cargo_toml_translation() {
        let kosl_text = r#"
name=my Krait_project
version=0.1.0
edition=2021
dependencies=(rand=0.8.5, serde=1.0)
supportedOS=windows, macOS, ubuntu
        "#;

        let toml_output = kosl_to_cargo_toml(kosl_text);
        println!("Generated TOML:\n{}", toml_output);

        // 1. Verify standard package mappings
        assert!(toml_output.contains("[package]"));
        assert!(toml_output.contains("name = \"my Krait_project\""));
        assert!(toml_output.contains("version = \"0.1.0\""));
        assert!(toml_output.contains("edition = \"2021\""));

        // 2. Verify dependencies mapping expansion
        assert!(toml_output.contains("[dependencies]"));
        assert!(toml_output.contains("rand = \"0.8.5\""));
        assert!(toml_output.contains("serde = \"1.0\""));

        // 3. Verify Lists and custom values are syntactically sound via TOML Arrays
        assert!(toml_output.contains("[package.metadata.krait]"));
        assert!(toml_output.contains("supportedOS = [\"windows\", \"macOS\", \"ubuntu\"]"));
    }
}