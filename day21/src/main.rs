use std::collections::VecDeque;
use std::fs::{read_to_string};
use std::io::{Error};
use std::time::Instant;
use std::usize;

const NORTH: u32 = 3;
const WEST: u32 = 2;
const SOUTH: u32 = 1;
const EAST: u32 = 0;
const DIRECTIONS: [u32; 4] = [NORTH, WEST, SOUTH, EAST];

const DEBUG_PRINTING : bool = true;

fn print_state(cols: &Vec<Vec<char>>) {
    if DEBUG_PRINTING {
        for y in 0..cols[0].len() {
            for x in 0..cols.len() {
                print!("{} ", cols[x][y]);
            }
            println!()
        }
    }
}
fn print_distances(cols: &Vec<Vec<char>>, distances: &Vec<Vec<usize>>) {
    if DEBUG_PRINTING {
        for y in 0..distances[0].len() {
            for x in 0..distances.len() {
                if cols[x][y] == '#' {
                    print!("# ");
                }
                else{
                    print!("{} ", distances[x][y] % 10);
                }
            }
            println!()
        }
    }
}

fn step(coords: (usize, usize), direction: u32, num_steps: usize, cols: &Vec<Vec<char>>, valid_char: char) -> Option<(usize, usize)> {
    if coords.0 >= num_steps && direction == WEST {
        if cols[coords.0 - num_steps][coords.1] != valid_char {
            return None;
        }
        return Some((coords.0 - num_steps, coords.1));
    }
    // WEST
    if coords.0 + num_steps < cols.len() && direction == EAST {
        if cols[coords.0 + num_steps][coords.1] != valid_char {
            return None;
        }
        return Some((coords.0 + num_steps, coords.1));
    }
    // NORTH
    if coords.1 >= num_steps && direction == NORTH {
        if cols[coords.0][coords.1 - num_steps] != valid_char {
            return None;
        }
        return Some((coords.0, coords.1 - num_steps));
    }
    // SOUTH
    if coords.1 + num_steps < cols[0].len() && direction == SOUTH {
        if cols[coords.0][coords.1 + num_steps] != valid_char {
            return None;
        }
        return Some((coords.0, coords.1 + num_steps));
    }
    return None;
}

fn get_distance_matrix_from(start: (usize, usize), cols: &Vec<Vec<char>>) -> Vec<Vec<usize>> {
    let mut distances : Vec<Vec<usize>> = vec![vec![usize::MAX; cols[0].len()]; cols.len()];
    let mut queue = VecDeque::new();
    distances[start.0][start.1] = 0;
    queue.push_back(start);
    while let Some(coords) = queue.pop_front() {
        for direction in DIRECTIONS {
            let new_coords_opt = step(coords, direction, 1, &cols, '.');
            if new_coords_opt.is_none() {
                continue;
            }
            let new_coords = new_coords_opt.unwrap();
            let new_distance = distances[coords.0][coords.1] + 1;
            if new_distance < distances[new_coords.0][new_coords.1] {
                distances[new_coords.0][new_coords.1] = new_distance;
                queue.push_back(new_coords);
            }
        }
    }
    return distances;
}

fn validate_assumptions_free_rows_and_cols_square(start: (usize, usize), cols: &Vec<Vec<char>>) {
    if cols.len() != cols[0].len() {
        panic!("Assumed square garden.")
    }

    let mut start_col_free = false;

    for x in 1..cols.len()-1 {
        let mut all_free = true;
        for y in 0..cols[x].len() {
            if cols[x][y] != '.' {
                all_free = false;
                break;
            }
        }
        if x == start.0 {
            start_col_free = true;
            continue;
        }
        if all_free {
            panic!("Assumption broken, that only border columns and Start column are freely traversable. Col {} is not", x);
        }
    }

    if !start_col_free {
        panic!("Assumed start col to be free.")
    }


    let mut start_row_free = false;
    for y in 1..cols[0].len()-1 {
        let mut all_free = true;
        for x in 0..cols.len() {
            if cols[x][y] != '.' {
                all_free = false;
                break;
            }
        }
        if y == start.1 {
            start_row_free = true;
            continue;
        }
        if all_free {
            panic!("Assumption broken, that only border rows and start rows freely traversable. Row {} is not.", y);
        }
    }

    if !start_row_free {
        panic!("Assumed start row to be free.")
    }
}

fn go_straight(mut steps_left: usize, distances: &Vec<Vec<usize>>) -> u64 {
    let count_per_garden =
        [distances.iter().flatten().filter(|x| **x != usize::MAX && (**x % 2 == 0)).count(),
            distances.iter().flatten().filter(|x| **x != usize::MAX && (**x % 2 == 1)).count()];
    let max_reachable = distances.iter().flatten().filter(|x| **x != usize::MAX).max().unwrap();

    let mut count = 0;
    let mut parity = steps_left % 2;

    while steps_left >= *max_reachable {
        count += count_per_garden[parity];
        steps_left -= distances.len();
        parity = steps_left % 2;
    }
    loop {
        count += distances.iter().flatten()
            .filter(|x| (**x <= steps_left) && (**x % 2 == parity)).count();
        if steps_left <= distances.len() {
            break;
        }
        steps_left -= distances.len();
        parity = steps_left % 2;
    }
    return count as u64;
}

fn go_diagonal(mut steps_left: usize, distances: &Vec<Vec<usize>>) -> u64 {
    let count_per_garden =
        [distances.iter().flatten().filter(|x| **x != usize::MAX && (**x % 2 == 0)).count(),
            distances.iter().flatten().filter(|x| **x != usize::MAX && (**x % 2 == 1)).count()];
    let max_reachable = distances.iter().flatten().filter(|x| **x != usize::MAX).max().unwrap();

    let mut number_of_gardens = 1;
    let mut count = 0;
    let mut parity = steps_left % 2;

    while steps_left >= *max_reachable {
        count += count_per_garden[parity] * number_of_gardens;
        number_of_gardens += 1;
        steps_left -= distances.len();
        parity = steps_left % 2;
    }
    loop {
        count += distances.iter().flatten()
            .filter(|x| (**x <= steps_left) && (**x % 2 == parity)).count()
            * number_of_gardens;
        if steps_left <= distances.len() {
            break;
        }
        number_of_gardens += 1;
        steps_left -= distances.len();
        parity = steps_left % 2;
    }
    return count as u64;
}

fn main() -> Result<(), Error> {
    let before_time = Instant::now();

    let steps : usize = 26501365;

    let filename = "input.txt";

    let input = read_to_string(filename).unwrap();
    let lines : Vec<&str> = input.split("\n").collect();
    let mut cols : Vec<Vec<char>> = vec![Vec::new(); lines[0].len()];
    let mut start : (usize, usize) = (0, 0);
    for (row, line) in lines.iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == 'S' {
                start = (col, row);
                cols[col].push('.');
            }
            else {
                cols[col].push(c);
            }
        }
    }

    validate_assumptions_free_rows_and_cols_square(start, &cols);

    let distances_from_middle : Vec<Vec<usize>> = get_distance_matrix_from(start, &cols);

    println!("Distances from the middle:");
    print_state(&cols);
    print_distances(&cols, &distances_from_middle);

    let distances_from_n : Vec<Vec<usize>> = get_distance_matrix_from((start.0, 0), &cols);

    println!("Finished distances_from_n.");

    let distances_from_w : Vec<Vec<usize>> = get_distance_matrix_from((0, start.1), &cols);

    println!("Finished distances_from_w.");

    let distances_from_s : Vec<Vec<usize>> = get_distance_matrix_from((start.0, cols.len() - 1), &cols);

    println!("Finished distances_from_s.");

    let distances_from_e : Vec<Vec<usize>> = get_distance_matrix_from((cols[0].len() - 1, start.1), &cols);

    println!("Finished distances_from_e.");

    let distances_from_nw : Vec<Vec<usize>> = get_distance_matrix_from((0, 0), &cols);

    println!("Finished distances_from_nw.");

    let distances_from_ne : Vec<Vec<usize>> = get_distance_matrix_from((0, cols.len() - 1), &cols);

    println!("Finished distances_from_ne.");

    let distances_from_sw : Vec<Vec<usize>> = get_distance_matrix_from((cols[0].len() - 1, 0), &cols);

    println!("Finished distances_from_sw.");

    let distances_from_se : Vec<Vec<usize>> = get_distance_matrix_from((cols[0].len() - 1, cols.len() - 1), &cols);

    println!("Finished distances_from_se.");
    println!("Finished all distance matrices.");

    let sol1 = distances_from_middle.iter().flatten().filter(|x| (**x <= 64) && (**x % 2 == 0)).count();

    let count_original = distances_from_middle.iter().flatten().filter(|x| **x != usize::MAX && (**x % 2 == steps % 2)).count() as u64;

    println!("Finished count_original.");

    let count_north = go_straight(steps - (distances_from_middle[start.0][0] + 1), &distances_from_s);

    println!("Finished count_north.");

    let count_south = go_straight(steps - (distances_from_middle[start.0][cols[0].len() - 1] + 1), &distances_from_n);

    println!("Finished count_south.");

    let count_west = go_straight(steps - (distances_from_middle[0][start.1] + 1), &distances_from_e);

    println!("Finished count_west.");

    let count_east = go_straight(steps - (distances_from_middle[cols.len() - 1][start.1] + 1), &distances_from_w);

    println!("Finished count_east.");

    let count_nw = go_diagonal(steps - (distances_from_middle[0][0] + 2), &distances_from_se);

    println!("Finished count_nw.");

    let count_ne = go_diagonal(steps - (distances_from_middle[0][cols.len() - 1] + 2), &distances_from_sw);

    println!("Finished count_ne.");

    let count_sw = go_diagonal(steps - (distances_from_middle[cols.len() - 1][cols.len() - 1] + 2), &distances_from_ne);

    println!("Finished count_sw.");

    let count_se = go_diagonal(steps - (distances_from_middle[cols.len() - 1][cols.len() - 1] + 2), &distances_from_nw);

    println!("Finished count_se.");

    let sol2 = count_original
        + count_north + count_south + count_west + count_east
        + count_ne + count_nw + count_se + count_sw;

    println!("Result part1: {}", sol1);
    println!("Result part2: {}", sol2);
    println!("Time: {} micros", before_time.elapsed().as_micros());

    return Ok(());
}
