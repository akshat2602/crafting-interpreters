use scanner::Scanner;
use std::{error::Error, fs, io::stdin, process::exit};

mod scanner;

fn run(code: String) -> Result<(), Box<dyn Error>> {
    let mut scanner = Scanner::new(code);
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("{}", token);
    }

    // If error occurs then exit with code 65

    Ok(())
}

pub fn run_prompt() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();

    loop {
        print!(">> ");
        match stdin().read_line(&mut buffer) {
            Ok(s) => {
                if s == 0 {
                    () // EOF
                }
            }
            Err(err) => {
                eprintln!("Error reading input: {}", err);
                exit(1);
            }
        };
        run(buffer.clone())?;
        buffer.clear();
    }
}

pub fn run_file(file_name: &String) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(file_name).expect("Error reading the file");
    run(contents)?;

    Ok(())
}

pub fn error(line: usize, message: &str) {
    report(line, "", message);
}

fn report(line: usize, location: &str, message: &str) {
    eprintln!("[line {}] Error {}: {}", line, location, message);
}
