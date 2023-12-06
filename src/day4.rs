use crate::util;
use std::{collections::HashSet, str::Split};

pub fn solve() {
    println!("Day 4\n====");
    let input = util::read_lines("./input/4.txt");
    println!("Input line size: {}", input.len());
    println!("Part 1\n---");
    let part1 = part_1(&input);
    println!("Result: {}", part1);
    println!("Part 2\n---");
    let part2 = part_2(&input);
    println!("Result: {}\n====", part2);
}

fn part_1(input: &Vec<String>) -> u64 {
    input
        .into_iter()
        .map(to_card)
        .map(get_score)
        .sum()
}

fn part_2(input: &Vec<String>) -> u64 {
    let winning_number_counts: Vec<u8> = input
        .into_iter()
        .map(to_card)
        .map(count_winning_numbers)
        .collect();

    let mut card_counts: Vec<u64> = Vec::with_capacity(winning_number_counts.len());
    for _ in 0..winning_number_counts.len() {
        card_counts.push(1);
    }

    for (card_idx, winning_cards_count) in winning_number_counts.iter().enumerate() {
        for next_card_offset in 1..*winning_cards_count + 1 {
            match card_counts.get(card_idx + next_card_offset as usize) {
                Some(_) => card_counts[card_idx + next_card_offset as usize] += card_counts[card_idx],
                None => { /*no op, out of bounds */ }
            }
        }
    }
    card_counts.iter().sum::<u64>()
}

#[derive(Debug)]
struct Card {
    numbers_you_have: Vec<u8>,
    winning_numbers: Vec<u8>,
}

fn count_winning_numbers(c: Card) -> u8 {
    let n: HashSet<&u8> = HashSet::from_iter(c.numbers_you_have.iter());
    let w: HashSet<&u8> = HashSet::from_iter(c.winning_numbers.iter());
    n.intersection(&w).count() as u8
}

fn get_score(c: Card) -> u64 {
    match count_winning_numbers(c) {
        0 => 0,
        d => 2_u64.pow(d as u32 - 1),
    }
}

fn to_card(s: &String) -> Card {
    let parts: Vec<Vec<u8>> = s
        .split(":")
        .into_iter()
        .nth(1)
        .unwrap()
        .split("|")
        .map(to_u8_vec)
        .collect();

    Card {
        numbers_you_have: parts.get(0).unwrap().to_vec(),
        winning_numbers: parts.get(1).unwrap().to_vec(),
    }
}

fn to_u8_vec(s: &str) -> Vec<u8> {
    s.trim()
        .split_ascii_whitespace()
        .map(|a| a.parse().unwrap())
        .collect()
}
