
#[macro_use]
extern crate clap;

pub mod utils;
pub mod markov;
pub mod fourier;

use clap::App;
use std::fs::File;
use std::io::prelude::*;

/* Statistical basic analisys
 *  -> bits (1 and 0 count)
 *  -> dibits (00, 01, 10, 11 count)
 *  -> bytes (hex : 00 to ff count)
 *  -> word, 4-bytes word
 *      We try to do it smart, register the position of the byte in the qword
 *      We use a [u128; 1024], so that the byte x according to its position y
 *      such that y in [0-3] increments [y*256 + x]
 */
fn bit_dibit_byte_stat(s : &Vec<u8>) -> ( [u128; 2], [u128; 4], [u128; 256], [u128; 1024] ) {
    let mut bits:   [u128; 2] =     [0; 2];
    let mut dibits: [u128; 4] =     [0; 4];
    let mut bytes:  [u128; 256] =   [0; 256];
    let mut words:  [u128; 1024] =  [0; 1024];

    let tot_bits = (s.len() * 8) as u128;
    let mut i: u128 = 0;

    for byte in s {
        let mut b = *byte;
        for _ in 0..4 {
            let d = (b & 3) as usize;
            dibits[d] += 1;
            b >>= 2;
        }
        bytes[b as usize] += 1;

        words[ (( (i & 3) * 256) + b as u128) as usize ] += 1;
        i += 1;
    }

    bits[1] = dibits[1] + dibits[2] + (dibits[3] << 1);
    bits[0] = tot_bits - bits[1];

    return (bits, dibits, bytes, words);
}

fn analyze(filename : &str) -> std::io::Result<()> { // TODO: pass the File object as parameter?
    let mut file = File::open(filename)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;

    let (bits, dibits, bytes, words) = bit_dibit_byte_stat(&mut contents);

    /* Bit test */
    println!("Bit test : Z({}), O({})", bits[0], bits[1]);
    let mut bit_diff = utils::u_substract(bits[0], bits[1]);
    bit_diff *= bit_diff;
    bit_diff /= bits[0] + bits[1];
    println!("Bit test === {}", bit_diff);

    /* Dibit test */
    println!("Dibit test: 00:{} - 01:{} - 10:{} - 11:{}", dibits[0], dibits[1], dibits[2], dibits[3]);
    let mut dibit_diff2= utils::iter_sums_u_subs(&dibits);
    dibit_diff2 *= dibit_diff2;
    let mut sum: u128 = dibits.iter().sum();
    dibit_diff2 /= sum;
    println!("Dibit2 test === {}", dibit_diff2);

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
            println!("Usage : cargo run /tmp/rand markov -p /tmp/rand.png")
        }
    }
    else {
        analyze(matches.value_of("INPUT").unwrap())?;
    }

    Ok(())
}
