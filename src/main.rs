pub mod cli;
pub mod diagnostics;
pub mod interpreter;
pub mod lexer;
pub mod parser;
pub mod utils;

use cli::Cli;
use interpreter::Interpreter;
use lexer::Lexer;
use parser::Parser;

fn main() {
    utils::print_welcome();

    let args = Cli::parse();
    if let Some(path) = args.file_path {
        println!("Reading source file: {}", path);
        match std::fs::read_to_string(&path) {
            Ok(code) => run(&code),
            Err(err) => eprintln!("Error reading file '{}': {}", path, err),
        }
    } else {
        println!("No input file provided. Run with a .kl / .bay file path.");
    }
}

fn run(source: &str) {
    let mut lexer = Lexer::new(source);
    match lexer.tokenize() {
        Ok(tokens) => {
            let mut parser = Parser::new(tokens);
            match parser.parse() {
                Ok(ast) => {
                    let mut interpreter = Interpreter::new();
                    if let Err(err) = interpreter.interpret(&ast) {
                        eprintln!("{}", err);
                    }
                }
                Err(err) => eprintln!("{}", err),
            }
        }
        Err(err) => eprintln!("{}", err),
    }
}
