pub mod value;
pub mod parser;
pub mod serde_impl;

pub use serde_impl::deserialize_from_str;

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[derive(Deserialize, Debug, PartialEq)]
    struct Metadata {
        author: String,
        speed: String,
    }

    #[derive(Deserialize, Debug, PartialEq)]
    struct Config {
        name: String,
        version: f64,
        is_active: bool,
        metadata: Metadata,
        tags: Vec<String>,
    }

    #[test]
    fn test_kosl_parsing() {
        let input = r#"
name = "Platinum KOSL"
version = 1.0
is_active = true
metadata = (author = DevKid, speed = fast)
tags = systems, config, crate
"#;

        let config: Config = deserialize_from_str(input).expect("Failed to deserialize valid KOSL spec text layout");
        
        assert_eq!(config.name, "Platinum KOSL");
        assert_eq!(config.version, 1.0);
        assert_eq!(config.is_active, true);
        assert_eq!(config.metadata.author, "DevKid");
        assert_eq!(config.metadata.speed, "fast");
        assert_eq!(
            config.tags, 
            vec!["systems".to_string(), "config".to_string(), "crate".to_string()]
        );
    }
}
