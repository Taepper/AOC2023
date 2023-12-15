use std::fs::{read_to_string};
use std::io::{Error};
use std::time::Instant;



fn hash(input: &str) -> u32 {
    let mut value = 0;
    for char in input.chars() {
        let ascii_code = char as u32;
        value = ((value + ascii_code) * 17) % 256;
    }
    return value;
}


fn main() -> Result<(), Error> {
    let before_time = Instant::now();

    let filename = "input.txt";

    let input = read_to_string(filename).unwrap();
    let steps : Vec<&str> = input.split(",").collect();


    let mut sol1 : u32 = 0;
    for step in &steps {
        let hash = hash(step);
        sol1 += hash;
    }


    let mut boxes: Vec<Vec<(&str, u32)>> = std::iter::repeat(vec![]).take(256).collect::<Vec<_>>();

    for step in &steps {
        if step.ends_with("-") {
            let label = step.split("-").next().unwrap();
            let box_num = hash(label) as usize;
            let index = boxes[box_num].iter().position(|(item_label, _focal)| *item_label == label);
            if index.is_some(){
                boxes[box_num].remove(index.unwrap());
            }
        }
        else {
            let label = step.split("=").next().unwrap();
            let focal = step.split("=").skip(1).next().unwrap().parse::<u32>().unwrap();
            let box_num = hash(label) as usize;
            let index = boxes[box_num].iter().position(|(item_label, _focal)| *item_label == label);
            if index.is_some() {
                boxes[box_num][index.unwrap()].1 = focal;
            }
            else {
                boxes[box_num].push((label, focal));
            }
        }
    }

    let mut sol2 : u32 = 0;
    for (box_num, b) in boxes.iter().enumerate() {
        for (item_num, (_label, focal)) in b.iter().enumerate() {
            sol2 += (box_num + 1) as u32 * (item_num + 1) as u32 * focal;
        }
    }

    println!("Result part1: {}", sol1);
    println!("Result part2: {}", sol2);
    println!("Time: {} micros", before_time.elapsed().as_micros());

    return Ok(());
}
