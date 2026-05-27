use std::collections::HashMap;

/// Represents the possible native dynamic types in KOSL.
#[derive(Debug, Clone, PartialEq)]
pub enum KoslValue {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    Object(HashMap<String, KoslValue>),
    List(Vec<KoslValue>),
}

impl KoslValue {
    /// Returns the value as a plain string. Useful for explicitly formatting
    /// numbers back into semantic string values (like exact version requirements).
    pub fn as_unquoted_string(&self) -> String {
        match self {
            KoslValue::String(s) => s.clone(),
            KoslValue::Int(i) => i.to_string(),
            KoslValue::Float(f) => {
                let mut s = f.to_string();
                if !s.contains('.') {
                    s.push_str(".0"); // Preserves pure ".0" floats like `1.0`
                }
                s
            },
            KoslValue::Bool(b) => b.to_string(),
            KoslValue::Object(_) => "[Object]".to_string(),
            KoslValue::List(_) => "[List]".to_string(),
        }
    }

    /// Converts the KOSL value natively into a valid TOML element.
    pub fn to_toml_value(&self) -> String {
        match self {
            KoslValue::String(s) => format!("\"{}\"", s),
            KoslValue::Int(i) => i.to_string(),
            KoslValue::Float(f) => {
                let mut s = f.to_string();
                if !s.contains('.') { s.push_str(".0"); }
                s
            },
            KoslValue::Bool(b) => b.to_string(),
            KoslValue::List(l) => {
                let items: Vec<String> = l.iter().map(|i| i.to_toml_value()).collect();
                format!("[{}]", items.join(", "))
            },
            KoslValue::Object(o) => {
                let mut items: Vec<String> = o.iter()
                    .map(|(k, v)| format!("{} = {}", k, v.to_toml_value()))
                    .collect();
                items.sort(); // Predictable rendering sequence
                format!("{{ {} }}", items.join(", "))
            }
        }
    }
}

/// Parses a raw krait.kosl text string into a dynamic HashMap of KoslValues.
pub fn parse_kosl(text: &str) -> HashMap<String, KoslValue> {
    let mut result = HashMap::new();

    for line in text.lines() {
        let trimmed = line.trim();
        
        // 1. Ignore blank lines and comments
        if trimmed.is_empty() || trimmed.starts_with('#') { continue; }

        // 2. Parse standard assignments: key=value
        if let Some((key, value_str)) = trimmed.split_once('=') {
            let k = key.trim().to_string();
            let v = parse_value(value_str.trim());
            result.insert(k, v);
        }
    }
    result
}

fn parse_value(s: &str) -> KoslValue {
    let s = s.trim();
    
    // 3. Inline/Grouped Objects wrapped in parentheses
    if s.starts_with('(') && s.ends_with(')') {
        let inner = &s[1..s.len() - 1];
        let mut map = HashMap::new();
        for pair in inner.split(',') {
            if let Some((k, v)) = pair.split_once('=') {
                map.insert(k.trim().to_string(), parse_primitive(v.trim()));
            }
        }
        KoslValue::Object(map)
        
    // 4. Lists/Arrays are comma-separated
    } else if s.contains(',') {
        let list = s.split(',').map(|i| parse_primitive(i.trim())).collect();
        KoslValue::List(list)
        
    } else {
        parse_primitive(s)
    }
}

fn parse_primitive(s: &str) -> KoslValue {
    let s = s.trim();
    
    // Strip surrounding quotes if the user explicitly added them
    if s.starts_with('"') && s.ends_with('"') && s.len() >= 2 {
        return KoslValue::String(s[1..s.len()-1].to_string());
    }
    if s.starts_with('\'') && s.ends_with('\'') && s.len() >= 2 {
        return KoslValue::String(s[1..s.len()-1].to_string());
    }

    if s == "true" { return KoslValue::Bool(true); }
    if s == "false" { return KoslValue::Bool(false); }
    if let Ok(i) = s.parse::<i64>() { return KoslValue::Int(i); }
    if let Ok(f) = s.parse::<f64>() { return KoslValue::Float(f); }
    
    KoslValue::String(s.to_string())
}