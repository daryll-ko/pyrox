use std::env;

pub mod error;
pub mod run;
pub mod scanner;
pub mod token;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        panic!("(｡>﹏<) too many arguments...")
    } else if args.len() == 2 {
        let file_path = &args[1];
        run::run_file(&file_path);
    } else {
        run::run_repl();
    }
}
