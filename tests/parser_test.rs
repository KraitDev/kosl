use kosl_parser::Parser;
use kosl_ast::Value;

#[test]
fn test_implicit_array() {
    let input = "supported=windows10,ubuntu16.5,macOS10";
    let mut parser = Parser::new(input);
    let ast = parser.parse().unwrap();
    
    if let Value::Object(map) = ast {
        if let Value::Array(arr) = &map["supported"] {
            assert_eq!(arr.len(), 3);
            assert_eq!(arr[0], Value::String("windows10".to_string()));
            assert_eq!(arr[1], Value::String("ubuntu16.5".to_string()));
        } else { panic!("Expected array"); }
    }
}

#[test]
fn test_semver_fallback() {
    let input = "version=0.1.0";
    let mut parser = Parser::new(input);
    let ast = parser.parse().unwrap();
    
    if let Value::Object(map) = ast {
        assert_eq!(map["version"], Value::String("0.1.0".to_string())); // Did not fail f64 parsing!
    }
}