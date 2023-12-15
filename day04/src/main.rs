use std::cmp::min;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn main() -> Result<(), Error> {
    let filename = "input4.txt";
    let file = File::open(filename)?;
    let buffered = BufReader::new(file);
    let line_count = buffered.lines().count();

    let file = File::open(filename)?;

    // Create a buffered reader to efficiently read lines
    let reader = BufReader::new(file);

    let mut sum = 0;

    let mut queue : Vec<u128> = Vec::with_capacity(line_count);
    queue.resize(line_count, 1);

    // Iterate over lines in the file
    for (id, line_result) in reader.lines().enumerate() {
        sum += queue[id];

        // Handle any potential errors reading a line
        let line = line_result?;

        let mut pieces = line.split_whitespace();

        assert_eq!(pieces.next().unwrap(), "Card");

        assert_eq!(pieces.next().unwrap(), format!("{}:", id + 1));

        let mut winners : HashSet<u128> = HashSet::new();

        let mut current_sum : u128 = 0;

        loop {
            let piece = pieces.next();
            if piece.unwrap() == "|" {
                break;
            }
            winners.insert(piece.unwrap().parse::<u128>().unwrap());
        }

        loop {
            let piece = pieces.next();
            if piece.is_none() {
                break;
            }
            let num = piece.unwrap().parse::<u128>().unwrap();
            if winners.contains(&num) {
                current_sum += 1;
            }
        }

        println!("We have card {} so many times {} with value: {}", id, queue[id], current_sum);

        for i in (id+1)..min(line_count, id + current_sum as usize + 1){
            queue[i] += queue[id];
        }
    }



    println!("Result: {}", sum);
    Ok(())
}
