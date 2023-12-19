use std::cmp::{max, min};
use std::collections::HashMap;
use std::fs::{read_to_string};
use std::io::{Error};
use std::time::Instant;

struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

fn parse_part(line: &str) -> Part {

    let stripped = line.strip_prefix("{").unwrap().strip_suffix("}").unwrap();
    let split: Vec<&str> = stripped.split(",").collect();
    assert_eq!(split.len(), 4);
    let x = split[0].chars().skip(2).collect::<String>().parse::<u32>().unwrap();
    let m = split[1].chars().skip(2).collect::<String>().parse::<u32>().unwrap();
    let a = split[2].chars().skip(2).collect::<String>().parse::<u32>().unwrap();
    let s = split[3].chars().skip(2).collect::<String>().parse::<u32>().unwrap();
    return Part {x, m, a, s}
}

#[derive(Clone)]
struct Constraint {
    variable: usize,
    smaller: bool,
    value: u32
}

#[derive(Clone)]
struct Rule {
    constraint: Constraint,
    destination_num: usize
}

fn parse_rule(line: &str, label_nums: &mut HashMap<String, usize>) -> Rule {
    let first_split: Vec<&str> = line.split(":").collect();
    assert_eq!(first_split.len(), 2);
    let condition = first_split[0];
    let variable_char: char = condition.chars().nth(0).unwrap();
    let variable =
        if variable_char == 'x' {0}
        else if variable_char == 'm' {1}
        else if variable_char == 'a' {2}
        else {3};
    let smaller: bool = condition.chars().nth(1).unwrap() == '<';
    let value: u32 = condition.chars().skip(2).collect::<String>().parse::<u32>().unwrap();
    let destination: String = first_split[1].to_string();
    if !label_nums.contains_key(&destination) {
        let new_num = label_nums.len();
        label_nums.insert(destination.clone(), new_num);
    }
    let destination_num = label_nums[&destination];
    return Rule{constraint: Constraint{variable, smaller, value}, destination_num};
}

#[derive(Clone)]
struct Workflow {
    label_num: usize,
    rules: Vec<Rule>,
    default_num: usize
}

impl Workflow {
    fn print(self: &Workflow) {
        println!("Workflow {} with default {} and rules:", self.label_num, self.default_num);
        for rule in &self.rules {
            println!("\t[{}] {} {}: {}",
                     rule.constraint.variable, if rule.constraint.smaller {'<'} else {'>'}, rule.constraint.value,
                     rule.destination_num)
        }
    }
}

fn parse_workflow(line: &str, label_nums: &mut HashMap<String, usize>) -> Workflow {
    let first_split: Vec<&str> = line.strip_suffix("}").unwrap().split("{").collect();
    assert_eq!(first_split.len(), 2);
    let label = first_split[0].to_string();
    if !label_nums.contains_key(&label) {
        let new_num = label_nums.len();
        label_nums.insert(label.clone(), new_num);
    }
    let label_num = label_nums[&label];
    let rule_split: Vec<&str> = first_split[1].split(",").collect();

    let mut rules: Vec<Rule> = Vec::new();
    for i in 0..rule_split.len()-1 {
        rules.push(parse_rule(rule_split[i], label_nums));
    }

    let default = rule_split[rule_split.len() - 1].to_string();
    if !label_nums.contains_key(&default) {
        let new_num = label_nums.len();
        label_nums.insert(default.clone(), new_num);
    }
    let default_num = label_nums[&default];

    return Workflow{
        label_num, rules, default_num
    };
}

fn add_constraint(range: &[(u32, u32); 4],
                  constraint: &Constraint) -> Option<[(u32, u32); 4]> {
    return if constraint.smaller {
        // Completely contained:
        //  |    (     )    |
        //  |           <...|
        if range[constraint.variable].1 < constraint.value {
            Some(range.clone())
        }
        // Completely outside:
        //  |    (     )    |
        //  |..<..          |
        else if range[constraint.variable].0 >= constraint.value {
            None
        }
        // Splitting:
        //  |    (     )    |
        //  |     .<....    |
        else {
            let mut range_cpy = range.clone();
            range_cpy[constraint.variable].1 = constraint.value - 1;
            Some(range_cpy)
        }
    } else {
        // Completely inside:
        //  |    (     )    |
        //  |..>.           |
        if range[constraint.variable].0 > constraint.value {
            Some(range.clone())
        }
        // Completely outside:
        //  |    (     )    |
        //  |          .>...|
        else if range[constraint.variable].1 <= constraint.value {
            None
        }
        // Splitting:
        //  |    (     )    |
        //  |    ..>...     |
        else {
            let mut range_cpy = range.clone();
            range_cpy[constraint.variable].0 = constraint.value + 1;
            Some(range_cpy)
        }
    }
}


fn add_constraint_to_vec(ranges_before: &Vec<[(u32, u32); 4]>,
                  constraint: &Constraint) -> Vec<[(u32, u32); 4]>{
    let mut new_ranges: Vec<[(u32, u32); 4]> = Vec::new();
    for range in ranges_before {
        let opt_range = add_constraint(range, constraint);
        if opt_range.is_some() {
            new_ranges.push(opt_range.unwrap());
        }
    }
    return new_ranges;
}


fn intersect(range1: &[(u32, u32); 4], range2: &[(u32, u32); 4]) -> Option<[(u32, u32); 4]> {
    let mut result = [(0,0); 4];
    for i in 0..4 {
        let lower = max(range1[i].0, range2[i].0);
        let upper = min(range1[i].1, range2[i].1);
        if lower > upper {
            return None;
        }
        result[i] = (lower, upper);
    }
    return Some(result);
}


fn winnable_ranges(workflow: &Workflow, workflow_map: &Vec<Workflow>, dp_table: &mut HashMap<usize, Vec<[(u32, u32); 4]>>) {
    let mut my_ranges: Vec<[(u32, u32); 4]> = Vec::new();

    let mut push_through_ranges: Option<[(u32, u32); 4]> = Some([(1, 4000); 4]);
    for rule in &workflow.rules {
        assert!(push_through_ranges.is_some());
        if !dp_table.contains_key(&rule.destination_num) {
            winnable_ranges(&workflow_map[rule.destination_num], workflow_map, dp_table);
        }
        for range in add_constraint_to_vec(&dp_table[&rule.destination_num], &rule.constraint) {
            let intersected_with_filter = intersect(&push_through_ranges.unwrap(), &range);
            if intersected_with_filter.is_some(){
                my_ranges.push(intersected_with_filter.unwrap());
            }
        }
        let opposite_constraint = Constraint{variable: rule.constraint.variable,
            smaller: !rule.constraint.smaller,
            value: if rule.constraint.smaller { rule.constraint.value - 1 }
                else {rule.constraint.value + 1}
        };
        push_through_ranges = add_constraint(&push_through_ranges.unwrap(), &opposite_constraint);
        if push_through_ranges.is_none(){
            break;
        }
    }
    if push_through_ranges.is_some() {
        if !dp_table.contains_key(&workflow.default_num) {
            winnable_ranges(&workflow_map[workflow.default_num], workflow_map, dp_table);
        }
        for range in &dp_table[&workflow.default_num] {
            let intersected_with_filter = intersect(&push_through_ranges.unwrap(), &range);
            if intersected_with_filter.is_some(){
                my_ranges.push(intersected_with_filter.unwrap());
            }
        }
    }

    println!();
    println!("Got the ranges for my workflow:");
    workflow.print();
    println!("Ranges: (xmas)");
    for range in &my_ranges {
        println!("({},{}) ({},{}) ({},{}) ({},{})",
                 range[0].0, range[0].1,
                 range[1].0, range[1].1,
                 range[2].0, range[2].1,
                 range[3].0, range[3].1)
    }
    dp_table.insert(workflow.label_num, my_ranges);
}

fn main() -> Result<(), Error> {
    let before_time = Instant::now();

    let filename = "input.txt";

    let mut label_nums: HashMap<String, usize> = HashMap::new();
    label_nums.insert("A".to_string(), 0);
    label_nums.insert("R".to_string(), 1);

    let input = read_to_string(filename).unwrap();
    let input_split: Vec<&str> = input.split("\n\n").collect();
    assert_eq!(input_split.len(), 2);
    let workflows: Vec<Workflow> = input_split[0].split("\n")
        .map(|line| parse_workflow(line, &mut label_nums))
        .collect();
    let parts: Vec<Part> = input_split[1].split("\n")
        .map(|line| parse_part(line))
        .collect();


    let mut workflow_lookup: Vec<Workflow> = Vec::new();
    workflow_lookup.resize(label_nums.len(),
                           Workflow { label_num: 0, default_num: 0, rules: Vec::new() });

    for workflow in workflows {
        workflow_lookup[workflow.label_num] = workflow.clone();
    }

    let start_workflow = workflow_lookup[label_nums["in"]].clone();

    let mut dp_table: HashMap<usize, Vec<[(u32, u32); 4]>> = HashMap::new();
    dp_table.insert(0, vec![[(1, 4000); 4]; 1]);
    dp_table.insert(1, Vec::new());
    winnable_ranges(&start_workflow, &workflow_lookup, &mut dp_table);

    let sol2 = dp_table[&start_workflow.label_num].iter()
        .map(|range|
            (range[0].1 - range[0].0 + 1) as u64 * (range[1].1 - range[1].0 + 1) as u64 *
                (range[2].1 - range[2].0 + 1) as u64 * (range[3].1 - range[3].0 + 1) as u64)
        .sum::<u64>();


    let mut sol1 = 0;
    for part in parts {
        for range in &dp_table[&start_workflow.label_num] {
            if part.x < range[0].0 || range[0].1 < part.x {
                continue;
            }
            if part.m < range[1].0 || range[1].1 < part.m {
                continue;
            }
            if part.a < range[2].0 || range[2].1 < part.a {
                continue;
            }
            if part.s < range[3].0 || range[3].1 < part.s {
                continue;
            }
            sol1 += part.x + part.m + part.a + part.s;
        }
    }

    println!("Result part1: {}", sol1);
    println!("Result part2: {}", sol2);
    println!("Time: {} micros", before_time.elapsed().as_micros());

    return Ok(());
}
