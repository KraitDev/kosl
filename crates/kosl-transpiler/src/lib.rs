use kosl_ast::Value;
use anyhow::Result;
use toml::Value as TomlValue;
use toml::map::Map;

pub fn kosl_to_toml(kosl: &Value) -> Result<String> {
    let toml_ast = convert_value(kosl)?;
    Ok(toml::to_string_pretty(&toml_ast)?)
}

fn convert_value(val: &Value) -> Result<TomlValue> {
    match val {
        Value::Null => Ok(TomlValue::String("".to_string())), // TOML has no null, fallback or drop
        Value::Bool(b) => Ok(TomlValue::Boolean(*b)),
        Value::Int(i) => Ok(TomlValue::Integer(*i)),
        Value::Float(f) => Ok(TomlValue::Float(*f)),
        Value::String(s) => Ok(TomlValue::String(s.clone())),
        Value::Array(arr) => {
            let mut t_arr = Vec::new();
            for item in arr {
                t_arr.push(convert_value(item)?);
            }
            Ok(TomlValue::Array(t_arr))
        }
        Value::Object(obj) => {
            let mut map = Map::new();
            for (k, v) in obj {
                map.insert(k.clone(), convert_value(v)?);
            }
            Ok(TomlValue::Table(map))
        }
    }
}