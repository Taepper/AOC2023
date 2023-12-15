use std::cmp::min;
use std::fs::{read_to_string};
use std::time::Instant;

fn find_first_symmetry(nums : &Vec<u64>) -> usize {
    for i in 1..nums.len() {
        let mut symmetric = true;
        for x in 0..min(i,nums.len() - i){
            if nums[i+x] != nums[i-1-x] {
                symmetric = false;
                break;
            }
        }
        if symmetric {
            return i;
        }
    }
    return 0;
}

fn find_smudge_symmetry(nums : &Vec<u64>) -> usize {
    for i in 1..nums.len() {
        let mut smudges = 0;
        for x in 0..min(i,nums.len() - i){
            let difference = nums[i+x] ^ nums[i-1-x];
            smudges += difference.count_ones();
            if smudges > 1 {
                break;
            }
        }
        if smudges == 1 {
            return i;
        }
    }
    return 0;
}

fn main() {
    let before_time = Instant::now();

    let filename = "input.txt";

    let input = read_to_string(filename).unwrap();
    let puzzles : Vec<&str> = input.split("\n\n").collect();

    let mut sol1 : u32 = 0;
    let mut sol2 : u32 = 0;
    for (_puzzle_num, puzzle) in puzzles.iter().enumerate() {
        let lines : Vec<&str> = puzzle.split("\n").collect();

        let mut row_nums : Vec<u64> = vec![0; lines.len()];
        let mut col_nums : Vec<u64> = vec![0; lines[0].len()];
        for (row, line) in lines.iter().enumerate() {
            for (col, char) in line.chars().enumerate() {
                let b = if char == '.' {1} else {0};
                col_nums[col] <<= 1;
                col_nums[col] += b;

                row_nums[row] <<= 1;
                row_nums[row] += b;
            }
        }
        let sym_row = find_first_symmetry(&row_nums);
        let sym_col = find_first_symmetry(&col_nums);
        sol1 += sym_row as u32 * 100;
        sol1 += sym_col as u32;

        sol2 += find_smudge_symmetry(&row_nums) as u32 * 100;
        sol2 += find_smudge_symmetry(&col_nums) as u32;
    }


    println!("Result part1: {}", sol1);
    println!("Result part2: {}", sol2);
    println!("Time: {} micros", before_time.elapsed().as_micros());

}
