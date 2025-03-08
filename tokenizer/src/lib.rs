use anyhow::Result;
use std::iter::Peekable;

#[derive(Debug, PartialEq)]
pub enum Token {
    Num(u32),
    Plus,
    Minus,
}

pub struct TokenStream<I: Iterator<Item = Token>> {
    tokens: Peekable<I>,
}

impl<I> TokenStream<I> where I: Iterator<Item = Token> {
    pub fn new(tokens: I) -> Self {
        Self {
            tokens: tokens.peekable()
        }
    }

    pub fn consume(&mut self, expected: Token) -> Option<bool> {
        self.tokens.next_if_eq(&expected).map(|_| true)
    }

    pub fn expect(&mut self, expected: Token) -> Result<bool, String> {
        match self.tokens.next_if_eq(&expected) {
            Some(_) => Ok(true),
            None => Err(format!("Not expected token, expected: {:?}", expected)),
        }
    }

    pub fn expect_number(&mut self) -> Result<u32, String> {
        match self.tokens.peek() {
            Some(Token::Num(x)) => Ok(*x),
            _ => Err(format!("Not num")),
        }
    }

    pub fn next(&mut self) -> Option<Token> {
        self.tokens.next()
    }
}

pub fn tokenize(s: String) -> Result<Vec<Token>, String> {
    let mut tokens: Vec<Token> = Vec::new();

    let mut s = s.chars();
    while let Some(c) = s.next() {
        match c {
            ' ' => {},
            '+' => tokens.push(Token::Plus),
            '-' => tokens.push(Token::Minus),
            '0'..'9' => {
                let mut num = c.to_string();
                while let Some(c) = s.next() {
                    if !c.is_ascii_digit() {
                        break;
                    }
                    num.push(c);
                }
                tokens.push(Token::Num(num.parse::<u32>().unwrap()))
            }
            _ => return Err(format!("invalid token: {}", c)),
        }
    }

    Ok(tokens)
}


