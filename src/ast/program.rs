use crate::ast::statement::Statement;

/// The `Program` struct contains the statements in the program.
#[derive(Debug)]
pub struct Program<'i> {
    /// The statements in the program.
    pub statements: Vec<Statement<'i>>,
}