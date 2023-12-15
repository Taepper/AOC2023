use std::cmp::{max, min, Ordering};
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::time::Instant;

fn main() -> Result<(), Error> {
    let before_time = Instant::now();

    let filename = "input6.txt";

    let file = File::open(filename)?;

    // Create a buffered reader to efficiently read lines
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let seeds_line = &lines.next().unwrap().unwrap()[6..];
    let seeds_raw : Vec<u64> = seeds_line.split_whitespace().map(|val| val.parse::<u64>().unwrap()).collect();
    let mut seeds : Vec<(u64, u64)> = Vec::new();
    for i in 0..seeds_raw.len() / 2 {
        seeds.push((seeds_raw[2*i], seeds_raw[2*i + 1]));
    }

    assert!(lines.next().unwrap().unwrap().is_empty());

    let mut maps : Vec<Vec<(u64, u64, u64)>> = Vec::new();

    for _i in 0..7u32 {
        let line = lines.next().unwrap().unwrap();
        assert!(line.ends_with(" map:"));
        let mut current_map : Vec<(u64, u64, u64)> = Vec::new();
        loop {
            let line_opt = lines.next();
            if line_opt.is_none() {
                break;
            }
            let line = line_opt.unwrap().unwrap();
            if line.is_empty() {
                break;
            }
            let numbers : Vec<&str> = line.split_whitespace().collect();
            assert_eq!(numbers.len(), 3);
            current_map.push((
                numbers[1].parse::<u64>().unwrap(),
                numbers[0].parse::<u64>().unwrap(),
                numbers[2].parse::<u64>().unwrap()));
        }

        current_map.sort();

        maps.push(current_map);
    }


    let mut current_ranges = seeds;

    for i in 0..7 {
        let mut new_ranges : Vec<(u64, u64)> = Vec::new();
        for (lower, l) in &current_ranges {
            let upper = lower + l;
            let current_map = &maps[i];
            let lower_bound = current_map.binary_search_by(|(a, _b, _l)| match a.cmp(&lower) {
                Ordering::Equal => Ordering::Less,
                cmp => cmp
            });
            let idx1 = lower_bound.unwrap_err();
            let lower_bound = current_map.binary_search_by(|(a, _b, _l)| match a.cmp(&upper) {
                Ordering::Equal => Ordering::Less,
                cmp => cmp
            });
            let idx2 = lower_bound.unwrap_err();
            if 0 < idx2 {
                let mut last_upper = *lower;
                for idx in (max(idx1, 1)-1)..idx2 {
                    let (a, b, l) = current_map[idx];
                    let shift : i64 = b as i64 - a as i64;

                    if a + l < *lower {
                        continue;
                    }

                    if last_upper < a {
                        new_ranges.push((last_upper, a - last_upper));
                    }
                    let lower_of_range_shift = max(a, last_upper);
                    let upper_of_range_shift = min(a + l, upper);
                    if lower_of_range_shift < upper_of_range_shift {
                        let l_of_range_shift = upper_of_range_shift - lower_of_range_shift;
                        if l_of_range_shift > 0 {
                            let shifted_lower = max(lower_of_range_shift as i64 + shift, 0) as u64;
                            new_ranges.push((shifted_lower, l_of_range_shift));
                        }
                        last_upper = upper_of_range_shift;
                    }
                }
                if last_upper < upper {
                    new_ranges.push((last_upper, upper - last_upper));
                }
            } else {
                new_ranges.push((*lower, upper - *lower));
            }
        }

        if i == 6 {
            let mut lowest = u64::MAX;
            for (a, _l) in &new_ranges {
                if a < &lowest {
                    lowest = *a;
                }
            }
            println!("Result: {}", lowest);
            println!("Elapsed time: {:.2?}", before_time.elapsed());

            return Ok(());
        }

        new_ranges.sort();

        current_ranges.resize(0, (0,0));

        let mut current_lower = new_ranges[0].0;
        let mut current_upper = new_ranges[0].0 + new_ranges[0].1;
        for i in 1..new_ranges.len() {
            let lower = new_ranges[i].0;
            let upper = new_ranges[i].0 + new_ranges[i].1;
            if lower > current_upper{
                current_ranges.push((current_lower, current_upper - current_lower));
                current_lower = lower;
                current_upper = upper;
            }
            else{
                current_upper = max(upper, current_upper);
            }
        }
        current_ranges.push((current_lower, current_upper - current_lower));
    }

    return Ok(());
}
