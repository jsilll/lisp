use lisp::frontend::lexer::{Lexer, Located};
use lisp::frontend::parser::Parser;
use lisp::frontend::token::Token;

#[test]
fn test_lexer() {
    let source = "(define (plus_one x) (+ x 1))";

    let mut lexer = Lexer::new(&source);

    assert_eq!(
        lexer.next(),
        Some(Ok(Located {
            begin: 0,
            end: 1,
            value: Token::LParen,
        }))
    );
    assert_eq!(
        lexer.next(),
        Some(Ok(Located {
            begin: 1,
            end: 7,
            value: Token::Symbol("define"),
        }))
    );
    assert_eq!(
        lexer.next(),
        Some(Ok(Located {
            begin: 8,
            end: 9,
            value: Token::LParen,
        }))
    );
    assert_eq!(
        lexer.next(),
        Some(Ok(Located {
            begin: 9,
            end: 17,
            value: Token::Symbol("plus_one"),
        }))
    );
    assert_eq!(
        lexer.next(),
        Some(Ok(Located {
            begin: 18,
            end: 19,
            value: Token::Symbol("x"),
        }))
    );
    assert_eq!(
        lexer.next(),
        Some(Ok(Located {
            begin: 19,
            end: 20,
            value: Token::RParen,
        }))
    );
    assert_eq!(
        lexer.next(),
        Some(Ok(Located {
            begin: 21,
            end: 22,
            value: Token::LParen,
        }))
    );
    assert_eq!(
        lexer.next(),
        Some(Ok(Located {
            begin: 22,
            end: 23,
            value: Token::Symbol("+"),
        }))
    );
    assert_eq!(
        lexer.next(),
        Some(Ok(Located {
            begin: 24,
            end: 25,
            value: Token::Symbol("x"),
        }))
    );
    assert_eq!(
        lexer.next(),
        Some(Ok(Located {
            begin: 26,
            end: 27,
            value: Token::Integer(1),
        }))
    );
}

#[test]
fn test_parser() {
    let source = "(define (plus_one x) (+ x 1))";
    let lexer = Lexer::new(&source);
    let mut parser = Parser::new(lexer);

    let expr = parser.parse().unwrap();

    assert_eq!(expr.to_string(), "(define (plus_one x) (+ x 1))");
}
