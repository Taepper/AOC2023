use std::cmp::{max, min, Ordering};
use std::collections::{BinaryHeap, HashMap};
use std::fs::{read_to_string};
use std::io::{Error};
use std::time::Instant;
use std::usize;


const NORTH: u32 = 0;
const WEST: u32 = 1;
const SOUTH: u32 = 2;
const EAST: u32 = 3;

const OPPOSSITES: [u32; 4] = [SOUTH, EAST, NORTH, WEST];

const DIRECTIONS: [u32; 4] = [NORTH, WEST, SOUTH, EAST];

#[derive(Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Position {
    coords: (usize, usize),
    last_direction: u32,
    steps_in_last_direction: u32,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: Position
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn step(coords: (usize, usize), direction: u32, num_steps: usize, cols: &Vec<Vec<usize>>) -> Option<(usize, usize)> {
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

fn get_cost(coords: (usize, usize), new_coords: (usize, usize), cols: &Vec<Vec<usize>>) -> usize {
    if coords.0 == new_coords.0 {
        let x = coords.0;
        let mut sum = 0;
        for y in min(coords.1, new_coords.1)..max(coords.1, new_coords.1)+1 {
            sum += cols[x][y];
        }
        return sum - cols[coords.0][coords.1];
    }
    else {
        let y = coords.1;
        let mut sum = 0;
        for x in min(coords.0, new_coords.0)..max(coords.0, new_coords.0)+1 {
            sum += cols[x][y];
        }
        return sum - cols[coords.0][coords.1];
    }
}

fn shortest_path(start: (usize, usize), goal: (usize, usize), cols: &Vec<Vec<usize>>,
                 min_steps: usize, max_steps: u32) -> (usize, Vec<(usize, usize)>) {

    let start_pos = Position{ coords: start, last_direction: EAST, steps_in_last_direction: 0 };

    let mut heap = BinaryHeap::new();
    heap.push(State {cost: 0, position: start_pos.clone()});

    let mut dist: HashMap<Position, (usize, Position)> = HashMap::new();
    dist.insert(start_pos, (0, start_pos));

    while let Some(state) = heap.pop() {
        let cost = state.cost;
        let coords = state.position.coords;
        if coords == goal {
            let mut backtrack = state.position;
            let mut trace = Vec::new();
            while backtrack.coords != start {
                trace.push(backtrack.coords);
                backtrack = dist[&backtrack].1;
            }

            return (cost, trace);
        }
        // println!("{} {} with cost {}", coords.0, coords.1, cost);
        let last_direction = state.position.last_direction;
        let steps_in_last_direction = state.position.steps_in_last_direction;

        for direction in DIRECTIONS {
            if direction == OPPOSSITES[last_direction as usize]{
                continue;
            }
            let new_coords_opt = if direction == last_direction
            { step(coords, direction, 1, cols) }
            else { step(coords, direction, min_steps, cols)};
            if new_coords_opt.is_none() {
                continue;
            }
            if last_direction == direction && steps_in_last_direction == max_steps {
                continue;
            }
            let new_coords = new_coords_opt.unwrap();
            let new_steps = if last_direction == direction { steps_in_last_direction + 1 } else { min_steps as u32 };
            let new_position = Position {coords: new_coords, last_direction: direction, steps_in_last_direction: new_steps};
            let new_cost = cost + get_cost(coords, new_coords, cols);

            if !dist.contains_key(&new_position) {
                dist.insert(new_position, (new_cost, state.position));
                heap.push(State {cost: new_cost, position: new_position});
            }
            else if dist[&new_position].0 > new_cost {
                dist.remove(&new_position);
                dist.insert(new_position, (new_cost, state.position));
                heap.push(State {cost: new_cost, position: new_position});
            }
        }
    }

    panic!("End not reached")
}

fn main() -> Result<(), Error> {
    let before_time = Instant::now();

    let filename = "input.txt";

    let input = read_to_string(filename).unwrap();
    let lines : Vec<&str> = input.split("\n").collect();
    let mut cols : Vec<Vec<usize>> = vec![Vec::new(); lines[0].len()];
    for (_row, line) in lines.iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            cols[col].push(c.to_digit(10).unwrap() as usize);
        }
    }

    let sol1 = shortest_path((0,0), (cols.len() - 1, cols[0].len() - 1), &cols, 1, 3);
    let sol2 = shortest_path((0,0), (cols.len() - 1, cols[0].len() - 1), &cols, 4, 10);

    println!("Numbers on path1");
    for pos in &sol1.1 {
        print!("{} ", cols[pos.0][pos.1]);
    }
    println!("");

    for y in 0..cols[0].len() {
        for x in 0..cols.len() {
            if sol2.1.contains(&(x,y)){
                print!("X");
            }
            else {
                print!("{}", cols[x][y]);
            }
        }
        println!()
    }

    println!("Result part1: {}", sol1.0);
    println!("Result part2: {}", sol2.0);
    println!("Time: {} micros", before_time.elapsed().as_micros());

    return Ok(());
}
