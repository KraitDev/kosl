use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Array(Vec<Value>),
    Object(IndexMap<String, Value>),
}

impl Value {
    /// Helper to enforce duplicate key rejection during AST construction
    pub fn insert_object_key(
        map: &mut IndexMap<String, Value>,
        key: String,
        value: Value,
    ) -> Result<(), String> {
        if map.contains_key(&key) {
            return Err(format!("Duplicate key strictly prohibited: '{}'", key));
        }
        map.insert(key, value);
        Ok(())
    }
}