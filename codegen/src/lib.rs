use tokenizer::{Token, TokenStream};
use parser::Parser;
use anyhow::Result;
use std::io::{self, Write};

pub fn generate<W: Write>(f: &mut W, tokens: Vec<Token>) -> Result<()> {
    let mut stream = TokenStream::new(tokens.into_iter());
    let parser = Parser::new(stream);

    writeln!(f, ".intel_syntax noprefix")?;
    writeln!(f, ".globl main")?;
    writeln!(f, "main:")?;

    let num = stream.consume_number().unwrap();
    writeln!(f, "   mov rax, {}", num)?;

    while let Some(token) = stream.next() {
        match token {
            Token::Plus => writeln!(f, "    add rax, {}", stream.consume_number().unwrap())?,
            Token::Minus => writeln!(f, "   sub rax, {}", stream.consume_number().unwrap())?,
            Token::Plus => writeln!(f, "    add rax, {}", stream.consume_number().unwrap())?,
            Token::Plus => writeln!(f, "    add rax, {}", stream.consume_number().unwrap())?,
            _ => {},
        }
    }

    writeln!(f, "   ret")?;

    Ok(())
}

