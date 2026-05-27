use crate::parser::{parse_kosl, KoslValue};

/// Translates krait.kosl directly into a syntactically valid Cargo.toml string.
pub fn kosl_to_cargo_toml(kosl_text: &str) -> String {
    let mut kosl_map = parse_kosl(kosl_text);
    let mut toml = String::new();

    // -- Standard [package] header mapping --
    toml.push_str("[package]\n");
    let standard_keys = ["name", "version", "edition"];
    for key in standard_keys {
        if let Some(val) = kosl_map.remove(key) {
            // Core standard Cargo fields must be quoted
            toml.push_str(&format!("{} = \"{}\"\n", key, val.as_unquoted_string()));
        }
    }

    // -- Expanded [dependencies] section --
    if let Some(KoslValue::Object(deps)) = kosl_map.remove("dependencies") {
        toml.push_str("\n[dependencies]\n");
        let mut deps_keys: Vec<_> = deps.keys().collect();
        deps_keys.sort(); 
        
        for k in deps_keys {
            let v = deps.get(k).unwrap();
            toml.push_str(&format!("{} = \"{}\"\n", k, v.as_unquoted_string()));
        }
    }

    // -- Non-standard Metadata mapping --
    // Remaining properties (like supportedOS) map to a custom package.metadata block
    if !kosl_map.is_empty() {
        toml.push_str("\n[package.metadata.krait]\n");
        let mut custom_keys: Vec<_> = kosl_map.keys().collect();
        custom_keys.sort();
        
        for k in custom_keys {
            let v = kosl_map.get(k).unwrap();
            toml.push_str(&format!("{} = {}\n", k, v.to_toml_value()));
        }
    }

    toml
}