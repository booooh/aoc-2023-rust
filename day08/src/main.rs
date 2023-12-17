use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    fs,
};
type Instructions = HashMap<String, [String; 2]>;

#[derive(Debug)]
struct Network {
    step_list: Vec<usize>,
    instructions: Instructions,
}

impl From<&str> for Network {
    fn from(value: &str) -> Self {
        let mut lines = value.split("\n").filter(|l| !l.is_empty());
        let steps = lines.next().unwrap();
        let step_list = steps
            .chars()
            .map(|c| match c {
                'L' => 0,
                'R' => 1,
                _ => panic!("This should not happen"),
            })
            .collect();
        let re = Regex::new(r"(?<from>[^ ]+) = \((?<left>.*), (?<right>.*)\)").unwrap();
        let instructions: Instructions = lines
            .filter(|line| !line.is_empty())
            .map(|line| re.captures(line).unwrap())
            .map(|cap| {
                (
                    cap["from"].into(),
                    [cap["left"].into(), cap["right"].into()],
                )
            })
            .collect();

        Network {
            step_list,
            instructions,
        }
    }
}

fn part1() {
    let lines = fs::read_to_string("day08/input").unwrap();
    let network: Network = lines[..].into();
    let mut curr_node = "AAA".to_string();

    let mut step_list = network.step_list.iter().cycle();
    let mut num_steps = 0;
    while curr_node != "ZZZ" {
        let step = step_list.next().unwrap();
        curr_node = network.instructions[&curr_node][*step].clone();
        num_steps += 1;
        // println!("went to node {}", curr_node);
    }
    println!("num steps: {}", num_steps);
}

fn get_factors_functional(n: usize) -> Vec<usize> {
    (2..n)
        .into_iter()
        .filter(|&x| n % x == 0)
        .collect::<Vec<usize>>()
}

fn part2() {
    let lines = fs::read_to_string("day08/input").unwrap();
    let network: Network = lines[..].into();
    let curr_nodes = network
        .instructions
        .keys()
        .filter(|n| n.ends_with("A"))
        .map(|x| x.clone())
        .collect::<Vec<String>>();

    // curr_nodes = vec![("AAA".to_string())];
    // let mut step_list = network.step_list.iter().cycle();
    // let mut num_steps = 0;
    println!("current nodes: {:?}", curr_nodes);
    let mut num_steps_list = Vec::<_>::new();

    for start_node in curr_nodes.iter() {
        let mut curr_node = start_node.clone();
        let mut step_list = network.step_list.iter().cycle();
        let mut num_steps = 0;
        println!("going to search for a path from {}", curr_node);
        while !curr_node.ends_with("Z") {
            let step = step_list.next().unwrap();
            curr_node = network.instructions[&curr_node][*step].clone();
            num_steps += 1;
            // println!("went to node {}", curr_node);
        }
        println!(
            "start_node {}, end_node {}, num steps: {}",
            start_node, curr_node, num_steps
        );
        num_steps_list.push(num_steps);
    }
    let factors_set = num_steps_list
        .iter()
        .map(|n| get_factors_functional(*n))
        .flatten()
        .collect::<HashSet<_>>();
    println!("{:?}", factors_set.iter().product::<usize>());
}

fn main() {
    part1();
    part2();
}
