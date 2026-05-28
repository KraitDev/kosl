use anyhow::Result;
use kosl_ast::Value;

/// Formats a parsed KOSL Value into a standardized, deterministic string.
pub fn format(value: &Value) -> Result<String> {
    let mut out = String::new();
    format_value(value, 0, &mut out)?;
    // Ensure file ends with a trailing newline
    if !out.ends_with('\n') {
        out.push('\n');
    }
    Ok(out)
}

fn format_value(value: &Value, indent_level: usize, out: &mut String) -> Result<()> {
    let indent = "  ".repeat(indent_level);
    match value {
        Value::Null => out.push_str("null"),
        Value::Bool(b) => out.push_str(&b.to_string()),
        Value::Int(i) => out.push_str(&i.to_string()),
        Value::Float(f) => out.push_str(&f.to_string()),
        Value::String(s) => {
            // Check if string contains spaces or structural characters
            let needs_quotes = s.is_empty()
                || s.chars().any(|c| {
                    c.is_whitespace()
                        || c == ','
                        || c == '='
                        || c == '('
                        || c == ')'
                        || c == '['
                        || c == ']'
                });
            if needs_quotes {
                out.push_str(&format!("\"{}\"", s));
            } else {
                out.push_str(s);
            }
        }
        Value::Array(arr) => {
            if arr.is_empty() {
                out.push_str("[]");
            } else {
                out.push_str("[\n");
                for (i, item) in arr.iter().enumerate() {
                    out.push_str(&"  ".repeat(indent_level + 1));
                    format_value(item, indent_level + 1, out)?;
                    if i < arr.len() - 1 {
                        out.push_str(",");
                    }
                    out.push_str("\n");
                }
                out.push_str(&indent);
                out.push_str("]");
            }
        }
        Value::Object(obj) => {
            if obj.is_empty() {
                out.push_str("()");
            } else if indent_level == 0 {
                // At the document root level, print without surrounding parentheses
                for (i, (k, v)) in obj.iter().enumerate() {
                    out.push_str(k);
                    out.push_str("=");
                    format_value(v, 0, out)?;
                    if i < obj.len() - 1 {
                        out.push_str("\n");
                    }
                }
            } else {
                out.push_str("(\n");
                for (i, (k, v)) in obj.iter().enumerate() {
                    out.push_str(&"  ".repeat(indent_level + 1));
                    out.push_str(k);
                    out.push_str("=");
                    format_value(v, indent_level + 1, out)?;
                    if i < obj.len() - 1 {
                        out.push_str(",");
                    }
                    out.push_str("\n");
                }
                out.push_str(&indent);
                out.push_str(")");
            }
        }
    }
    Ok(())
}
