use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::process;

pub fn run(args: Vec<String>){
    if args.len() > 1 {
        println!("Usage: rlox [script]");
        process::exit(64);
    } else if args.len() == 2 {
        run_file(&args[1]).expect("TODO: panic message");
    } else {
        run_prompt();
    }
}

fn run_file(file_path: &String) -> Result<(), dyn Error>{
    let file = File::open(Path::new(file_path))?;
    let mut buf_reader = BufReader::new(file);

    Ok(())
}

fn run_prompt(){
 todo!()
}