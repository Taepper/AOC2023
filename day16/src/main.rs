use std::cmp::max;
use std::fs::{read_to_string};
use std::io::{Error};
use std::time::Instant;


const NORTH: u32 = 1;
const WEST: u32 = 1 << 1;
const SOUTH: u32 = 1 << 2;
const EAST: u32 = 1 << 3;

fn beam(initial_x: i32, initial_y: i32, initial_dir: u32, print: bool, cols: &Vec<Vec<char>>) -> u32 {
    let mut history = vec![vec![0u32; cols[0].len()]; cols.len()];
    let mut queue : Vec<(i32, i32, u32)> = Vec::new();
    queue.push((initial_x, initial_y, initial_dir));

    while !queue.is_empty() {
        let cur = queue.pop().unwrap();
        let mut x_raw = cur.0;
        let mut y_raw = cur.1;
        let mut dir = cur.2;
        loop {
            if x_raw < 0 || x_raw >= cols[0].len() as i32 || y_raw < 0 || y_raw >= cols.len() as i32 {
                break;
            }
            let x = x_raw as usize;
            let y = y_raw as usize;
            if history[x][y] & dir > 0 {
                break;
            }
            history[x][y] |= dir;
            if cols[x][y] == '|' {
                if dir == EAST || dir == WEST {
                    queue.push((x_raw, y_raw-1, NORTH));
                    y_raw += 1;
                    dir = SOUTH;
                    continue;
                }
            }
            else if cols[x][y] == '-' {
                if dir == NORTH || dir == SOUTH {
                    queue.push((x_raw-1, y_raw, WEST));
                    x_raw += 1;
                    dir = EAST;
                    continue;
                }
            }
            else if cols[x][y] == '/' {
                if dir == SOUTH {
                    dir = WEST;
                }
                else if dir == WEST {
                    dir = SOUTH;
                }
                else if dir == NORTH {
                    dir = EAST;
                }
                else if dir == EAST {
                    dir = NORTH;
                }
            }
            else if cols[x][y] == '\\' {
                if dir == SOUTH {
                    dir = EAST;
                }
                else if dir == EAST {
                    dir = SOUTH;
                }
                else if dir == NORTH {
                    dir = WEST;
                }
                else if dir == WEST {
                    dir = NORTH;
                }
            }
            if dir == SOUTH {
                y_raw += 1;
            }
            else if dir == EAST {
                x_raw += 1;
            }
            else if dir == NORTH {
                y_raw -= 1;
            }
            else if dir == WEST {
                x_raw -= 1;
            }
        }
    }

    if print {
        println!("Resulting field:");
        for y in 0..cols[0].len() {
            for x in 0..cols.len() {
                if history[x][y] > 0 {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!("Directions taken field:");
        for y in 0..cols[0].len() {
            for x in 0..cols.len() {
                if cols[x][y] == '.' {
                    if history[x][y] & SOUTH > 0 {
                        print!("v");
                    } else if history[x][y] & EAST > 0 {
                        print!(">");
                    } else if history[x][y] & NORTH > 0 {
                        print!("^");
                    } else if history[x][y] & WEST > 0 {
                        print!("<");
                    } else {
                        print!(".");
                    }
                } else {
                    print!("{}", cols[x][y]);
                }
            }
            println!();
        }
    }


    let mut energized : u32 = 0;

    for y in 0..cols[0].len() {
        for x in 0..cols.len() {
            if history[x][y] > 0 {
                energized += 1;
            }
        }
    }

    return energized;
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

    let sol1 = beam(0, 0, EAST, false, &cols);
    let mut sol2 = 0;

    for y in 0..cols[0].len() {
        let right_edge_sol = beam((cols.len() - 1) as i32, y as i32, WEST, false, &cols);
        if right_edge_sol > sol2 {
            println!("New best: {} {} WEST: {}", (cols.len() - 1) as i32, y as i32, sol2);
            sol2 = right_edge_sol;
        }

        let left_edge_sol = beam(0, y as i32, EAST, false, &cols);
        if left_edge_sol > sol2 {
            sol2 = left_edge_sol;
            println!("New best: {} {} EAST: {}", 0, y as i32, sol2);
        }
    }
    for x in 0..cols.len() {
        let upper_edge_sol = beam(x as i32, 0, SOUTH, false, &cols);
        if upper_edge_sol > sol2 {
            println!("New best: {} {} SOUTH: {}", x as i32, 0, sol2);
            sol2 = upper_edge_sol;
        }

        let lower_edge_sol = beam(x as i32, (cols[0].len() - 1) as i32, NORTH, false, &cols);
        if lower_edge_sol > sol2 {
            println!("New best: {} {} NORTH: {}", x as i32, (cols[0].len() - 1) as i32, sol2);
            sol2 = lower_edge_sol;
        }
    }


    println!("Result part1: {}", sol1);
    println!("Result part2: {}", sol2);
    println!("Time: {} micros", before_time.elapsed().as_micros());

    return Ok(());
}
