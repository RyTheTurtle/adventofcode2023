use crate::util;
use regex::Regex;
use std::{
    cmp::Ordering,
    collections::HashMap,
    collections::HashSet,
    time::{Duration, Instant},
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
    let node_pattern = Regex::new(r"^([A-Z]{3})\W\=\W\(([A-Z]{3})\,\W([A-Z]{3})\)$").unwrap();
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

fn part_1(input: &Vec<String>) -> u64 {
    let mut m: Map = parse_map(input);
    let mut total_steps: u64 = 0;
    let mut current_pos = String::from("AAA");
    let end = String::from("ZZZ");

    while current_pos != end {
        let directions = m.points.get(&current_pos).unwrap();
        // this could be more efficient with iterator , but for now it's fine to jus treat the instructions
        // as a queue
        let next_direction = m.instructions.remove(0);
        match next_direction {
            'L' => {
                current_pos = directions.0.to_string();
            }
            'R' => {
                current_pos = directions.1.to_string();
            }
            _ => panic!("invalid direction"),
        }
        // put on end of queue
        m.instructions.push(next_direction);
        total_steps += 1
    }
    total_steps
}

fn part_2(input: &Vec<String>) -> u64 {
    0
}
