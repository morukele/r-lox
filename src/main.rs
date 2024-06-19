use lox;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // call the main function in the lox library
    lox::main(args);
}
