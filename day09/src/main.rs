use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::time::Instant;

fn main() -> Result<(), Error> {
    let before_time = Instant::now();

    let filename = "input9.txt";

    let file = File::open(filename)?;

    let reader = BufReader::new(file);

    let mut sum_forwards = 0;
    let mut sum_backwards = 0;

    for line_result in reader.lines() {
        let line = line_result.unwrap();
        let mut current_numbers : Vec<i32> = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
        let mut sign = 1;

        while !current_numbers.iter().all(|x| *x == 0) {
            // println!("forwards adding: {}", current_numbers[current_numbers.len() - 1]);
            sum_forwards += current_numbers[current_numbers.len() - 1];
            // println!("backwards adding: {}", current_numbers[0] * sign);
            sum_backwards += current_numbers[0] * sign;
            sign = -sign;
            for i in 0..current_numbers.len() - 1 {
                current_numbers[i] = current_numbers[i+1] - current_numbers[i];
            }
            current_numbers.resize(current_numbers.len()-1, 0);
        }
    }

    println!("Result forwards {}", sum_forwards);
    println!("Result backwards {}", sum_backwards);
    println!("Time {} micros", before_time.elapsed().as_micros());

    return Ok(());
}
