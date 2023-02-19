#![warn(clippy::all)]

use lang::lexer::Lexer;
use lang::parser::Parser;
use lang::display::Location;

fn main() {
    // Read the source code from a file.
    let path = "tests/sources/fib.la";
    let source = std::fs::read_to_string(path).unwrap_or_else(|err| {
        eprintln!("Failed to read file {}: {}", path, err);
        std::process::exit(1);
    });

    // Initialize the lexer and parser.
    let mut parser = Parser::new(Lexer::new(&source));

    // Parse the source code.
    #[allow(unused)]
    let ast = parser.parse().unwrap_or_else(|err| {
        match err {
            lang::parser::Err::LexerError(err) => {
                let location = Location::new(path, &source, err.begin);
                match err.value {
                    lang::lexer::Err::IntegerParseError(err) => {
                        eprintln!("Failed to parse integer at {}: {:?}", location, err);
                        std::process::exit(1);
                    }
                    lang::lexer::Err::UnexpectedCharacter(err) => {
                        eprintln!("Unexpected character at {}: {:?}", location, err);
                        std::process::exit(1);
                    }
                }
            }
            lang::parser::Err::UnexpectedToken(token) => {
                let location = Location::new(path, &source, token.begin);
                eprintln!("Unexpected token at {}: {:?}", location, token.value);
                std::process::exit(1);
            }
        }
    });

    // Print the produced AST.
    println!("{:#?}", ast);
}
