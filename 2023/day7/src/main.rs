use std::{collections::HashMap, io::stdin};

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: Vec<char>,
    bet: u64,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let tier_cmp = tier(self).cmp(&tier(other));
        if tier_cmp != std::cmp::Ordering::Equal {
            return tier_cmp;
        }
        for i in 0..5 {
            match card_val(self.cards[i]).cmp(&card_val(other.cards[i])) {
                std::cmp::Ordering::Equal => continue,
                x => return x,
            }
        }
        return std::cmp::Ordering::Equal;
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn tier(hand: &Hand) -> u64 {
    let mut map = HashMap::new();
    for card in hand.cards.iter() {
        let count = map.entry(card).or_insert(0);
        *count += 1;
    }

    let joker_count = *map.get(&'J').unwrap_or(&0);
    let tier = match map
        .values()
        .pad_using(5, |_| &0)
        .sorted()
        .rev()
        .collect_tuple()
        .unwrap()
    {
        (5, 0, 0, 0, 0) => 6,
        (4, 1, 0, 0, 0) => 5,
        (3, 2, 0, 0, 0) => 4,
        (3, _, _, _, _) => 3,
        (2, 2, _, _, _) => 2,
        (2, _, _, _, _) => 1,
        _ => 0,
    };
    if joker_count > 0 {
        return match (tier, joker_count) {
            (6, _) => 6,
            (5, _) => 6,
            (4, _) => 6,
            (3, _) => 5,
            (2, 1) => 4,
            (2, 2) => 5,
            (1, _) => 3,
            (0, _) => 1,
            _ => panic!("Invalid tier {:?} with joker count {}", tier, joker_count),
        };
    }
    return tier;
}

fn card_val(card: char) -> u64 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 1,
        'T' => 10,
        _ => card.to_digit(10).unwrap() as u64,
    }
}

fn main() {
    let mut hands: Vec<Hand> = stdin()
        .lines()
        .map(Result::unwrap)
        .map(|s| {
            let (cards, bet) = s.split_whitespace().collect_tuple().unwrap();
            return Hand {
                cards: cards.chars().collect(),
                bet: bet.parse().unwrap(),
            };
        })
        .collect();

    hands.sort();
    let mut result2 = 0;
    for i in 1..hands.len() + 1 {
        result2 += hands[i - 1].bet * i as u64;
    }
    println!("Part 1: {}", result2);
}
