use std::env;

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
    println!("Running file [{file_path}]...")
}

fn run_repl() -> () {
    println!("Running REPL...")
}
