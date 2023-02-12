/// Tokens are the smallest unit of the language.
/// They are the output of the lexer.
#[derive(Debug)]
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
#[derive(Debug)]
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
#[derive(Debug)]
pub enum Literal<'i> {
    Float(f64),
    Double(f64),
    Integer(i64),
    Boolean(bool),
    
    Char(char),
    String(&'i str),
}

/// An operator is a sequence
/// of characters that represents an operation.
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Operator {
    // Arithmetic
    Plus,
    Minus,
    Modulo,
    Division,
    Multiplication,

    // Logical
    Equals,
    NotEquals,

    LogicalOr,
    LogicalAnd,
    LogicalNot,

    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,

    // Bitwise
    BitwiseLeftShift,
    BitwiseRightShift,

    // Bitwise Logical
    BitwiseOr,
    BitwiseAnd,
    BitwiseNot,
    BitwiseXor,
}

/// A separator is a token meant to delimit other tokens.
#[derive(Debug)]
pub enum Separator {
    Comma,
    Colon,
    Semicolon,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
}