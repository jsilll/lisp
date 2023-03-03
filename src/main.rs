use std::io::Write;

use lisp::display::location::Location;

use lisp::frontend::lexer::Lexer;
use lisp::frontend::parser::{Err as ParserError, Parser};

use lisp::repl::Evaluator;

/// Evaluate the given source code.
///
/// # Arguments
/// - `source`: The source code to evaluate.
///
/// # Returns
/// The result of the evaluation.
fn eval(source: &str) -> Result<String, String> {
    let lexer = Lexer::new(&source);
    let mut parser = Parser::new(lexer);
    match parser.parse() {
        Err(err) => {
            let idx = match &err {
                ParserError::UnexpectedEndOfFile => source.len(),
                ParserError::LexerError(err) => err.begin,
                ParserError::UnexpectedToken(located) => located.begin,
            };
            let loc = Location::new("", &source, idx);

            Result::Err(format!("Error: {}: {}.", loc, err))
        }
        Ok(ast) => {
            dbg!(&ast);
            let evaluator = Evaluator::new(ast);
            let result = evaluator.eval();
            Result::Ok(format!("{}", result))
        }
    }
}

fn main() {
    println!("Welcome to the lisp repl! Type 'exit' to exit.");
    loop {
        print!("> ");
        std::io::stdout().flush().unwrap_or_else(|err| {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        });

        let mut source = String::new();
        std::io::stdin()
            .read_line(&mut source)
            .unwrap_or_else(|err| {
                eprintln!("Error: {}", err);
                std::process::exit(1);
            });

        match source.trim() {
            "exit" => break,
            _ if !source.is_ascii() => {
                eprintln!("Error: Input must contain only valid ascii characters.");
                continue;
            }
            _ => match eval(&source) {
                Err(err) => eprintln!("{}", err),
                Ok(result) => println!("{}", result),
            },
        }
    }
}
