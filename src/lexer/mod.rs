pub mod tokens;

use std::iter::Peekable;
use std::str::CharIndices;

use tokens::{Keyword, Literal, Operator, Separator, Token};

/// A result from the Lexer, it is either a token or an error located in the input string 
pub type Result<'i> = std::result::Result<Located<Token<'i>>, Located<Err<'i>>>;

/// Some value with a location in the input string
#[derive(Debug)]
pub struct Located<T> {
    /// The index of the beginning of the token
    pub begin: usize,
    /// The index of the end of the token
    pub end: usize,
    /// The contained value 
    pub value: T,
}

/// An error from the lexer
#[derive(Debug)]
pub enum Err<'i> {
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
    type Item = Result<'i>;

    /// Returns the next token in the file
    ///
    /// # Returns
    /// If there is a next token, it will be returned. Otherwise, `None` will be returned.
    fn next(&mut self) -> Option<Self::Item> {
        // None will be returned if there are no more characters to lex
        let (begin, c) = self.skip_whitespace()?;

        match c {
            // Inlined Comments
            '#' => {
                let end = self.consume_while(|c| c != '\r' && c != '\n');
                Some(Ok(Located {
                    begin,
                    end,
                    value : Token::Comment(&self.input[begin..end]),
                }))
            }

            // Identifiers, Keywords and Boolean Literals
            'a'..='z' | 'A'..='Z' | '_' => {
                let end = self.consume_while(|c| c.is_alphanumeric());
                match &self.input[begin..end] {
                    // Keywords
                    "fn" => Some(Ok(Located {
                        begin,
                        end,
                        value: Token::Keyword(Keyword::Fn),
                    })),
                    "let" => Some(Ok(Located {
                        begin,
                        end,
                        value: Token::Keyword(Keyword::Let),
                    })),
                    "if" => Some(Ok(Located {
                        begin,
                        end,
                        value: Token::Keyword(Keyword::If),
                    })),
                    "match" => Some(Ok(Located {
                        begin,
                        end,
                        value: Token::Keyword(Keyword::Match),
                    })),
                    "else" => Some(Ok(Located {
                        begin,
                        end,
                        value: Token::Keyword(Keyword::Else),
                    })),
                    "while" => Some(Ok(Located {
                        begin,
                        end,
                        value: Token::Keyword(Keyword::While),
                    })),
                    "for" => Some(Ok(Located {
                        begin,
                        end,
                        value: Token::Keyword(Keyword::For),
                    })),
                    "loop" => Some(Ok(Located {
                        begin,
                        end,
                        value: Token::Keyword(Keyword::Loop),
                    })),

                    // Boolean Literals
                    "true" => Some(Ok(Located {
                        begin,
                        end,
                        value: Token::Literal(Literal::Boolean(true)),
                    })),
                    "false" => Some(Ok(Located {
                        begin,
                        end,
                        value: Token::Literal(Literal::Boolean(false)),
                    })),

                    // Identifiers
                    _ => Some(Ok(Located {
                        begin,
                        end,
                        value: Token::Identifier(&self.input[begin..end]),
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
                    Ok(num) => Some(Ok(Located {
                        begin,
                        end,
                        value: Token::Literal(Literal::Integer(num)),
                    })),
                    Err(_) => Some(Err(Located {
                        begin,
                        end,
                        value: Err::IntegerParseError(&self.input[begin..end]),
                    })),
                }
            }

            // String Literals
            '"' => {
                // TODO: Handle escape sequences
                let end = self.consume_while(|c| c != '"');
                Some(Ok(Located {
                    begin,
                    end,
                    value: Token::Literal(Literal::String(&self.input[begin + 1..end])),
                }))
            }

            // Separators
            ',' => Some(Ok(Located {
                begin,
                end: begin + 1,
                value: Token::Separator(Separator::Comma),
            })),
            ':' => Some(Ok(Located {
                begin,
                end: begin + 1,
                value: Token::Separator(Separator::Colon),
            })),
            ';' => Some(Ok(Located {
                begin,
                end: begin + 1,
                value: Token::Separator(Separator::Semicolon),
            })),
            '(' => Some(Ok(Located {
                begin,
                end: begin + 1,
                value: Token::Separator(Separator::LeftParen),
            })),
            ')' => Some(Ok(Located {
                begin,
                end: begin + 1,
                value: Token::Separator(Separator::RightParen),
            })),
            '{' => Some(Ok(Located {
                begin,
                end: begin + 1,
                value: Token::Separator(Separator::LeftBrace),
            })),
            '}' => Some(Ok(Located {
                begin,
                end: begin + 1,
                value: Token::Separator(Separator::RightBrace),
            })),

            // Operators
            '+' => Some(Ok(Located {
                begin,
                end: begin + 1,
                value: Token::Operator(Operator::Plus),
            })),
            '-' => Some(Ok(Located {
                begin,
                end: begin + 1,
                value: Token::Operator(Operator::Minus),
            })),
            '%' => Some(Ok(Located {
                begin,
                end: begin + 1,
                value: Token::Operator(Operator::Modulo),
            })),
            '=' => Some(Ok(Located {
                begin,
                end: begin + 1,
                value: Token::Operator(Operator::Equals),
            })),
            '/' => Some(Ok(Located {
                begin,
                end: begin + 1,
                value: Token::Operator(Operator::Division),
            })),
            '<' => Some(Ok(Located {
                begin,
                end: begin + 1,
                value: Token::Operator(Operator::LessThan),
            })),
            '!' => Some(Ok(Located {
                begin,
                end: begin + 1,
                value: Token::Operator(Operator::BitwiseNot),
            })),
            '|' => Some(Ok(Located {
                begin,
                end: begin + 1,
                value: Token::Operator(Operator::BitwiseOr),
            })),
            '&' => Some(Ok(Located {
                begin,
                end: begin + 1,
                value: Token::Operator(Operator::BitwiseAnd),
            })),
            '^' => Some(Ok(Located {
                begin,
                end: begin + 1,
                value: Token::Operator(Operator::BitwiseXor),
            })),
            '>' => Some(Ok(Located {
                begin,
                end: begin + 1,
                value: Token::Operator(Operator::GreaterThan),
            })),
            '*' => Some(Ok(Located {
                begin,
                end: begin + 1,
                value: Token::Operator(Operator::Multiplication),
            })),

            // Unexpected character
            c => Some(Err(Located {
                begin,
                end: begin + 1,
                value: Err::UnexpectedCharacter(c),
            })),
        }
    }
}
