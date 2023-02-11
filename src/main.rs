// #[macro_use]
// extern crate lalrpop_util;

// lalrpop_mod!(pub parser);

use lang::error::Location;
use lang::lexer::Lexer;

fn main() {
    let source = std::fs::read_to_string("tests/sources/fib.l").unwrap();

    let lexer = Lexer::new(&source);

    for token in lexer {
        match token {
            Ok(token) => {
                let loc = Location::new(&source, token.begin);
                println!("{}:{} -- {:?}", loc.line, loc.column, token.token);
            }
            Err(err) => {
                let loc = Location::new(&source, err.begin);
                eprintln!("{}:{} -- {:?}", loc.line, loc.column, err.error);
                std::process::exit(2);
            }
        }
    }
}
