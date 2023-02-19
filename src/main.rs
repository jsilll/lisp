#![warn(clippy::all)]

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(#[allow(clippy::all)] pub parser);

use lang::display::Location;
use lang::lexer::Lexer;
use lang::parser::ProgramParser;

fn main() {
    // TODO: Read arguments from the cli using clap.

    let path = "tests/sources/fib.l";
    let source = std::fs::read_to_string(path).unwrap_or_else(|err| {
        panic!("Failed to read file {}: {}", path, err);
    });

    let lexer = Lexer::new(&source);

    let program = ProgramParser::new().parse(lexer).unwrap_or_else(|err| {
        let loc = Location::new(path, &source, err.begin);
        let err = err.value;
        panic!("{:?} at {}", err, loc);
    });

    // for token in lexer {
    //     match token {
    //         Ok(token) => {
    //             let loc = Location::new(path, &source, token.begin);
    //             let token = token.value;
    //             println!("{:?} at {}", token, loc);
    //         }
    //         Err(err) => {
    //             let loc = Location::new(path, &source, err.begin);
    //             let err = err.value;
    //             println!("{:?} at {}", err, loc);
    //         }
    //     }
    // }
}
