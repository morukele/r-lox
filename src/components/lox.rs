use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::process;
use crate::{Scanner, Token};

// main Lox class for running the code
#[derive(Default)]
pub struct Lox {
    pub has_error: bool
}

impl Lox {
    pub fn new() -> Self {
        Lox {
            has_error: false
        }
    }
    pub fn main(&self, args: Vec<String>) {
        if args.len() > 1 {
            println!("Usage: rlox [script]");
            process::exit(64);
        } else if args.len() == 2 {
            self.run_file(&args[1]).expect("TODO: panic message");
        } else {
            self.run_prompt();
        }
    }

    pub fn run_file(&self, file_path: &String) -> Result<(), dyn Error> {
        let file = File::open(Path::new(file_path));
        match file {
            Ok(file) => {
                let mut buf_reader = BufReader::new(file);
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    pub fn run_prompt(&self) {
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
                    }
                }
                Err(e) => {
                    eprintln!("Error: {e}")
                }
            }
        }
    }

    pub fn run(&self, source: &String) {
        let scanner = Scanner::new(source);
        let tokens: Vec<Token> = scanner.scan_tokens();

        // print the tokens to screen
        for token in tokens {
            println!("{token}")
        }
    }
}

