use lisp::display::location::Location;

use lisp::frontend::lexer::Lexer;
use lisp::frontend::parser::{Err as PErr, Parser};

use lisp::repl::Evaluator;

fn main() {
    // Getting the source code
    let path = "tests/sources/1.lisp";
    let source = std::fs::read_to_string(path).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    });

    // Generating the AST
    let lexer = Lexer::new(&source);
    let mut parser = Parser::new(lexer);
    let ast = parser.parse().unwrap_or_else(|err| {
        let idx = match &err {
            PErr::UnexpectedEndOfFile => source.len(),
            PErr::LexerError(err) => err.begin,
            PErr::UnexpectedToken(located) => located.begin,
        };
        let loc = Location::new(path, &source, idx);
        eprintln!("{}: {}", loc, err);
        std::process::exit(1);
    });
    
    // Printing the AST
    println!("AST: {}", ast);

    // Evaluating the AST
    let evaluator = Evaluator::new(ast);
    let result = evaluator.eval();

    // Printing the final result
    println!("Result: {}", result);
}
