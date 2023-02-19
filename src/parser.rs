use std::iter::Peekable;

use crate::ast::Program;
use crate::lexer::{Lexer, Located};
use crate::tokens::Token;

/// The result of parsing a source file.
pub type Result<'i> = std::result::Result<Program<'i>, Err<'i>>;

/// The parser.
/// 
/// The parser is responsible for transforming a stream of tokens into an AST.
/// 
/// # Example
/// ```
/// use lang::lexer::Lexer;
/// use lang::parser::Parser;
/// 
/// let source = "1 + 2";
/// let mut parser = Parser::new(Lexer::new(source));
/// let ast = parser.parse().unwrap();
/// ```
pub struct Parser<'i> {
    lexer: Peekable<Lexer<'i>>,
}

/// The error type for the parser.
#[derive(Debug)]
pub enum Err<'i> {
    UnexpectedToken(Located<Token<'i>>),
    LexerError(Located<crate::lexer::Err<'i>>),
}

impl<'i> Parser<'i> {
    /// Create a new parser.
    /// 
    /// # Example
    /// ```
    /// use lang::lexer::Lexer;
    /// use lang::parser::Parser;
    /// 
    /// let source = "1 + 2";
    /// let mut parser = Parser::new(Lexer::new(source));
    /// ```
    pub fn new(lexer: Lexer<'i>) -> Self {
        Self {
            lexer: lexer.peekable(),
        }
    }

    /// Parse the source code.
    /// 
    /// # Example
    /// ```
    /// use lang::lexer::Lexer;
    /// use lang::parser::Parser;
    /// 
    /// let source = "1 + 2";
    /// let mut parser = Parser::new(Lexer::new(source));
    /// let ast = parser.parse().unwrap();
    /// ```
    pub fn parse(&mut self) -> Result<'i> {
        println!("Parsing source code...");

        while let Some(res) = self.lexer.next() {
            match res {
                Err(err) => {
                    return Err(Err::LexerError(err));
                }
                Ok(token) => {
                    println!("{:?}", token.value);
                }
            }
        }

        Ok(Program {
            statements: Vec::new(),
        })
    }

    // TODO: Implement parsing functions
}
