use crate::util;
use std::{
    collections::HashSet,
    time::{Duration, Instant},
};

pub fn solve() {
    println!("Day 6\n====");
    let input = util::read_lines("./input/6.txt");
    println!("Input line size: {}", input.len());
    println!("Part 1\n---");
    let part1 = part_1(&input);
    println!("Result: {}", part1);
    println!("Part 2\n---");
    let part2 = part_2(&input);
    println!("Result: {}\n====", part2);
}

fn part_1(input: &Vec<String>) -> u64 {
    // parse inputs
    let times = util::parse_number_vec_following_colon(&input[0]);
    let distances = util::parse_number_vec_following_colon(&input[1]);
    let mut ways_to_beat_record: Vec<u64> = Vec::new();
    for (i, time) in times.into_iter().enumerate() {
        let target_distance = distances[i];
        let mut winning_options = 0;
        for j in 0..time {
            if j * (time - j) > target_distance {
                winning_options += 1;
            }
        }
        ways_to_beat_record.push(winning_options);
    }
    return ways_to_beat_record
        .iter()
        .fold(1, |acc, &e| acc * e);
}

fn part_2(input: &Vec<String>) -> u64 {
    // parse inputs
    let times = util::parse_number_vec_following_colon(&input[0]);
    let distances = util::parse_number_vec_following_colon(&input[1]);
    let mut ways_to_beat_record: Vec<u64> = Vec::new();
    for (i, time) in times.into_iter().enumerate() {
        let target_distance = distances[i];
        let mut winning_options = 0;
        for j in 0..time {
            if j * (time - j) > target_distance {
                winning_options += 1;
            }
        }
        ways_to_beat_record.push(winning_options);
    }
    return *ways_to_beat_record.get(0).unwrap();
}
