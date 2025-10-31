use crate::{error::RutenError, interpreter::Interpreter, lexer, parser};
use colored::Colorize;
use std::io::{self, Write};

pub fn start() {
    println!("{}", "ruten repl v2.0.0".bold().cyan());
    println!("{}", "blazingly fast scripting language".dimmed());
    println!("{}", "made by ogcae\n".dimmed());
    println!("{}", "type 'exit' or 'quit' to exit\n".dimmed());

    let mut interpreter = Interpreter::new();
    let mut line_number = 1;

    loop {
        // print prompt
        print!("{} ", format!("[{}]>", line_number).green().bold());
        io::stdout().flush().unwrap();

        // read input
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim();

                // check for exit commands
                if input == "exit" || input == "quit" {
                    println!("{}", "goodbye!".cyan());
                    break;
                }

                // skip empty lines
                if input.is_empty() {
                    continue;
                }

                // check for special commands
                if input == "help" {
                    print_help();
                    continue;
                }

                if input == "clear" {
                    clear_screen();
                    continue;
                }

                // evaluate input
                match eval_line(&mut interpreter, input) {
                    Ok(Some(value)) => {
                        println!("{}", format!("=> {}", value.to_string()).yellow());
                    }
                    Ok(None) => {}
                    Err(err) => {
                        eprintln!("{}", format!("{}", err).red());
                    }
                }

                line_number += 1;
            }
            Err(err) => {
                eprintln!("{}", format!("error reading input: {}", err).red());
                break;
            }
        }
    }
}

fn eval_line(interpreter: &mut Interpreter, input: &str) -> Result<Option<crate::interpreter::Value>, RutenError> {
    // tokenize
    let tokens = lexer::tokenize(input)?;

    // parse
    let ast = parser::parse(tokens)?;

    // check if it's a single expression statement
    if ast.len() == 1 {
        if let crate::parser::Stmt::Expression(expr) = &ast[0] {
            let value = interpreter.eval_expr(expr)?;
            return Ok(Some(value));
        }
    }

    // otherwise evaluate as statements
    interpreter.eval_program(ast)?;
    Ok(None)
}

fn print_help() {
    println!("\n{}", "ruten repl commands:".bold().cyan());
    println!("  {}  - show this help message", "help".green());
    println!("  {}  - clear the screen", "clear".green());
    println!("  {}  - exit the repl", "exit/quit".green());
    println!("\n{}", "language features:".bold().cyan());
    println!("  - variables: {} or {}", "x = 10".yellow(), "name = \"ruten\"".yellow());
    println!("  - functions: {}", "def add(a, b): return a + b".yellow());
    println!("  - control flow: {}, {}, {}", "if".yellow(), "while".yellow(), "for".yellow());
    println!("  - data structures: {}, {}", "[1, 2, 3]".yellow(), "{\"key\": \"value\"}".yellow());
    println!("  - modules: {}", "import math".yellow());
    println!("\n{}", "available modules:".bold().cyan());
    println!("  {}", "core:".bold());
    println!("    math, strings, json, crypto, http, webhook");
    println!("  {}", "apis:".bold());
    println!("    weather, crypto_prices, github, news, quotes");
    println!("    jokes, facts, geo, blockchain");
    println!("  {}", "utilities:".bold());
    println!("    stats, utils, database, email, image, logger, testing");
    println!("  {}", "advanced:".bold());
    println!("    subproc, multiproc, downloads, ai, vision, nlp");
    println!("    torrent, apps, format");
    println!();
}

fn clear_screen() {
    if cfg!(target_os = "windows") {
        std::process::Command::new("cmd")
            .args(&["/C", "cls"])
            .status()
            .ok();
    } else {
        std::process::Command::new("clear")
            .status()
            .ok();
    }
}