use crate::Error::OpenFileError;
use crate::{Error, Scanner};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::process;

// main Lox class for running the code
#[derive(Default)]
pub struct Lox {
    pub has_error: bool,
}

impl Lox {
    pub fn new() -> Self {
        Lox { has_error: false }
    }
    pub fn main(&mut self, args: Vec<String>) {
        if args.len() > 1 {
            println!("Usage: rlox [script]");
            process::exit(64);
        } else if args.len() == 2 {
            self.run_file(&args[1]).expect("TODO: panic message");
        } else {
            self.run_prompt();
        }
    }

    pub fn run_file(&self, file_path: &String) -> Result<(), Error> {
        let file = File::open(Path::new(file_path));
        match file {
            Ok(file) => {
                let mut buf_reader = BufReader::new(file);

                // Indicate an error in the exit code.
                if self.has_error {
                    process::exit(65)
                };

                Ok(())
            }
            Err(err) => Err(OpenFileError(err.to_string())),
        }
    }

    pub fn run_prompt(&mut self) {
        let mut input = String::new();
        loop {
            print!(">");
            let line = std::io::stdin().read_line(&mut input);
            match line {
                Ok(n) => {
                    println!("{n} bytes read");
                    if input == "\n" {
                        break;
                    } else {
                        self.run(&input);
                        self.has_error = false;
                    }
                }
                Err(e) => {
                    eprintln!("Error: {e}")
                }
            }
        }
    }

    pub fn run(&mut self, source: &str) {
        let mut scanner = Scanner::new(source, self);
        let tokens = scanner.scan_tokens();

        // print the tokens to screen
        for token in tokens {
            println!("{:?}", token);
        }
    }

    pub fn error(&mut self, line: i32, message: String) {
        self.report(line, "", message);
    }

    pub fn report(&mut self, line: i32, location: &str, message: String) {
        eprintln!("[line {line} ] Error {location}: {message}");
        self.has_error = true;
    }
}
