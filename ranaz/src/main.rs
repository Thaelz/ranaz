#[macro_use]
extern crate clap;

pub mod markov;
pub mod fourier;

use clap::App;
use std::fs::File;
use std::io::prelude::*;

fn analyze(filename : &str) -> std::io::Result<()> { // TODO: pass the File object as parameter?
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("{}",contents);
    Ok(())
}

// TODO: use a logger system and print more if verbose_level is high?

fn main() -> std::io::Result<()> {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let mut input_file = File::open(matches.value_of("INPUT").unwrap())?;

    let verbose_level = match matches.occurrences_of("v") {
        lvl if lvl < 3 => lvl,
        _ => panic!("Cannot take more than 2 verbose arguments.")
    };

    if let Some(matches) = matches.subcommand_matches("markov") {
        if matches.is_present("png") {
            let mut bytes = Vec::<u8>::new();
            input_file.read_to_end(&mut bytes)?;
            let matrix = markov::get_markov_array(&bytes);
            markov::to_img(&matrix,
                matches.value_of("OUTPUT").unwrap());
        }
        else {
            println!("No command => does nothing...")
        }
    }
    else {
        analyze(matches.value_of("INPUT").unwrap());
    }

    Ok(())
}
