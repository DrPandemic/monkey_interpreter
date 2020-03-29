use crate::lexer::*;
use crate::token::*;
use std::io::*;

const PROMPT: &'static str = "> ";

pub fn start_repl(input: Stdin) {
    let mut sout = stdout();
    loop {
        print!("{}", PROMPT);
        sout.flush().unwrap();
        let mut line = String::new();
        match input.read_line(&mut line) {
            Ok(_) => {
                let mut lexer = Lexer::new(line);
                let mut token = lexer.next_token();
                while token.token_type != TokenType::EOF {
                    println!("{:?}", token);
                    token = lexer.next_token();
                }
            }
            _ => return,
        }
    }
}
