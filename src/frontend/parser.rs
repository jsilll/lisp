use crate::ast::Object;

use crate::frontend::lexer::{Err as LErr, Lexer, Located};
use crate::frontend::token::Token;

/// Parser for the lisp language.
pub struct Parser<'i> {
    /// The lexer to use.
    lexer: Lexer<'i>,
}

/// Error type for the parser.
#[derive(Debug, PartialEq)]
pub enum Err<'i> {
    /// Unexpected end of file.
    UnexpectedEndOfFile,
    /// Lexer error.
    LexerError(Located<LErr<'i>>),
    /// Unexpected token.
    UnexpectedToken(Located<Token<'i>>),
}

impl<'i> std::fmt::Display for Err<'i> {
    /// Formats the value using the given formatter.
    ///
    /// # Arguments
    /// * `f` - The formatter to use.
    ///
    /// # Returns
    /// The result of the formatting.
    ///
    /// # Examples
    /// ```
    /// use lisp::frontend::parser::Err;
    ///
    /// let err = Err::UnexpectedEndOfFile;
    /// assert_eq!(format!("{}", err), "Unexpected end of file.");
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Err::UnexpectedEndOfFile => write!(f, "Unexpected end of file."),
            Err::LexerError(err) => write!(f, "Lexical error: {}", err.value),
            Err::UnexpectedToken(token) => write!(f, "Unexpected token: '{}'", token.value),
        }
    }
}

impl<'i> Parser<'i> {
    /// Creates a new parser.
    ///
    /// # Arguments
    /// * `lexer` - The lexer to use.
    ///
    /// # Returns
    /// A new parser.
    ///
    /// # Examples
    /// ```
    /// use lisp::frontend::lexer::Lexer;
    /// use lisp::frontend::parser::Parser;
    ///
    /// let lexer = Lexer::new("()");
    /// let parser = Parser::new(lexer);
    /// ```
    pub fn new(lexer: Lexer) -> Parser {
        Parser { lexer }
    }

    /// Parses the input.
    ///
    /// # Returns
    /// The parsed object.
    ///
    /// # Examples
    /// ```
    /// use lisp::frontend::lexer::Lexer;
    /// use lisp::frontend::parser::Parser;
    ///
    /// let lexer = Lexer::new("()");
    /// let mut parser = Parser::new(lexer);
    /// let res = parser.parse();
    /// ```
    pub fn parse(&mut self) -> Result<Object<'i>, Err<'i>> {
        let mut lists = Vec::new();
        while let Some(res) = self.lexer.next() {
            match res {
                Ok(located) => match located.value {
                    Token::LParen => match self.parse_list() {
                        Ok(obj) => lists.push(obj),
                        Err(err) => return Err(err),
                    },
                    _ => return Err(Err::UnexpectedToken(located)),
                },
                Err(err) => return Err(Err::LexerError(err)),
            };
        }

        match lists.len() {
            0 => Ok(Object::Void),
            1 => Ok(lists.pop().unwrap_or(Object::Void)),
            _ => Ok(Object::List(lists)),
        }
    }

    /// Parses a list.
    ///
    /// # Returns
    /// The parsed list.
    ///
    /// # Examples
    /// ```
    /// use lisp::frontend::lexer::Lexer;
    /// use lisp::frontend::parser::Parser;
    ///
    /// let lexer = Lexer::new("+ 1 2)");
    /// let mut parser = Parser::new(lexer);
    /// let res = parser.parse_list();
    /// ```
    pub fn parse_list(&mut self) -> Result<Object<'i>, Err<'i>> {
        let mut list = Vec::new();
        while let Some(res) = self.lexer.next() {
            match res {
                Ok(token) => match token.value {
                    Token::LParen => match self.parse_list() {
                        Ok(obj) => list.push(obj),
                        Err(err) => return Err(err),
                    },
                    Token::Symbol(s) => {
                        list.push(Object::Symbol(s));
                    }
                    Token::Integer(i) => {
                        list.push(Object::Integer(i));
                    }
                    Token::RParen => {
                        return Ok(Object::List(list));
                    }
                },
                Err(err) => return Err(Err::LexerError(err)),
            };
        }
        Err(Err::UnexpectedEndOfFile)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests the `parse` method.
    #[test]
    fn test_parse() {
        let source = "(+ 1 2)";
        let lexer = Lexer::new(source);
        let mut parser = Parser::new(lexer);
        let res = parser.parse();
        assert_eq!(
            res,
            Ok(Object::List(vec![
                Object::Symbol("+"),
                Object::Integer(1),
                Object::Integer(2),
            ])),
        );
    }

    /// Tests the `parse_list` method.
    #[test]
    fn test_parse_list() {
        let source = "+ 1 2)";
        let lexer = Lexer::new(source);
        let mut parser = Parser::new(lexer);
        let res = parser.parse_list();
        assert_eq!(
            res,
            Ok(Object::List(vec![
                Object::Symbol("+"),
                Object::Integer(1),
                Object::Integer(2),
            ])),
        );
    }
}
