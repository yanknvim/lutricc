use std::env;
use codegen::generate;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("invalid args");
        return;
    }

    match generate(args[1].parse::<u32>().unwrap()) {
        Ok(asm) => println!("{}", asm),
        Err(_) => println!("some error"),
    }
}
