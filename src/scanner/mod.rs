use crate::error;
use crate::scanner::token::Token;
use crate::scanner::token_type::TokenType;

mod token;
mod token_type;

pub struct Scanner {
    pub source: String,
    pub tokens: Vec<Token>,
    pub start: u32,
    pub current: u32,
    pub line: u32,
    pub _had_error: bool,
}

impl Scanner {
    pub fn new(source_code: &str, mut _had_error: &mut bool) -> Scanner {
        Scanner {
            source: String::from(source_code),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
            _had_error: *_had_error,
        }
    }

    pub fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        let mut eof_token: Vec<Token> = vec![Token::new(TokenType::EOF, "", "null", self.line)];
        self.tokens.append(&mut eof_token);
    }
    fn scan_token(&mut self) {
        let c: char = self.advance();
        match c {
            '(' => self.add_token_single(TokenType::LEFT_PAREN),
            ')' => self.add_token_single(TokenType::RIGHT_PAREN),
            '{' => self.add_token_single(TokenType::LEFT_BRACE),
            '}' => self.add_token_single(TokenType::RIGHT_BRACE),
            ',' => self.add_token_single(TokenType::COMMA),
            '.' => self.add_token_single(TokenType::DOT),
            '-' => self.add_token_single(TokenType::MINUS),
            '+' => self.add_token_single(TokenType::PLUS),
            ';' => self.add_token_single(TokenType::SEMICOLON),
            '*' => self.add_token_single(TokenType::STAR),
            '!' => self.add_token_expect_double(TokenType::BANG),
            '=' => self.add_token_expect_double(TokenType::EQUAL),
            '<' => self.add_token_expect_double(TokenType::LESS),
            '>' => self.add_token_expect_double(TokenType::GREATER),
            '/' => self.add_token_expect_double(TokenType::SLASH),
            // Whitespace
            ' ' => (),
            '\r' => (),
            '\t' => (),
            '\n' => self.line += 1,
            '"' => self.handle_string(),

            _ => self.handle_non_single_char(c),
        }
    }
    fn handle_non_single_char(&mut self, c: char) {
        if self.is_digit(c) {
            self.handle_number();
        } else if self.is_alpha(c) {
            self.identifier();
        } else {
            error(self.line, "Unexpected character.", &mut self._had_error);
        }
    }
    fn handle_string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        // Unterminated string
        if self.is_at_end() {
            error(self.line, "Unterminated string.", &mut self._had_error);
            return;
        }
        // The closing "
        self.advance();
        let s = (self.start + 1) as usize;
        let c = (self.current - 1) as usize;
        let value = String::from(&self.source[s..c]);
        self.add_token(TokenType::STRING, &value);
    }

    fn handle_number(&mut self) {
        let mut p = self.peek();
        while self.is_digit(p) {
            self.advance();
            p = self.peek();
        }
        let peek_next = self.peek_next();
        if p == '.' && self.is_digit(peek_next) {
            self.advance();
            while self.is_digit(p) {
                self.advance();
            }
        }
        let s = self.start as usize;
        let c = self.current as usize;
        let value = String::from(&self.source[s..c]);
        self.add_token(TokenType::NUMBER, &value);
    }

    fn identifier(&mut self) {
        let mut p: char = self.peek();
        while self.is_alphanum(p) {
            self.advance();
            p = self.peek();
        }
        let s = self.start as usize;
        let c = self.current as usize;
        let text = &self.source[s..c];
        match text {
            "and" => self.add_token_single(TokenType::AND),
            "class" => self.add_token_single(TokenType::CLASS),
            "else" => self.add_token_single(TokenType::ELSE),
            "false" => self.add_token_single(TokenType::FALSE),
            "fun" => self.add_token_single(TokenType::FUN),
            "for" => self.add_token_single(TokenType::FOR),
            "if" => self.add_token_single(TokenType::IF),
            "nil" => self.add_token_single(TokenType::NIL),
            "or" => self.add_token_single(TokenType::OR),
            "print" => self.add_token_single(TokenType::PRINT),
            "return" => self.add_token_single(TokenType::RETURN),
            "super" => self.add_token_single(TokenType::SUPER),
            "this" => self.add_token_single(TokenType::THIS),
            "true" => self.add_token_single(TokenType::TRUE),
            "var" => self.add_token_single(TokenType::VAR),
            "while" => self.add_token_single(TokenType::WHILE),

            _ => self.add_token_single(TokenType::IDENTIFIER),
        }
    }

    fn add_token(&mut self, ki: TokenType, lit: &str) {
        let s = self.start as usize;
        let c = self.current as usize;
        let text = String::from(&self.source[s..c]);
        let t = Token::new(ki, &text, lit, self.line);
        self.tokens.append(&mut vec![t]);
    }
    fn add_token_single(&mut self, ki: TokenType) {
        self.add_token(ki, "null");
    }
    fn add_token_expect_double(&mut self, arg: TokenType) {
        if self.match_op('/') {
            // Comment goes until end of line.
            while self.peek() != '\n' && self.is_at_end() {
                self.advance();
            }
        } else {
            self.add_token_single(TokenType::SLASH);
        }
        if self.match_op('=') {
            match arg {
                TokenType::BANG => self.add_token_single(TokenType::BANG_EQUAL),
                TokenType::EQUAL => self.add_token_single(TokenType::EQUAL_EQUAL),
                TokenType::LESS => self.add_token_single(TokenType::LESS_EQUAL),
                TokenType::GREATER => self.add_token_single(TokenType::GREATER_EQUAL),
                _ => error(self.line, "Unexpected character.", &mut self._had_error),
            }
        } else {
            match arg {
                TokenType::BANG => self.add_token_single(arg),
                TokenType::EQUAL => self.add_token_single(arg),
                TokenType::LESS => self.add_token_single(arg),
                TokenType::GREATER => self.add_token_single(arg),
                _ => error(self.line, "Unexpected character.", &mut self._had_error),
            }
        }
    }
    fn match_op(&mut self, expected: char) -> bool {
        let e: bool;
        let get_char = self.source_char_at(self.current as usize);
        match get_char {
            None => e = false,
            Some(ch) => e = ch != expected,
        }

        if self.is_at_end() || e {
            false
        } else {
            self.current += 1;
            true
        }
    }
    fn advance(&mut self) -> char {
        self.current += 1;
        let get_char = self.source_char_at((self.current - 1) as usize);
        match get_char {
            None => '\0',
            Some(ch) => ch,
        }
    }

    fn peek(&mut self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            let get_char = self.source_char_at(self.current as usize);
            match get_char {
                None => '\0',
                Some(ch) => ch,
            }
            // self.source_char_at(self.current as usize)
        }
    }
    fn peek_next(&mut self) -> char {
        let source_length = self.source.chars().count();
        let get_char = self.source_char_at((self.current) as usize);
        if (self.current) as usize >= source_length {
            '\0'
        } else {
            // self.source_char_at((self.current + 1) as usize)
            match get_char {
                None => '\0',
                Some(ch) => ch,
            }
        }
    }
    fn source_char_at(&mut self, idx: usize) -> Option<char> {
        self.source.chars().nth(idx)
    }
    fn is_at_end(&self) -> bool {
        self.current >= self.source.chars().count() as u32
    }

    fn is_alpha(&mut self, c: char) -> bool {
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
    }
    fn is_alphanum(&mut self, c: char) -> bool {
        self.is_alpha(c) || self.is_digit(c)
    }
    fn is_digit(&mut self, c: char) -> bool {
        c >= '0' && c <= '9'
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn identifier_ok() {
        // instantiate scanner
        let mut _had_error = false;
        let src = String::from("class");
        let mut scnr = Scanner::new(&src, &mut _had_error);

        let mut p: char = scnr.peek();
        while scnr.is_alphanum(p) {
            scnr.advance();
            p = scnr.peek();
        }
        //
        let s = scnr.start as usize;
        let c = scnr.current as usize;
        let text = &scnr.source[s..c];
        match text {
            "and" => scnr.add_token_single(TokenType::AND),
            "class" => scnr.add_token_single(TokenType::CLASS),
            "else" => scnr.add_token_single(TokenType::ELSE),

            _ => scnr.add_token_single(TokenType::IDENTIFIER),
        }
        assert!(scnr.tokens.len() > 0);
    }

    #[test]
    fn slice_source() {
        let sourcey = String::from("apple");
        let s = 1 as usize;
        let c = 5 as usize;
        let text = &sourcey[s..c];
        assert!(text == String::from("pple"), "{}", text);

        let s = 1 as usize;
        let c = 3 as usize;
        let value = String::from(&sourcey[s..c]);
        assert!(value == String::from("pp"), "{}", value);
    }

    #[test]
    fn end_bool() {
        // instantiate scanner
        let mut _had_error = false;
        let src = String::from("fn main() {");
        let mut scnr = Scanner::new(&src, &mut _had_error);

        assert!(scnr.is_at_end() == false);
        while !scnr.is_at_end() {
            scnr.advance();
        }
        assert!(scnr.is_at_end() == true);
    }
    #[test]
    fn adding_token() {
        // instantiate scanner
        let mut _had_error = false;
        let src = String::from(
            "fn main() {
                nan42;
                _counter != i // description goes until end.
                2 * 2 is 4;
            }",
        );
        let mut scnr = Scanner::new(&src, &mut _had_error);

        let ki: TokenType = TokenType::NUMBER;
        let text = "42000";

        let t = Token::new(ki, text, text, scnr.line);
        scnr.tokens.append(&mut vec![t]);

        assert!(scnr.tokens[0].lexeme == text);
    }
    #[test]
    fn literate() {
        let mut _had_error = false;
        let src = String::from("THE_QUICK_BROWN_FOX_JUMPS_OVER_THE_LAZY_DOG___the_quick_brown_fox_jumps_over_the_lazy_dog___1234567890",
        );
        let mut scnr = Scanner::new(&src, &mut _had_error);
        let alphas: Vec<char> = vec![
            '_', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
            'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G',
            'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X',
            'Y', 'Z',
        ];
        let digits = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

        for c in &alphas {
            assert!(scnr.is_alpha(*c), "{} not recognized as alpha.", c);
        }
        for c in &digits {
            assert!(scnr.is_digit(*c), "{} not recognized as digit.", c);
        }
        for c in src.chars() {
            assert!(scnr.is_alphanum(c), "{} not recognized as alphanum.", c);
        }
    }
    #[test]
    fn e2e() {
        let mut _had_error = false;
        let src = String::from("9");
        let mut scnr = Scanner::new(&src, &mut _had_error);
        scnr.scan_tokens();

        for token in &scnr.tokens {
            print!("{}", token.line);
        }
    }
    #[test]
    fn handling_number() {
        // instantiate scanner
        let mut _had_error = false;
        let src = String::from(
            "fn main() {
                nan42;
                _counter != i // description goes until end.
                2 * 2 is 4;
            }",
        );
        let mut scnr = Scanner::new(&src, &mut _had_error);

        // assert peek advance
        let first_ch;
        match src.chars().nth(0) {
            Some(ch) => first_ch = ch,
            None => panic!("empty src."),
        }
        let mut p = scnr.peek();
        while scnr.is_digit(p) {
            assert_eq!(scnr.advance(), first_ch);
            p = scnr.peek();
        }

        // assert token is added
        let peek_next = scnr.peek_next();
        if p == '.' && scnr.is_digit(peek_next) {
            scnr.advance();
            while scnr.is_digit(p) {
                scnr.advance();
            }
        }

        assert!(scnr.tokens.len() == 0, "Tokens start empty.");

        let s = scnr.start as usize;
        let c = scnr.current as usize;
        let value = String::from(&scnr.source[s..c]);
        scnr.add_token(TokenType::NUMBER, &value);

        assert!(scnr.tokens.len() > 0, "Token not added.");
    }
}
