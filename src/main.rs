use std::io::{stdin, BufRead, BufReader};
use std::result::Result;
use std::{env, fs};

use rlox::error::LoxError;
use rlox::scanner::Scanner;

fn main() -> Result<(), LoxError> {
    let mut args = env::args();

    match args.len() {
        1 => repl(),
        2 => run_file(&(args.nth(1).unwrap())),
        _ => usage(),
    }
}

fn run_file(path: &str) -> Result<(), LoxError> {
    let source = fs::read_to_string(path);

    match source {
        Ok(source) => {
            run(source);
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    }
    Ok(())
}

fn repl() -> Result<(), LoxError> {
    let input = BufReader::new(stdin());
    print_cursor(1);

    for (line, src) in input.lines().enumerate() {
        let source = src;

        match source {
            Ok(source) => {
                run(source);
                print_cursor(line + 1);
            }
            Err(_err) => { /* Ignore error*/ }
        }
    }

    Ok(())
}

fn run(source: String) -> Result<(), LoxError> {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens()?;

    for token in tokens {
        println!("{:?}", token);
    }

    Ok(())
}

fn print_cursor(line: usize) {
    eprint!("[{:03}]> ", line)
}

fn usage() -> Result<(), LoxError> {
    eprintln!("Usage: rlox [path]");
    Ok(())
}
