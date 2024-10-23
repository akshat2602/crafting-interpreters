use std::{env, error::Error, fs, io::stdin, process::exit};

fn run(code: String) -> Result<(), Box<dyn Error>> {
    Ok(())
}

fn run_prompt() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();

    loop {
        print!("> ");
        match stdin().read_line(&mut buffer) {
            Ok(_) => {}
            Err(err) => {
                eprintln!("Error reading input: {}", err);
                exit(1);
            }
        };
        run(buffer.clone())?;
        buffer.clear();
    }
}

fn run_file(file_name: &String) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(file_name).expect("Error reading the file");
    run(contents)?;

    Ok(())
}

// Takes in an argument, the file name to be executed,
// if no file name is provided then enters REPL mode.
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: rlox [script]");
        exit(64);
    } else if args.len() == 2 {
        run_file(&args[1])
    } else {
        run_prompt()
    }
}
