#[cfg(test)]
mod tests {
    use tokenizer::{Token, tokenize};

    #[test]
    fn tokenize_plus_and_minus() {
        let tokens = tokenize("12 + 34 - 5".to_string()).unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::Num(12),
                Token::Plus,
                Token::Num(34),
                Token::Minus,
                Token::Num(5)
            ]
        )
    }
}
