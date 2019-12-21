
#[macro_use]
extern crate clap;

pub mod markov;
pub mod fourier;

use clap::App;
use std::fs::File;
use std::io::prelude::*;

fn bit_dibit_byte_stat(s : &Vec<u8>) -> ( [i128; 2], [i128; 4], [i128; 256] ) {
    let mut bits: [i128; 2] = [0; 2];
    let mut dibits: [i128; 4] = [0; 4];
    let mut bytes: [i128; 256] = [0; 256];

    let tot = (s.len() * 8) as i128;

    for byte in s {
        let mut b = *byte;
        for _ in 0..4 {
            let d = (b & 3) as usize;
            dibits[d] += 1;
            b >>= 2;
        }
        bytes[b as usize] += 1;
    }

    bits[1] = dibits[1] + dibits[2] + (dibits[3] << 1);
    bits[0] = tot - bits[1];

    return (bits, dibits, bytes);
}

fn analyze(filename : &str) -> std::io::Result<()> { // TODO: pass the File object as parameter?
    let mut file = File::open(filename)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;

    let (bits, dibits, bytes) = bit_dibit_byte_stat(&mut contents);

    /* Bit test */
    println!("Bit test : Z({}), O({})", bits[0], bits[1]);
    let mut bit_diff = bits[0] - bits[1];
    bit_diff *= bit_diff;
    bit_diff /= bits[0] + bits[1];
    println!("Bit test === {}", bit_diff);

    /* Dibit test */
    println!("Dibit test: 00:{} - 01:{} - 10:{} - 11:{}", dibits[0], dibits[1], dibits[2], dibits[3]);

    /* Byte test */

    Ok(())
}

// TODO: use a logger system and print more if verbose_level is high?

fn main() -> std::io::Result<()> {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let mut input_file = File::open(matches.value_of("INPUT").unwrap())?;

    let _verbose_level = match matches.occurrences_of("v") {
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
        analyze(matches.value_of("INPUT").unwrap())?;
    }

    Ok(())
}
