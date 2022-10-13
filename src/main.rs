pub mod parser;

use std::{ops::Add, time::Instant};

use parser::{
    parser::{Node, Operator},
    token::Token,
};

use crate::parser::{lexer::lex, parser::Parser};

fn evaluate_r(node: Node) -> f64 {
    match node {
        Node::Binary(op, left, right) => match op {
            Operator::Mult => evaluate_r(*left) * evaluate_r(*right),
            Operator::Div => evaluate_r(*left) / evaluate_r(*right),
            Operator::Sub => evaluate_r(*left) - evaluate_r(*right),
            Operator::Add => evaluate_r(*left) + evaluate_r(*right),
        },
        Node::Value(x) => x,
    }
}

fn main() {
    let t0 = Instant::now();
    let tokens = lex("344 * 13 + 103 + 432 - 343 / 4342");
    println!("time taken to lex : {:?}", Instant::now() - t0);
    println!("tokens : {:?}", tokens);

    let mut parser = Parser::new(tokens);
    let t0 = Instant::now();
    let ast = parser.parse();
    let t1 = Instant::now();
    println!("Time to parse {:?}", t1 - t0);
    println!("ast : {:#?}", ast);
    println!("evaluate : {}", evaluate_r(ast));
}
