use std::fs::{read_to_string};
use std::io::{Error};
use std::iter::zip;
use std::time::Instant;
use crate::Direction::{E, N};

#[derive(PartialEq, PartialOrd, Eq, Debug, Clone, Copy, Ord)]
enum Direction {
    N, W, S, E
}

#[derive(PartialEq, PartialOrd, Eq, Debug, Clone, Copy, Ord)]
enum Tile {
    NW, NE, SW, SE, NS, WE, GROUND, START
}

fn to_tile(c: char) -> Tile{
    if c == '|' {
        return Tile::NS;
    }
    if c == '-' {
        return Tile::WE;
    }
    if c == 'L' {
        return Tile::NE;
    }
    if c == '7' {
        return Tile::SW;
    }
    if c == 'J' {
        return Tile::NW;
    }
    if c == 'F' {
        return Tile::SE;
    }
    if c == 'S' {
        return Tile::START;
    }
    return Tile::GROUND;
}

struct Board {
    tiles : Vec<Vec<Tile>>,
    start : (usize, usize)
}

impl Board {
    fn step(&self, pos: (usize, usize), direction: &Direction) -> Option<((usize,usize), Direction)> {
        let tile: &Tile = &self.tiles[pos.0][pos.1];
        let mut new_direction : Direction = Direction::S;
        if *direction == Direction::N {
            if *tile == Tile::SE {
                new_direction = Direction::E;
            }
            else if *tile == Tile::SW {
                new_direction = Direction::W;
            }
            else if *tile == Tile::NS || *tile == Tile::START {
                new_direction = Direction::N;
            }
            else {
                return None;
            }
        }
        if *direction == Direction::S {
            if *tile == Tile::NE {
                new_direction = E;
            }
            else if *tile == Tile::NW {
                new_direction = Direction::W;
            }
            else if *tile == Tile::NS || *tile == Tile::START {
                new_direction = Direction::S;
            }
            else {
                return None;
            }
        }
        if *direction == Direction::W {
            if *tile == Tile::NE {
                new_direction = Direction::N;
            }
            else if *tile == Tile::SE {
                new_direction = Direction::S;
            }
            else if *tile == Tile::WE || *tile == Tile::START {
                new_direction = Direction::W;
            }
            else {
                return None;
            }
        }
        if *direction == Direction::E {
            if *tile == Tile::SW {
                new_direction = Direction::S;
            }
            else if *tile == Tile::NW {
                new_direction = Direction::N;
            }
            else if *tile == Tile::WE || *tile == Tile::START {
                new_direction = Direction::E;
            }
            else {
                return None;
            }
        }

        if new_direction == Direction::W {
            if pos.1 == 0 {
                return None
            }
            return Some(((pos.0, pos.1 - 1), Direction::W));
        }

        if new_direction == Direction::E {
            if pos.1 + 1 == self.tiles[0].len() {
                return None
            }
            return Some(((pos.0, pos.1 + 1), E));
        }

        if new_direction == Direction::N {
            if pos.0 == 0 {
                return None
            }
            return Some(((pos.0 - 1, pos.1), N));
        }

        if new_direction == Direction::S {
            if pos.0 + 1 == self.tiles.len() {
                return None
            }
            return Some(((pos.0 + 1, pos.1), Direction::S));
        }

        return None
    }

    fn explore(&self, start_direction: Direction) -> Option<(u32, u32)> {
        let mut blocks : Vec<Vec<bool>> = vec![vec![false; self.tiles[0].len()]; self.tiles.len()];
        let mut cross_detector : Vec<Vec<i32>> = vec![vec![0; self.tiles[0].len()]; self.tiles.len()];

        let mut pos: (usize, usize) = self.start;
        let mut direction: Direction = start_direction;

        let mut distance = 0;

        loop {
            let last_pos = pos;

            let step = self.step(pos, &direction)?;
            distance += 1;

            pos = step.0;
            direction = step.1;

            blocks[pos.0][pos.1] = true;
            if direction == Direction::N {
                cross_detector[pos.0][pos.1] -= 1;
                cross_detector[last_pos.0][last_pos.1] -= 1;
            }
            else if direction == Direction::S {
                cross_detector[pos.0][pos.1] += 1;
                cross_detector[last_pos.0][last_pos.1] += 1;
            }

            // println!("{},{}", pos.0, pos.1);

            if pos == self.start {
                break;
            }
        }

        let mut enclosed_blocks = 0u32;
        for block_row in zip(blocks, cross_detector) {
            let mut try_enter : i32 = 0;
            let mut inside: bool = false;
            for (block, cross_value) in zip(block_row.0, block_row.1) {
                if block {
                    print!(" .");
                    if try_enter != 0 {
                        inside = !inside;
                        try_enter = 0;
                    }
                    if inside {
                        enclosed_blocks += 1;
                    }
                }
                else {
                    if cross_value < 0 {
                        print!("{}", cross_value);
                    }
                    else{
                        print!(" {}", cross_value);
                    }
                    try_enter += cross_value;
                }
            }
            println!();
        }

        return Some((distance, enclosed_blocks));
    }
}

fn parse_board(input : &String) -> Board {
    let mut tiles : Vec<Vec<Tile>> = Vec::new();
    let mut start = (0usize,0usize);
    for line in input.split('\n') {
        let mut tile_row = Vec::new();
        for c in line.chars() {
            if c == 'S' {
                start = (tiles.len(), tile_row.len());
            }
            tile_row.push(to_tile(c));
        }
        tiles.push(tile_row);
    }
    return Board{ tiles, start };
}

fn main() -> Result<(), Error> {
    let before_time = Instant::now();

    let filename = "input.txt";

    let input = read_to_string(filename).unwrap();

    let board = parse_board(&input);

    let result_north = board.explore(Direction::N).unwrap_or((0,0));
    println!("{}, {}", result_north.0, result_north.1);
    let result_south = board.explore(Direction::S).unwrap_or((0,0));
    println!("{}, {}", result_south.0, result_south.1);
    let result_east = board.explore(Direction::E).unwrap_or((0,0));
    println!("{}, {}", result_east.0, result_east.1);
    let result_west = board.explore(Direction::W).unwrap_or((0,0));
    println!("{}, {}", result_west.0, result_west.1);

    println!("{} microseconds elapsed", before_time.elapsed().as_micros());

    return Ok(());
}
