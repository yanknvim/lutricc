use parser::Node;
use anyhow::Result;
use std::io::{self, Write};

pub fn generate_head<W: Write>(f: &mut W) -> Result<()>{
    writeln!(f, ".intel_syntax noprefix")?;
    writeln!(f, ".globl main")?;
    writeln!(f, "main:")?;

    Ok(())
}

pub fn generate<W: Write>(f: &mut W, node: Node) -> Result<()> {
    match node {
        Node::Add(left, right) => {
            generate(f, *left)?;
            generate(f, *right)?;

            writeln!(f, "   pop rdi")?;
            writeln!(f, "   pop rax")?;
            writeln!(f, "   add rax, rdi")?;
            writeln!(f, "   push rax")?;
        },
        Node::Sub(left, right) => {
            generate(f, *left)?;
            generate(f, *right)?;

            writeln!(f, "   pop rdi")?;
            writeln!(f, "   pop rax")?;
            writeln!(f, "   sub rax, rdi")?;
            writeln!(f, "   push rax")?;
        },
        Node::Mul(left, right) => {
            generate(f, *left)?;
            generate(f, *right)?;

            writeln!(f, "   pop rdi")?;
            writeln!(f, "   pop rax")?;
            writeln!(f, "   imul rax, rdi")?;
            writeln!(f, "   push rax")?;
        },
        Node::Div(left, right) => {
            generate(f, *left)?;
            generate(f, *right)?;

            writeln!(f, "   pop rdi")?;
            writeln!(f, "   pop rax")?;
            writeln!(f, "   cqo")?;
            writeln!(f, "   idiv rdi")?;
            writeln!(f, "   push rax")?;
        },
        Node::Num(x) => writeln!(f, "   push {}", x)?,
    }

    Ok(())
}

pub fn generate_tail<W: Write>(f: &mut W) -> Result<()> {
    writeln!(f, "   pop rax")?;
    writeln!(f, "   ret")?;

    Ok(())
}


