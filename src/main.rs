#![warn(clippy::all)]

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(#[allow(clippy::all)] pub parser);

use lang::lexer::Lexer;
use lang::parser::ExprParser;
// use lang::error::FileLocation;

fn main() {
    let input = "22 * pi + 66";
    let lexer = Lexer::new(input);
    let expr = ExprParser::new().parse(input, lexer).unwrap();
    println!("{:?}", expr);

    // let source = std::fs::read_to_string("tests/sources/fib.l").unwrap();
    // let lexer = Lexer::new(&source);
    // for token in lexer {
    //     match token {
    //         Ok(token) => {
    //             let loc = FileLocation::new(&source, token.begin);
    //             println!("{}:{} -- {:?}", loc.line, loc.column, token.token);
    //         }
    //         Err(err) => {
    //             let loc = FileLocation::new(&source, err.begin);
    //             eprintln!("{}:{} -- {:?}", loc.line, loc.column, err.error);
    //             std::process::exit(2);
    //         }
    //     }
    // }
}
