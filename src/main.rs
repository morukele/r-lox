use lox::Lox;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // call the main function in the lox library
    let mut lox = Lox::new();
    lox.main(args);
}
