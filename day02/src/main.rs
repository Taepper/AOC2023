use std::cmp::{max, min};
use std::fs::File;
use std::i16::MAX;
use std::io::{BufRead, BufReader, Error};

fn main() -> Result<(), Error> {
    // Open the file
    let file = File::open("input2.txt")?;

    // Create a buffered reader to efficiently read lines
    let reader = BufReader::new(file);


    let mut sum1 = 0;
    let mut sum2 = 0;

    let mut id = 1;

    // Iterate over lines in the file
    for line_result in reader.lines() {
        // Handle any potential errors reading a line
        let line = line_result?;

        let line_split : Vec<&str>= line.split(":").collect();

        let parts = line_split.get(1).unwrap().split(';');

        let mut possible = true;
        let mut min_num_red = 0;
        let mut min_num_blue = 0;
        let mut min_num_green = 0;

        for play in parts {
            let moves = play.split(",");
            for mov in moves {
                let num = mov.split(" ").nth(1).unwrap().parse::<i32>().unwrap();
                if mov.contains("red"){
                    min_num_red = max(min_num_red, num);

                    if num > 12 {
                        possible = false;
                    }
                }
                else if mov.contains("blue"){
                    min_num_blue = max(min_num_blue, num);

                    if num > 14{
                        possible = false;
                    }
                }
                else if mov.contains("green"){
                    min_num_green = max(min_num_green, num);

                    if num > 13 {
                        possible = false;
                    }
                }
            }
        }

        if possible {
            sum1 += id;
        }
        sum2 += min_num_blue * min_num_green * min_num_red;

        id += 1;

    }

    println!("Sum of possibles: {}", sum1);
    println!("Sum of powers: {}", sum2);

    Ok(())
}
