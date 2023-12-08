use crate::util;
use std::{
    cmp::Ordering,
    collections::HashSet,
    time::{Duration, Instant},
};

pub fn solve() {
    println!("Day 7\n====");
    let input = util::read_lines("./input/7.txt");
    println!("Input line size: {}", input.len());
    println!("Part 1\n---");
    let part1 = part_1(&input);
    println!("Result: {}", part1);
    println!("Part 2\n---");
    let part2 = part_2(&input);
    println!("Result: {}\n====", part2);
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
enum Card {
    n2,
    n3,
    n4,
    n5,
    n6,
    n7,
    n8,
    n9,
    T,
    J,
    Q,
    K,
    A,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Hand {
    cards: Vec<Card>,
    hand_type: HandType,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Bid {
    hand: Hand,
    amount: u32,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

fn rank(h: &Vec<Card>) -> HandType {
    let unique_cards: HashSet<&Card> = HashSet::from_iter(h.iter());
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

fn parse_bid(input: &String) -> Bid {
    let mut iter = input.split_ascii_whitespace();
    let hand = parse_hand(iter.next().unwrap());
    let amount: u32 = iter.next().unwrap().parse().unwrap();
    Bid {
        hand: hand,
        amount: amount,
    }
}

fn parse_hand(h: &str) -> Hand {
    let mut hand: Vec<Card> = Vec::new();
    for c in h.trim().chars() {
        match c {
            '2' => hand.push(Card::n2),
            '3' => hand.push(Card::n3),
            '4' => hand.push(Card::n4),
            '5' => hand.push(Card::n5),
            '6' => hand.push(Card::n6),
            '7' => hand.push(Card::n7),
            '8' => hand.push(Card::n8),
            '9' => hand.push(Card::n9),
            'T' => hand.push(Card::T),
            'J' => hand.push(Card::J),
            'Q' => hand.push(Card::Q),
            'K' => hand.push(Card::K),
            'A' => hand.push(Card::A),
            _ => {
                panic!("Invalid char for hand {:?}", c)
            }
        }
    }
    let rank = rank(&hand);
    Hand {
        cards: hand,
        hand_type: rank,
    }
}

fn compare(b1: &Bid, b2: &Bid) -> Ordering {
    match b1
        .hand
        .hand_type
        .cmp(&b2.hand.hand_type)
    {
        Ordering::Greater => Ordering::Greater,
        Ordering::Less => Ordering::Less,
        Ordering::Equal => {
            for card_idx in 0..5 {
                match b1
                    .hand
                    .cards
                    .get(card_idx)
                    .cmp(&b2.hand.cards.get(card_idx))
                {
                    Ordering::Equal => {
                        continue;
                    }
                    Ordering::Greater => {
                        return Ordering::Greater;
                    }
                    Ordering::Less => {
                        return Ordering::Less;
                    }
                }
            }
            panic!("Unable to compare bids");
        }
    }
}

fn part_1(input: &Vec<String>) -> u64 {
    let mut bids: Vec<Bid> = input.iter().map(parse_bid).collect();
    bids.sort_by(compare);
    let mut result: u64 = 0;
    for (i, bid) in bids.into_iter().enumerate() {
        result += ((i + 1) * bid.amount as usize) as u64;
    }
    result
}

fn part_2(input: &Vec<String>) -> u64 {
    0
}
