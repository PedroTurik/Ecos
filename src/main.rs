mod lexer;
mod parser;

use lexer::Lexer;
use parser::Parser;

fn main() {
    let mut lexer: Lexer = Lexer::new(
        r#"
        let a = ijfa;
        let bacon_c = fefaf ;
        let adaw =  aefaf ;
        
        "#
        .to_string(),
    );

    for _ in 0..40 {
        dbg!(lexer.next_token().unwrap());
    }

    let parser = Parser::new(lexer);
    let program = parser.parse_progam();
    dbg!(program);
}
