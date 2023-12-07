use std::{cmp::Ordering, str::FromStr};

use crate::util::load;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
#[repr(u8)]
pub enum HandCategory {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPairs = 3,
    OnePair = 2,
    HighCard = 1,
}

#[derive(Debug, Eq)]
pub struct Hand {
    cards: Vec<u32>,
    bid: u32,
    category: HandCategory,
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn card2value(card: &char) -> u32 {
            match card {
                'T' => 10,
                'J' => 11,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => card.to_digit(10).unwrap(),
            }
        }
        let tokens: Vec<&str> = s.split_whitespace().collect();
        let cards = tokens[0]
            .chars()
            .map(|c| card2value(&c))
            .collect::<Vec<u32>>();
        let bid = tokens[1].parse::<u32>().unwrap();
        Ok(Hand {
            cards,
            bid,
            category: HandCategory::HighCard,
        })
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        Some(Ordering::Equal) == self.partial_cmp(other)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.category.partial_cmp(&other.category) {
            Some(Ordering::Equal) => {
                match self
                    .cards
                    .iter()
                    .zip(other.cards.iter())
                    .skip_while(|(a, b)| a == b)
                    .next()
                {
                    None => Some(Ordering::Equal),
                    Some((a, b)) => a.partial_cmp(&b),
                }
            }
            o => o,
        }
    }
}

impl Hand {
    /* Longest  Jokers  Longest+Jokers  Other  Pairs  Result
         5        0            5          0      2    Five-of-a-kind
         4        0            4          1      2    Four-of-a-kind
         4        1            5          0      2    Five-of-a-kind
         3        0            3          2      1    Three-of-a-kind
         3        0            3          2      2    Full house
         3        1            4          1      1    Four-of-a-kind
         3        2            5          0      1    Five-of-a-kind
         2        0            2          3      1    One pair
         2        0            2          3      2    Two pair
         2        1            3          2      1    Three-of-a-kind
         2        1            3          2      2    Full house
         2        2            4          1      1    Four-of-a-kind
         2        3            5          0      1    Five-of-a-kind
         1        0            1          4      0    High card
         1        1            2          3      0    One pair
         1        2            3          2      0    Three-of-a-kind
         1        3            4          1      0    Four-of-a-kind
         1        4            5          0      0    Five-of-a-kind
    */
    pub fn categorise(mut self) -> Self {
        let mut sorted = self.cards.clone();
        sorted.sort();
        let mut pair_count = 0;
        let mut longest_equals = 0;
        let mut num_equals = 1;
        let mut prev = sorted[0];
        let mut jokers = if prev == 1 { 1 } else { 0 };
        sorted.into_iter().skip(1).for_each(|c| {
            if c == 1 {
                jokers += 1;
            } else {
                if c == prev {
                    num_equals += 1;
                } else {
                    num_equals = 1;
                }
                if num_equals == 2 {
                    pair_count += 1;
                }
                if num_equals > longest_equals {
                    longest_equals = num_equals;
                }
            }
            prev = c;
        });
        self.category = if longest_equals + jokers == 5 {
            HandCategory::FiveOfAKind
        } else if longest_equals + jokers >= 4 {
            HandCategory::FourOfAKind
        } else if longest_equals + jokers >= 3 {
            if pair_count == 2 {
                HandCategory::FullHouse
            } else {
                HandCategory::ThreeOfAKind
            }
        } else if longest_equals + jokers >= 2 {
            if pair_count == 2 {
                HandCategory::TwoPairs
            } else {
                HandCategory::OnePair
            }
        } else {
            HandCategory::HighCard
        };
        self
    }
}

fn input() -> Vec<Hand> {
    load::<String>("data/day07.txt")
        .into_iter()
        .map(|line| line.parse::<Hand>().unwrap())
        .collect()
}

fn total_winnings(with_joker: bool) -> u32 {
    let mut hands: Vec<Hand> = input()
        .into_iter()
        .map(|mut h| {
            h.cards = h
                .cards
                .into_iter()
                .map(|c| if c == 11 && with_joker { 1 } else { c })
                .collect();
            h
        })
        .map(|h| h.categorise())
        .collect();
    hands.sort();
    hands
        .into_iter()
        .enumerate()
        .map(|(i, h)| (i + 1) as u32 * h.bid)
        .sum()
}

pub fn part1() -> u32 {
    total_winnings(false)
}

pub fn part2() -> u32 {
    total_winnings(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 246912307);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 246894760);
    }
}
