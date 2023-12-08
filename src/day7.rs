use crate::util;
use std::{
    cmp::Ordering,
    collections::HashSet,
    time::{Duration, Instant},
};

/**
 * NOTE: To avoid too much copy/pasting, for part 2 I re-ranked 'J' to be lowest
 * to conform to the problem requirements. For part 1, J should be after T and before Q.
 */
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
enum Card {
    J,
    n2,
    n3,
    n4,
    n5,
    n6,
    n7,
    n8,
    n9,
    T,
    Q,
    K,
    A,
}

fn get(c: char) -> Card {
    return match c {
        '2' => Card::n2,
        '3' => Card::n3,
        '4' => Card::n4,
        '5' => Card::n5,
        '6' => Card::n6,
        '7' => Card::n7,
        '8' => Card::n8,
        '9' => Card::n9,
        'T' => Card::T,
        'J' => Card::J,
        'Q' => Card::Q,
        'K' => Card::K,
        'A' => Card::A,
        _ => {
            panic!("Invalid char for hand {:?}", c)
        }
    };
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

fn rank_jokers(h: &Vec<Card>) -> HandType {
    let joker_count = h.iter().filter(|c| **c == Card::J).count();
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

fn parse_bid(
    input: &String,
    hand_parser_strategy: fn(&str, fn(&Vec<Card>) -> HandType) -> Hand,
    hand_rank_strategy: fn(&Vec<Card>) -> HandType,
) -> Bid {
    let mut iter = input.split_ascii_whitespace();
    let hand = hand_parser_strategy(iter.next().unwrap(), hand_rank_strategy);
    let amount: u32 = iter.next().unwrap().parse().unwrap();
    Bid {
        hand: hand,
        amount: amount,
    }
}

fn parse_hand(h: &str, hand_rank_strategy: fn(&Vec<Card>) -> HandType) -> Hand {
    let hand: Vec<Card> = h.trim().chars().map(get).collect();
    let rank = hand_rank_strategy(&hand);
    Hand {
        cards: hand,
        hand_type: rank,
    }
}

fn compare(b1: &Bid, b2: &Bid) -> Ordering {
    match b1.hand.hand_type.cmp(&b2.hand.hand_type) {
        Ordering::Greater => Ordering::Greater,
        Ordering::Less => Ordering::Less,
        Ordering::Equal => {
            for card_idx in 0..5 {
                let b1_card = b1.hand.cards.get(card_idx);
                let b2_card = b2.hand.cards.get(card_idx);
                match b1_card.cmp(&b2_card) {
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

pub fn part_1(input: &Vec<String>) -> u64 {
    let mut bids: Vec<Bid> = input
        .iter()
        .map(|s| parse_bid(s, parse_hand, rank))
        .collect();
    bids.sort_by(compare);
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
    bids.sort_by(compare);
    let mut result: u64 = 0;
    for (i, bid) in bids.into_iter().enumerate() {
        result += (((i as u64) + 1) * bid.amount as u64) as u64;
    }
    result
}
