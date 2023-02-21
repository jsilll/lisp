use std::iter::Peekable;
use std::str::CharIndices;

use crate::frontend::token::Token;

/// The Lexer struct is used to tokenize the input.
pub struct Lexer<'i> {
    /// The input string.
    input: &'i str,
    /// The stream of characters.
    chars: Peekable<CharIndices<'i>>,
}

impl<'i> Lexer<'i> {
    /// Creates a new lexer for the given input.
    ///
    /// # Arguments
    /// * `input` - The input string.
    ///
    /// # Returns
    /// A new lexer.
    ///
    /// # Examples
    /// ```
    /// use lisp::frontend::lexer::Lexer;
    ///
    /// let lexer = Lexer::new("()");
    /// ```
    pub fn new(input: &'i str) -> Self {
        Lexer {
            input: input,
            chars: input.char_indices().peekable(),
        }
    }

    /// Returns the next token in the input.
    ///
    /// # Returns
    /// The next token in the input.
    fn skip_whitespace(&mut self) -> Option<(usize, char)> {
        while let Some((_, c)) = self.chars.peek() {
            if !c.is_whitespace() {
                break;
            }
            self.chars.next();
        }
        self.chars.next()
    }

    /// Returns the next token in the input.
    ///
    /// # Returns
    /// The next token in the input.
    fn consume_while(&mut self, predicate: impl Fn(char) -> bool) -> usize {
        loop {
            match self.chars.peek() {
                Some((idx, c)) => {
                    if predicate(*c) {
                        self.chars.next()
                    } else {
                        return *idx;
                    }
                }
                None => return self.input.len(),
            };
        }
    }
}

/// Some value with a location in the input string
#[derive(Debug, PartialEq)]
pub struct Located<T> {
    /// The index of the beginning of the token
    pub begin: usize,
    /// The index of the end of the token
    pub end: usize,
    /// The contained value
    pub value: T,
}

/// An error from the lexer
#[derive(Debug, PartialEq)]
pub enum Err<'i> {
    /// An unexpected character was encountered by the lexer
    UnexpectedCharacter(char),
    /// An invalid integer literal was encountered by the lexer
    IntegerParseError(&'i str),
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
    /// use lisp::frontend::lexer::Err;
    /// 
    /// let err = Err::UnexpectedCharacter('a');
    /// assert_eq!(format!("{}", err), "Unexpected character: 'a'");
    /// ``` 
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Err::UnexpectedCharacter(c) => write!(f, "Unexpected character: '{}'", c),
            Err::IntegerParseError(s) => write!(f, "Invalid integer literal: '{}'", s),
        }
    }
}

impl<'i> Iterator for Lexer<'i> {
    type Item = std::result::Result<Located<Token<'i>>, Located<Err<'i>>>;

    /// Returns the next token in the input.
    ///
    /// # Returns
    /// The next token in the input.
    ///
    /// # Examples
    /// ```
    /// use lisp::frontend::token::Token;
    /// use lisp::frontend::lexer::{Lexer, Located};
    ///
    /// let mut lexer = Lexer::new("()");
    /// assert_eq!(lexer.next(), Some(Ok(Located {
    ///    begin: 0,
    ///   end: 1,
    ///   value: Token::LParen,
    /// })));
    fn next(&mut self) -> Option<Self::Item> {
        let (begin, c) = self.skip_whitespace()?;
        match c {
            '(' => Some(Ok(Located {
                begin,
                end: begin + 1,
                value: Token::LParen,
            })),

            ')' => Some(Ok(Located {
                begin,
                end: begin + 1,
                value: Token::RParen,
            })),

            '+' => Some(Ok(Located {
                begin,
                end: begin + 1,
                value: Token::Plus,
            })),

            '-' => Some(Ok(Located {
                begin,
                end: begin + 1,
                value: Token::Minus,
            })),

            '0'..='9' => {
                let end = self.consume_while(|c| c.is_digit(10));
                let res = self.input[begin..end].parse::<i64>();
                match res {
                    Ok(num) => Some(Ok(Located {
                        begin,
                        end,
                        value: Token::Integer(num),
                    })),
                    Err(_) => Some(Err(Located {
                        begin,
                        end,
                        value: Err::IntegerParseError(&self.input[begin..end]),
                    })),
                }
            }

            c if c.is_alphabetic() => {
                let end = self.consume_while(|c| c.is_alphanumeric() || c == '_');
                Some(Ok(Located {
                    begin,
                    end,
                    value: Token::Symbol(&self.input[begin..end]),
                }))
            }

            _ => Some(Err(Located {
                begin,
                end: begin + 1,
                value: Err::UnexpectedCharacter(c),
            })),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests the `fmt` method of `Err`.
    #[test]
    fn test_err_fmt() {
        let err = Err::UnexpectedCharacter('a');
        assert_eq!(format!("{}", err), "Unexpected character: 'a'");

        let err = Err::IntegerParseError("123a");
        assert_eq!(format!("{}", err), "Invalid integer literal: '123a'");
    }

    /// Tests the `skip_whitespace` method of `Lexer`.
    #[test]
    fn test_skip_whitespace() {
        let mut lexer = Lexer::new("  123");
        assert_eq!(lexer.skip_whitespace(), Some((2, '1')));
    }

    /// Tests the `consume_while` method of `Lexer`.
    #[test]
    fn test_consume_while() {
        let mut lexer = Lexer::new("123abc");
        assert_eq!(lexer.consume_while(|c| c.is_digit(10)), 3);
    }

    /// Tests the lexer.
    #[test]
    fn test_lexer() {
        let source = "(+ 1 2)";
        let lexer = Lexer::new(source);
        let tokens = lexer.collect::<Vec<_>>();
        assert_eq!(
            tokens,
            vec![
                Ok(Located {
                    begin: 0,
                    end: 1,
                    value: Token::LParen,
                }),
                Ok(Located {
                    begin: 1,
                    end: 2,
                    value: Token::Plus,
                }),
                Ok(Located {
                    begin: 3,
                    end: 4,
                    value: Token::Integer(1),
                }),
                Ok(Located {
                    begin: 5,
                    end: 6,
                    value: Token::Integer(2),
                }),
                Ok(Located {
                    begin: 6,
                    end: 7,
                    value: Token::RParen,
                }),
            ]
        );
    }
}
