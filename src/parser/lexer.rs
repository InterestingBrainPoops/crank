use super::token::Token;

pub struct Lexer {
    tokens: Vec<Token>,
    position: usize,
}
impl Default for Lexer {
    fn default() -> Self {
        Self::new()
    }
}
impl Lexer {
    pub fn new() -> Self {
        Self {
            tokens: vec![],
            position: 0,
        }
    }
    pub fn lex(&mut self, input: &str) -> Vec<Token> {
        while self.position < input.len() {
            match input.chars().nth(self.position) {
                None => {}
                Some(x) => match x {
                    '/' => self.tokens.push(Token::ForwardSlash),
                    '*' => self.tokens.push(Token::Asterisk),
                    '+' => self.tokens.push(Token::Plus),
                    '-' => self.tokens.push(Token::Hyphen),
                    '(' => self.tokens.push(Token::OpenParen),
                    ')' => self.tokens.push(Token::CloseParen),
                    '>' => match input.chars().nth(self.position + 1) {
                        Some('=') => {
                            self.tokens.push(Token::GreEq);
                            self.position += 1;
                        }
                        _ => self.tokens.push(Token::Gre),
                    },
                    '<' => match input.chars().nth(self.position + 1) {
                        Some('=') => {
                            self.tokens.push(Token::LesEq);
                            self.position += 1;
                        }
                        _ => self.tokens.push(Token::Less),
                    },
                    '=' => match input.chars().nth(self.position + 1) {
                        Some('=') => {
                            self.tokens.push(Token::EqualsEquals);
                            self.position += 1;
                        }
                        _ => self.tokens.push(Token::Equals),
                    },
                    '!' => match input.chars().nth(self.position + 1) {
                        Some('=') => {
                            self.tokens.push(Token::NotEquals);
                            self.position += 1;
                        }
                        _ => self.tokens.push(Token::Bang),
                    },
                    ' ' => {}
                    _ => match x.to_digit(10) {
                        Some(_) => {
                            let x = self.grab_until("{}!/*+-()= ".chars().collect(), input);
                            self.tokens
                                .push(Token::Number(x.parse::<u32>().unwrap() as f64));
                        }
                        None => {
                            let x = self.grab_until("{}!/*+-()= ".chars().collect(), input);
                            match x.as_str() {
                                "var" => self.tokens.push(Token::Var),
                                _ => self.tokens.push(Token::Literal(x)),
                            }
                        }
                    },
                },
            }
            self.position += 1;
        }
        self.tokens.clone()
    }

    fn grab_until(&mut self, delim: Vec<char>, input: &str) -> String {
        let mut out = String::new();
        while self.position != input.len()
            && !delim.contains(&input.chars().nth(self.position).unwrap())
        {
            out.push(input.chars().nth(self.position).unwrap());
            self.position += 1;
        }
        self.position -= 1;
        out
    }
}
