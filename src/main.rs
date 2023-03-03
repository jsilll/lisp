use std::io::Write;

use lisp::display::location::Location;

use lisp::frontend::lexer::Lexer;
use lisp::frontend::parser::{Err as ParserError, Parser};

use lisp::repl::Evaluator;

fn main() {
    loop {
        // Print the prompt
        print!("> ");
        std::io::stdout().flush().unwrap_or_else(|err| {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        });

        // Getting input from the user
        let mut source = String::new();
        std::io::stdin()
            .read_line(&mut source)
            .unwrap_or_else(|err| {
                eprintln!("Error: {}", err);
                std::process::exit(1);
            });
        
        // Check source is valid ascii
        if !source.is_ascii() {
            eprintln!("Error: Input must be valid ascii.");
            continue;
        }

        // Initializing the lexer and parser
        let lexer = Lexer::new(&source);
        let mut parser = Parser::new(lexer);

        // Parsing and evaluating the input
        match parser.parse() {
            Err(err) => {
                let idx = match &err {
                    ParserError::UnexpectedEndOfFile => source.len(),
                    ParserError::LexerError(err) => err.begin,
                    ParserError::UnexpectedToken(located) => located.begin,
                };
                let loc = Location::new("", &source, idx);
                eprintln!("Error: {}: {}.", loc, err);
                continue;
            }
            Ok(ast) => {
                println!("{:?}", ast);
                let evaluator = Evaluator::new(ast);
                let result = evaluator.eval();
                println!("{}", result);
            }
        };
    }
}
