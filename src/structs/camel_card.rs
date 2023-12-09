use std::cmp::Ordering;

/**
 * structs for camel cards game
 */
/**
 * NOTE: To avoid too much copy/pasting, for part 2 I re-ranked 'J' to be lowest
 * to conform to the problem requirements. For part 1, J should be after T and before Q.
 */
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum CamelCard {
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Hand {
    pub cards: Vec<CamelCard>,
    pub hand_type: HandType,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Bid {
    pub hand: Hand,
    pub amount: u32,
}

pub fn compare_bid(b1: &Bid, b2: &Bid) -> Ordering {
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

pub fn get(c: char) -> CamelCard {
    return match c {
        '2' => CamelCard::n2,
        '3' => CamelCard::n3,
        '4' => CamelCard::n4,
        '5' => CamelCard::n5,
        '6' => CamelCard::n6,
        '7' => CamelCard::n7,
        '8' => CamelCard::n8,
        '9' => CamelCard::n9,
        'T' => CamelCard::T,
        'J' => CamelCard::J,
        'Q' => CamelCard::Q,
        'K' => CamelCard::K,
        'A' => CamelCard::A,
        _ => {
            panic!("Invalid char for hand {:?}", c)
        }
    };
}

pub fn parse_bid(
    input: &String,
    hand_parser_strategy: fn(&str, fn(&Vec<CamelCard>) -> HandType) -> Hand,
    hand_rank_strategy: fn(&Vec<CamelCard>) -> HandType,
) -> Bid {
    let mut iter = input.split_ascii_whitespace();
    let hand = hand_parser_strategy(iter.next().unwrap(), hand_rank_strategy);
    let amount: u32 = iter.next().unwrap().parse().unwrap();
    Bid {
        hand: hand,
        amount: amount,
    }
}

pub fn parse_hand(h: &str, hand_rank_strategy: fn(&Vec<CamelCard>) -> HandType) -> Hand {
    let hand: Vec<CamelCard> = h.trim().chars().map(get).collect();
    let rank = hand_rank_strategy(&hand);
    Hand {
        cards: hand,
        hand_type: rank,
    }
}
