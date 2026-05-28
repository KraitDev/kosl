use kosl_ast::Value;
use indexmap::IndexMap;
use anyhow::{Result, bail};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Ident(String),
    String(String),
    Eq, Comma, LParen, RParen, LBracket, RBracket,
    Eof,
}

pub struct Lexer<'a> {
    input: std::iter::Peekable<std::str::Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input: input.chars().peekable() }
    }

    fn skip_whitespace_and_comments(&mut self) {
        while let Some(&c) = self.input.peek() {
            if c.is_whitespace() {
                self.input.next();
            } else if c == '#' || c == '/' {
                let next = self.input.clone().nth(1);
                if c == '#' || (c == '/' && next == Some('/')) {
                    while let Some(ch) = self.input.next() {
                        if ch == '\n' { break; }
                    }
                } else { break; }
            } else { break; }
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace_and_comments();
        match self.input.next() {
            Some('=') => Token::Eq,
            Some(',') => Token::Comma,
            Some('(') => Token::LParen,
            Some(')') => Token::RParen,
            Some('[') => Token::LBracket,
            Some(']') => Token::RBracket,
            Some('"') => {
                let mut s = String::new();
                while let Some(c) = self.input.next() {
                    if c == '"' { break; }
                    s.push(c);
                }
                Token::String(s)
            }
            Some(c) if c.is_alphanumeric() || c == '_' || c == '-' || c == '.' => {
                let mut s = String::from(c);
                while let Some(&ch) = self.input.peek() {
                    if ch.is_alphanumeric() || ch == '_' || ch == '-' || ch == '.' {
                        s.push(self.input.next().unwrap());
                    } else { break; }
                }
                Token::Ident(s)
            }
            Some(c) => panic!("Unexpected character: {}", c),
            None => Token::Eof,
        }
    }
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current: Token,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer::new(input);
        let current = lexer.next_token();
        Self { lexer, current }
    }

    fn advance(&mut self) {
        self.current = self.lexer.next_token();
    }

    pub fn parse(&mut self) -> Result<Value> {
        let mut root = IndexMap::new();
        while self.current != Token::Eof {
            let (k, v) = self.parse_pair()?;
            Value::insert_object_key(&mut root, k, v).map_err(anyhow::Error::msg)?;
            if self.current == Token::Comma {
                self.advance(); // Top level trailing commas allowed between pairs
            }
        }
        Ok(Value::Object(root))
    }

    fn parse_pair(&mut self) -> Result<(String, Value)> {
        let key = match &self.current {
            Token::Ident(s) | Token::String(s) => s.clone(),
            _ => bail!("Expected key, found {:?}", self.current),
        };
        self.advance();
        
        if self.current != Token::Eq { bail!("Expected '=' after key '{}'", key); }
        self.advance();

        let val = self.parse_value_or_implicit_array()?;
        Ok((key, val))
    }

    fn parse_value_or_implicit_array(&mut self) -> Result<Value> {
        let first_val = self.parse_single_value()?;
        
        // Check for implicit array
        if self.current == Token::Comma {
            let mut arr = vec![first_val];
            while self.current == Token::Comma {
                self.advance(); // consume comma
                // Handle trailing comma before structural close
                if matches!(self.current, Token::RParen | Token::RBracket | Token::Eof) {
                    break;
                }
                arr.push(self.parse_single_value()?);
            }
            return Ok(Value::Array(arr));
        }
        
        Ok(first_val)
    }

    fn parse_single_value(&mut self) -> Result<Value> {
        match self.current.clone() {
            Token::LParen => self.parse_object(),
            Token::LBracket => self.parse_explicit_array(),
            Token::String(s) => {
                self.advance();
                Ok(Value::String(s))
            }
            Token::Ident(s) => {
                self.advance();
                Ok(self.infer_bareword(s))
            }
            _ => bail!("Unexpected token parsing value: {:?}", self.current),
        }
    }

    fn parse_object(&mut self) -> Result<Value> {
        self.advance(); // consume '('
        let mut obj = IndexMap::new();
        while self.current != Token::RParen {
            let (k, v) = self.parse_pair()?;
            Value::insert_object_key(&mut obj, k, v).map_err(anyhow::Error::msg)?;
            if self.current == Token::Comma {
                self.advance();
            } else {
                break;
            }
        }
        if self.current != Token::RParen { bail!("Expected ')'"); }
        self.advance();
        Ok(Value::Object(obj))
    }

    fn parse_explicit_array(&mut self) -> Result<Value> {
        self.advance(); // consume '['
        let mut arr = Vec::new();
        while self.current != Token::RBracket {
            arr.push(self.parse_single_value()?);
            if self.current == Token::Comma {
                self.advance();
            } else {
                break;
            }
        }
        if self.current != Token::RBracket { bail!("Expected ']'"); }
        self.advance();
        Ok(Value::Array(arr))
    }

    fn infer_bareword(&self, s: String) -> Value {
        if s == "true" { return Value::Bool(true); }
        if s == "false" { return Value::Bool(false); }
        if s == "null" { return Value::Null; }
        if let Ok(i) = s.parse::<i64>() { return Value::Int(i); }
        if let Ok(f) = s.parse::<f64>() {
            // Must contain exactly one dot to be a float. (Prevents 0.1.0 parsing as float logic bugs)
            if s.chars().filter(|&c| c == '.').count() == 1 {
                return Value::Float(f);
            }
        }
        Value::String(s) // Fallback for 0.1.0, words, etc.
    }
}