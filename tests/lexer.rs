use ecos::lexer::{Lexer, Token};

#[test]
fn test_next_token() {
    let input = "let x = 12;";

    let correct_sequence = vec![
        Token::Let,
        Token::Ident("x".to_owned()),
        Token::Assign,
        Token::Int("12".to_owned()),
        Token::Semicolon,
        Token::Eof,
    ];

    let mut lexer = Lexer::new(input.to_owned());

    for token in correct_sequence {
        assert_eq!(token, lexer.next_token().unwrap());
    }
}
