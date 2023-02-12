use crate::ast::expression::Expr;

/// The `StatementBody` struct contains 
/// the identifier and expression for a statement.
#[derive(Debug)]
pub struct StatementBody<'i> {
    /// The identifier for the statement.
    pub identifier: &'i str,
    /// The expression for the statement.
    pub expression: Box<Expr<'i>>,
}

/// The `Statement` enum contains the different types of statements.
#[derive(Debug)]
pub enum Statement<'i> {
    /// A statement that declares a variable.
    Assignment(StatementBody<'i>),
    /// A statement that declares a function.
    Definition(StatementBody<'i>),
}