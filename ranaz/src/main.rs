use std::env;
use std::fs::File;
use std::io::prelude::*;

fn analyze(filename : &str) -> std::io::Result<()> { 
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("{}",contents);
    Ok(())
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    
    match args.len() {
        // no arguments passed
        2 => {
            let filename = &args[1];
            analyze(filename)?;
        },
        _ => {
            println!("This is an unstoppable killing machine");
        }
    };
    Ok(())
}
