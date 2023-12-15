use std::arch::aarch64::vuqadds_s32;
use std::cmp::{max, min};
use std::fs::{read_to_string};
use std::io::{Error};
use std::time::Instant;

fn main() -> Result<(), Error> {
    let before_time = Instant::now();

    let filename = "input.txt";

    let input = read_to_string(filename).unwrap();
    let lines : Vec<&str> = input.split("\n").collect();

    let mut galaxies : Vec<(u32, u32)> = Vec::new();

    let mut empty_rows = vec![true; lines.len()];
    let mut empty_cols = vec![true; lines[0].len()];

    for row in 0..lines.len() {
        for (col, char) in lines[row].chars().enumerate() {
            if char == '#' {
                empty_rows[row] = false;
                empty_cols[col] = false;
                galaxies.push((row as u32, col as u32));
            }
        }
    }

    let mut row_shift = vec![0; lines.len()];
    let mut col_shift = vec![0; lines[0].len()];

    let mut current_shift = 0;
    for (row, empty) in empty_rows.iter().enumerate(){
        if *empty { current_shift += 1000000 - 1; }
        row_shift[row] = current_shift;
    }
    let mut current_shift = 0;
    for (col, empty) in empty_cols.iter().enumerate(){
        if *empty { current_shift += 1000000 - 1; }
        col_shift[col] = current_shift;
    }

    let mut shifted_galaxies : Vec<(u32, u32)> = Vec::new();
    for (row, col) in &galaxies {
        shifted_galaxies.push((row + row_shift[*row as usize], col + col_shift[*col as usize]));
    }

    let mut sum : u64 = 0;

    for (row1, col1) in &shifted_galaxies {
        for (row2, col2) in &shifted_galaxies {
            let distance = max(row1, row2) - min(row1, row2)
                                + max(col1, col2) - min(col1, col2);
            // println!("Distance: {}", distance);
            sum += distance as u64;
        }
    }

    println!("Result: {}", sum / 2);
    println!("Time: {} micros", before_time.elapsed().as_micros());

    return Ok(());
}
