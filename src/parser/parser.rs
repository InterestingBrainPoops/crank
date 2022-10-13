use super::token::Token;

#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    Binary(Operator, Box<Node>, Box<Node>),
    Unary(Operator, Box<Node>),
    Value(f64),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Operator {
    Mult,
    Div,
    Sub,
    Add,
}

pub struct Parser {
    tokens: Vec<Token>,
    counter: usize,
}
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, counter: 0 }
    }
    fn peek_token(&self) -> Token {
        self.tokens[self.counter + 1]
    }
    fn next_token(&mut self) -> Token {
        if !self.at_end() {
            self.counter += 1;
        }

        self.previous_token()
    }
    fn previous_token(&self) -> Token {
        self.tokens[self.counter - 1]
    }

    fn current(&self) -> Token {
        self.tokens[self.counter]
    }

    fn matches(&mut self, pattern: Vec<Token>) -> bool {
        if self.at_end() {
            return false;
        }
        if pattern.contains(&self.current()) {
            self.next_token();
            return true;
        }
        false
    }
    fn at_end(&self) -> bool {
        return self.counter == self.tokens.len();
    }

    pub fn parse(&mut self) -> Node {
        self.addsub()
    }

    fn addsub(&mut self) -> Node {
        let mut node = self.muldiv();
        while (self.matches(vec![Token::Plus, Token::Hyphen])) {
            let operator = self.previous_token().operator();
            let right = self.muldiv();
            node = Node::Binary(operator, Box::new(node), Box::new(right));
        }
        node
    }

    fn muldiv(&mut self) -> Node {
        let mut node = self.primary();
        while (self.matches(vec![Token::Asterisk, Token::ForwardSlash])) {
            let operator = self.previous_token().operator();
            let right = self.primary();
            node = Node::Binary(operator, Box::new(node), Box::new(right));
        }
        node
    }

    fn unary(&mut self) -> Node {
        if self.matches(vec![Token::Hyphen]) {
            let operator = self.previous_token().operator();
            let right = self.primary();
            return Node::Unary(operator, right);
        }
        self.primary()
    }
    fn primary(&mut self) -> Node {
        if let Token::Number(x) = self.next_token() {
            return Node::Value(x);
        }
        unreachable!();
    }
}
