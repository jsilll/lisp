pub mod tokens;

use std::iter::Peekable;
use std::str::CharIndices;

use tokens::{Keyword, Literal, Operator, Separator, Token};

/// A result from the lexer
pub type LexerResult<'i> = Result<LocatedToken<'i>, LocatedError<'i>>;

/// A token with its location in the file
#[derive(Debug)]
pub struct LocatedToken<'i> {
    pub begin: usize,
    pub end: usize,
    pub token: Token<'i>,
}

/// An error with its location in the file
#[derive(Debug)]
pub struct LocatedError<'i> {
    pub begin: usize,
    pub end: usize,
    pub error: LexerError<'i>,
}

/// An error from the lexer
#[derive(Debug)]
pub enum LexerError<'i> {
    UnexpectedCharacter(char),
    IntegerParseError(&'i str),
}

/// A lexer for the language
///
/// # Examples
/// ```
/// use lang::lexer::Lexer;
///
/// let lexer = Lexer::new("# This is a comment\n # This is another comment\n");
/// for token in lexer {
///    println!("{:?}", token);
/// }
/// ```
pub struct Lexer<'i> {
    /// The input string
    input: &'i str,
    /// The stream of characters in the input string
    chars: Peekable<CharIndices<'i>>,
}

impl<'i> Lexer<'i> {
    /// Creates a new lexer
    ///
    /// # Arguments
    /// * `input` - The input string
    pub fn new(input: &'i str) -> Self {
        Self {
            input,
            chars: input.char_indices().peekable(),
        }
    }

    /// Skips all whitespace in the file
    ///
    /// # Returns
    /// The index of the next non-whitespace character
    /// If there are no more characters, `None` will be returned
    fn skip_whitespace(&mut self) -> Option<(usize, char)> {
        loop {
            match self.chars.next() {
                Some((_, c)) if c.is_whitespace() => continue,
                Some((idx, c)) => return Some((idx, c)),
                None => return None,
            }
        }
    }

    /// Consumes all characters in the file that match the predicate
    ///
    /// # Arguments
    /// * `predicate` - The predicate to match
    ///
    /// # Returns
    /// The index of the next character that does not match the predicate
    /// If there are no more characters, the length of the input string will be returned
    fn consume_while(&mut self, predicate: impl Fn(char) -> bool) -> usize {
        loop {
            match self.chars.peek() {
                Some((_, c)) if predicate(*c) => self.chars.next(),
                Some((idx, _)) => return *idx,
                None => return self.input.len(),
            };
        }
    }
}

impl<'i> Iterator for Lexer<'i> {
    type Item = LexerResult<'i>;

    /// Returns the next token in the file
    ///
    /// # Returns
    /// If there is a next token, it will be returned. Otherwise, `None` will be returned.
    fn next(&mut self) -> Option<Self::Item> {
        let (begin, c) = self.skip_whitespace()?;

        match c {
            // Inlined Comments
            '#' => {
                let end = self.consume_while(|c| c != '\r' && c != '\n');
                Some(Ok(LocatedToken {
                    begin,
                    end,
                    token: Token::Comment(&self.input[begin..end]),
                }))
            }

            // Identifiers, Keywords and Boolean Literals
            'a'..='z' | 'A'..='Z' | '_' => {
                let end = self.consume_while(|c| c.is_alphanumeric());
                match &self.input[begin..end] {
                    // Keywords
                    "fn" => Some(Ok(LocatedToken {
                        begin,
                        end,
                        token: Token::Keyword(Keyword::Fn),
                    })),
                    "if" => Some(Ok(LocatedToken {
                        begin,
                        end,
                        token: Token::Keyword(Keyword::If),
                    })),
                    "for" => Some(Ok(LocatedToken {
                        begin,
                        end,
                        token: Token::Keyword(Keyword::For),
                    })),
                    "let" => Some(Ok(LocatedToken {
                        begin,
                        end,
                        token: Token::Keyword(Keyword::Let),
                    })),
                    "else" => Some(Ok(LocatedToken {
                        begin,
                        end,
                        token: Token::Keyword(Keyword::Else),
                    })),
                    "match" => Some(Ok(LocatedToken {
                        begin,
                        end,
                        token: Token::Keyword(Keyword::Match),
                    })),
                    "while" => Some(Ok(LocatedToken {
                        begin,
                        end,
                        token: Token::Keyword(Keyword::While),
                    })),

                    // Boolean Literals
                    "true" => Some(Ok(LocatedToken {
                        begin,
                        end,
                        token: Token::Literal(Literal::Boolean(true)),
                    })),
                    "false" => Some(Ok(LocatedToken {
                        begin,
                        end,
                        token: Token::Literal(Literal::Boolean(false)),
                    })),

                    // Identifiers
                    _ => Some(Ok(LocatedToken {
                        begin,
                        end,
                        token: Token::Identifier(&self.input[begin..end]),
                    })),
                }
            }

            // Integer Literals
            '0'..='9' => {
                // TODO: Handle errors
                // TODO: Handle floating point numbers
                let end = self.consume_while(|c| c.is_numeric());
                let res = self.input[begin..end].parse::<i64>();
                match res {
                    Ok(num) => Some(Ok(LocatedToken {
                        begin,
                        end,
                        token: Token::Literal(Literal::Integer(num)),
                    })),
                    Err(_) => Some(Err(LocatedError {
                        begin,
                        end,
                        error: LexerError::IntegerParseError(&self.input[begin..end]),
                    })),
                }
            }

            // String Literals
            '"' => {
                // TODO: Handle escape sequences
                let end = self.consume_while(|c| c != '"');
                Some(Ok(LocatedToken {
                    begin,
                    end,
                    token: Token::Literal(Literal::String(&self.input[begin + 1..end])),
                }))
            }

            // Separators
            ',' => Some(Ok(LocatedToken {
                begin,
                end: begin + 1,
                token: Token::Separator(Separator::Comma),
            })),
            ':' => Some(Ok(LocatedToken {
                begin,
                end: begin + 1,
                token: Token::Separator(Separator::Colon),
            })),
            ';' => Some(Ok(LocatedToken {
                begin,
                end: begin + 1,
                token: Token::Separator(Separator::Semicolon),
            })),
            '(' => Some(Ok(LocatedToken {
                begin,
                end: begin + 1,
                token: Token::Separator(Separator::LeftParen),
            })),
            ')' => Some(Ok(LocatedToken {
                begin,
                end: begin + 1,
                token: Token::Separator(Separator::RightParen),
            })),
            '{' => Some(Ok(LocatedToken {
                begin,
                end: begin + 1,
                token: Token::Separator(Separator::LeftBrace),
            })),
            '}' => Some(Ok(LocatedToken {
                begin,
                end: begin + 1,
                token: Token::Separator(Separator::RightBrace),
            })),

            // Operators
            '+' => Some(Ok(LocatedToken {
                begin,
                end: begin + 1,
                token: Token::Operator(Operator::Plus),
            })),
            '-' => Some(Ok(LocatedToken {
                begin,
                end: begin + 1,
                token: Token::Operator(Operator::Minus),
            })),
            '%' => Some(Ok(LocatedToken {
                begin,
                end: begin + 1,
                token: Token::Operator(Operator::Modulo),
            })),
            '=' => Some(Ok(LocatedToken {
                begin,
                end: begin + 1,
                token: Token::Operator(Operator::Equals),
            })),
            '/' => Some(Ok(LocatedToken {
                begin,
                end: begin + 1,
                token: Token::Operator(Operator::Division),
            })),
            '<' => Some(Ok(LocatedToken {
                begin,
                end: begin + 1,
                token: Token::Operator(Operator::LessThan),
            })),
            '!' => Some(Ok(LocatedToken {
                begin,
                end: begin + 1,
                token: Token::Operator(Operator::BitwiseNot),
            })),
            '|' => Some(Ok(LocatedToken {
                begin,
                end: begin + 1,
                token: Token::Operator(Operator::BitwiseOr),
            })),
            '&' => Some(Ok(LocatedToken {
                begin,
                end: begin + 1,
                token: Token::Operator(Operator::BitwiseAnd),
            })),
            '^' => Some(Ok(LocatedToken {
                begin,
                end: begin + 1,
                token: Token::Operator(Operator::BitwiseXor),
            })),
            '>' => Some(Ok(LocatedToken {
                begin,
                end: begin + 1,
                token: Token::Operator(Operator::GreaterThan),
            })),
            '*' => Some(Ok(LocatedToken {
                begin,
                end: begin + 1,
                token: Token::Operator(Operator::Multiplication),
            })),

            // Unexpected character
            c => Some(Err(LocatedError {
                begin,
                end: begin + 1,
                error: LexerError::UnexpectedCharacter(c),
            })),
        }
    }
}
