
use std::fs::{read_to_string};
use std::io::{Error};
use std::time::Instant;

fn get_north_weight(cols: &Vec<Vec<char>>) -> u32 {
    let mut result = 0;
    for col in cols {
        for (row, char) in col.iter().enumerate() {
            if *char == 'O' {
                result += (col.len() - row) as u32;
            }
        }
    }
    return result;
}

fn get_roll_north_weight(cols: &Vec<Vec<char>>) -> u32 {
    let mut result = 0;
    for col in cols {
        let mut stacked_weight = 0;
        for (row, char) in col.iter().enumerate() {
            if *char == '.' {
                stacked_weight += 1;
            }
            else if *char == 'O' {
                result += (col.len() - row) as u32 + stacked_weight;
            }
            else if *char == '#' {
                stacked_weight = 0;
            }
        }
    }
    return result;
}

fn print_matrix(cols: &Vec<Vec<char>>) {
    for y in 0..cols[0].len() {
        for x in 0..cols.len() {
            print!("{}", cols[x][y]);
        }
        println!();
    }
}

fn cycle(cols: &mut Vec<Vec<char>>) {
    for x in 0..cols.len() {
        let mut stacked_weight = 0;
        for y in 0..cols[0].len() {
            let char = cols[x][y];
            if char == '.' {
                stacked_weight += 1;
            }
            else if char == 'O' {
                cols[x][y] = '.';
                cols[x][y - stacked_weight] = 'O';
            }
            else if char == '#' {
                stacked_weight = 0;
            }
        }
    }
    // println!("AFTER NORTH");
    // print_matrix(&cols);
    // WEST
    for y in 0..cols[0].len() {
        let mut stacked_weight = 0;
        for x in 0..cols.len() {
            let char = cols[x][y];
            if char == '.' {
                stacked_weight += 1;
            }
            else if char == 'O' {
                cols[x][y] = '.';
                cols[x - stacked_weight][y] = 'O';
            }
            else if char == '#' {
                stacked_weight = 0;
            }
        }
    }
    // println!("AFTER WEST");
    // print_matrix(&cols);
    // SOUTH
    for x in 0..cols.len() {
        let mut stacked_weight = 0;
        for y in (0..cols[0].len()).rev() {
            let char = cols[x][y];
            if char == '.' {
                stacked_weight += 1;
            }
            else if char == 'O' {
                cols[x][y] = '.';
                cols[x][y + stacked_weight] = 'O';
            }
            else if char == '#' {
                stacked_weight = 0;
            }
        }
    }
    // println!("AFTER SOUTH");
    // print_matrix(&cols);
    // EAST
    for y in 0..cols[0].len() {
        let mut stacked_weight = 0;
        for x in (0..cols.len()).rev() {
            let char = cols[x][y];
            if char == '.' {
                stacked_weight += 1;
            }
            else if char == 'O' {
                cols[x][y] = '.';
                cols[x + stacked_weight][y] = 'O';
            }
            else if char == '#' {
                stacked_weight = 0;
            }
        }
    }
    // println!("AFTER EAST");
    // print_matrix(&cols);
    // println!("Weight {}", get_north_weight(&cols));
}

fn get_period(periodicity_detector : &Vec<u32>) -> Option<u32>{
    for period in 1..periodicity_detector.len()/2 {
        let mut valid = true;
        for i in 0..periodicity_detector.len() - period {
            if periodicity_detector[i] != periodicity_detector[i+period]{
                valid = false;
                break;
            }
        }
        if valid {
            return Some(period as u32);
        }
    }
    return None;
}

fn main() -> Result<(), Error> {
    let before_time = Instant::now();

    let filename = "input.txt";

    let input = read_to_string(filename).unwrap();
    let lines : Vec<&str> = input.split("\n").collect();
    let mut cols : Vec<Vec<char>> = vec![Vec::new(); lines[0].len()];
    for (_row, line) in lines.iter().enumerate() {
        for (col, char) in line.chars().enumerate() {
            cols[col].push(char);
        }
    }

    let sol1 : u32 = get_roll_north_weight(&cols);

    let mut periodicity_detector : Vec<u32> = Vec::new();

    let max_periodicity = 32;

    // println!("START");
    // print_matrix(&cols);
    for _i in 0..max_periodicity*2 {
        cycle(&mut cols);
        periodicity_detector.push(get_north_weight(&cols));
    }
    periodicity_detector.push(0);

    let mut sol2 : u32 = 0;
    for i in max_periodicity*2..1000 {
        cycle(&mut cols);

        let last_pos = periodicity_detector.len() - 1;
        periodicity_detector[last_pos] = get_north_weight(&cols);

        let opt_period = get_period(&periodicity_detector);
        if opt_period.is_some() {
            let period = opt_period.unwrap();
            println!("Started repeating with period: {} at point {}", period, i);
            let mut target_position = i;
            target_position += ((999999999 - target_position) / period) * period;
            println!("Will have still value: {} at point {}", get_north_weight(&cols), target_position);
            for _ in 0..period {
                if target_position == 999999999 {
                    sol2 = get_north_weight(&cols);
                    break;
                }
                cycle(&mut cols);
                target_position += 1;
            }
            break;
        }

        for i in 0..periodicity_detector.len()-1{
            periodicity_detector[i] = periodicity_detector[i+1];
        }
    }

    println!("FINAL");
    print_matrix(&cols);

    println!("With periods:");

    for period in periodicity_detector {
        println!("{}", period);
    }


    println!("Result part1: {}", sol1);
    println!("Result part2: {}", sol2);
    println!("Time: {} micros", before_time.elapsed().as_micros());

    return Ok(());
}
