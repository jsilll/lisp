use crate::lexer::tokens::Operator;

/// The `Expr` enum contains the different types of expressions.
#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Expr<'i> {
    /// An integer literal
    #[allow(dead_code)]
    Integer(i32),
    /// An identifier
    #[allow(dead_code)]
    Identifier(&'i str),
    /// Two expressions combined by an operator
    #[allow(dead_code)]
    Operation{lhs : Box<Expr<'i>>, op : Operator, rhs : Box<Expr<'i>>}
}

