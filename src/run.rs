use std::fs;
use std::io::{self, Write};

use crate::scanner::Scanner;

pub fn run_file(file_path: &str) -> () {
    println!("Running file [ {file_path} ]...");
    let contents =
        fs::read_to_string(file_path).unwrap_or("(｡>﹏<) couldn't read file...".to_string());
    run(&contents);
}

pub fn run_repl() -> () {
    println!("Running REPL...");
    loop {
        print!("Arona> ");
        io::stdout()
            .flush()
            .expect("(｡>﹏<) couldn't flush stdout...");

        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("(｡>﹏<) couldn't read a line...");
        let line = line.trim();

        if line.len() > 0 {
            run(&line);
        } else {
            break;
        }
    }
}

fn run(code: &str) -> () {
    let scanner = Scanner::new(code);
    let tokens = scanner.scan_for_tokens();

    for token in tokens {
        println!("{token}")
    }
}
