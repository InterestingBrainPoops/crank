use super::Operator;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Equals,
    Var,
    Literal(String),
    OpenParen,
    CloseParen,
    ForwardSlash,
    Asterisk,
    Plus,
    Hyphen,
    Number(f64),
    Bang,
    NotEquals,
    EqualsEquals,
    GreEq,
    Gre,
    LesEq,
    Less,
}

impl Token {
    pub fn precedence(&self) -> u32 {
        match self {
            Token::ForwardSlash => 3,
            Token::Asterisk => 3,
            Token::Plus => 2,
            Token::Hyphen => 2,
            _ => 0,
        }
    }

    pub fn operator(&self) -> Operator {
        match self {
            Token::ForwardSlash => Operator::Div,
            Token::Asterisk => Operator::Mult,
            Token::Plus => Operator::Add,
            Token::Hyphen => Operator::Sub,
            Token::NotEquals => Operator::NotEquals,
            Token::EqualsEquals => Operator::EqualsEquals,
            Token::LesEq => Operator::LessEqual,
            Token::Less => Operator::Less,
            Token::Gre => Operator::Greater,
            Token::GreEq => Operator::GreaterEqual,
            c => panic!("Attempted to find the operator of {c:?}"),
        }
    }
}
