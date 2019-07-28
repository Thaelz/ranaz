
#[macro_use]
extern crate clap;

use clap::App;
use std::fs::File;
use std::io::prelude::*;

/* mono_bit_stat 
 *  Return a tuple (zeroes, ones) which are respectively the number of ones
 *  and zeros, as single bits, in the file content.
 *  INPUT : String
 *  OUTPUT: (nb_zeroes, nb_ones)
 */
fn mono_bit_stat(s: &mut String) -> (u128, u128) {
    let mut c = 0;
    let tot = (s.chars().count() * 8) as u128;
    for bi in s.as_bytes() {
        let mut b = *bi;
        for _ in 0..8 {
            if b & 1 == 1 { c += 1; }
            b >>= 1;
        }
    }
    ((tot-c) as u128, c)
}

fn analyze(filename : &str) -> std::io::Result<()> { 
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    /* Bit test */
    let (count_z, count_o) = mono_bit_stat(&mut contents);
    println!("Bit test : Z({}), O({})", count_z, count_o);
    let mut bit_diff = count_z - count_o;
    bit_diff *= bit_diff;
    bit_diff /= count_z + count_o; 
    println!("Bit test === {}", bit_diff);
    Ok(())
}

fn main() -> std::io::Result<()> {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let _verbose_level = match matches.occurrences_of("v") {
        lvl if lvl < 3 => lvl,
        _ => panic!("Cannot take more than 2 verbose arguments.")
    };

    analyze(matches.value_of("INPUT").unwrap())?;

    Ok(())
}
