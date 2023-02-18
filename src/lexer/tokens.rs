/// Tokens are the smallest unit of the language.
/// They are the output of the lexer.
#[derive(Debug, PartialEq)]
pub enum Token<'i> {
    /// A comment is a sequence of
    /// characters that is ignored by the compiler.
    Comment(&'i str),
    /// A keyword is a sequence of characters
    /// that has a special meaning to the compiler.
    Keyword(Keyword),
    /// An operator is a sequence
    /// of characters that represents an operation.
    Operator(Operator),
    /// A separator is a token meant to delimit other tokens.
    Separator(Separator),
    /// An identifier is a sequence of
    /// characters that is not a keyword.
    Identifier(&'i str),
    /// A literal is a sequence of characters
    /// that represents a compile-time constant.
    Literal(Literal<'i>),
}

/// A keyword is a sequence of characters
/// that has a special meaning to the compiler.
#[derive(Debug, PartialEq)]
pub enum Keyword {
    /// Used to declare a function.
    Fn,
    /// Used to declare a variable.
    Let,
    /// Used for conditional branching.
    If,
    /// Used for conditional branching.
    Else,
    /// Used for conditional branching.
    Match,
    /// Used for looping.
    While,
    /// Used for looping.
    For,
    /// Used for looping.
    Loop,
}

/// A literal is a sequence of characters
/// that represents a compile-time constant.
#[derive(Debug, PartialEq)]
pub enum Literal<'i> {
    /// A float literal
    Float(f64),
    /// A double 
    Double(f64),
    /// An integer literal
    Integer(i64),
    /// A boolean literal
    Boolean(bool),

    /// A character literal
    Char(char),
    /// A string literal
    String(&'i str),
}

/// An operator is a sequence
/// of characters that represents an operation.
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Operator {
    /// Assignment operator
    Assignment,

    /// Plus operator
    Plus,
    /// Minus operatorj
    Minus,
    /// Modulo operator
    Modulo,
    /// Division operator
    Division,
    /// Multiplication operator
    Multiplication,

    /// Equals operator
    Equals,
    /// Not equals operator
    NotEquals,

    /// Logical or operator
    LogicalOr,
    /// Logical and operator
    LogicalAnd,
    /// Logical not operator
    LogicalNot,

    /// Less than operator
    LessThan,
    /// Greater than operator
    GreaterThan,
    /// Less than or equal to operator
    LessThanOrEqual,
    /// Greater than or equal to operator
    GreaterThanOrEqual,

    /// Bitwise left shift operator
    BitwiseLeftShift,
    /// Bitwise right shift operator
    BitwiseRightShift,

    /// Bitwise or operator
    BitwiseOr,
    /// Bitwise and operator
    BitwiseAnd,
    /// Bitwise not operator
    BitwiseNot,
    /// Bitwise xor operator
    BitwiseXor,
}

/// A separator is a token meant to delimit other tokens.
#[derive(Debug, PartialEq)]
pub enum Separator {
    /// A dot '.'
    Dot,
    /// A comma ','
    Comma,
    /// A colon ':'
    Colon,
    /// A semicolon ';'
    Semicolon,
    /// A left parenthesis '('  
    LeftParen,
    /// A right parenthesis ')'
    RightParen,
    /// A left brace '{'
    LeftBrace,
    /// A right brace '}'
    RightBrace,
    /// A left bracket '['
    LeftBracket,
    /// A right bracket ']'
    RightBracket,
}