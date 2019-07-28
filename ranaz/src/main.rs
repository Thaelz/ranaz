
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
fn mono_bit_stat(s: &mut Vec<u8>) -> (i128, i128) {
    let mut c = 0;
    let tot = (s.len() * 8) as i128;
    for bi in s {
        let mut b = *bi;
        for _ in 0..8 {
            if b & 1 == 1 { c += 1; }
            b >>= 1;
        }
    }
    ((tot-c) as i128, c as i128)
}

/* duo_bit_stat 
 *  Return an array (nb_00, nb_01, nb_10, nb_00) which are respectively the number of 
 *  occurences of each dibit.
 *  INPUT : String
 *  OUTPUT: (nb_00, nb_01, nb_10, nb_11)
 */
fn dibit_stat(s: &mut Vec<u8>) -> [i128; 4] {
    let mut a: [i128; 4] = [0; 4];
    let tot = (s.len() * 8) as i128;
    for bi in s {
        let mut b = *bi;
        for _ in 0..4 {
            let i = (b & 3) as usize;
            a[i] += 1;
            b >>= 2;
        }
    }
    a
}

fn analyze(filename : &str) -> std::io::Result<()> { 
    let mut file = File::open(filename)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;

    /* Bit test */
    let (count_z, count_o) = mono_bit_stat(&mut contents);
    println!("Bit test : Z({}), O({})", count_z, count_o);
    let mut bit_diff = count_z - count_o;
    bit_diff *= bit_diff;
    bit_diff /= count_z + count_o; 
    println!("Bit test === {}", bit_diff);

    /* Dibit test */
    let a = dibit_stat(&mut contents);
    println!("Dibit test : 00:{} - 01:{} - 10:{} - 11:{}", a[0], a[1], a[2], a[3]);

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
