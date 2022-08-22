#![allow(dead_code)]

mod error;
mod scanner;

use anyhow::{Context, Ok, Result};
use scanner::Scanner;
use std::env;
use std::fs;
use std::io;

fn run(input: &str) {
    let mut scanner = Scanner::new(input.to_string());
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("{:?}", token);
    }
}

fn run_file(filename: &str) -> Result<()> {
    let content = fs::read_to_string(filename)
        .with_context(|| format!("Failed to read file from {}", filename))?;

    println!("{}", content);
    Ok(())
}

fn run_prompt() -> Result<()> {
    loop {
        println!("> ");
        let mut raw_input = String::new();

        io::stdin()
            .read_line(&mut raw_input)
            .expect("Failed to read line");

        let input = raw_input.trim();
        if input.is_empty() {
            continue;
        }
        run(input);
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => run_prompt()?,
        2 => run_file(&args[1])?,
        _ => panic!("Usage: cargo run <filename>"),
    };

    Ok(())
}
