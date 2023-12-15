use std::cmp::min;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::time::Instant;

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    return a;
}

fn get_all_gcds_recursive(steps_matrix: &Vec<Vec<u32>>, offset: usize) -> Vec<u64> {
    if offset == steps_matrix.len(){
        let mut ret = Vec::new();
        for steps in &steps_matrix[steps_matrix.len() - 1] {
            ret.push(steps.clone() as u64);
        }
        return ret;
    }

    let recursive = get_all_gcds_recursive(&steps_matrix, offset + 1);

    let mut ret : Vec<u64> = Vec::new();
    for gcd_before in recursive {
        for value in &steps_matrix[offset] {
            ret.push((value.clone() as u64) * gcd_before / gcd(value.clone() as u64, gcd_before));
        }
    }
    return ret;
}

fn get_steps_required(from: usize, to: usize, map: &Vec<(usize, usize)>, instruction: &String) -> Option<u32> {
    let limit = (map.len() * instruction.len()) as u32;

    let mut steps : u32 = 0;
    let mut current = from;

    loop {
        for c in instruction.chars() {
            if c == 'L' {
                current = map[current].0;
            }
            else {
                current = map[current].1;
            }
            steps += 1;

            if current == to {
                return Some(steps);
            }

            if steps > limit {
                return None;
            }
        }
    }
}

fn main() -> Result<(), Error> {
    let before_time = Instant::now();

    let filename = "input8.txt";

    let file = File::open(filename)?;

    let reader = BufReader::new(file);
    let mut lines = reader.lines();


    let mut map_raw : Vec<(String, String, String)> = Vec::new();
    let instruction = lines.next().unwrap()?;
    assert_eq!(lines.next().unwrap()?.len(), 0);

    for line_result in lines {
        let line = line_result.unwrap();
        let line_split : Vec<&str>= line.split_whitespace().collect();
        let from : String = line_split[0].to_string();
        let left : String = line_split[2][1..4].to_string();
        let right : String = line_split[3][0..3].to_string();

        map_raw.push((from, left, right));
    }

    let mut current_id : usize = 0;
    let mut id_lookup : HashMap<String, usize> = HashMap::new();
    let mut start_nodes : Vec<usize> = Vec::new();
    for line in &map_raw {
        if line.0.ends_with("A") {
            id_lookup.insert(line.0.to_string(), current_id);
            start_nodes.push(current_id);
            current_id += 1;
        }
    }
    for line in &map_raw {
        if !line.0.ends_with("Z") && !line.0.ends_with("A") {
            id_lookup.insert(line.0.to_string(), current_id);
            current_id += 1;
        }
    }
    let z_start = current_id;
    for line in &map_raw {
        if line.0.ends_with("Z") {
            id_lookup.insert(line.0.to_string(), current_id);
            current_id += 1;
        }
    }
    let node_count = current_id;

    let mut map : Vec<(usize, usize)> = Vec::new();
    map.resize(current_id as usize, (0,0));

    for line in map_raw {
        let from_id = id_lookup.get(line.0.as_str()).unwrap();
        let left_id = id_lookup.get(line.1.as_str()).unwrap();
        let right_id = id_lookup.get(line.2.as_str()).unwrap();
        map[*from_id] = (*left_id, *right_id);
    }

    let mut steps_matrix = vec![Vec::new(); start_nodes.len()];
    for start_node in start_nodes {
        for goal_node in z_start..node_count {
            let steps = get_steps_required(start_node, goal_node, &map, &instruction);;

            if steps.is_some() {
                steps_matrix[start_node].push(steps.unwrap());
                println!("{} to {} takes {} steps", start_node, goal_node, steps.unwrap());
            }
            else {
                println!("{} cannot reach {}", start_node, goal_node);
            }
        }
    }

    let gcds = get_all_gcds_recursive(&steps_matrix, 0);
    let mut result = u64::MAX;
    for gcd in gcds {
        println!("We can combine it with: {}", gcd);
        result = min(result, gcd)
    }


    println!("Time: {}", before_time.elapsed().as_micros());
    println!("Result: {}", result);

    return Ok(())
}
