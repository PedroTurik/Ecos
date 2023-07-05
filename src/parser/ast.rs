use crate::lexer::Token;

pub trait Node {
    fn token_literal(&self) -> String;
}

#[derive(Debug)]
pub enum Statement {
    LetStatement(LetStatement),
    IfStatement,
}


impl Node for Statement {
    fn token_literal(&self) -> String {
        match self {
            Statement::LetStatement(lstmt) => lstmt.token_literal(),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub enum Expression {
    Identifier(Identifier),
    TestExp,
}

impl Node for Expression {
    fn token_literal(&self) -> String {
        match self {
            Expression::Identifier(ident) => ident.token_literal(),
            _ => unreachable!(),
        }
    }
}

// pub trait Statement: Node { fn statement_node(&self); }

// pub trait Expression: Node { fn expression_node(&self);}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Node for Program {
    fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            return self.statements[0].token_literal();
        }

        return "".to_string();
    }
}

#[derive(Debug)]
pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Expression,
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal().to_owned()
    }
}

#[derive(Debug)]
pub struct Identifier {
    pub token: Token,
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal().to_owned()
    }
}
