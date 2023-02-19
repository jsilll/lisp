use crate::tokens::Operator;

/// The `Program` struct contains the statements in the program.
#[derive(Debug)]
pub struct Program<'i> {
    /// The statements in the program.
    pub statements: Vec<Statement<'i>>,
}

/// The `Statement` enum contains the different types of statements.
#[derive(Debug)]
pub enum Statement<'i> {
    /// A statement that declares a variable.
    #[allow(dead_code)]
    Assignment(StatementBody<'i>),
    /// A statement that declares a function.
    #[allow(dead_code)]
    Definition(StatementBody<'i>),
}

/// The `StatementBody` struct contains 
/// the identifier and expression for a statement.
#[derive(Debug)]
pub struct StatementBody<'i> {
    /// The identifier for the statement.
    pub identifier: &'i str,
    /// The expression for the statement.
    pub expression: Box<Expr<'i>>,
}

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

