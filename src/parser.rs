use std::collections::HashMap;
use crate::value::KoslValue;

/// Parses a raw KOSL configuration string slice into a root-level `HashMap`.
pub fn from_str(input: &str) -> Result<HashMap<String, KoslValue>, String> {
    let mut map = HashMap::new();
    
    for (line_idx, line) in input.lines().enumerate() {
        let trimmed = line.trim();
        
        // Skip empty lines and comment lines
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        
        if let Some((key, val)) = trimmed.split_once('=') {
            let key = key.trim().to_string();
            if key.is_empty() {
                return Err(format!("Syntax Error: Empty key found on line {}", line_idx + 1));
            }
            let value = parse_value(val);
            map.insert(key, value);
        } else {
            return Err(format!(
                "Syntax Error: Invalid line layout on line {}. Key-value pairs must be separated by '='",
                line_idx + 1
            ));
        }
    }
    
    Ok(map)
}

/// Recursively processes and typesets primitive and compound strings into native `KoslValue` elements.
fn parse_value(s: &str) -> KoslValue {
    let s = s.trim();

    // 1. Process inline structural grouped objects
    if s.starts_with('(') && s.ends_with(')') {
        let inner = &s[1..s.len() - 1];
        let mut map = HashMap::new();
        for pair in inner.split(',') {
            let pair = pair.trim();
            if pair.is_empty() {
                continue;
            }
            if let Some((k, v)) = pair.split_once('=') {
                map.insert(k.trim().to_string(), parse_value(v));
            }
        }
        return KoslValue::Object(map);
    }

    // 2. Process non-nested flat list structures
    if s.contains(',') {
        let mut items = Vec::new();
        for item in s.split(',') {
            let item_trimmed = item.trim();
            if item_trimmed.is_empty() {
                continue;
            }
            items.push(parse_value(item_trimmed));
        }
        return KoslValue::List(items);
    }

    // 3. Strip outer matched single or double quote wrappers if present
    if (s.starts_with('"') && s.ends_with('"')) || (s.starts_with('\'') && s.ends_with('\'')) {
        if s.len() >= 2 {
            return KoslValue::String(s[1..s.len() - 1].to_string());
        }
    }

    // 4. Fallthrough matching of raw scalar literals and boolean types
    if s == "true" {
        return KoslValue::Bool(true);
    }
    if s == "false" {
        return KoslValue::Bool(false);
    }
    if let Ok(i) = s.parse::<i64>() {
        return KoslValue::Int(i);
    }
    if let Ok(f) = s.parse::<f64>() {
        return KoslValue::Float(f);
    }

    // Default catch-all type assignment for plain unquoted strings
    KoslValue::String(s.to_string())
}
