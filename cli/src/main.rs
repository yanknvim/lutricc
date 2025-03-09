use std::env;
use std::io::{Write, BufWriter, stdout};
use tokenizer;
use parser;
use codegen;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("invalid args");
        return;
    }

    let tokens = tokenizer::tokenize(args[1].clone()).unwrap();
    let stream = tokenizer::TokenStream::new(tokens.into_iter());
    let mut parser = parser::Parser::new(stream);
    let node = parser.parse_expr();
    
    let mut writer = BufWriter::new(stdout());
    codegen::generate_head(&mut writer).unwrap();
    codegen::generate(&mut writer, node).unwrap();
    codegen::generate_tail(&mut writer).unwrap();
}
