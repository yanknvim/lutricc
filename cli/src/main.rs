use std::env;
use std::io::{Write, BufWriter, stdout};
use tokenizer;
use codegen;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("invalid args");
        return;
    }

   let tokens = tokenizer::tokenize(args[1].clone()).unwrap();

   let mut writer = BufWriter::new(stdout());
   codegen::generate(&mut writer, tokens).unwrap();
}
