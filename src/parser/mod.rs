pub mod lexer;
pub mod token;

use token::Token;

#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    Binary(Operator, Box<Node>, Box<Node>),
    Unary(Operator, Box<Node>),
    Value(Literal),
    /// Define variable
    Define(String, Box<Node>),
    Statements(Vec<Node>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Name(String),
    StringT(String),
    Float(f64),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Operator {
    Mult,
    Div,
    Sub,
    Add,
    NotEquals,
    EqualsEquals,
    GreaterEqual,
    Greater,
    LessEqual,
    Less,
}

pub struct Parser {
    tokens: Vec<Token>,
    counter: usize,
}
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, counter: 0 }
    }

    fn peek_token(&self) -> &Token {
        &self.tokens[self.counter + 1]
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.counter += 1;
        }

        self.previous()
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.counter - 1]
    }

    fn current(&self) -> &Token {
        &self.tokens[self.counter]
    }

    fn matches(&mut self, pattern: Vec<Token>) -> bool {
        if self.is_at_end() {
            return false;
        }
        if pattern.contains(self.current()) {
            self.advance();
            return true;
        }
        false
    }
    fn step_back(&mut self) -> &Token {
        if self.counter != 0 {
            self.counter -= 1;
        }
        self.peek_token()
    }
    fn is_at_end(&self) -> bool {
        self.counter == self.tokens.len()
    }

    pub fn parse(&mut self) -> Node {
        let mut statements = vec![];
        while self.matches(vec![Token::Var]) {
            self.step_back();
            statements.push(self.declare_var());
        }
        Node::Statements(statements)
    }

    fn declare_var(&mut self) -> Node {
        self.advance(); // var
        let name = self.advance().clone();
        self.advance(); // =
        let expr = self.conditional();
        if let Token::Literal(name) = name {
            Node::Define(name, Box::new(expr))
        } else {
            panic!("name was not a literal {name:?}");
        }
    }

    fn conditional(&mut self) -> Node {
        let mut node = self.compare();
        while self.matches(vec![Token::NotEquals, Token::EqualsEquals]) {
            let operator = self.previous().operator();
            let right = self.compare();
            node = Node::Binary(operator, Box::new(node), Box::new(right));
        }
        node
    }

    fn compare(&mut self) -> Node {
        let mut node = self.term();
        while self.matches(vec![Token::Gre, Token::GreEq, Token::LesEq, Token::Less]) {
            let operator = self.previous().operator();
            let right = self.term();
            node = Node::Binary(operator, Box::new(node), Box::new(right));
        }
        node
    }

    fn term(&mut self) -> Node {
        let mut node = self.factor();
        while self.matches(vec![Token::Plus, Token::Hyphen]) {
            let operator = self.previous().operator();
            let right = self.factor();
            node = Node::Binary(operator, Box::new(node), Box::new(right));
        }
        node
    }

    fn factor(&mut self) -> Node {
        let mut node = self.unary();
        while self.matches(vec![Token::Asterisk, Token::ForwardSlash]) {
            let operator = self.previous().operator();
            let right = self.unary();
            node = Node::Binary(operator, Box::new(node), Box::new(right));
        }
        node
    }

    fn unary(&mut self) -> Node {
        if self.matches(vec![Token::Hyphen]) {
            let operator = self.previous().operator();
            let right = self.primary();
            return Node::Unary(operator, Box::new(right));
        }
        self.primary()
    }
    fn primary(&mut self) -> Node {
        match self.advance() {
            Token::Literal(x) => Node::Value(Literal::StringT(x.clone())),
            Token::Number(x) => Node::Value(Literal::Float(*x)),
            _ => unreachable!(),
        }
    }
}
