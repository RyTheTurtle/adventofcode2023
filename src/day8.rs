use crate::util;
use regex::Regex;
use std::{
    collections::HashMap,
    time::{Instant},
};

pub fn solve() {
    println!("Day 8\n====");
    let input = util::read_lines("./input/8.txt");
    println!("Input line size: {}", input.len());
    println!("Part 1\n---");
    let part1 = part_1(&input);
    println!("Result: {}", part1);
    println!("Part 2\n---");
    let part2 = part_2(&input);
    println!("Result: {}\n====", part2);
}

#[derive(Debug)]
struct Map {
    instructions: Vec<char>,
    points: HashMap<String, (String, String)>,
}

fn parse_map(input: &Vec<String>) -> Map {
    let mut iter = input.iter();
    let instructions: Vec<char> = iter.next().unwrap().chars().collect();
    iter.next().unwrap(); // eat the separator line
    let mut nodes: HashMap<String, (String, String)> = HashMap::new();
    // need to break this out to separate strings,but matches AAA = (BBB, CCC) with match groups AAA, BBB, CCC
    let node_pattern =
        Regex::new(r"^([1-9A-Z]{3})\W\=\W\(([1-9A-Z]{3})\,\W([1-9A-Z]{3})\)$").unwrap();
    loop {
        match iter.next() {
            Some(line) => {
                let c = node_pattern.captures(line).unwrap();
                nodes.insert(
                    String::from(c.get(1).unwrap().as_str()),
                    (
                        String::from(c.get(2).unwrap().as_str()),
                        String::from(c.get(3).unwrap().as_str()),
                    ),
                );
            }
            None => break,
        }
    }
    Map {
        instructions: instructions,
        points: nodes,
    }
}

fn ending_in_z_strategy(pos: &String) -> bool {
    pos.ends_with("Z")
}

fn ending_all_z_strategy(pos: &String) -> bool {
    pos.chars().all(|c| c == 'Z')
}

fn count_steps_to_end(map: &Map, start: &String, end_strategy: fn(&String) -> bool) -> u64 {
    let mut total_steps: u64 = 0;
    let mut current_pos = start;
    let mut instructions = map.instructions.clone();
    while !end_strategy(current_pos) {
        let directions = map.points.get(current_pos).unwrap();
        // this could be more efficient with iterator , but for now it's fine to jus treat the instructions
        // as a queue
        let next_direction = instructions.remove(0);
        current_pos = match next_direction {
            'L' =>  &directions.0,
            'R' =>  &directions.1,
            _ => panic!("invalid direction"),
        };
        // put on end of queue
        instructions.push(next_direction);
        total_steps += 1
    }
    total_steps
}

fn part_1(input: &Vec<String>) -> u64 {
    let desertMap: Map = parse_map(input);
    return count_steps_to_end(&desertMap, &String::from("AAA"), ending_all_z_strategy);
}

fn get_factors(n: u64) -> Vec<u64> {
    // iterates over numbers 1-n, filters by n being divisible by x
    (1..n / 2 + 1)
        .into_iter()
        .filter(|&x| n % x == 0)
        .collect::<Vec<u64>>()
}

/**the LCM of two numbers 'a' and 'b' is equal to the product of the 2 numbers divided by the highest common factor (HCF) of the 2 numbers. */
fn get_lcm(a: u64, b: u64) -> u64 {
    let smaller = a.min(b);
    let bigger = a.max(b);
    if bigger % smaller == 0 {
        return a.min(b);
    }
    let smaller_factors = get_factors(smaller);
    let hcf = smaller_factors
        .iter()
        .filter(|a| b % *a == 0)
        .max()
        .unwrap();
    (a * b) / hcf
}

fn get_lcm_of_vec(n: &Vec<u64>) -> u64 {
    let mut stack = n.clone();
    while stack.len() > 1 {
        let a = stack.pop().unwrap();
        let b = stack.pop().unwrap();
        stack.push(get_lcm(a, b))
    }
    return stack.get(0).unwrap().clone();
}

fn part_2(input: &Vec<String>) -> u64 {
    // parse the map
    // collect a list of all nodes that end with A
    // on each step, iterate all of the nodes toward the next step
    // break loop once every node is ending with Z
    let m: Map = parse_map(input);
    let start: Instant = Instant::now();

    let step_counts: Vec<u64> = m
        .points
        .keys()
        .filter(|k| k.ends_with("A"))
        .map(|p| count_steps_to_end(&m, p, ending_in_z_strategy))
        .collect();
    
    let total_steps = get_lcm_of_vec(&step_counts);
    println!(
        "Finished finding LCM of steps after {:?} ms",
        start.elapsed().as_millis()
    );
    total_steps
}
