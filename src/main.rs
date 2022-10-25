pub mod parser;

use std::time::Instant;

use parser::{Node, Operator};

use crate::parser::{lexer::Lexer, Parser};

fn main() {
    let t0 = Instant::now();
    let mut lexer = Lexer::new();
    let input = "var x = 3 + 4 - 3 var x = 3 * 4";
    dbg!(input);
    let tokens = lexer.lex(input);
    println!("time taken to lex : {:?}", Instant::now() - t0);
    println!("tokens : {:?}", tokens);

    let mut parser = Parser::new(tokens);
    let t0 = Instant::now();
    let ast = parser.parse();
    let t1 = Instant::now();
    println!("Time to parse {:?}", t1 - t0);
    println!("ast : {:?}", ast);
    // println!("evaluate : {}", evaluate_r(ast));
}
