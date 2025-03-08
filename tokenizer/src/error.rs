use thiserror::Error;

#[derive(Error, Debug)]
pub enum TokenizeError {
    #[error("unexpected character")]
    UnexpectedCharacter(char, usize),
}
