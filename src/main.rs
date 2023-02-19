#![warn(clippy::all)]

#[macro_use]
extern crate lalrpop_util;

// lalrpop_mod!(#[allow(clippy::all)] pub parser);

use lang::lexer::Lexer;
// use lang::parser::ExprParser;
use lang::location::FileLocation;

fn main() {
    // TODO: Implement Parser 
    // let input = "22 * pi + 66";
    // let lexer = Lexer::new(input);
    // let expr = ExprParser::new().parse(input, lexer).unwrap();
    // println!("{:?}", expr);

    let path = "tests/sources/fib.l";
    let source = std::fs::read_to_string(path).unwrap_or_else(|err| {
        panic!("Failed to read file {}: {}", path, err);
    });

    let lexer = Lexer::new(&source);

    for token in lexer {
        match token {
            Ok(token) => {
                let loc = FileLocation::new(path, &source, token.begin);
                let token = token.value;
                println!("{:?} at {}", token, loc);
            },
            Err(err) => {
                let loc = FileLocation::new(path, &source, err.begin);
                let err = err.value;
                println!("{:?} at {}", err, loc);
            }
        }
    }
}