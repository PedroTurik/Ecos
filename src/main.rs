mod lexer;

use lexer::{Lexer, Token};

fn main() {
    let mut lexer = Lexer::new(
        r#"
        let a = 0;

        let bacon_c = 634 + 142
        "#.to_string()
    );

    loop {
        let tok = lexer.next_token().unwrap();
        println!("{tok:?}");

        if tok == Token::Eof{
            break;
        }

    }
}
