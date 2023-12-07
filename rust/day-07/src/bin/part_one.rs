use std::cmp::Ordering;
use std::collections::HashMap;
use std::iter::zip;

const CARD_VALUES: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug)]
struct CamelHand {
    cards: String,
    bid: u32,
    hand_type: HandType,
}

impl CamelHand {
    fn from_str(str: &str) -> Self {
        let mut str_iter = str.split_whitespace();
        let cards = str_iter.next().unwrap();
        let hand_type = derive_hand_type(cards);
        let cards = String::from(cards);
        let bid = str_iter.next().unwrap().parse::<u32>().unwrap();

        Self {
            cards,
            bid,
            hand_type,
        }
    }
}

impl Ord for CamelHand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand_type > other.hand_type {
            return Ordering::Greater;
        }

        if self.hand_type < other.hand_type {
            return Ordering::Less;
        }

        for (self_card, other_card) in zip(self.cards.chars(), other.cards.chars()) {
            if CARD_VALUES.iter().position(|&val| val == self_card)
                > CARD_VALUES.iter().position(|&val| val == other_card)
            {
                return Ordering::Greater;
            } else if CARD_VALUES.iter().position(|&val| val == self_card)
                < CARD_VALUES.iter().position(|&val| val == other_card)
            {
                return Ordering::Less;
            }
        }

        Ordering::Equal
    }
}

impl PartialOrd for CamelHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for CamelHand {
    fn eq(&self, other: &Self) -> bool {
        (&self.hand_type, &self.cards) == (&other.hand_type, &other.cards)
    }
}

impl Eq for CamelHand {}

fn main() {
    let input = include_str!("input.txt");
    let output = part_one(input);
    dbg!(output);
}

fn part_one(input: &str) -> u32 {
    let mut hands: Vec<CamelHand> = input
        .lines()
        .map(|line| CamelHand::from_str(line))
        .collect();
    hands.sort();

    let result = hands
        .iter()
        .inspect(|x| println!("{:?}", x))
        .enumerate()
        .map(|(i, hand)| (i as u32 + 1) * hand.bid)
        .sum::<u32>();

    result
}

fn count_items(hand: &Vec<char>) -> HashMap<char, usize> {
    let mut card_map = HashMap::new();

    for card in hand {
        let count = card_map.entry(*card).or_insert(0);
        *count += 1;
    }

    card_map
}

fn derive_hand_type(hand: &str) -> HandType {
    let card_vec: Vec<char> = hand.chars().collect();
    let card_map = count_items(&card_vec);

    match card_map.len() {
        1 => HandType::FiveOfAKind,
        2 => {
            if card_map.values().any(|&card_count| card_count == 4) {
                HandType::FourOfAKind
            } else {
                HandType::FullHouse
            }
        }
        3 => {
            if card_map.values().any(|&card_count| card_count == 3) {
                HandType::ThreeOfAKind
            } else {
                HandType::TwoPair
            }
        }
        4 => HandType::OnePair,
        _ => HandType::HighCard,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(
            "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483",
        );
        assert_eq!(result, 6440)
    }
}
