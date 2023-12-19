use std::collections::HashMap;
use rayon::prelude::*;
use std::fs::{read_to_string};
use std::io::{Error};
use std::time::Instant;

const DEBUG_PRINTING : bool = false;

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
struct Rule {
    variable: usize,
    smaller: bool,
    value: u32,
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
    return Rule{variable, smaller, value, destination_num};
}

#[derive(Clone)]
struct Workflow {
    label_num: usize,
    rules: Vec<Rule>,
    default_num: usize
}

fn parse_workflow(line: &str, label_nums: &mut HashMap<String, usize>) -> Workflow {
    let first_split: Vec<&str> = line.split("}").collect::<Vec<&str>>()[0].split("{").collect();
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

fn move_part(variables: [u32; 4], workflow: &Workflow, workflow_map: &Vec<Workflow>) -> bool {
    for rule in &workflow.rules {
        let to_compare = variables[rule.variable];
        if (rule.smaller && to_compare < rule.value) || (!rule.smaller && to_compare > rule.value) {
            if rule.destination_num == 0 {
                return true;
            }
            else if rule.destination_num == 1 {
                return false;
            }
            return move_part(variables, &workflow_map[rule.destination_num], workflow_map);
        }
    }
    if workflow.default_num == 0 {
        return true;
    }
    else if workflow.default_num == 1 {
        return false;
    }
    let default_label = workflow.default_num;
    return move_part(variables, &workflow_map[default_label], workflow_map);
}

fn sub(x: u32, m: u32, start_workflow: &Workflow, workflow_lookup: &Vec<Workflow>) -> u64 {
    let mut num = 0;
    for a in 1..4001 {
        for s in 1..4001 {
            if move_part([x, m, a, s], &start_workflow, &workflow_lookup) {
                num += 1;
            }
        }
    }
    return num;
}

fn main() -> Result<(), Error> {
    let before_time = Instant::now();

    let filename = "input.txt";

    let mut label_nums: HashMap<String, usize> = HashMap::new();
    label_nums.insert("A".to_string(), 0);
    label_nums.insert("R".to_string(), 1);

    let input = read_to_string(filename).unwrap();
    let input_split : Vec<&str> = input.split("\n\n").collect();
    assert_eq!(input_split.len(), 2);
    let workflows : Vec<Workflow> = input_split[0].split("\n")
        .map(|line| parse_workflow(line, &mut label_nums))
        .collect();
    let parts: Vec<Part> = input_split[1].split("\n")
        .map(|line| parse_part(line))
        .collect();


    let mut workflow_lookup: Vec<Workflow> = Vec::new();
    workflow_lookup.resize(label_nums.len(),
                           Workflow{label_num:0, default_num: 0, rules: Vec::new()});

    for workflow in workflows {
        println!("Workflow {} with default {} and rules:", workflow.label_num, workflow.default_num);
        for rule in &workflow.rules {
            println!("\t{} {} {}: {}",
                     rule.variable, if rule.smaller {'<'} else {'>'}, rule.value, rule.destination_num)
        }
        workflow_lookup[workflow.label_num] = workflow.clone();
    }

    let start_workflow = workflow_lookup[label_nums["in"]].clone();

    let mut sol1 = 0;
    for part in parts {
        println!("Part {} {} {} {}",
                 part.x, part.m, part.a, part.s);
        if move_part([part.x, part.m, part.a, part.s], &start_workflow, &workflow_lookup) {
            sol1 += part.x + part.m + part.a + part.s;
        }
    }

    let mut sol2 = 0;
    for x in 1..1001 {
        println!("x: {}", x);
        let m_res : Vec<u64> =
        (1..101).into_par_iter().map(
            |m| sub(x, m, &start_workflow, &workflow_lookup)
        ).collect();

        for (m, num) in m_res.iter().enumerate() {
            println!("m {} num {}", m, num);
            sol2 += num;
        }
    }

    println!("Result part1: {}", sol1);
    println!("Result part2: {}", sol2);
    println!("Time: {} micros", before_time.elapsed().as_micros());

    return Ok(());
}
