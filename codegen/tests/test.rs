#[cfg(test)]
mod tests {
    use tokenizer;
    use codegen;
    use parser;
    use tempfile::tempdir;
    use std::fs::File;
    use std::io::{BufWriter, Write};
    use std::process::Command;

    #[test]
    fn compile_add_and_sub() {
        let tmp_dir = tempdir().unwrap();
        let asm_path = tmp_dir.path().join("tmp.s");
        let bin_path = tmp_dir.path().join("tmp");

        {
            let mut file = BufWriter::new(File::create(&asm_path).unwrap());
            let tokens = tokenizer::tokenize(" 12 + 34 - 5 ".to_string()).unwrap();
            let stream = tokenizer::TokenStream::new(tokens.into_iter());
            let mut parser = parser::Parser::new(stream);
            let node = parser.parse_expr();
            codegen::generate_head(&mut file).unwrap();
            codegen::generate(&mut file, node).unwrap();
            codegen::generate_tail(&mut file).unwrap();
        } 
        
        let status = Command::new("cc")
            .arg("-o")
            .arg(&bin_path)
            .arg(&asm_path)
            .status().unwrap();

        assert!(status.success());

        let status = Command::new(bin_path)
            .status()
            .unwrap();

        assert_eq!(status.code(), Some(41));
    }

    #[test]
    fn compile_mul() {
        let tmp_dir = tempdir().unwrap();
        let asm_path = tmp_dir.path().join("tmp.s");
        let bin_path = tmp_dir.path().join("tmp");

        {
            let mut file = BufWriter::new(File::create(&asm_path).unwrap());
            let tokens = tokenizer::tokenize("6 * (2 + 3) - 4".to_string()).unwrap();
            let stream = tokenizer::TokenStream::new(tokens.into_iter());
            let mut parser = parser::Parser::new(stream);
            let node = parser.parse_expr();
            codegen::generate_head(&mut file).unwrap();
            codegen::generate(&mut file, node).unwrap();
            codegen::generate_tail(&mut file).unwrap();
        } 
        
        let status = Command::new("cc")
            .arg("-o")
            .arg(&bin_path)
            .arg(&asm_path)
            .status().unwrap();

        assert!(status.success());

        let status = Command::new(bin_path)
            .status()
            .unwrap();

        assert_eq!(status.code(), Some(26));
    }

}
