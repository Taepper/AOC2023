use std::fs::File;
use std::io::{self, BufRead, BufReader, Error};

fn main() -> Result<(), Error> {
    // Open the file
    let file = File::open("input1.txt")?;

    // Create a buffered reader to efficiently read lines
    let reader = BufReader::new(file);


    let mut sum = 0;

    let numbers = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    // Iterate over lines in the file
    for line_result in reader.lines() {
        // Handle any potential errors reading a line
        let line = line_result?;

        let mut first_digit_found = false;
        let mut first_digit = 0;
        let mut last_digit = 0;

        for (index, character) in line.char_indices() {
            if let Some(numeric_value) = character.to_digit(10) {
                last_digit = numeric_value;
                if !first_digit_found {
                    first_digit_found = true;
                    first_digit = numeric_value;
                }
            }
            else if let Some(numeric_value2) = numbers.iter().position(|&x| line[index..].starts_with(x)){
                last_digit = numeric_value2 as u32;
                if !first_digit_found {
                    first_digit_found = true;
                    first_digit = numeric_value2 as u32;
                }
            }
        }

        sum += first_digit * 10 + last_digit;
    }

    println!("Result: {}", sum);

    Ok(())
}
