mod lexer;
mod repl;
mod token;
use std::io::*;

use crate::repl::*;

fn main() {
    println!("Hello, world!");
    start_repl(stdin());
}
