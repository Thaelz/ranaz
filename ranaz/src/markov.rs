//! Implements some Markov computations

extern crate image;

use image::ColorType;
use image::png::PNGEncoder;
use std::fs::File;

/// Returns the Markov matrix (1rst values are the lines, 2nd ones are the columns) of a given
/// sequence of bytes.
///
/// # Arguments
/// * 'bytes' - a vector of u8
///
/// # Examples
/// ```no_run
/// let mut input_file = File::open("my_bin");
/// let mut bytes = Vec::<u8>::new();
/// input_file.read_to_end(&mut bytes)?;
/// let matrix = get_markov_array(&bytes);
/// ```
pub fn get_markov_array(bytes: &Vec<u8>) -> [[u32; 256]; 256] {
    let mut matrix : [[u32; 256]; 256] = [[0; 256]; 256];
    let mut old: u8 = 0;

    for b in bytes.iter() {
        matrix[old as usize][*b as usize] += 1;
        old = *b;
    }

    matrix
}

/// Exports the given Markov matrix to a PNG file
///
/// # Arguments
/// * matrix - reference to the matrix
/// * filename - path to the file to create/write
///
/// # Examples
/// ```no_run
/// let matrix = get_markov_array(&bytes);
/// to_img(&matrix, "my_img.png");
/// ```
pub fn to_img(matrix: &[[u32; 256]; 256], filename: &str) -> Result<(), std::io::Error> {
    let output = File::create(filename)?;
    let encoder = PNGEncoder::new(output);

    // get the highest value in the matrix
    let max = matrix
        .iter()
        .flat_map(|line| line.iter())
        .max()
        .unwrap();

    println!("Max= {}", max);

    // Normalise u8 values + flatten the matrix
    let mut pixels : [u8; 512*512] = [0; 512*512];
    for i in 0 .. 256 {
        for j in 0 .. 256 {
            let v = (256 - matrix[i][j] * 256 / *max) as u8;
            pixels[i * 256 + j] = v;
            pixels[i * 256 + j + 1] = v;
            pixels[(i+1) * 256 + j] = v;
            pixels[(i+1) * 256 + j + 1] = v;
        }
    }

    // write the PNG file
    encoder.encode(&pixels, 512, 512, ColorType::Gray(8))?;

    Ok(())
}
