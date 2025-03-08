use anyhow::Result;
use tokenizer::{Token, TokenStream};

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Add(Box<Node>, Box<Node>),
    Sub(Box<Node>, Box<Node>),
    Mul(Box<Node>, Box<Node>),
    Div(Box<Node>, Box<Node>),
    Num(u32)
}

pub struct Parser<I: Iterator<Item = Token>> {
    stream: TokenStream<I>
}

impl<I> Parser<I> where I: Iterator<Item = Token> {
    pub fn new(s: TokenStream<I>) -> Self {
        Self { stream: s }
    }

    pub fn parse_expr(&mut self) -> Node {
        let mut node = self.parse_mul();

        loop {
            match self.stream.consume_if(|t| {println!("{:?}", t); *t == Token::Plus || *t == Token::Minus}) {
                Some(Token::Plus) => { node = Node::Add(Box::new(node), Box::new(self.parse_mul())); },
                Some(Token::Minus) => { node = Node::Sub(Box::new(node), Box::new(self.parse_mul())); },
                _ => return node,
            }
        }
    }


    fn parse_mul(&mut self) -> Node {
        let mut node = self.parse_primary();

        loop {
            match self.stream.consume_if(|t| *t == Token::Asterisk || *t == Token::Slash) {
                Some(Token::Asterisk) => { node = Node::Mul(Box::new(node), Box::new(self.parse_primary())); },
                Some(Token::Slash) => { node = Node::Mul(Box::new(node), Box::new(self.parse_primary())); },
                _ => return node,
            }
        }
    }

    fn parse_primary(&mut self) -> Node {
        match self.stream.consume(Token::LParen) {
            Some(_) => {
                let expr = self.parse_expr();
                self.stream.consume(Token::RParen).expect("Paren error");
                expr
            },
            _ => self.parse_num(),
        }
    }
    
    fn parse_num(&mut self) -> Node {
        Node::Num(self.stream.consume_number().unwrap())
    }
}

#[test]
fn test() {
    use tokenizer;
    let tokens = tokenizer::tokenize("12 + 3 * 4".to_string()).unwrap();
    let stream = tokenizer::TokenStream::new(tokens.into_iter());

    let mut parser = Parser::new(stream);
    let node = parser.parse_expr();

    assert_eq!(node, Node::Add(Box::new(Node::Num(12)), Box::new(Node::Mul(Box::new(Node::Num(3)), Box::new(Node::Num(4))))));
}

