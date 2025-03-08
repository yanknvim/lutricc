mod error;
use error::TokenizeError;

use anyhow::Result;
use std::iter::Peekable;

#[derive(Debug, PartialEq)]
pub enum Token {
    Num(u32),
    Plus,
    Minus,
    Asterisk,
    Slash,
    LParen,
    RParen,
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

    pub fn consume_if(&mut self, f: impl FnOnce(&<I as Iterator>::Item) -> bool) -> Option<Token> {
        self.tokens.next_if(f)
    }

    pub fn consume_number(&mut self) -> Option<u32> {
        self.tokens.next_if(|t| match t {
            Token::Num(_) => true,
            _ => false,
        }).map(|t| if let Token::Num(x) = t { Some(x) } else { None }).flatten()
    }

    pub fn next(&mut self) -> Option<Token> {
        self.tokens.next()
    }
}

pub fn tokenize(s: String) -> Result<Vec<Token>, TokenizeError> {
    let mut tokens: Vec<Token> = Vec::new();

    let mut s = s.chars().enumerate();
    while let Some((index, c)) = s.next() {
        match c {
            ' ' => {},
            '+' => tokens.push(Token::Plus),
            '-' => tokens.push(Token::Minus),
            '*' => tokens.push(Token::Asterisk),
            '/' => tokens.push(Token::Slash),
            '(' => tokens.push(Token::LParen),
            ')' => tokens.push(Token::RParen),
            '0'..'9' => {
                let mut num = c.to_string();
                while let Some((_, c)) = s.next() {
                    if !c.is_ascii_digit() {
                        break;
                    }
                    num.push(c);
                }
                tokens.push(Token::Num(num.parse::<u32>().unwrap()));
            },
            _ => return Err(TokenizeError::UnexpectedCharacter(c, index)),
        }
    }

    Ok(tokens)
}


