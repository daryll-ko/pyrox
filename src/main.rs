use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        panic!("too many arguments")
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
        fs::read_to_string(file_path).unwrap_or("[ could not read file... ]".to_string());
    println!("Contents:\n{contents}")
}

fn run_repl() -> () {
    println!("Running REPL...")
}
