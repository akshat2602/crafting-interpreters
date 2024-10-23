use rlox::{run_file, run_prompt};
use std::{env, error::Error, process::exit};

// Takes in an argument, the file name to be executed,
// if no file name is provided then enters REPL mode.
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => run_prompt(),
        2 => run_file(&args[1]),
        _ => {
            eprintln!("Usage: rlox [script]");
            exit(64);
        }
    }
}
