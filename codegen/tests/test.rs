#[cfg(test)]
mod tests {
    use codegen;
    use tempfile::tempdir;
    use std::fs::File;
    use std::io::Write;
    use std::process::Command;

    #[test]
    fn compile_decimal() {
        let asm = codegen::generate(42).unwrap();

        let tmp_dir = tempdir().unwrap();
        let asm_path = tmp_dir.path().join("tmp.s");
        let bin_path = tmp_dir.path().join("tmp");

        let mut file = File::create(&asm_path).unwrap();
        file.write_all(asm.as_bytes()).unwrap();

        Command::new("cc")
            .arg("-o")
            .arg(&bin_path)
            .arg(&asm_path)
            .status()
            .expect("Failed to assemble");

        let status = Command::new(bin_path)
            .status()
            .unwrap();

        assert_eq!(status.code(), Some(42));
    }
}
