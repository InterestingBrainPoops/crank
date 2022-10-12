use std::{ops::Add, time::Instant};

#[derive(Debug)]
enum Node {
    Expression(Operator, Box<Node>, Box<Node>),
    Value(u32),
}

#[derive(Debug)]
enum Operator {
    Mult,
    Div,
    Sub,
    Add,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Token {
    OpenParen,
    CloseParen,
    ForwardSlash,
    Asterisk,
    Plus,
    Hyphen,
    Number(u32),
}

impl Token {
    fn precedence(&self) -> u32 {
        match self {
            Token::ForwardSlash => 3,
            Token::Asterisk => 3,
            Token::Plus => 2,
            Token::Hyphen => 2,
            c => 0,
        }
    }

    fn operator(&self) -> Operator {
        match self {
            Token::ForwardSlash => Operator::Div,
            Token::Asterisk => Operator::Mult,
            Token::Plus => Operator::Add,
            Token::Hyphen => Operator::Sub,
            _ => panic!("Attempted to do something unreasonable"),
        }
    }
}

fn tokenize(input: &str) -> Vec<Token> {
    let mut out = vec![];
    let mut cursor = 0;
    while cursor < input.len() {
        match input.chars().nth(cursor) {
            None => {}
            Some(x) => match x {
                '/' => out.push(Token::ForwardSlash),
                '*' => out.push(Token::Asterisk),
                '+' => out.push(Token::Plus),
                '-' => out.push(Token::Hyphen),
                '(' => out.push(Token::OpenParen),
                ')' => out.push(Token::CloseParen),
                ' ' => {}
                _ => match x.to_digit(10) {
                    Some(y) => match out.last_mut() {
                        Some(Token::Number(v)) => {
                            println!("{}", v);
                            *v = ((*v) * 10) + y;
                        }
                        _ => {
                            out.push(Token::Number(y));
                        }
                    },
                    None => {
                        panic!("Invalid character {}", x);
                    }
                },
            },
        }
        cursor += 1;
    }
    out
}

fn parse(tokens: &Vec<Token>) -> Vec<Token> {
    let mut out = vec![];
    let mut op_stack = vec![];
    for x in tokens {
        match x {
            Token::Number(_) => out.push(*x),
            Token::OpenParen => op_stack.push(*x),
            Token::CloseParen => {
                while *op_stack.last().unwrap() != Token::OpenParen {
                    out.push(op_stack.pop().unwrap());
                }
                op_stack.pop()
            }
            _ => {
                if op_stack.is_empty() {
                    op_stack.push(*x);
                    continue;
                }

                let last = op_stack.last().unwrap();

                if last.precedence() < x.precedence() {
                    op_stack.push(*x);
                } else {
                    while !op_stack.is_empty()
                        && op_stack.last().unwrap().precedence() >= x.precedence()
                    {
                        out.push(op_stack.pop().unwrap());
                    }
                    op_stack.push(*x);
                }
            }
        }
    }
    out.append(&mut op_stack);
    out
}

fn evaluate(tokens: &Vec<Token>) -> u32 {
    let mut stack = vec![];
    for x in tokens {
        println!("{stack:?}");
        match x {
            Token::Number(num) => stack.push(*num),
            _ => {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                match x {
                    Token::ForwardSlash => stack.push(left / right),
                    Token::Asterisk => stack.push(left * right),
                    Token::Plus => stack.push(left + right),
                    Token::Hyphen => stack.push(left - right),
                    _ => {}
                }
            }
        }
    }
    stack.pop().unwrap()
}

fn parse_ast(tokens: &mut Vec<Token>) -> Node {
    match tokens.pop().unwrap() {
        Token::Number(x) => Node::Value(x),
        z => {
            let right = parse_ast(tokens);
            let left = parse_ast(tokens);
            Node::Expression(z.operator(), Box::new(left), Box::new(right))
        }
    }
}

fn main() {
    let t0 = Instant::now();
    let tokens = tokenize("5+5- (3 + 4)");
    println!("time taken to tokenize : {:?}", Instant::now() - t0);
    println!("tokens : {:?}", tokens);
    let parsed = parse(&tokens);
    println!("parsed: {:?}", parsed);
    let parse_ast = parse_ast(&mut parsed.clone());
    println!("parsed_ast: {parse_ast:?}");
    println!("evaluate : {}", evaluate(&parsed));
}
