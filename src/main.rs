use std::io::{stdin, BufRead, BufReader, Error};
use std::rc::Rc;
use std::result::Result;
use std::{env, fs};

fn main() -> Result<(), Error> {
    let mut args = env::args();

    match args.len() {
        1 => repl(),
        2 => run_file(&(args.nth(1).unwrap())),
        _ => usage(),
    }
}

fn run_file(path: &str) -> Result<(), Error> {
    let source = Rc::new(fs::read_to_string(path)?);

    println!("{:?}", source);
    Ok(())
}

fn repl() -> Result<(), Error> {
    let input = BufReader::new(stdin());
    print_cursor(1);

    for (line, src) in input.lines().enumerate() {
        let source = Rc::new(src?);

        print_cursor(line + 1);
        println!("{}", source);
    }

    Ok(())
}

fn print_cursor(line: usize) {
    eprint!("[{:03}]> ", line)
}

fn usage() -> Result<(), Error> {
    eprintln!("Usage: rlox [path]");
    Ok(())
}
