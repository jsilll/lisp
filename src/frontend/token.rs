/// A token in the input.
#[derive(Debug, PartialEq)]
pub enum Token<'i> {
    /// A left parenthesis.
    LParen,
    /// A right parenthesis.
    RParen,

    /// A plus sign.
    Plus,
    /// A minus sign.
    Minus,

    /// An integer.
    Integer(i64),
    /// A symbol.
    Symbol(&'i str),
}

impl<'i> std::fmt::Display for Token<'i> {
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
    /// use lisp::frontend::token::Token;
    /// 
    /// let token = Token::Plus;
    /// assert_eq!(format!("{}", token), "+");
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::Integer(i) => write!(f, "{}", i),
            Token::Symbol(s) => write!(f, "{}", s),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests the `fmt` method.
    #[test]
    fn test_fmt() {
        let token = Token::Plus;
        assert_eq!(format!("{}", token), "+");

        let token = Token::Minus;
        assert_eq!(format!("{}", token), "-");

        let token = Token::LParen;
        assert_eq!(format!("{}", token), "(");

        let token = Token::RParen;
        assert_eq!(format!("{}", token), ")");

        let token = Token::Integer(42);
        assert_eq!(format!("{}", token), "42");

        let token = Token::Symbol("foo");
        assert_eq!(format!("{}", token), "foo");
    }
}