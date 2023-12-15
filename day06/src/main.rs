use std::cmp::{max, min, Ordering};
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::time::Instant;

fn main() -> Result<(), Error> {
    let before_time = Instant::now();

    let filename = "input6.txt";

    let file = File::open(filename)?;

    // Create a buffered reader to efficiently read lines
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let times_line = &lines.next().unwrap().unwrap()[6..];
    let mut time_string : String = String::new();
    let times_raw = times_line.split_whitespace();
    for time in times_raw {
        time_string.push_str(time);
    }
    let time = time_string.parse::<u64>().unwrap();

    let distance_line = &lines.next().unwrap().unwrap()[9..];
    let mut distance_string : String = String::new();
    let distances_raw = distance_line.split_whitespace();
    for distance in distances_raw {
        distance_string.push_str(distance);
    }
    let distance = distance_string.parse::<u64>().unwrap();

    assert!(lines.next().is_none());

    let mut product = 1;

    let mut possibilities = 0;
    println!{"Total time {}, distance {}", time, distance};
    for press_time in 1..time{
        let time_remaining = time - press_time;
        let distance_travelled = time_remaining * press_time as u64;
        // println!{"\tIf I press {} seconds, time remaining is {}, where I travel {} * {} = {}",
//                 press_time, time_remaining, time_remaining, press_time, distance_travelled};
        if distance_travelled > distance{
            possibilities += 1;
        }
    }
    product *= possibilities;

    println!("{}", product);

    return Ok(());
}
