use crate::scanner::token_type::TokenType;

// #[derive(PartialEq)]
pub struct Token {
    pub kind: TokenType,
    pub lexeme: String,
    // string, number, or null literal
    pub literal: String, // s.parse::<f64>().is_ok()
    pub line: u32,
}

impl Token {
    pub fn new(ki: TokenType, lex: &str, lit: &str, ln: u32) -> Token {
        Token {
            kind: ki,
            lexeme: String::from(lex),
            literal: String::from(lit),
            line: ln,
        }
    }
    #[allow(dead_code)]
    pub fn to_string(self) -> String {
        String::from(format!(
            "{:?} {} {}",
            self.kind, &self.lexeme, &self.literal
        ))
    }
}
