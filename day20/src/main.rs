use std::collections::{HashMap, VecDeque};
use std::fs::{read_to_string};
use std::io::{Error};
use std::time::Instant;
use crate::NodeType::{FlipFlopOn, FlipFlopOff, Conjunction, Broadcast, Noop};

#[derive(PartialEq, Clone)]
enum NodeType {
    FlipFlopOn, FlipFlopOff, Conjunction, Broadcast, Noop
}

#[derive(Clone)]
struct Node {
    label: String,
    node_type: NodeType,
    // last_signal_sent: bool,
    inputs: Vec<usize>,
    outputs: Vec<usize>
}

fn parse_node(line: &str, label_nums: &mut HashMap<String, usize>, all_nodes: &mut Vec<Node>) {
    let split = line.split_once(" -> ").unwrap();

    let node_type: NodeType;
    let label: String;
    if split.0 == "broadcaster" {
        node_type = Broadcast;
        label = "broadcaster".to_string();
    }
    else{
        node_type = if split.0.chars().nth(0).unwrap() == '&' { Conjunction } else { FlipFlopOff };
        label = split.0.chars().skip(1).collect::<String>();
    }

    if !label_nums.contains_key(&label.to_string()) {
        label_nums.insert(label.to_string(), label_nums.len());
        all_nodes.push(Node{
            label: label.to_string(),
            node_type: Noop,
            // last_signal_sent: false,
            inputs: Vec::new(), outputs: Vec::new()});
    }
    let my_num: usize = label_nums[&label];

    let output_labels: Vec<&str> = split.1.split(", ").collect();
    let mut outputs: Vec<usize> = Vec::new();
    for output_label in &output_labels {
        if !label_nums.contains_key(&output_label.to_string()) {
            label_nums.insert(output_label.to_string(), label_nums.len());
            all_nodes.push(Node{
                label: output_label.to_string(),
                node_type: Noop,
                // last_signal_sent: false,
                inputs: Vec::new(), outputs: Vec::new()});
        }
        let output_num = label_nums[&output_label.to_string()];
        outputs.push(output_num);
        all_nodes[output_num].inputs.push(my_num);
    }

    all_nodes[my_num] = Node{
        label,
        node_type,
        // last_signal_sent: false,
        inputs: all_nodes[my_num].inputs.clone(),
        outputs,
    };
}
/*
struct PeriodicCounter {
    nodes: Vec<usize>,
    input: usize,
    output: usize,
    period: u64
}

fn detect_periodic_counter(node_num: usize, all_nodes : &Vec<Node>) -> Option<PeriodicCounter> {
    let node: &Node = &all_nodes[node_num];
    if node.node_type != Conjunction {
        return None;
    }
    if node.inputs.iter().any(|x| all_nodes[*x].node_type != FlipFlopOff) {
        return None;
    }
    for input in &node.inputs {
        if all_nodes[*input].outputs.len() == 1 {
            if all_nodes[*input][0] != node_num {
                return None;
            }
        }
        else if all_nodes[*input].outputs.len() == 2 {
            if !all_nodes[*input].outputs.contains(&node_num) {
                return None;
            }
        }
    }
    return PeriodicCounter {nodes: node.inputs.clone(), };
}

 */
fn main() -> Result<(), Error> {
    let before_time = Instant::now();

    let filename = "input.txt";

    let input = read_to_string(filename).unwrap();
    let lines: Vec<&str> = input.lines().collect();
    let mut all_nodes: Vec<Node> = Vec::new();
    let mut label_nums: HashMap<String, usize> = HashMap::new();
    for line in lines {
        parse_node(line, &mut label_nums, &mut all_nodes);
    }


/*
    let mut periodic_counters: Vec<PeriodicCounter> = Vec::new();

    for node in &all_nodes {
        let maybe_periodic = detect_periodic_counter(node, &all_nodes);
        if maybe_periodic.is_some() {
            periodic_counters.push(maybe_periodic.unwrap())
        }
    } */


    let mut conjunction_memory: Vec<Vec<bool>> = vec![Vec::new(); all_nodes.len()];
    for (num, node) in all_nodes.iter().enumerate() {
        if node.node_type == Conjunction {
            conjunction_memory[num].resize(all_nodes.len(), false);
        }
    }

    let mut low_signals = 0;
    let mut high_signals = 0;

    let tracking_nums =
        [label_nums[&"vg".to_string()], label_nums[&"nb".to_string()],label_nums[&"vc".to_string()],label_nums[&"ls".to_string()]];

    let rx_num = label_nums[&"rx".to_string()];

    let mut sol2 = 0;

    let mut signal_queue: VecDeque<(usize, bool, usize)> = VecDeque::new();
    let mut step: u64 = 0;
    loop {
        step += 1;
        if step % 1000000 == 0 {
            println!("{}", step);
        }

        signal_queue.push_back((label_nums[&"broadcaster".to_string()], false, label_nums[&"broadcaster".to_string()]));

        // print_memory(&conjunction_memory);

        while let Some((from, high, to)) = signal_queue.pop_front() {
            if tracking_nums.contains(&to) && !high {
                println!("{} high on press {}", all_nodes[to].label, step);
                // sol2 = step;
            }
            if false {
                if high {
                    high_signals += 1;
                }
                else {
                    low_signals += 1;
                }
            }

            let node_type = &all_nodes[to].node_type;
            // println!("{} -{}> {}", &all_nodes[from].label, if high {"high"} else {"low"}, &all_nodes[to].label);

            if *node_type == Broadcast {
                for i in &all_nodes[to].outputs {
                    signal_queue.push_back((to, high, *i));
                }
            }
            else if *node_type == FlipFlopOff {
                if !high {
                    all_nodes[to].node_type = FlipFlopOn;
                    for i in &all_nodes[to].outputs {
                        signal_queue.push_back((to, true, *i));
                    }
                }
            }
            else if *node_type == FlipFlopOn {
                if !high {
                    all_nodes[to].node_type = FlipFlopOff;
                    for i in &all_nodes[to].outputs {
                        signal_queue.push_back((to, false, *i));
                    }
                }
            }
            else if *node_type == Conjunction {
                conjunction_memory[to][from] = high;
                let mut all_high = true;
                for i in &all_nodes[to].inputs {
                    if !conjunction_memory[to][*i] {
                        all_high = false;
                        break;
                    }
                }
                if all_high {
                    for i in &all_nodes[to].outputs {
                        signal_queue.push_back((to, false, *i));
                    }
                }
                else {
                    for i in &all_nodes[to].outputs {
                        signal_queue.push_back((to, true, *i));
                    }
                }
            }
        }
    }

    let sol1 = high_signals * low_signals;


    println!("Result part1: {}", sol1);
    println!("Result part2: {}", sol2);
    println!("Time: {} micros", before_time.elapsed().as_micros());

    return Ok(());
}
