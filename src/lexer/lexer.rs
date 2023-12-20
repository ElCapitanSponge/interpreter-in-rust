use anyhow::Result;
use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Ident(String),
    Int(String),
    Illegal,
    Eof,
    Assign,
    Bang,
    Dash,
    ForwardSlash,
    Asterisk,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    Plus,
    Comma,
    Semicolon,
    Lparen,
    Rparen,
    LSquirly,
    RSquirly,
    Function,
    Let,
    If,
    Else,
    Return,
    True,
    False,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            TokenType::Ident(x) => write!(f, "Ident({})", x),
            TokenType::Int(x) => write!(f, "Int({})", x),
            TokenType::Illegal => write!(f, "Illegal"),
            TokenType::Eof => write!(f, "Eof"),
            TokenType::Assign => write!(f, "Assign"),
            TokenType::Bang => write!(f, "Bang"),
            TokenType::Dash => write!(f, "Dash"),
            TokenType::ForwardSlash => write!(f, "ForwardSlash"),
            TokenType::Asterisk => write!(f, "Asterisk"),
            TokenType::Equal => write!(f, "Equal"),
            TokenType::NotEqual => write!(f, "NotEqual"),
            TokenType::LessThan => write!(f, "LessThan"),
            TokenType::GreaterThan => write!(f, "GreaterThan"),
            TokenType::Plus => write!(f, "Plus"),
            TokenType::Comma => write!(f, "Comma"),
            TokenType::Semicolon => write!(f, "Semicolon"),
            TokenType::Lparen => write!(f, "Lparen"),
            TokenType::Rparen => write!(f, "Rparen"),
            TokenType::LSquirly => write!(f, "LSquirly"),
            TokenType::RSquirly => write!(f, "RSquirly"),
            TokenType::Function => write!(f, "Function"),
            TokenType::Let => write!(f, "Let"),
            TokenType::If => write!(f, "If"),
            TokenType::Else => write!(f, "Else"),
            TokenType::Return => write!(f, "Return"),
            TokenType::True => write!(f, "True"),
            TokenType::False => write!(f, "False"),
        };
    }
}

pub struct Lexer {
    input: Vec<u8>,
    position: usize,
    read_position: usize,
    ch: u8,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut lexer = Lexer {
            input: input.into_bytes(),
            position: 0,
            read_position: 0,
            ch: 0,
        };

        lexer.read_char();

        lexer
    }

    pub fn read_char(&mut self) {
        if self.read_position < self.input.len() {
            self.ch = self.input[self.read_position];
        } else {
            self.ch = 0;
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Result<TokenType> {
        self.skip_whitespace();

        let tok = match self.ch {
            b'{' => TokenType::LSquirly,
            b'}' => TokenType::RSquirly,
            b'(' => TokenType::Lparen,
            b')' => TokenType::Rparen,
            b',' => TokenType::Comma,
            b';' => TokenType::Semicolon,
            b'+' => TokenType::Plus,
            b'-' => TokenType::Dash,
            b'!' => {
                if self.peek() == b'=' {
                    self.read_char();
                    TokenType::NotEqual
                } else {
                    TokenType::Bang
                }
            },
            b'>' => TokenType::GreaterThan,
            b'<' => TokenType::LessThan,
            b'*' => TokenType::Asterisk,
            b'/' => TokenType::ForwardSlash,
            b'=' => {
                if self.peek() == b'=' {
                    self.read_char();
                    TokenType::Equal
                } else {
                    TokenType::Assign
                }
            },
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let ident = self.read_ident();
                return Ok(match ident.as_str() {
                    "fn" => TokenType::Function,
                    "let" => TokenType::Let,
                    "if" => TokenType::If,
                    "false" => TokenType::False,
                    "true" => TokenType::True,
                    "return" => TokenType::Return,
                    "else" => TokenType::Else,
                    _ => TokenType::Ident(ident),
                });
            },
            b'0'..=b'9' => return Ok(TokenType::Int(self.read_int())),
            0 => TokenType::Eof,
            _ => unreachable!("no monkey program should contain these characters and you should feel bad about yourself")
        };

        self.read_char();
        Ok(tok)
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }

    fn peek(&self) -> u8 {
        if self.read_position >= self.input.len() {
            return 0;
        } else {
            return self.input[self.read_position];
        }
    }

    fn read_ident(&mut self) -> String {
        let pos = self.position;
        while self.ch.is_ascii_alphabetic() || self.ch == b'_' {
            self.read_char();
        }

        return String::from_utf8_lossy(&self.input[pos..self.position]).to_string();
    }

    fn read_int(&mut self) -> String {
        let pos = self.position;
        while self.ch.is_ascii_digit() {
            self.read_char();
        }

        return String::from_utf8_lossy(&self.input[pos..self.position]).to_string();
    }
}

#[cfg(test)]
mod test {
    use anyhow::Result;

    use super::{Lexer, TokenType};

    #[test]
    fn test_lexer() -> Result<()> {
        let mut lexer = Lexer::new(String::from("1234567890"));
        let mut next_token = lexer.next_token()
            .map(|t| t)
            .map_err(|err| err.into());
        assert_eq!(
            TokenType::Int(String::from("1234567890")),
            next_token
        );
        assert_eq!(TokenType::Eof, next_token);
        Ok(())
    }
}
