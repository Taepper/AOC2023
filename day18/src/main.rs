use std::fs::{read_to_string};
use std::io::{Error};
use std::time::Instant;
use std::usize;

const NORTH: u32 = 3;
const WEST: u32 = 2;
const SOUTH: u32 = 1;
const EAST: u32 = 0;
const DIRECTIONS: [u32; 4] = [NORTH, WEST, SOUTH, EAST];

const DEBUG_PRINTING : bool = false;

fn print_state(cols: &Vec<Vec<char>>, row_sizes: &Vec<i64>, col_sizes: &Vec<i64>) {
    if DEBUG_PRINTING {
        assert_eq!(cols[0].len(), row_sizes.len());
        for y in 0..cols[0].len() {
            for x in 0..cols.len() {
                print!("{} ", cols[x][y]);
            }
            print!("{}", row_sizes[y]);
            println!()
        }
        for x in col_sizes {
            print!("{} ", x);
        }
        println!()
    }
}

fn step_unbounded(coords: (i64, i64), direction: u32, num_steps: i64) -> (i64, i64) {
    if direction == WEST {
        return (coords.0 - num_steps, coords.1);
    }
    // WEST
    if direction == EAST {
        return (coords.0 + num_steps, coords.1);
    }
    // NORTH
    if direction == NORTH {
        return (coords.0, coords.1 - num_steps);
    }
    // SOUTH
    if direction == SOUTH {
        return (coords.0, coords.1 + num_steps);
    }
    panic!("Wrong direction.");
}


fn step(coords: (usize, usize), direction: u32, num_steps: usize, cols: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    if coords.0 >= num_steps && direction == WEST {
        return Some((coords.0 - num_steps, coords.1));
    }
    // WEST
    if coords.0 + num_steps < cols.len() && direction == EAST {
        return Some((coords.0 + num_steps, coords.1));
    }
    // NORTH
    if coords.1 >= num_steps && direction == NORTH {
        return Some((coords.0, coords.1 - num_steps));
    }
    // SOUTH
    if coords.1 + num_steps < cols[0].len() && direction == SOUTH {
        return Some((coords.0, coords.1 + num_steps));
    }
    return None;
}

struct Instruction {
    direction: u32,
    steps: i64
}

fn parse_instruction(line: &str) -> Instruction {
    let split : Vec<&str> = line.split_whitespace().collect();
    let steps = split[1].parse::<i64>().unwrap();
    if split[0] == "U" {
        return Instruction {direction: NORTH, steps};
    }
    else if split[0] == "L" {
        return Instruction {direction: WEST, steps };
    }
    else if split[0] == "D" {
        return Instruction {direction: SOUTH, steps};
    }
    else if split[0] == "R" {
        return Instruction {direction: EAST, steps};
    }
    panic!("Unknown direction.")
}

fn parse_real_instruction(line: &str) -> Instruction {
    let split : Vec<&str> = line.split_whitespace().collect();
    let color_code: String = split[2].chars().skip(2).take(6).collect();
    let steps = i64::from_str_radix(&*color_code.chars().take(5).collect::<String>(), 16).unwrap();
    let direction = color_code.chars().skip(5).next().unwrap().to_digit(16).unwrap();
    return Instruction{steps, direction};
}

fn fill(cols: &mut Vec<Vec<char>>) {
    let mut queue = Vec::new();
    for x in 0..cols.len() {
        queue.push((x,0));
        queue.push((x,cols[0].len() - 1));
    }
    for y in 0..cols[0].len() {
        queue.push((0, y));
        queue.push((cols.len() - 1, y));
    }

    while let Some(coords) = queue.pop() {
        if cols[coords.0][coords.1] != '.' {
            continue;
        }
        cols[coords.0][coords.1] = '_';
        for direction in DIRECTIONS {
            let new_coords_opt = step(coords, direction, 1, cols);
            if new_coords_opt.is_none() {
                continue;
            }
            let new_coords = new_coords_opt.unwrap();
            queue.push(new_coords);
        }
    }
}

fn resize_scaled_x(cols: &mut Vec<Vec<char>>, col_sizes: &mut Vec<i64>,
                 new_x: i64) -> i64 {
    if new_x < 0 {
        if new_x == -1 {
            cols.insert(0, vec!['.'; cols[0].len()]);
            col_sizes.insert(0, 1);
        } else {
            cols.insert(0, vec!['.'; cols[0].len()]);
            col_sizes.insert(0, -new_x - 1);
            cols.insert(0, vec!['.'; cols[0].len()]);
            col_sizes.insert(0, 1);
        }
        return 0;
    } else if new_x >= col_sizes.iter().sum::<i64>() {
        if new_x == col_sizes.iter().sum::<i64>() {
            cols.push(vec!['.'; cols[0].len()]);
            col_sizes.push(1);
        } else {
            cols.push(vec!['.'; cols[0].len()]);
            col_sizes.push(new_x - col_sizes.iter().sum::<i64>());
            cols.push(vec!['.'; cols[0].len()]);
            col_sizes.push(1);
        }
        return new_x;
    } else {
        let mut wrk = 0;
        let mut i: usize = 0;
        let mut count = col_sizes[i];
        while wrk + count <= new_x {
            wrk += count;
            i += 1;
            count = col_sizes[i];
        }
        assert!(wrk <= new_x);
        if count == 1 {
            return new_x;
        }
        if wrk == new_x {
            cols.insert(i+1, cols[i].clone());
            col_sizes[i] -= 1;
            col_sizes.insert(i, 1);
        }
        else if wrk + count - 1 == new_x {
            cols.insert(i+1, cols[i].clone());
            col_sizes[i] -= 1;
            col_sizes.insert(i + 1, 1);
        }
        else {
            cols.insert(i+1, cols[i].clone());
            cols.insert(i+2, cols[i].clone());
            col_sizes[i] = new_x - wrk;
            col_sizes.insert(i + 1, 1);
            col_sizes.insert(i + 2, count - (new_x - wrk) - 1);
        }
        return new_x;
    }
}
fn resize_scaled_y(cols: &mut Vec<Vec<char>>, row_sizes: &mut Vec<i64>,
                   new_y: i64) -> i64 {
    if new_y < 0 {
        if new_y == -1 {
            for col in &mut *cols {
                col.insert(0, '.');
            }
            row_sizes.insert(0, 1);
        }
        else {
            for col in &mut *cols {
                col.insert(0, '.');
            }
            row_sizes.insert(0, -new_y - 1);
            for col in &mut *cols {
                col.insert(0, '.');
            }
            row_sizes.insert(0, 1);
        }
        return 0;
    }
    else if new_y >= row_sizes.iter().sum() {
        if new_y == row_sizes.iter().sum() {
            for col in &mut *cols {
                col.push('.');
            }
            row_sizes.push(1);
        }
        else {
            for col in &mut *cols {
                col.push('.');
            }
            row_sizes.push(new_y - row_sizes.iter().sum::<i64>());
            for col in &mut *cols {
                col.push('.');
            }
            row_sizes.push(1);
        }
        return new_y;
    }
    else {
        let mut wrk = 0;
        let mut i: usize = 0;
        let mut count = row_sizes[i];
        while wrk + count <= new_y {
            wrk += count;
            i += 1;
            count = row_sizes[i];
        }
        assert!(wrk <= new_y);
        if count == 1 {
            return new_y;
        }
        if wrk == new_y {
            for col in &mut *cols {
                col.insert(i+1, col[i]);
            }
            row_sizes[i] -= 1;
            row_sizes.insert(i, 1);
        }
        else if wrk + count - 1 == new_y {
            for col in &mut *cols {
                col.insert(i+1, col[i]);
            }
            row_sizes[i] -= 1;
            row_sizes.insert(i + 1, 1);
        }
        else {
            for col in &mut *cols {
                col.insert(i+1, col[i]);
            }
            for col in &mut *cols {
                col.insert(i+2, col[i]);
            }
            row_sizes[i] = new_y - wrk;
            row_sizes.insert(i + 1, 1);
            row_sizes.insert(i + 2, count - (new_y - wrk) - 1);
        }
        return new_y;
    }
}

fn draw_y(cols: &mut Vec<Vec<char>>, col_sizes: &Vec<i64>, row_sizes: &Vec<i64>, pos: (i64, i64), mut steps: i64) {
    let mut col_index = 0;
    let mut x = col_sizes[col_index];
    while x <= pos.0 {
        col_index += 1;
        x += col_sizes[col_index];
    }
    let mut row_index = 0;
    let mut y = row_sizes[row_index];
    while y <= pos.1 {
        row_index += 1;
        y += row_sizes[row_index];
    }
    assert_eq!(col_sizes[col_index], 1);
    assert_eq!(row_sizes[row_index], 1);
    if steps > 0 {
        cols[col_index][row_index] = '#';
        while steps > 0 {
            steps -= row_sizes[row_index];
            cols[col_index][row_index] = '#';
            row_index += 1;
        }
    }
    else {
        steps = -steps;
        cols[col_index][row_index] = '#';
        while steps > 0 {
            steps -= row_sizes[row_index];
            cols[col_index][row_index] = '#';
            row_index -= 1;
        }
    }
}

fn draw_x(cols: &mut Vec<Vec<char>>, col_sizes: &Vec<i64>, row_sizes: &Vec<i64>, pos: (i64, i64), mut steps: i64) {
    let mut col_index = 0;
    let mut x = col_sizes[col_index];
    while x <= pos.0 {
        col_index += 1;
        x += col_sizes[col_index];
    }
    let mut row_index = 0;
    let mut y = row_sizes[row_index];
    while y <= pos.1 {
        row_index += 1;
        y += row_sizes[row_index];
    }
    assert_eq!(col_sizes[col_index], 1);
    assert_eq!(row_sizes[row_index], 1);
    if steps > 0 {
        cols[col_index][row_index] = '#';
        while steps > 0 {
            steps -= col_sizes[col_index];
            cols[col_index][row_index] = '#';
            col_index += 1;
        }
    }
    else {
        steps = -steps;
        cols[col_index][row_index] = '#';
        while steps > 0 {
            steps -= col_sizes[col_index];
            cols[col_index][row_index] = '#';
            col_index -= 1;
        }
    }
}

fn outline_scaled(start: (i64, i64), instructions: &Vec<Instruction>, cols: &mut Vec<Vec<char>>,
                  col_sizes: &mut Vec<i64>, row_sizes: &mut Vec<i64>) {

    println!("Now outlining the scaled version");
    let mut pos = start;
    println!("Before");
    print_state(cols, row_sizes, col_sizes);

    for instruction in instructions {
        println!("Instruction: {} steps in direction {}", instruction.steps, instruction.direction);

        let next_pos = step_unbounded(pos, instruction.direction, instruction.steps as i64);


        println!("Target position: {} {}", next_pos.0, next_pos.1);

        if instruction.direction == NORTH {
            let resized_y = resize_scaled_y(cols, row_sizes, next_pos.1);
            println!("Resized");
            print_state(cols, row_sizes, col_sizes);
            pos = (next_pos.0, resized_y);
            draw_y(cols, col_sizes, row_sizes, pos, instruction.steps);
        }
        else if instruction.direction == SOUTH {
            let resized_y = resize_scaled_y(cols, row_sizes, next_pos.1);
            println!("Resized");
            print_state(cols, row_sizes, col_sizes);
            pos = (next_pos.0, resized_y);
            draw_y(cols, col_sizes, row_sizes, pos, -instruction.steps);
        }
        else if instruction.direction == WEST {
            let resized_x = resize_scaled_x(cols, col_sizes, next_pos.0);
            println!("Resized");
            print_state(cols, row_sizes, col_sizes);
            pos = (resized_x, next_pos.1);
            draw_x(cols, col_sizes, row_sizes, pos, instruction.steps);
        }
        else if instruction.direction == EAST {
            let resized_x = resize_scaled_x(cols, col_sizes, next_pos.0);
            println!("Resized");
            print_state(cols, row_sizes, col_sizes);
            pos = (resized_x, next_pos.1);
            draw_x(cols, col_sizes, row_sizes, pos, -instruction.steps);
        }


        println!("After instruction");
        print_state(cols, row_sizes, col_sizes);
    }
}

fn part2(instructions: &Vec<Instruction>) -> i64 {
    let start = (0,0);
    let mut cols: Vec<Vec<char>> = vec![vec!['#';1];1];
    let mut col_sizes: Vec<i64> = vec![1];
    let mut row_sizes: Vec<i64> = vec![1];

    outline_scaled(start, instructions, &mut cols, &mut col_sizes, &mut row_sizes);

    println!("Outline finished");
    print_state(&cols, &row_sizes, &col_sizes);

    fill(&mut cols);

    println!("Fill finished");
    print_state(&cols, &row_sizes, &col_sizes);

    let mut result = 0;
    for (col_index, col) in cols.iter().enumerate() {
        for (row_index, char) in col.iter().enumerate() {
            if *char != '_' {
                result += row_sizes[row_index] * col_sizes[col_index];
            }
        }
    }
    return result;
}

fn main() -> Result<(), Error> {
    let before_time = Instant::now();

    let filename = "input.txt";

    let input = read_to_string(filename).unwrap();
    let lines : Vec<&str> = input.split("\n").collect();

    let instructions : Vec<Instruction> = lines.iter()
        .map(|i_line| parse_instruction(i_line))
        .collect();
    let real_instructions : Vec<Instruction> = lines.iter()
        .map(|i_line| parse_real_instruction(i_line))
        .collect();

    let sol1 = part2(&instructions);
    let sol2 = part2(&real_instructions);

    println!("Result part1: {}", sol1);
    println!("Result part2: {}", sol2);
    println!("Time: {} micros", before_time.elapsed().as_micros());

    return Ok(());
}
