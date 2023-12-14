use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::iter::Zip;
use crate::ORDER;

pub fn main(file_content: &String) -> u32 {
    let mut lines = file_content.lines();

    let mut hand_and_bids: Vec<(Hand, i32)> = lines.map(|line| {
        let mut hand_and_bid = line.split_whitespace();
        let mut hand = hand_and_bid.next().unwrap();
        let bid = hand_and_bid.next().unwrap().parse::<i32>().unwrap();

        (Hand::new(hand), bid)
    }).collect();

    hand_and_bids.sort_by(|a, b| b.0.cmp(&a.0));

    hand_and_bids.reverse();

    let total_winnings: i32 = hand_and_bids.iter()
        .enumerate()
        .map(|(index, (hand, bid))| bid * (index as i32 + 1))
        .sum();

    total_winnings as u32
}

#[derive(Eq, PartialEq, PartialOrd)]
struct Hand {
    cards: Vec<char>,
    hand_type: HandType,
}


impl Hand {
    fn new(hand: &str) -> Hand {
        let mut cards = hand.chars().collect::<Vec<char>>();

        let hand_type = HandType::new(&cards);

        Hand {
            cards,
            hand_type,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand_type == other.hand_type {
            let mut zip = self.cards.iter().zip(other.cards.iter());

            let mut result = Ordering::Equal;

            for (a, b) in zip {
                let a_index = ORDER.iter().position(|&c| c == *a).unwrap();
                let b_index = ORDER.iter().position(|&c| c == *b).unwrap();

                match a_index.cmp(&b_index) {
                    Ordering::Less => {
                        result = Ordering::Greater;
                        break;
                    }
                    Ordering::Greater => {
                        result = Ordering::Less;
                        break;
                    }
                    Ordering::Equal => continue,
                };
            }

            result
        } else {
            self.hand_type.cmp(&other.hand_type)
        }
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    pub fn new(cards: &Vec<char>) -> HandType {
        let mut frequencies = HashMap::new();

        let mut cards = cards.clone();

        if cards == "JJJJJ".chars().collect::<Vec<char>>() {
            return HandType::FiveOfAKind;
        }

        for index in 0..cards.len() {
            let card = cards[index];
            if card == 'J' {
                let neighbor_index = match index {
                    0 => index + 1,
                    4 => index - 1,
                    _ => {
                        if cards[index - 1] == 'J' {
                            index + 1
                        } else if cards[index + 1] == 'J' {
                            index - 1
                        } else {
                            match cards[index - 1].cmp(&cards[index + 1]) {
                                Ordering::Less => index + 1,
                                Ordering::Greater => index - 1,
                                Ordering::Equal => index - 1,
                            }
                        }
                    }
                };

                *frequencies.entry(cards[neighbor_index]).or_insert(0) += 1;
                cards[index] = cards[neighbor_index];
            } else {
                *frequencies.entry(card).or_insert(0) += 1;
            }
        }

        let mut freq_values: Vec<_> = frequencies.values().collect();
        freq_values.sort_by(|a, b| b.cmp(a));

        match freq_values.as_slice() {
            [5] => HandType::FiveOfAKind,
            [4, _] => HandType::FourOfAKind,
            [3, 2] | [2, 3] => HandType::FullHouse,
            [3, 1, 1] => HandType::ThreeOfAKind,
            [2, 2, 1] => HandType::TwoPair,
            [2, 1, 1, 1] => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }
}
