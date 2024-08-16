use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        panic!("too many arguments")
    } else if args.len() == 2 {
        run_file();
        let file_path = &args[1];
        println!("{file_path}")
    } else {
        run_repl();
    }
}

fn run_file() -> () {
    println!("Running file...")
}

fn run_repl() -> () {
    println!("Running REPL...")
}
