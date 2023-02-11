/// Tokens are the smallest unit of the language. They are the output of the lexer.
#[derive(Debug)]
pub enum Token<'i> {
    Keyword(Keyword),
    Operator(Operator),
    Separator(Separator),
    Comment(&'i str),
    Identifier(&'i str),
    Literal(Literal<'i>),
}

#[derive(Debug)]
pub enum Keyword {
    Fn,
    Let,
    If,
    Else,
    Match,
    While,
    For,
}

#[derive(Debug)]
pub enum Literal<'i> {
    Float(f64),
    Double(f64),
    Integer(i64),
    Boolean(bool),
    String(&'i str),
}

#[derive(Debug)]
pub enum Operator {
    Plus,
    Minus,
    Multiplication,
    Division,
    Modulo,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Equals,
    NotEquals,
    LogicalAnd,
    LogicalOr,
    LogicalNot,
    BitwiseAnd,
    BitwiseOr,
    BitwiseNot,
    BitwiseXor,
    BitwiseLeftShift,
    BitwiseRightShift,
}

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
