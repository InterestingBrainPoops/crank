use super::token::Token;

pub fn lex(input: &str) -> Vec<Token> {
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
                            *v = ((*v) * 10.) + y as f64;
                        }
                        _ => {
                            out.push(Token::Number(y as f64));
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
