use std::cmp::{Ordering,max, min};
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn is_symbol_adjacent(digits : &String, gears : &Vec<Vec<usize>>, gear_lists : &mut Vec<Vec<Vec<u32>>>, row : usize, col : usize) -> u32{
    let number = digits.parse::<u32>().unwrap();

    let lower_row_index = max(1, row) - 1;
    let upper_row_index = min(gears.len(), row + 2);

    let lower_col_index = max(1, col - digits.len()) - 1;
    let upper_col_index = col;

    for idx1 in lower_row_index..upper_row_index {
        let gear_row = &gears[idx1];
        let lower_search : Result<usize, usize> =
            gear_row.binary_search_by(|element| match element.cmp(&lower_col_index) {
                // Since we try to find position right after searched value,
                // we treat all equal values as less to move right.
                Ordering::Equal => Ordering::Greater,
                ord => ord,
            });

        let mut gear_search_index : usize;
        match lower_search{
            Ok(val) => gear_search_index = val,
            Err(val) => gear_search_index = val,
        }
        while gear_search_index < gear_row.len() && gear_row[gear_search_index] <= upper_col_index{
            println!("Number {} is next to gear in place {}:{}", number, idx1, gear_row[gear_search_index]);
            gear_lists[idx1][gear_search_index].push(number);
            gear_search_index += 1;
        }
    }

    return 0;
}

fn main() -> Result<(), Error> {
    let filename = "input3.txt";

    let mut gears : Vec<Vec<usize>> = Vec::new();
    let mut gear_lists : Vec<Vec<Vec<u32>>> = Vec::new();
    {
        // Open the file
        let file = File::open(filename)?;

        // Create a buffered reader to efficiently read lines
        let reader = BufReader::new(file);


        // Iterate over lines in the file
        for line_result in reader.lines() {
            // Handle any potential errors reading a line
            let line = line_result?;

            gears.push(Vec::new());
            gear_lists.push(Vec::new());

            for (col, char) in line.chars().enumerate() {
                if char == '*' {
                    // Symbol
                    let idx = gears.len() - 1;
                    gears[idx].push(col);
                    gear_lists[idx].push(Vec::new());
                }
            }
        }
    }

    let file = File::open(filename)?;
    let reader2 = BufReader::new(file);

    for (row, line_result) in reader2.lines().enumerate() {
        let line = line_result?;

        let mut digits : String = String::new();

        for (col, char) in line.chars().enumerate() {
            if char.is_digit(10) {
                digits.push(char);
            }
            else if !digits.is_empty() {
                is_symbol_adjacent(&digits, &gears, &mut gear_lists, row, col);
                digits = String::new();
            }
        }
        if !digits.is_empty() {
            is_symbol_adjacent(&digits, &gears, &mut gear_lists, row, line.len());
        }
    }

    let mut sum = 0;
    for gear_list_row in gear_lists{
        for gear_list in gear_list_row {
            if gear_list.len() == 2 {
                sum += gear_list[0] * gear_list[1];
            }
        }
    }

    println!("Result: {}", sum);
    Ok(())
}
