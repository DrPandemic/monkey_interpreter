use crate::token::*;
use std::fs;

#[derive(Debug)]
pub struct Lexer {
    input: Vec<char>,
    position: usize,
    read_position: usize,
    current_char: char,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut lexer = Lexer {
            input: input.chars().collect(),
            position: 0,
            read_position: 0,
            current_char: '\0',
        };
        lexer.read_char();
        lexer
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespances();

        let token = match self.current_char {
            '\0' => Token::eof(),
            ',' => Token::from_char(TokenType::Comma, self.current_char),
            ';' => Token::from_char(TokenType::Semicolon, self.current_char),
            '(' => Token::from_char(TokenType::LParen, self.current_char),
            ')' => Token::from_char(TokenType::RParen, self.current_char),
            '{' => Token::from_char(TokenType::LBrace, self.current_char),
            '}' => Token::from_char(TokenType::RBrace, self.current_char),
            '+' => Token::from_char(TokenType::Plus, self.current_char),
            '-' => Token::from_char(TokenType::Minus, self.current_char),
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::from_string(TokenType::Eq, String::from("=="))
                } else {
                    Token::from_char(TokenType::Assign, self.current_char)
                }
            }
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::from_string(TokenType::Not_Eq, String::from("!="))
                } else {
                    Token::from_char(TokenType::Bang, self.current_char)
                }
            }
            '*' => Token::from_char(TokenType::Asterisk, self.current_char),
            '/' => Token::from_char(TokenType::Slash, self.current_char),
            '<' => Token::from_char(TokenType::LT, self.current_char),
            '>' => Token::from_char(TokenType::GT, self.current_char),
            _ => {
                if is_letter(&self.current_char) {
                    let identifier = self.read_identifier();
                    return Token::new(Token::lookup_identifier(&identifier), identifier);
                } else if is_digit(&self.current_char) {
                    return Token::new(TokenType::Int, self.read_number());
                } else {
                    Token::illegal()
                }
            }
        };

        self.read_char();
        token
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.current_char = '\0';
        } else {
            self.current_char = self.input[self.read_position];
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn read_identifier(&mut self) -> Vec<char> {
        let initial_position = self.position;
        while is_letter(&self.current_char) {
            self.read_char();
        }

        self.input[initial_position..self.position].to_vec()
    }

    fn read_number(&mut self) -> Vec<char> {
        let initial_position = self.position;
        while is_digit(&self.current_char) {
            self.read_char();
        }

        self.input[initial_position..self.position].to_vec()
    }

    fn skip_whitespances(&mut self) {
        while self.current_char == ' '
            || self.current_char == '\t'
            || self.current_char == '\n'
            || self.current_char == '\r'
        {
            self.read_char();
        }
    }

    fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            '\0'
        } else {
            self.input[self.read_position]
        }
    }
}

fn is_letter(ch: &char) -> bool {
    &'a' <= ch && ch <= &'z' || &'A' <= ch && ch <= &'Z' || ch == &'_'
}

fn is_digit(ch: &char) -> bool {
    &'0' <= ch && ch <= &'9'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let input = fs::read_to_string("./examples/add.monkey").unwrap();

        let expected = [
            Token::from_string(TokenType::Let, String::from("let")),
            Token::from_string(TokenType::Identifier, String::from("five")),
            Token::from_char(TokenType::Assign, '='),
            Token::from_string(TokenType::Int, String::from("5")),
            Token::from_char(TokenType::Semicolon, ';'),
            Token::from_string(TokenType::Let, String::from("let")),
            Token::from_string(TokenType::Identifier, String::from("ten")),
            Token::from_char(TokenType::Assign, '='),
            Token::from_string(TokenType::Int, String::from("10")),
            Token::from_char(TokenType::Semicolon, ';'),
            Token::from_string(TokenType::Let, String::from("let")),
            Token::from_string(TokenType::Identifier, String::from("add")),
            Token::from_char(TokenType::Assign, '='),
            Token::from_string(TokenType::Function, String::from("fn")),
            Token::from_char(TokenType::LParen, '('),
            Token::from_string(TokenType::Identifier, String::from("x")),
            Token::from_char(TokenType::Comma, ','),
            Token::from_string(TokenType::Identifier, String::from("y")),
            Token::from_char(TokenType::RParen, ')'),
            Token::from_char(TokenType::LBrace, '{'),
            Token::from_string(TokenType::Identifier, String::from("x")),
            Token::from_char(TokenType::Plus, '+'),
            Token::from_string(TokenType::Identifier, String::from("y")),
            Token::from_char(TokenType::Semicolon, ';'),
            Token::from_char(TokenType::RBrace, '}'),
            Token::from_char(TokenType::Semicolon, ';'),
            Token::from_string(TokenType::Let, String::from("let")),
            Token::from_string(TokenType::Identifier, String::from("result")),
            Token::from_char(TokenType::Assign, '='),
            Token::from_string(TokenType::Identifier, String::from("add")),
            Token::from_char(TokenType::LParen, '('),
            Token::from_string(TokenType::Identifier, String::from("five")),
            Token::from_char(TokenType::Comma, ','),
            Token::from_string(TokenType::Identifier, String::from("ten")),
            Token::from_char(TokenType::RParen, ')'),
            Token::from_char(TokenType::Semicolon, ';'),
            Token::from_char(TokenType::Bang, '!'),
            Token::from_char(TokenType::Minus, '-'),
            Token::from_char(TokenType::Slash, '/'),
            Token::from_char(TokenType::Asterisk, '*'),
            Token::from_char(TokenType::Int, '5'),
            Token::from_char(TokenType::Semicolon, ';'),
            Token::from_char(TokenType::Int, '5'),
            Token::from_char(TokenType::LT, '<'),
            Token::from_string(TokenType::Int, String::from("10")),
            Token::from_char(TokenType::GT, '>'),
            Token::from_char(TokenType::Int, '5'),
            Token::from_char(TokenType::Semicolon, ';'),
            Token::from_string(TokenType::If, String::from("if")),
            Token::from_char(TokenType::LParen, '('),
            Token::from_char(TokenType::Int, '5'),
            Token::from_char(TokenType::LT, '<'),
            Token::from_string(TokenType::Int, String::from("10")),
            Token::from_char(TokenType::RParen, ')'),
            Token::from_char(TokenType::LBrace, '{'),
            Token::from_string(TokenType::Return, String::from("return")),
            Token::from_string(TokenType::True, String::from("true")),
            Token::from_char(TokenType::Semicolon, ';'),
            Token::from_char(TokenType::RBrace, '}'),
            Token::from_string(TokenType::Else, String::from("else")),
            Token::from_char(TokenType::LBrace, '{'),
            Token::from_string(TokenType::Return, String::from("return")),
            Token::from_string(TokenType::False, String::from("false")),
            Token::from_char(TokenType::Semicolon, ';'),
            Token::from_char(TokenType::RBrace, '}'),
            Token::from_string(TokenType::Int, String::from("10")),
            Token::from_string(TokenType::Eq, String::from("==")),
            Token::from_string(TokenType::Int, String::from("10")),
            Token::from_char(TokenType::Semicolon, ';'),
            Token::from_string(TokenType::Int, String::from("10")),
            Token::from_string(TokenType::Not_Eq, String::from("!=")),
            Token::from_char(TokenType::Int, '9'),
            Token::from_char(TokenType::Semicolon, ';'),
            Token::eof(),
        ];

        let mut lexer = Lexer::new(input);

        for (i, tt) in expected.iter().enumerate() {
            let token = lexer.next_token();
            assert_eq!(tt, &token, "At position {}", i)
        }
    }
}
