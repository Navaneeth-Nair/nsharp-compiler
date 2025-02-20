use std::dmt::(Display, Formatter);

use crate::text::span::TextSpan;

pub enum TokenKind{
    Number(i64),
    Plus,
    Minus,
    Asterisk,
    Slash,
    Ampersand,
    Equals,
    Pipe,
    Caret,
    DoubleAsterisk,
    Tilde,
    Whitespace,
    DoubleEquals,
    LessThan,
    LessThanEquals,
    GreaterThan,
    GreaterThanEquals,
    Bang,
    BangEquals,
    Identifier,
    String,
    OpenParenthesis,
    CloseParenthesis,
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
    Comma,
    Colon,
    Semicolon,
    Arrow,
    FatArrow,
    True,
    False,
    Let,
    If,
    Else,
    While,
    For,
    In,
    Function,
    Return,
    Break,
    Continue,
    Null,
    Eof,
    Error
}

impl Display for TokenKind{
    fn fmt(&self, f: &mut Formatter<'_>)->Result<(), Error>{
        match self{
            TokenKind::Number(value)=>write!(f, "Number({})", value),
            TokenKind::Plus=>write!(f, "Plus"),
            TokenKind::Minus=>write!(f, "Minus"),
            TokenKind::Asterisk=>write!(f, "Asterisk"),
            TokenKind::Slash=>write!(f, "Slash"),
            TokenKind::Ampersand=>write!(f, "Ampersand"),
            TokenKind::Equals=>write!(f, "Equals"),
            TokenKind::Pipe=>write!(f, "Pipe"),
            TokenKind::Caret=>write!(f, "Caret"),
            TokenKind::DoubleAsterisk=>write!(f, "DoubleAsterisk"),
            TokenKind::Tilde=>write!(f, "Tilde"),
            TokenKind::Whitespace=>write!(f, "Whitespace"),
            TokenKind::DoubleEquals=>write!(f, "DoubleEquals"),
            TokenKind::LessThan=>write!(f, "LessThan"),
            TokenKind::LessThanEquals=>write!(f, "LessThanEquals"),
            TokenKind::GreaterThan=>write!(f, "GreaterThan"),
            TokenKind::GreaterThanEquals=>write!(f, "GreaterThanEquals"),
            TokenKind::Bang=>write!(f, "Bang"),
            TokenKind::BangEquals=>write!(f, "BangEquals"),
            TokenKind::Identifier=>write!(f, "Identifier"),
            TokenKind::String=>write!(f, "String"),
            TokenKind::OpenParenthesis=>write!(f, "OpenParenthesis"),
            TokenKind::CloseParenthesis=>write!(f, "CloseParenthesis"),
            TokenKind::OpenBrace=>write!(f, "OpenBrace"),
            TokenKind::CloseBrace=>write!(f, "CloseBrace"),
            TokenKind::OpenBracket=>write!(f, "OpenBracket"),
            TokenKind::CloseBracket=>write!(f, "CloseBracket"),
            TokenKind::Comma=>write!(f, "Comma"),
            TokenKind::Colon=>write!(f, "Colon"),
            TokenKind::Semicolon=>write!(f, "Semicolon"),
            TokenKind::Arrow=>write!(f, "Arrow"),
            TokenKind::FatArrow=>write!(f, "FatArrow"),
            TokenKind::True=>write!(f, "True"),
            TokenKind::False=>write!(f, "False"),
            TokenKind::Let=>write!(f, "Let"),
            TokenKind::If=>write!(f, "If"),
            TokenKind::Else=>write!(f, "Else"),
            TokenKind::While=>write!(f, "While"),
            TokenKind::For=>write!(f, "For"),
            TokenKind::In=>write!(f, "In"),
            TokenKind::Function=>write!(f, "Function"),
            TokenKind::Return=>write!(f, "Return"),
            TokenKind::Break=>write!(f, "Break"),
            TokenKind::Continue=>write!(f, "Continue"),
            TokenKind::Null=>write!(f, "Null"),
            TokenKind::Eof=>write!(f, "Eof"),
            TokenKind::Error=>write!(f, "Error"),
        }
    }
}

pub struct Token{
    pub kind: TokenKind,
    pub span: TextSpan,
}

impl Token{
    pub fn new(kind: TokenKind, span: TextSpan)->Self{
        Self{kind, span}
    }
}

pub struct Lexer<'a>{
    input: &'a String,
    position: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            current_pos: 0,
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        if self.current_pos == self.input.len() {
            let eof_char: char = '\0';
            self.current_pos += 1;
            return Some(Token::new(
                TokenKind::Eof,
                TextSpan::new(0, 0, eof_char.to_string()),
            ));
        }
        let c = self.current_char();
        return c.map(|c| {
            let start = self.current_pos;
            let mut kind = TokenKind::Bad;
            if Self::is_number_start(&c) {
                let number: i64 = self.consume_number();
                kind = TokenKind::Number(number);
            } else if Self::is_whitespace(&c) {
                self.consume();
                kind = TokenKind::Whitespace;
            } else if Self::is_identifier_start(&c) {
                let identifier = self.consume_identifier();
                kind = match identifier.as_str() {
                    "let" => TokenKind::Let,
                    "if" => TokenKind::If,
                    "else" => TokenKind::Else,
                    "true" => TokenKind::True,
                    "false" => TokenKind::False,
                    "while" => TokenKind::While,
                    "func" => TokenKind::Func,
                    "return" => TokenKind::Return,
                    _ => TokenKind::Identifier,
                }
            } else {
                kind = self.consume_punctuation();
            }

            let end = self.current_pos;
            let literal = self.input[start..end].to_string();
            let span = TextSpan::new(start, end, literal);
            Token::new(kind, span)
        });
    }

    fn consume_punctuation(&mut self) -> TokenKind {
        let c = self.consume().unwrap();
        match c {
            '+' => TokenKind::Plus,
            '-' => self.lex_potential_double_char_operator('>', TokenKind::Minus, TokenKind::Arrow),
            '*' => self.lex_potential_double_char_operator(
                '*',
                TokenKind::Asterisk,
                TokenKind::DoubleAsterisk,
            ),
            '%' => TokenKind::Percent,
            '/' => TokenKind::Slash,
            '(' => TokenKind::LeftParen,
            ')' => TokenKind::RightParen,
            '=' => self.lex_potential_double_char_operator(
                '=',
                TokenKind::Equals,
                TokenKind::EqualsEquals,
            ),
            '&' => TokenKind::Ampersand,
            '|' => TokenKind::Pipe,
            '^' => TokenKind::Caret,
            '~' => TokenKind::Tilde,
            '>' => self.lex_potential_double_char_operator(
                '=',
                TokenKind::GreaterThan,
                TokenKind::GreaterThanEquals,
            ),
            '<' => self.lex_potential_double_char_operator(
                '=',
                TokenKind::LessThan,
                TokenKind::LessThanEquals,
            ),
            '!' => {
                self.lex_potential_double_char_operator('=', TokenKind::Bad, TokenKind::BangEquals)
            }
            '{' => TokenKind::OpenBrace,
            '}' => TokenKind::CloseBrace,
            ',' => TokenKind::Comma,
            ':' => TokenKind::Colon,
            ';' => TokenKind::SemiColon,

            _ => TokenKind::Bad,
        }
    }

    fn lex_potential_double_char_operator(
        &mut self,
        expected: char,
        one_char_kind: TokenKind,
        double_char_kind: TokenKind,
    ) -> TokenKind {
        if let Some(next) = self.current_char() {
            if next == expected {
                self.consume();
                double_char_kind
            } else {
                one_char_kind
            }
        } else {
            one_char_kind
        }
    }

    fn is_number_start(c: &char) -> bool {
        c.is_digit(10)
    }

    fn is_identifier_start(c: &char) -> bool {
        c.is_alphabetic() || c == &'_'
    }

    fn is_whitespace(c: &char) -> bool {
        c.is_whitespace()
    }

    fn current_char(&self) -> Option<char> {
        self.input.chars().nth(self.current_pos)
    }

    fn consume(&mut self) -> Option<char> {
        if self.current_pos >= self.input.len() {
            return None;
        }
        let c = self.current_char();
        self.current_pos += 1;

        c
    }

    fn consume_identifier(&mut self) -> String {
        let mut identifier = String::new();
        while let Some(c) = self.current_char() {
            if Self::is_identifier_start(&c) {
                self.consume().unwrap();
                identifier.push(c);
            } else {
                break;
            }
        }
        identifier
    }

    fn consume_number(&mut self) -> i64 {
        let mut number: i64 = 0;
        while let Some(c) = self.current_char() {
            if c.is_digit(10) {
                self.consume().unwrap();
                number = number * 10 + c.to_digit(10).unwrap() as i64;
            } else {
                break;
            }
        }
        number
    }
}