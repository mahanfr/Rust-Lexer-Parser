#![allow(dead_code)]
mod lexer;
mod parser;
mod tests;
use std::process::exit;
use lexer::Lexer;
use parser::program::Program;

fn main() {
    let mut lexer = Lexer::from_str("@hello = \"facts\";\nfun main(a b,c d) u32 {}");
    let program = Program::new(&mut lexer);
    println!("{:#?}",program);
}

