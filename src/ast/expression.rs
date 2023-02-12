use crate::lexer::tokens::Operator;

/// The `Expr` enum contains the different types of expressions.
#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Expr<'i> {
    /// An integer literal
    Integer(i32),
    /// An identifier
    Identifier(&'i str),
    /// Two expressions combined by an operator
    Operation(Box<Expr<'i>>, Operator, Box<Expr<'i>>),
}

