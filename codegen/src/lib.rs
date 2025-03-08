use anyhow::Result;

pub fn generate(x: u32) -> Result<String> {
    Ok(format!(
        ".intel_syntax noprefix
        .globl main

        main:
            mov rax, {}
            ret
        ",
        x
    ))
}

