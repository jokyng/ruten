mod lexer;
mod parser;
mod interpreter;
mod modules;
mod repl;
mod error;

use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        // start interactive repl
        repl::start();
    } else if args.len() == 2 {
        // run script file
        let filename = &args[1];
        run_file(filename);
    } else {
        eprintln!("usage: ruten [script.rtn]");
        process::exit(1);
    }
}

fn run_file(filename: &str) {
    let source = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("error reading file '{}': {}", filename, err);
            process::exit(1);
        }
    };

    if let Err(err) = run(&source) {
        eprintln!("{}", err);
        process::exit(1);
    }
}

fn run(source: &str) -> Result<(), error::RutenError> {
    let tokens = lexer::tokenize(source)?;
    let ast = parser::parse(tokens)?;
    let mut interpreter = interpreter::Interpreter::new();
    interpreter.eval_program(ast)?;
    Ok(())
}