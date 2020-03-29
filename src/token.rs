#[derive(PartialEq, Debug)]
pub enum TokenType {
    Illegal,
    EOF,

    Identifier,
    Int,

    Comma,
    Semicolon,

    LParen,
    RParen,
    LBrace,
    RBrace,

    // Operations
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,

    LT,
    GT,

    Eq,
    Not_Eq,

    // Keywords
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

#[derive(PartialEq, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: Vec<char>,
}

impl Token {
    pub fn from_string(token_type: TokenType, literal: String) -> Token {
        Token::new(token_type, literal.chars().collect())
    }

    pub fn from_char(token_type: TokenType, literal: char) -> Token {
        Token::new(token_type, vec![literal])
    }

    pub fn new(token_type: TokenType, literal: Vec<char>) -> Token {
        Token {
            token_type: token_type,
            literal: literal,
        }
    }

    pub fn illegal() -> Token {
        Token::new(TokenType::Illegal, vec!['\0'])
    }

    pub fn eof() -> Token {
        Token::new(TokenType::EOF, vec!['\n'])
    }

    pub fn identifier(literal: Vec<char>) -> Token {
        Token {
            token_type: TokenType::Identifier,
            literal: literal,
        }
    }

    pub fn lookup_identifier(identifier: &Vec<char>) -> TokenType {
        match identifier[..] {
            ['f', 'n'] => TokenType::Function,
            ['l', 'e', 't'] => TokenType::Let,
            ['t', 'r', 'u', 'e'] => TokenType::True,
            ['f', 'a', 'l', 's', 'e'] => TokenType::False,
            ['i', 'f'] => TokenType::If,
            ['e', 'l', 's', 'e'] => TokenType::Else,
            ['r', 'e', 't', 'u', 'r', 'n'] => TokenType::Return,
            _ => TokenType::Identifier,
        }
    }
}
