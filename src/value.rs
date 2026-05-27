use std::collections::HashMap;

/// Represents the Abstract Syntax Tree (AST) for parsed KOSL configuration fragments.
#[derive(Debug, Clone, PartialEq)]
pub enum KoslValue {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    Object(HashMap<String, KoslValue>),
    List(Vec<KoslValue>),
}
