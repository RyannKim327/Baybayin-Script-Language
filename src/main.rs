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
    if let Some(path) = args.file_path && path.ends_with(".bay") {
        match std::fs::read_to_string(&path) {
            Ok(code) => run(&code),
            Err(err) => eprintln!("Error reading file '{}': {}", path, err),
        }
    } else {
        println!("ᜏᜎᜅ᜔ ᜈᜃᜎᜀᜅ᜔ ᜆᜎᜃ᜔ᜐᜈ᜔᜶ ᜄᜋᜒᜆᜒᜈ᜔ ᜁᜆᜓ ᜈ ᜄᜋᜒᜆ᜔ ᜀᜅ᜔ .bay ᜈ ᜉᜈᜓᜃᜓᜌ᜔ ᜈᜅ᜔ ᜆᜎᜃ᜔ᜐᜈ᜔᜶");
        println!("Walang nakalaang talaksan. Gamitin ito na gamit ang .bay na panukoy ng talaksan.");
        println!("No input file provided. Run with a .bay file extension.");
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
