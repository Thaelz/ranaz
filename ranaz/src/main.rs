use std::env;
use std::fs::File;
use std::io::prelude::*;

fn countOnesInByte(b: Byte) -> u8 {
    let mut c = 0;
    for x in 0..8 {
        if b & 1 == 1 {
            c += 1;
        }
    }
    c
}

fn analyze(filename : &str) -> std::io::Result<()> { 
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    for byte in contents.bytes();
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
