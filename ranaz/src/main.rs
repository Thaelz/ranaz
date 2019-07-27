
#[macro_use]
extern crate clap;

use clap::App;
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
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let verbose_level = match matches.occurrences_of("v") {
        lvl if lvl < 3 => lvl,
        _ => panic!("Cannot take more than 2 verbose arguments.")
    };

    analyze(matches.value_of("INPUT").unwrap());

    Ok(())
}
