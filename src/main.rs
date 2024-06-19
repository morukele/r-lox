use std::{env, process};
pub use lox;

fn main() {
    let args: Vec<String> = env::args().collect();

    // call the main function in the lox library
    lox::run(args);
}
