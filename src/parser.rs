use crate::ast::{Binary, Expr, NumberLiteral};
use crate::tokens::Token;


/// Create AST from tokens
fn build_ast(tokens: Vec<Token>) -> Box<dyn Expr> {
    Binary::new(
        NumberLiteral::new(2.3),
        Token::Plus,
        NumberLiteral::new(1.2),
    )
}