use std::env;
use lox::Lox;

fn main() {
    let args: Vec<String> = env::args().collect();

    // call the main function in the lox library
    let lox = Lox::new();
    lox.main(args);
}
