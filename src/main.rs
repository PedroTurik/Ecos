mod lexer;
mod parser;

use lexer::Lexer;
use parser::Parser;

fn main() {
    let lexer: Lexer = Lexer::new(
        r#"
        let a = 0 ;
        let bacon_c = 634 + 142 ;
        let adaw =  (bacon_c + 50) ;
        
        "#
        .to_string(),
    );

    let mut parser = Parser::new(lexer);
    let program = parser.parse_progam();
    dbg!(program.statements);
}
