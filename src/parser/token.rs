use super::parser::Operator;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Token {
    OpenParen,
    CloseParen,
    ForwardSlash,
    Asterisk,
    Plus,
    Hyphen,
    Number(f64),
}

impl Token {
    pub fn precedence(&self) -> u32 {
        match self {
            Token::ForwardSlash => 3,
            Token::Asterisk => 3,
            Token::Plus => 2,
            Token::Hyphen => 2,
            c => 0,
        }
    }

    pub fn operator(&self) -> Operator {
        match self {
            Token::ForwardSlash => Operator::Div,
            Token::Asterisk => Operator::Mult,
            Token::Plus => Operator::Add,
            Token::Hyphen => Operator::Sub,
            _ => panic!("Attempted to do something unreasonable"),
        }
    }
}
