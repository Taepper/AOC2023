use std::collections::VecDeque;
use std::fs::{read_to_string};
use std::io::{Error};
use std::time::Instant;
use std::usize;

struct Brick {
    start: (usize, usize, usize),
    end: (usize, usize, usize)
}

fn parse_brick(line: &str) -> Brick {
    let split = line.split_once("~").unwrap();
    let pos1 = split.0.split(",").map(|x| x.parse::<usize>()).collect();
    let pos2 = split.1.split(",").map(|x| x.parse::<usize>()).collect();
    return Brick {
        start: (pos1.0,pos1.1,pos1.2),
        end: (pos2.0,pos2.1,pos2.2)
    }
}

fn main() -> Result<(), Error> {
    let before_time = Instant::now();

    let filename = "input.txt";
    let input = read_to_string(filename).unwrap();
    let lines : Vec<Brick> = input.split("\n").map(|line| parse_brick(line)).collect();

    let sol1 = 0;
    let sol2 = 0;

    println!("Result part1: {}", sol1);
    println!("Result part2: {}", sol2);
    println!("Time: {} micros", before_time.elapsed().as_micros());

    return Ok(());
}
