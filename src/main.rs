use std::env;
use std::fs;
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        panic!("(｡>﹏<) too many arguments...")
    } else if args.len() == 2 {
        let file_path = &args[1];
        run_file(&file_path);
    } else {
        run_repl();
    }
}

fn run_file(file_path: &String) -> () {
    println!("Running file [ {file_path} ]...");
    let contents =
        fs::read_to_string(file_path).unwrap_or("(｡>﹏<) couldn't read file...".to_string());
    println!("Contents:\n{contents}")
}

fn run_repl() -> () {
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
            println!("Arona received [ {line} ]!")
        } else {
            break;
        }
    }
}
