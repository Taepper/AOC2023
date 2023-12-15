use std::collections::HashMap;
use std::fs::{read_to_string};
use std::io::{Error};
use std::time::Instant;


fn format_line(line: u128, len: usize) -> String {
    let mut res = "".to_string();

    for i in 0..len {
        let char1 = (line >> (len*2 - i * 2 - 2)) & 3;
        if char1 == 1 {
            res += ".";
        }
        else if char1 == 2 {
            res += "#";
        }
        else {
            res += "?";
        }
    }

    return res;
}

fn format_pointer(offset: usize, len: usize) -> String {
    let mut res = "".to_string();

    for i in 0..len {
        if i == offset || i == len - offset - 1 {
            res += "^";
        }
        else {
            res += " ";
        }
    }

    return res;
}

fn solve_line(line1: u128, line2: u128, len: usize, sizes: &Vec<u32>,
              start: usize, mut size: u32, mut size_ptr: usize,
              mut blocks_seen: i32, memoization: &mut HashMap<(usize, i32),u64>) -> u64 {
    for i in start..len {
        let char = ((line1 >> (len - i - 1)) & 1) + ((line2 >> (len - i - 1)) & 1);
        if memoization.contains_key(&(i, blocks_seen)){
            return memoization[&(i, blocks_seen)];
        }
        if char == 0 {
            // println!("{} growing left", format_line(line, len));
            // println!("{}", format_pointer(i, len));
            let s1 = solve_line(line1,
                                line2 + (1 << (len - i - 1)),
                                len, sizes, i, size,  size_ptr,
                                blocks_seen,
                                memoization);
            memoization.insert((i+1, blocks_seen), s1);

            let mut s2 = 0;
            if size_ptr < sizes.len() && size < sizes[size_ptr] {
                s2 = solve_line(line1 + (1 << (len - i - 1)),
                                line2 + (1 << (len - i - 1)),
                                len, sizes, i, size,  size_ptr,
                                blocks_seen,
                                memoization);
            }
            return s1 + s2;
        }

        if char == 1 {
            if size > 0 {
                if size_ptr == sizes.len() {
                    return 0;
                }

                if size != sizes[size_ptr] {
                    // println!("{} impossible left", format_line(line, len));
                    // println!("{}", format_pointer(i, len));
                    return 0;
                }
                size = 0;
                size_ptr += 1;
            }
        }
        else if char == 2 {
            blocks_seen += 1;
            size += 1;
            if size_ptr >= sizes.len() || size > sizes[size_ptr] {
                return 0;
            }
        }
    }

    if size > 0 {
        if size_ptr == sizes.len() {
            return 0;
        }
        if size != sizes[size_ptr] {
            // println!("{} ool impossible left (all the way right)", format_line(line, len));
            return 0;
        }
        size_ptr += 1;
    }
    if size_ptr != sizes.len() {
        // println!("{} not right number of groups", format_line(line, len));
        return 0;
    }

    // println!("{} is correct", format_line(line, len));

    return 1;
}

fn solve_line_wrapper(line1: u128, line2: u128, len: usize, sizes: &Vec<u32>) -> u64 {
    let mut memoization: HashMap<(usize, i32), u64> = HashMap::new();
    return solve_line(line1, line2, len, sizes, 0,0, 0, 0, &mut memoization);
}

fn solve_one(states_str: String, length_str: String) -> u64 {
    // println!("states_str {}", states_str);
    let mut states1 = 0;
    let mut states2 = 0;
    for c in states_str.chars() {
        states1 <<= 1;
        states2 <<= 1;
        if c == '.'{
            states1 += 1;
        }
        else if c == '#' {
            states1 += 1;
            states2 += 1;
        }
    }

    // println!("length_string {}", length_str);
    let lengths: Vec<u32> = length_str.split(",").map(|x| x.parse::<u32>().unwrap()).collect();

    let solution = solve_line_wrapper(states1, states2, states_str.len(), &lengths);
    return solution;
}

fn solve_mult(states_raw: &str, length_raw: &str, repeats: usize) -> u64 {

    let states_str: String = (states_raw.to_string() + "?").repeat(repeats - 1) + states_raw;
    // println!("states_str {}", states_str);
    let mut states1 = 0;
    let mut states2 = 0;
    for c in states_str.chars() {
        states1 <<= 1;
        states2 <<= 1;
        if c == '.'{
            states1 += 1;
        }
        else if c == '#' {
            states1 += 1;
            states2 += 1;
        }
    }

    let length_str: String = (length_raw.to_string() + ",").repeat(repeats - 1) + length_raw;
    // println!("length_str {}", length_str);
    let lengths: Vec<u32> = length_str.split(",").map(|x| x.parse::<u32>().unwrap()).collect();

    let solution = solve_line_wrapper(states1, states2, states_str.len(), &lengths);
    return solution;
}

fn main() -> Result<(), Error> {
    let before_time = Instant::now();

    let filename = "input.txt";

    let input = read_to_string(filename).unwrap();
    let lines : Vec<&str> = input.split("\n").collect();

    let mut sum1 : u64 = 0;
    let mut sum2 : u64 = 0;
    for (_line_num, line) in lines.iter().enumerate() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        sum1 += solve_one(parts[0].to_string(), parts[1].to_string());
        sum2 += solve_mult(parts[0], parts[1], 5);
    }


    println!("Result part1: {}", sum1);
    println!("Result part2: {}", sum2);
    println!("Time: {} micros", before_time.elapsed().as_micros());

    return Ok(());
}
