use crate::structs::camel_card::{compare_bid, parse_bid, parse_hand, Bid, CamelCard, HandType};
use std::collections::HashSet;

fn rank(h: &Vec<CamelCard>) -> HandType {
    let unique_cards: HashSet<&CamelCard> = HashSet::from_iter(h.iter());
    match unique_cards.len() {
        1 => {
            return HandType::FiveOfKind;
        }
        2 => {
            // need to check if it's a full house or  4 of a kind
            for card in unique_cards {
                match h.iter().filter(|c| *c == card).count() {
                    2 | 3 => {
                        // has to be a full house
                        return HandType::FullHouse;
                    }
                    4 => {
                        return HandType::FourOfKind;
                    }
                    _ => {
                        continue;
                    }
                }
            }
            panic!("Invalid hand {:?}", h);
        }
        3 => {
            // can be either three of a kind or two pair
            for card in unique_cards {
                match h.iter().filter(|c| *c == card).count() {
                    3 => {
                        // has to be a three of a kind since we have 3 distinct cards
                        return HandType::ThreeOfKind;
                    }
                    2 => {
                        return HandType::TwoPair;
                    }
                    _ => {
                        continue;
                    }
                }
            }
            panic!("Error parsing hand type {:?}", h);
        }
        4 => {
            return HandType::OnePair;
        }
        5 => return HandType::HighCard,
        _ => {
            panic!("Error parsing hand type {:?}", h)
        }
    }
}

fn rank_jokers(h: &Vec<CamelCard>) -> HandType {
    let joker_count = h.iter().filter(|c| **c == CamelCard::J).count();
    return match rank(h) {
        HandType::FiveOfKind | HandType::FourOfKind | HandType::FullHouse if joker_count > 0 => {
            HandType::FiveOfKind
        }

        HandType::ThreeOfKind if joker_count == 1 => HandType::FourOfKind,
        HandType::ThreeOfKind if joker_count == 3 => HandType::FourOfKind,

        HandType::TwoPair if joker_count == 1 => HandType::FullHouse,
        HandType::TwoPair if joker_count == 2 => HandType::FourOfKind,

        HandType::OnePair if joker_count == 1 => HandType::ThreeOfKind,
        HandType::OnePair if joker_count == 2 => HandType::ThreeOfKind,

        HandType::HighCard if joker_count == 1 => HandType::OnePair,
        _ => rank(h),
    };
}

pub fn part_1(input: &Vec<String>) -> u64 {
    let mut bids: Vec<Bid> = input
        .iter()
        .map(|s| parse_bid(s, parse_hand, rank))
        .collect();
    bids.sort_by(compare_bid);
    let mut result: u64 = 0;
    for (i, bid) in bids.into_iter().enumerate() {
        result += ((i + 1) * bid.amount as usize) as u64;
    }
    result
}

pub fn part_2(input: &Vec<String>) -> u64 {
    let mut bids: Vec<Bid> = input
        .iter()
        .map(|s| parse_bid(s, parse_hand, rank_jokers))
        .collect();
    bids.sort_by(compare_bid);
    let mut result: u64 = 0;
    for (i, bid) in bids.into_iter().enumerate() {
        result += (((i as u64) + 1) * bid.amount as u64) as u64;
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::*;
    // FIXME Move to integration tests

    #[test]
    pub fn test_part1() {
        let input = util::read_lines("./input/7.txt");
        assert_eq!(part_1(&input), 247815719);
    }

    #[test]
    pub fn test_part2() {
        let input = util::read_lines("./input/7.txt");
        assert_eq!(part_2(&input), 248747492);
    }
}
