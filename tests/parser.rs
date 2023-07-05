use ecos::lexer::{Lexer, Token};
use ecos::parser::{LetStatement, Node, Parser, Program, Statement};

fn test_single_let_statement(stmt: &Statement, name: &str) {
    assert_eq!(
        stmt.token_literal(),
        "let",
        "Expected token literal to be 'let', got {}",
        stmt.token_literal()
    );

    if let Statement::LetStatement(let_statement) = stmt {
        assert_eq!(
            let_statement.name.token_literal(),
            name,
            "Expected LetStatement.name.value to to be {}, got {}",
            name,
            let_statement.name.token_literal()
        );

        assert_eq!(
            let_statement.name.token_literal(),
            name,
            "Expected LetStatement.name.token_literal to be {}, got {} ",
            name,
            let_statement.name.token_literal()
        )
    } else {
        panic!("Failed to parse Statement to expected LetStatement")
    }
}

#[test]
fn test_let_statements() {
    let input = r#"
            let x = 5 ;
            let y = 1 0;
            let foobar = 838383 ;
            "#
    .to_owned();

    let l = Lexer::new(input);
    let mut p = Parser::new(l);

    let program = p.parse_progam();
    
    assert_eq!(
        program.statements.len(),
        3,
        "Expected 3 statements, got {}",
        program.statements.len()
    );

    let expected_indentifiers = ["x", "y", "foobar"];

    for i in 0..3 {
        let stmt = program
            .statements
            .get(i)
            .expect(&format!("Index {} not fount for statements vector", i));

        test_single_let_statement(stmt, expected_indentifiers.get(i).unwrap());
    }
}
