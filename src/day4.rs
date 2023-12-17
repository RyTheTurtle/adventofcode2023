use crate::structs::scratchcard::ScratchCard;
use std::collections::HashSet;

pub fn part_1(input: &Vec<String>) -> u64 {
    input.into_iter().map(to_card).map(get_score).sum()
}

pub fn part_2(input: &Vec<String>) -> u64 {
    let winning_number_counts: Vec<u8> =
        input.into_iter().map(to_card).map(count_winning_numbers).collect();

    let mut card_counts: Vec<u64> = Vec::with_capacity(winning_number_counts.len());
    for _ in 0..winning_number_counts.len() {
        card_counts.push(1);
    }

    for (card_idx, winning_cards_count) in winning_number_counts.iter().enumerate() {
        for next_card_offset in 1..*winning_cards_count + 1 {
            match card_counts.get(card_idx + next_card_offset as usize) {
                Some(_) => {
                    card_counts[card_idx + next_card_offset as usize] += card_counts[card_idx]
                }
                None => { /*no op, out of bounds */ }
            }
        }
    }
    card_counts.iter().sum::<u64>()
}

fn count_winning_numbers(c: ScratchCard) -> u8 {
    let n: HashSet<&u8> = HashSet::from_iter(c.numbers_you_have.iter());
    let w: HashSet<&u8> = HashSet::from_iter(c.winning_numbers.iter());
    n.intersection(&w).count() as u8
}

fn get_score(c: ScratchCard) -> u64 {
    match count_winning_numbers(c) {
        0 => 0,
        d => 2_u64.pow(d as u32 - 1),
    }
}

fn to_card(s: &String) -> ScratchCard {
    let parts: Vec<Vec<u8>> =
        s.split(":").into_iter().nth(1).unwrap().split("|").map(to_u8_vec).collect();

    ScratchCard {
        numbers_you_have: parts.get(0).unwrap().to_vec(),
        winning_numbers: parts.get(1).unwrap().to_vec(),
    }
}

fn to_u8_vec(s: &str) -> Vec<u8> {
    s.trim().split_ascii_whitespace().map(|a| a.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::*;
    // FIXME Move to integration tests

    #[test]
    pub fn test_part1() {
        let input = util::read_lines("./input/4.txt");
        assert_eq!(part_1(&input), 21485);
    }

    #[test]
    pub fn test_part2() {
        let input = util::read_lines("./input/4.txt");
        assert_eq!(part_2(&input), 11024379);
    }
}
